//! Puente `sddia_event_store` solo para laboratorio (`#[cfg(test)]`).

#[cfg(test)]
mod imp {
    use std::collections::HashSet;
    use std::fs;
    use std::path::{Path, PathBuf};

    use serde_json::Value;
    use sddia_core::ports::event_manager::{EventFamily, PublishSeal};
    use sddia_event_store::{EventStore, Family, StoreMode};

    use crate::engine::eda_port;

    fn events_family_dir(repo: &Path, family: &str) -> PathBuf {
        repo.join(format!(".events/{}", family.trim_start_matches("./")))
    }

    pub fn publish_ecst(
        repo: &Path,
        family: &str,
        event: &Value,
    ) -> Result<PublishSeal, String> {
        let mode = sddia_event_store::store_mode();
        if sddia_event_store::writes_filesystem(mode) {
            let dir = events_family_dir(repo, family);
            let seal = sddia_event_store::publish_ecst(repo, family, event, Some(&dir))
                .map_err(|e| e.to_string())?;
            let uri_family = EventFamily::from_publish_token(family)?;
            return PublishSeal::from_parts(uri_family, seal.event_id);
        }
        Err("legacy publish_ecst requiere modo filesystem".into())
    }

    pub fn record_ecst(repo: &Path, family: &str, event: &Value) -> Result<(), String> {
        let mode = sddia_event_store::store_mode();
        if sddia_event_store::writes_filesystem(mode) && !sddia_event_store::writes_store(mode) {
            let dir = events_family_dir(repo, family);
            sddia_event_store::publish_ecst(repo, family, event, Some(&dir))
                .map(|_| ())
                .map_err(|e| e.to_string())
        } else if sddia_event_store::writes_filesystem(mode) {
            publish_ecst(repo, family, event).map(|_| ())
        } else {
            Err("legacy record_ecst requiere modo filesystem".into())
        }
    }

    pub fn load_dispatched(
        repo: &Path,
        event_id: &str,
        logical: &Path,
    ) -> Result<Option<Value>, String> {
        if sddia_event_store::store_mode() == StoreMode::Filesystem {
            return sddia_event_store::load_dispatched_event(repo, event_id, logical)
                .map_err(|e| e.to_string());
        }
        Err("legacy load_dispatched requiere modo filesystem".into())
    }

    pub fn append_progress_leaf_fs(
        repo: &Path,
        correlation_id: &str,
        frame: &Value,
    ) -> Result<u64, String> {
        use crate::engine::fractal::load_progress_dir;
        let rel = load_progress_dir(repo).trim_start_matches("./").to_string();
        let leaf = repo.join(&rel).join(correlation_id);
        fs::create_dir_all(&leaf).map_err(|e| e.to_string())?;
        let trace_id = frame
            .get("trace_id")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .unwrap_or("frame");
        let path = leaf.join(format!("{trace_id}.json"));
        fs::write(
            &path,
            serde_json::to_string(frame).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        let n = fs::read_dir(&leaf).map(|d| d.count()).unwrap_or(1);
        Ok(n as u64)
    }

    pub fn append_progress(repo: &Path, correlation_id: &str, frame: &Value) -> Result<u64, String> {
        let mode = sddia_event_store::store_mode();
        if sddia_event_store::writes_filesystem(mode) {
            return append_progress_leaf_fs(repo, correlation_id, frame);
        }
        sddia_event_store::append_progress_monotonic(repo, correlation_id, frame)
            .map_err(|e| e.to_string())
    }

    pub fn load_domain_ecsts(repo: &Path, limit: u32) -> Result<Vec<Value>, String> {
        let mode = sddia_event_store::store_mode();
        if sddia_event_store::writes_filesystem(mode) {
            let dirs = [
                repo.join(".events/pending"),
                repo.join(".events/processed"),
                repo.join(".events/dead-letter"),
            ];
            return sddia_event_store::load_domain_ecsts(repo, &dirs.to_vec())
                .map_err(|e| e.to_string())
                .map(|rows| rows.into_iter().take(limit as usize).collect());
        }

        let mut out = Vec::new();
        let mut seen = HashSet::new();
        if sddia_event_store::writes_store(mode) {
            if let Ok(store) = EventStore::open(eda_port::runtime_db_path(repo)) {
                for row in store.list_family(Family::Domain).unwrap_or_default() {
                    let id = row
                        .ecst
                        .get("event_id")
                        .and_then(|v| v.as_str())
                        .unwrap_or(row.event_id.as_str())
                        .to_string();
                    if seen.insert(id) {
                        out.push(row.ecst);
                    }
                    if out.len() >= limit as usize {
                        return Ok(out);
                    }
                }
            }
        }
        Ok(out.into_iter().take(limit as usize).collect())
    }
}

#[cfg(test)]
pub use imp::*;
