//! Lua bytecode cache for faster subsequent loads.
//!
//! Stores compiled Lua 5.1 bytecode on disk, keyed by content hash. Warm
//! startup can then skip reparsing and recompiling loader chunks entirely.
//!
//! The pack file header carries the current [`crate::lua_api::hot_literals::WHITELIST_VERSION`]
//! so that when the Track 3 slot ABI changes, stale entries are
//! rejected atomically rather than accidentally interpreted against the
//! new whitelist. Bumping `WHITELIST_VERSION` discards the entire pack
//! on next load.

use crate::lua_api::hot_literals::WHITELIST_VERSION;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::ffi::OsStr;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

const PACK_FILE: &str = "pack.bin";
// Bumped from `WOWBC001` when the version-header field was introduced.
// Old packs without a version header are rejected on load.
const PACK_MAGIC: &[u8; 8] = b"WOWBC002";
const PACK_HEADER_LEN: usize = PACK_MAGIC.len() + 4;
const MAX_PACK_SIZE: u64 = 768 * 1024 * 1024;

#[derive(Default)]
struct CacheState {
    initialized: bool,
    pack_exists: bool,
    values: Vec<u8>,
    index: HashMap<u64, (usize, usize)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PutResult {
    Stored,
    Unchanged,
    Failed,
}

fn cache_state() -> &'static Mutex<CacheState> {
    static STATE: OnceLock<Mutex<CacheState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(CacheState::default()))
}

/// Check if bytecode caching is disabled.
/// Result is cached after first check.
pub fn is_disabled() -> bool {
    static DISABLED: OnceLock<bool> = OnceLock::new();
    *DISABLED.get_or_init(|| {
        if let Ok(enable) = std::env::var("WOW_SIM_ENABLE_BYTECODE_CACHE") {
            return !(enable == "1" || enable.eq_ignore_ascii_case("true"));
        }

        std::env::var("WOW_SIM_DISABLE_BYTECODE_CACHE")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    })
}

/// Compute a cache key from file content and chunk name.
pub fn content_hash(bytes: &[u8], chunk_name: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    chunk_name.hash(&mut hasher);
    WHITELIST_VERSION.hash(&mut hasher);
    hasher.finish()
}

/// Legacy cache key used by standalone `.luac` files before the slot
/// ABI version became part of the hash.
pub fn legacy_content_hash(bytes: &[u8], chunk_name: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    chunk_name.hash(&mut hasher);
    hasher.finish()
}

fn cache_dir() -> Option<PathBuf> {
    Some(
        dirs::cache_dir()?
            .join("wow-ui-sim")
            .join("lua-bytecode")
            .join(cache_namespace()),
    )
}

fn cache_namespace() -> String {
    cache_namespace_for_manifest(env!("CARGO_MANIFEST_DIR"))
}

fn cache_namespace_for_manifest(manifest_dir: &str) -> String {
    let mut hasher = DefaultHasher::new();
    manifest_dir.hash(&mut hasher);
    format!("worktree-{:016x}", hasher.finish())
}

fn pack_path() -> Option<PathBuf> {
    Some(cache_dir()?.join(PACK_FILE))
}

/// Load cached bytecode and pass it to a callback.
///
/// Current-key hits borrow directly from the in-memory pack instead of cloning
/// the cached chunk. Legacy hits still clone once because promotion appends a
/// second entry under the current hash.
pub fn with_cached_bytecode<R>(
    hash: u64,
    legacy_hash: u64,
    callback: impl FnOnce(&[u8]) -> R,
) -> Option<R> {
    let mut state = cache_state().lock().ok()?;
    ensure_loaded(&mut state);
    with_cached_bytecode_from_state(&mut state, hash, legacy_hash, callback)
}

/// Save compiled bytecode to cache.
pub fn put(hash: u64, bytecode: &[u8]) -> PutResult {
    let mut state = match cache_state().lock() {
        Ok(state) => state,
        Err(_) => return PutResult::Failed,
    };
    ensure_loaded(&mut state);

    if let Some((offset, len)) = state.index.get(&hash).copied()
        && state.values[offset..offset + len] == *bytecode
    {
        return PutResult::Unchanged;
    }

    let offset = state.values.len();
    state.values.extend_from_slice(bytecode);
    state.index.insert(hash, (offset, bytecode.len()));
    match append_pack_entry(&mut state, hash, bytecode) {
        Ok(()) => PutResult::Stored,
        Err(_) => PutResult::Failed,
    }
}

fn ensure_loaded(state: &mut CacheState) {
    if state.initialized {
        return;
    }

    if let Some(pack) = pack_path() {
        state.pack_exists = load_pack_from_path(state, &pack);
    }

    if !state.pack_exists {
        let _ = migrate_legacy_cache(state);
    }

    state.initialized = true;
}

