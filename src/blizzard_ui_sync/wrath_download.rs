//! Pinned Wrath source acquisition; no CASC UI-source fallback.
use std::{
    fs, io,
    path::{Path, PathBuf},
};
const PIN: &str = include_str!("../../data/blizzard-ui-builds/wrath.json");
const PROVENANCE: &str = ".wow-ui-sim-wrath-source.json";

pub(super) fn acquire_wrath_source() -> io::Result<PathBuf> {
    let cache = dirs::cache_dir().ok_or_else(|| error("cannot determine cache directory"))?;
    acquire_snapshot_at(&cache.join("wow-ui-sim/wow-ui-source/wrath"))
}

fn acquire_snapshot_at(snapshot: &Path) -> io::Result<PathBuf> {
    let revision = read_pinned_revision()?;
    if read_existing_snapshot(snapshot, &revision)? {
        return Ok(snapshot.to_owned());
    }
    fetch_fresh_snapshot(snapshot, &revision)?;
    Ok(snapshot.to_owned())
}

fn error(cause: impl std::fmt::Display) -> io::Error {
    io::Error::other(format!("Wrath source: {cause}"))
}

fn read_pinned_revision() -> io::Result<String> {
    let pin: serde_json::Value = serde_json::from_str(PIN).map_err(error)?;
    let revision = pin["gethe_revision"]
        .as_str()
        .ok_or_else(|| error("pin has no gethe_revision"))?;
    if revision.len() != 40 || !revision.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(error("pin has an invalid Gethe revision"));
    }
    Ok(revision.to_owned())
}

fn read_existing_snapshot(snapshot: &Path, revision: &str) -> io::Result<bool> {
    match fs::symlink_metadata(snapshot) {
        Err(cause) if cause.kind() == io::ErrorKind::NotFound => return Ok(false),
        Err(cause) => return Err(error(format!("inspect {}: {cause}", snapshot.display()))),
        Ok(_) => {}
    }
    inspect_existing_snapshot(snapshot, revision).map_err(|cause| {
        error(format!(
            "refusing existing snapshot {}; left untouched: {cause}",
            snapshot.display()
        ))
    })?;
    Ok(true)
}

fn inspect_existing_snapshot(snapshot: &Path, revision: &str) -> io::Result<()> {
    let bytes = fs::read(snapshot.join(PROVENANCE))?;
    let provenance: serde_json::Value = serde_json::from_slice(&bytes).map_err(error)?;
    if provenance["gethe_revision"].as_str() != Some(revision) {
        return Err(error("source revision differs from committed pin"));
    }
    if !snapshot.join("AddOns").is_dir() || !snapshot.join("FrameXML/FrameXML.toc").is_file() {
        return Err(error("source requires AddOns and FrameXML/FrameXML.toc"));
    }
    Ok(())
}

#[cfg(not(feature = "casc"))]
fn fetch_fresh_snapshot(snapshot: &Path, _revision: &str) -> io::Result<()> {
    Err(error(format!(
        "{} is absent; acquisition requires the casc feature",
        snapshot.display()
    )))
}

#[cfg(feature = "casc")]
fn fetch_fresh_snapshot(snapshot: &Path, revision: &str) -> io::Result<()> {
    let url = format!("https://codeload.github.com/Gethe/wow-ui-source/tar.gz/{revision}");
    let archive = fetch_archive(&url).map_err(|cause| error(format!("fetch {url}: {cause}")))?;
    let parent = snapshot
        .parent()
        .ok_or_else(|| error("snapshot has no parent"))?;
    fs::create_dir_all(parent)?;
    super::wrath_snapshot::extract_wrath_snapshot(&archive, snapshot)
}

#[cfg(feature = "casc")]
const ATTEMPTS: u32 = 3;

#[cfg(feature = "casc")]
fn fetch_archive(url: &str) -> io::Result<Vec<u8>> {
    use std::time::Duration;
    let client = reqwest::blocking::Client::builder()
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(60))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(error)?;
    for attempt in 0..ATTEMPTS {
        let response = match client.get(url).send() {
            Ok(response) => response,
            Err(cause) => {
                wait_before_transport_retry(cause, attempt)?;
                continue;
            }
        };
        if response.status() == reqwest::StatusCode::OK {
            match response.bytes() {
                Ok(bytes) => return Ok(bytes.to_vec()),
                Err(cause) => wait_before_transport_retry(cause, attempt)?,
            }
        } else {
            wait_before_status_retry(response, attempt)?;
        }
    }
    Err(error("download attempts exhausted"))
}

#[cfg(feature = "casc")]
fn wait_before_transport_retry(cause: reqwest::Error, attempt: u32) -> io::Result<()> {
    let transient = cause.is_connect() || cause.is_timeout();
    if !transient || attempt + 1 == ATTEMPTS {
        return Err(error(cause));
    }
    let delay = super::pinned_download::retry_delay(None, attempt).map_err(error)?;
    eprintln!("Retrying Wrath source after {cause} in {delay:?}");
    std::thread::sleep(delay);
    Ok(())
}

#[cfg(feature = "casc")]
fn wait_before_status_retry(response: reqwest::blocking::Response, attempt: u32) -> io::Result<()> {
    let status = response.status();
    let transient = matches!(status.as_u16(), 408 | 429) || status.is_server_error();
    if !transient || attempt + 1 == ATTEMPTS {
        return Err(error(format!("HTTP {status}; expected 200")));
    }
    let retry_after = response
        .headers()
        .get(reqwest::header::RETRY_AFTER)
        .map(|value| value.to_str())
        .transpose()
        .map_err(error)?;
    let delay = super::pinned_download::retry_delay(retry_after, attempt).map_err(error)?;
    drop(response);
    eprintln!("Retrying Wrath source after HTTP {status} in {delay:?}");
    std::thread::sleep(delay);
    Ok(())
}

#[cfg(test)]
#[path = "wrath_download_tests.rs"]
mod tests;

#[cfg(all(test, feature = "casc"))]
#[path = "wrath_download_http_tests.rs"]
mod http_tests;
