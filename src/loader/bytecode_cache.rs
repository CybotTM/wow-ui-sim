//! Lua bytecode cache for faster subsequent loads.
//!
//! Stores compiled Lua 5.1 bytecode on disk, keyed by content hash. Warm
//! startup can then skip reparsing and recompiling loader chunks entirely.

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::ffi::OsStr;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

const CACHE_DIR: &str = ".cache/lua-bytecode";
const PACK_FILE: &str = "pack.bin";
const PACK_MAGIC: &[u8; 8] = b"WOWBC001";
const MAX_PACK_SIZE: u64 = 100 * 1024 * 1024;

#[derive(Default)]
struct CacheState {
    initialized: bool,
    pack_exists: bool,
    values: Vec<u8>,
    index: HashMap<u64, (usize, usize)>,
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
    hasher.finish()
}

fn cache_dir() -> PathBuf {
    PathBuf::from(CACHE_DIR)
}

fn pack_path() -> PathBuf {
    cache_dir().join(PACK_FILE)
}

/// Load cached bytecode for the given content hash.
pub fn get(hash: u64) -> Option<Vec<u8>> {
    let mut state = cache_state().lock().ok()?;
    ensure_loaded(&mut state);
    let (offset, len) = *state.index.get(&hash)?;
    Some(state.values[offset..offset + len].to_vec())
}

/// Save compiled bytecode to cache.
pub fn put(hash: u64, bytecode: &[u8]) {
    let mut state = match cache_state().lock() {
        Ok(state) => state,
        Err(_) => return,
    };
    ensure_loaded(&mut state);

    if let Some((offset, len)) = state.index.get(&hash).copied()
        && state.values[offset..offset + len] == *bytecode
    {
        return;
    }

    let offset = state.values.len();
    state.values.extend_from_slice(bytecode);
    state.index.insert(hash, (offset, bytecode.len()));
    let _ = append_pack_entry(&mut state, hash, bytecode);
}

fn ensure_loaded(state: &mut CacheState) {
    if state.initialized {
        return;
    }

    let pack = pack_path();
    if let Ok(mut file) = std::fs::File::open(&pack) {
        let file_size = file.metadata().map(|meta| meta.len()).unwrap_or(0);
        if file_size > MAX_PACK_SIZE {
            drop(file);
            let _ = std::fs::remove_file(&pack);
        } else {
            let mut bytes = Vec::new();
            if file.read_to_end(&mut bytes).is_ok() && load_pack_bytes(state, &bytes) {
                state.pack_exists = true;
            }
        }
    }

    if !state.pack_exists {
        let _ = migrate_legacy_cache(state);
    }

    state.initialized = true;
}

fn load_pack_bytes(state: &mut CacheState, bytes: &[u8]) -> bool {
    if bytes.len() < PACK_MAGIC.len() || &bytes[..PACK_MAGIC.len()] != PACK_MAGIC {
        return false;
    }

    state.values.clear();
    state.index.clear();

    let mut pos = PACK_MAGIC.len();
    while pos + 12 <= bytes.len() {
        let hash = u64::from_le_bytes(bytes[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let len = u32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap()) as usize;
        pos += 4;
        if pos + len > bytes.len() {
            state.values.clear();
            state.index.clear();
            return false;
        }
        let offset = state.values.len();
        state.values.extend_from_slice(&bytes[pos..pos + len]);
        state.index.insert(hash, (offset, len));
        pos += len;
    }

    pos == bytes.len()
}

fn migrate_legacy_cache(state: &mut CacheState) -> std::io::Result<()> {
    let dir = cache_dir();
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Ok(());
    };

    let mut migrated = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.file_name() == Some(OsStr::new(PACK_FILE)) {
            continue;
        }
        if path.extension() != Some(OsStr::new("luac")) {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let Ok(hash) = u64::from_str_radix(stem, 16) else {
            continue;
        };
        let Ok(bytecode) = std::fs::read(&path) else {
            continue;
        };
        let offset = state.values.len();
        state.values.extend_from_slice(&bytecode);
        state.index.insert(hash, (offset, bytecode.len()));
        migrated.push((hash, bytecode));
    }

    if migrated.is_empty() {
        return Ok(());
    }

    std::fs::create_dir_all(&dir)?;
    let mut file = std::fs::File::create(pack_path())?;
    file.write_all(PACK_MAGIC)?;
    for (hash, bytecode) in migrated {
        write_pack_entry(&mut file, hash, &bytecode)?;
    }
    state.pack_exists = true;
    Ok(())
}

fn append_pack_entry(state: &mut CacheState, hash: u64, bytecode: &[u8]) -> std::io::Result<()> {
    std::fs::create_dir_all(cache_dir())?;
    let path = pack_path();

    let mut file = if state.pack_exists {
        std::fs::OpenOptions::new().append(true).open(&path)?
    } else {
        let mut file = std::fs::File::create(&path)?;
        file.write_all(PACK_MAGIC)?;
        state.pack_exists = true;
        file
    };

    write_pack_entry(&mut file, hash, bytecode)
}

fn write_pack_entry(file: &mut std::fs::File, hash: u64, bytecode: &[u8]) -> std::io::Result<()> {
    file.write_all(&hash.to_le_bytes())?;
    file.write_all(&(bytecode.len() as u32).to_le_bytes())?;
    file.write_all(bytecode)?;
    Ok(())
}