fn load_pack_from_path(state: &mut CacheState, pack: &Path) -> bool {
    let Ok(mut file) = std::fs::File::open(pack) else {
        return false;
    };

    if file.metadata().map(|meta| meta.len()).unwrap_or(0) > MAX_PACK_SIZE {
        drop(file);
        let _ = std::fs::remove_file(pack);
        return false;
    }

    let mut bytes = Vec::new();
    if file.read_to_end(&mut bytes).is_err() {
        return false;
    }

    let original_len = bytes.len();
    let Some(valid_len) = load_pack_bytes(state, bytes) else {
        // Pack file existed but was wrong magic or wrong whitelist version.
        // Remove it so the next write starts a clean file.
        drop(file);
        let _ = std::fs::remove_file(pack);
        return false;
    };

    if valid_len < original_len {
        drop(file);
        let _ = truncate_pack(pack, valid_len);
    }

    true
}

fn lookup_with_legacy_fallback(
    state: &mut CacheState,
    hash: u64,
    legacy_hash: u64,
) -> Option<Vec<u8>> {
    if let Some((offset, len)) = state.index.get(&hash).copied() {
        return Some(state.values[offset..offset + len].to_vec());
    }

    let (offset, len) = state.index.get(&legacy_hash).copied()?;
    let bytecode = state.values[offset..offset + len].to_vec();

    let promoted_offset = state.values.len();
    state.values.extend_from_slice(&bytecode);
    state.index.insert(hash, (promoted_offset, len));
    let _ = append_pack_entry(state, hash, &bytecode);

    Some(bytecode)
}

fn with_cached_bytecode_from_state<R>(
    state: &mut CacheState,
    hash: u64,
    legacy_hash: u64,
    callback: impl FnOnce(&[u8]) -> R,
) -> Option<R> {
    if let Some((offset, len)) = state.index.get(&hash).copied() {
        return Some(callback(&state.values[offset..offset + len]));
    }

    let bytecode = lookup_with_legacy_fallback(state, hash, legacy_hash)?;
    Some(callback(&bytecode))
}

fn load_pack_bytes(state: &mut CacheState, mut bytes: Vec<u8>) -> Option<usize> {
    if bytes.len() < PACK_HEADER_LEN || &bytes[..PACK_MAGIC.len()] != PACK_MAGIC {
        return None;
    }
    let version_bytes: [u8; 4] = bytes[PACK_MAGIC.len()..PACK_HEADER_LEN]
        .try_into()
        .expect("PACK_HEADER_LEN - PACK_MAGIC == 4");
    if u32::from_le_bytes(version_bytes) != WHITELIST_VERSION {
        // Slot ABI / whitelist version changed since this pack was
        // written. Discard the whole pack so fresh entries replace it.
        return None;
    }

    let mut index = HashMap::new();

    let mut pos = PACK_HEADER_LEN;
    while pos + 12 <= bytes.len() {
        let entry_start = pos;
        let hash = u64::from_le_bytes(bytes[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let len = u32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap()) as usize;
        pos += 4;
        if pos + len > bytes.len() {
            bytes.truncate(entry_start);
            state.values = bytes;
            state.index = index;
            return Some(entry_start);
        }
        index.insert(hash, (pos, len));
        pos += len;
    }

    bytes.truncate(pos);
    state.values = bytes;
    state.index = index;
    Some(pos)
}

fn truncate_pack(path: &Path, len: usize) -> std::io::Result<()> {
    std::fs::OpenOptions::new()
        .write(true)
        .open(path)?
        .set_len(len as u64)
}

fn migrate_legacy_cache(state: &mut CacheState) -> std::io::Result<()> {
    let Some(dir) = cache_dir() else {
        return Ok(());
    };
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Ok(());
    };

    let migrated = collect_legacy_cache_entries(state, entries);
    write_migrated_pack(state, &dir, migrated)
}

fn collect_legacy_cache_entries(
    state: &mut CacheState,
    entries: std::fs::ReadDir,
) -> Vec<(u64, Vec<u8>)> {
    let mut migrated = Vec::new();
    for entry in entries.flatten() {
        let Some((hash, bytecode)) = legacy_cache_entry(&entry.path()) else {
            continue;
        };
        let offset = state.values.len();
        state.values.extend_from_slice(&bytecode);
        state.index.insert(hash, (offset, bytecode.len()));
        migrated.push((hash, bytecode));
    }
    migrated
}

fn legacy_cache_entry(path: &Path) -> Option<(u64, Vec<u8>)> {
    if path.file_name() == Some(OsStr::new(PACK_FILE)) {
        return None;
    }
    if path.extension() != Some(OsStr::new("luac")) {
        return None;
    }

    let stem = path.file_stem()?.to_str()?;
    let hash = u64::from_str_radix(stem, 16).ok()?;
    let bytecode = std::fs::read(path).ok()?;
    Some((hash, bytecode))
}

fn write_migrated_pack(
    state: &mut CacheState,
    dir: &Path,
    migrated: Vec<(u64, Vec<u8>)>,
) -> std::io::Result<()> {
    if migrated.is_empty() {
        return Ok(());
    }

    let Some(pack) = pack_path() else {
        return Ok(());
    };
    std::fs::create_dir_all(&dir)?;
    let mut file = std::fs::File::create(&pack)?;
    write_pack_header(&mut file)?;
    for (hash, bytecode) in migrated {
        write_pack_entry(&mut file, hash, &bytecode)?;
    }
    state.pack_exists = true;
    Ok(())
}

fn append_pack_entry(state: &mut CacheState, hash: u64, bytecode: &[u8]) -> std::io::Result<()> {
    let Some(dir) = cache_dir() else {
        return Ok(());
    };
    let Some(path) = pack_path() else {
        return Ok(());
    };
    std::fs::create_dir_all(&dir)?;

    let mut file = if state.pack_exists {
        std::fs::OpenOptions::new().append(true).open(&path)?
    } else {
        let mut file = std::fs::File::create(&path)?;
        write_pack_header(&mut file)?;
        state.pack_exists = true;
        file
    };

    write_pack_entry(&mut file, hash, bytecode)
}

fn write_pack_header(file: &mut std::fs::File) -> std::io::Result<()> {
    file.write_all(PACK_MAGIC)?;
    file.write_all(&WHITELIST_VERSION.to_le_bytes())?;
    Ok(())
}

fn write_pack_entry(file: &mut std::fs::File, hash: u64, bytecode: &[u8]) -> std::io::Result<()> {
    file.write_all(&hash.to_le_bytes())?;
    file.write_all(&(bytecode.len() as u32).to_le_bytes())?;
    file.write_all(bytecode)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Arbitrary sentinel hash used in the load-path tests. Any u64 will
    /// do; the value only exists so we can assert the entry landed at
    /// the expected offset/length in the in-memory cache state.
    const SENTINEL_HASH: u64 = 0xdead_beef_cafe_babe;

    fn synth_pack_bytes(magic: &[u8; 8], version: u32, entries: &[(u64, &[u8])]) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(magic);
        buf.extend_from_slice(&version.to_le_bytes());
        for (hash, bytecode) in entries {
            buf.extend_from_slice(&hash.to_le_bytes());
            buf.extend_from_slice(&(bytecode.len() as u32).to_le_bytes());
            buf.extend_from_slice(bytecode);
        }
        buf
    }

    #[test]
    fn load_pack_bytes_accepts_current_version_header() {
        let bytes = synth_pack_bytes(PACK_MAGIC, WHITELIST_VERSION, &[(SENTINEL_HASH, b"xyz")]);
        let expected_len = bytes.len();
        let payload_offset = PACK_HEADER_LEN + 8 + 4;
        let mut state = CacheState::default();
        assert_eq!(load_pack_bytes(&mut state, bytes), Some(expected_len));
        assert_eq!(
            state.index.get(&SENTINEL_HASH).copied(),
            Some((payload_offset, 3))
        );
        assert_eq!(&state.values[payload_offset..payload_offset + 3], b"xyz");
    }

    #[test]
    fn load_pack_bytes_indexes_payloads_in_place() {
        let bytes = synth_pack_bytes(PACK_MAGIC, WHITELIST_VERSION, &[(SENTINEL_HASH, b"xyz")]);
        let expected_len = bytes.len();
        let payload_offset = PACK_HEADER_LEN + 8 + 4;
        let mut state = CacheState::default();

        assert_eq!(load_pack_bytes(&mut state, bytes), Some(expected_len));
        assert_eq!(
            state.index.get(&SENTINEL_HASH).copied(),
            Some((payload_offset, 3))
        );
        assert_eq!(&state.values[payload_offset..payload_offset + 3], b"xyz");
    }

    #[test]
    fn load_pack_bytes_rejects_mismatched_whitelist_version() {
        let stale_version = WHITELIST_VERSION.wrapping_add(1);
        let bytes = synth_pack_bytes(PACK_MAGIC, stale_version, &[(1, b"z")]);
        let mut state = CacheState::default();
        assert_eq!(load_pack_bytes(&mut state, bytes), None);
        assert!(state.index.is_empty());
        assert!(state.values.is_empty());
    }

    #[test]
    fn load_pack_bytes_rejects_legacy_wowbc001_magic() {
        // Packs written before the version header must be discarded on
        // load so they don't get re-interpreted against the new layout.
        let legacy_magic = *b"WOWBC001";
        let bytes = synth_pack_bytes(&legacy_magic, WHITELIST_VERSION, &[(1, b"z")]);
        let mut state = CacheState::default();
        assert_eq!(load_pack_bytes(&mut state, bytes), None);
    }

    #[test]
    fn load_pack_bytes_rejects_truncated_header() {
        // File shorter than magic + version — can't even check the
        // version. Reject rather than crash on the slice try_into.
        let mut bytes = Vec::new();
        bytes.extend_from_slice(PACK_MAGIC);
        bytes.extend_from_slice(&[0u8; 3]); // only 3 of 4 version bytes
        let mut state = CacheState::default();
        assert_eq!(load_pack_bytes(&mut state, bytes), None);
    }

    #[test]
    fn load_pack_bytes_keeps_valid_prefix_before_torn_entry() {
        let mut bytes = synth_pack_bytes(PACK_MAGIC, WHITELIST_VERSION, &[(SENTINEL_HASH, b"xyz")]);
        bytes.extend_from_slice(&2_u64.to_le_bytes());
        bytes.extend_from_slice(&16_u32.to_le_bytes());
        bytes.extend_from_slice(b"partial");

        let mut state = CacheState::default();
        assert_eq!(
            load_pack_bytes(&mut state, bytes),
            Some(PACK_HEADER_LEN + 12 + 3)
        );
        let payload_offset = PACK_HEADER_LEN + 8 + 4;
        assert_eq!(
            state.index.get(&SENTINEL_HASH).copied(),
            Some((payload_offset, 3))
        );
        assert_eq!(&state.values[payload_offset..payload_offset + 3], b"xyz");
    }

    #[test]
    fn content_hash_changes_with_whitelist_version() {
        let base = content_hash(b"abc", "=@chunk");
        let mut hasher = DefaultHasher::new();
        b"abc".hash(&mut hasher);
        "=@chunk".hash(&mut hasher);
        WHITELIST_VERSION.wrapping_add(1).hash(&mut hasher);
        let stale = hasher.finish();
        assert_ne!(base, stale);
    }

    #[test]
    fn cache_namespace_distinguishes_parallel_worktrees() {
        let retail = cache_namespace_for_manifest("/syncthing/Sync/Projects/wow/wow-ui-sim");
        let classic =
            cache_namespace_for_manifest("/syncthing/Sync/Projects/wow/wow-ui-sim-classic");

        assert_ne!(retail, classic);
        assert!(retail.starts_with("worktree-"));
        assert!(classic.starts_with("worktree-"));
    }

    #[test]
    fn legacy_content_hash_matches_pre_versioned_key() {
        let legacy = legacy_content_hash(b"abc", "=@chunk");
        let mut hasher = DefaultHasher::new();
        b"abc".hash(&mut hasher);
        "=@chunk".hash(&mut hasher);
        assert_eq!(legacy, hasher.finish());
    }

    #[test]
    fn max_pack_size_allows_full_addon_warm_cache() {
        let full_addon_pack_budget = 512 * 1024 * 1024;

        assert!(
            MAX_PACK_SIZE >= full_addon_pack_budget,
            "full addon cache observed at 454 MiB; cap must keep it reusable"
        );
    }

    #[test]
    fn put_reports_stored_and_unchanged_entries() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let hash = content_hash(format!("source-{unique}").as_bytes(), "=@put-test");

        assert_eq!(put(hash, b"compiled"), PutResult::Stored);
        assert_eq!(put(hash, b"compiled"), PutResult::Unchanged);
    }

    #[test]
    fn legacy_lookup_promotes_entry_under_current_hash() {
        let mut state = CacheState::default();
        let legacy_hash = legacy_content_hash(b"abc", "=@chunk");
        let current_hash = content_hash(b"abc", "=@chunk");

        state.values.extend_from_slice(b"compiled");
        state.index.insert(legacy_hash, (0, b"compiled".len()));

        let loaded = lookup_with_legacy_fallback(&mut state, current_hash, legacy_hash)
            .expect("legacy entry should be found");
        assert_eq!(loaded, b"compiled");
        assert_eq!(
            state.index.get(&current_hash).copied(),
            Some((b"compiled".len(), b"compiled".len()))
        );
        assert_eq!(&state.values[b"compiled".len()..], b"compiled");
    }

    #[test]
    fn with_cached_bytecode_borrows_current_hits() {
        let mut state = CacheState::default();
        state.values.extend_from_slice(b"current-bytecode");
        state.index.insert(SENTINEL_HASH, (0, state.values.len()));
        let pack_ptr = state.values.as_ptr();

        let borrowed =
            with_cached_bytecode_from_state(&mut state, SENTINEL_HASH, SENTINEL_HASH, |bytes| {
                bytes.as_ptr() == pack_ptr
            })
            .expect("current cache hit should call callback");

        assert!(
            borrowed,
            "current cache hits should pass a borrowed slice from the in-memory pack"
        );
    }
}
