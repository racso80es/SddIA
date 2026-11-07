//! FsEventAdapter — EventManagerPort sobre árbol de ficheros (eda.event_manager v1).

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sddia_core::ports::event_manager::{
    AppendTraceRequest, ClaimPendingRequest, ClaimPendingResult, ClaimedEvent, DeliveryStatus,
    DualVersionManifest, EventManagerError, EventManagerPort, GetEventRequest, GetEventResult,
    HandshakeVersionRequest, HandshakeVersionResult, ListedDlqEvent, ListDlqRequest, ListDlqResult,
    ListPendingRequest, ListPendingResult, ListedPendingEvent, ListTracesRequest, ListTracesResult,
    MoveToDlqRequest, PublishEcstRequest, PublishEcstResult, PurgeEventRequest, RecordDeliveryRequest,
    ReplayDlqRequest, CONTRACT_ID, CONTRACT_VERSION,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredEvent {
    event_id: String,
    ecst: Value,
    trace_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LeaseRec {
    event_id: String,
    owner: String,
    until_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DlqRec {
    event: StoredEvent,
    reason: String,
}

pub struct FsEventAdapter {
    root: PathBuf,
}

impl FsEventAdapter {
    pub fn open(root: impl AsRef<Path>) -> Result<Self, EventManagerError> {
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(&root).map_err(storage)?;
        for d in ["events", "leases", "deliveries", "dlq", "traces", "meta"] {
            fs::create_dir_all(root.join(d)).map_err(storage)?;
        }
        let a = Self { root };
        a.seal_manifest()?;
        Ok(a)
    }

    fn dir(&self, name: &str) -> PathBuf {
        self.root.join(name)
    }

    fn event_path(&self, id: &str) -> PathBuf {
        self.dir("events").join(format!("{id}.json"))
    }

    fn lease_path(&self, id: &str) -> PathBuf {
        self.dir("leases").join(format!("{id}.json"))
    }

    fn dlq_path(&self, id: &str) -> PathBuf {
        self.dir("dlq").join(format!("{id}.json"))
    }

    fn manifest(&self) -> DualVersionManifest {
        DualVersionManifest::builtin_v1("1.0.0")
    }

    fn write_atomic_json(path: &Path, val: &impl Serialize) -> Result<(), EventManagerError> {
        let _parent = path.parent().ok_or_else(|| EventManagerError::Storage("no parent".into()))?;
        let tmp = path.with_extension("json.part");
        let bytes = serde_json::to_vec_pretty(val).map_err(json_err)?;
        fs::write(&tmp, bytes).map_err(storage)?;
        fs::rename(&tmp, path).map_err(storage)?;
        Ok(())
    }

    fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, EventManagerError> {
        let raw = fs::read_to_string(path).map_err(storage)?;
        serde_json::from_str(&raw).map_err(json_err)
    }

    fn seal_manifest(&self) -> Result<(), EventManagerError> {
        let p = self.dir("meta").join("manifest.json");
        Self::write_atomic_json(&p, &self.manifest())
    }

    fn now_ms() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0)
    }

    fn expire_leases(&self) -> Result<(), EventManagerError> {
        let now = Self::now_ms();
        let leases_dir = self.dir("leases");
        for ent in fs::read_dir(&leases_dir).map_err(storage)? {
            let ent = ent.map_err(storage)?;
            let path = ent.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let rec: LeaseRec = match Self::read_json(&path) {
                Ok(r) => r,
                Err(_) => continue,
            };
            if rec.until_ms <= now {
                let _ = fs::remove_file(&path);
            }
        }
        Ok(())
    }

    pub fn load_traces(&self, trace_id: &str) -> Result<Vec<Value>, EventManagerError> {
        let p = self.dir("traces").join(format!("{trace_id}.json"));
        if !p.is_file() {
            return Ok(Vec::new());
        }
        Self::read_json(&p)
    }
}

fn storage<E: std::fmt::Display>(e: E) -> EventManagerError {
    EventManagerError::Storage(e.to_string())
}

fn json_err<E: std::fmt::Display>(e: E) -> EventManagerError {
    EventManagerError::Storage(format!("json: {e}"))
}

impl EventManagerPort for FsEventAdapter {
    fn publish_ecst(
        &self,
        req: PublishEcstRequest,
    ) -> Result<PublishEcstResult, EventManagerError> {
        let stored = StoredEvent {
            event_id: req.event_id.clone(),
            ecst: req.ecst,
            trace_id: req.trace_id,
        };
        let ep = self.event_path(&req.event_id);
        if ep.is_file() {
            return Err(EventManagerError::InvalidRequest(format!(
                "duplicate {}",
                req.event_id
            )));
        }
        let dp = self.dlq_path(&req.event_id);
        if dp.is_file() {
            return Err(EventManagerError::InvalidRequest(format!(
                "duplicate in dlq {}",
                req.event_id
            )));
        }
        FsEventAdapter::write_atomic_json(&ep, &stored)?;
        Ok(PublishEcstResult {
            event_id: req.event_id,
        })
    }

    fn claim_pending(
        &self,
        req: ClaimPendingRequest,
    ) -> Result<ClaimPendingResult, EventManagerError> {
        self.expire_leases()?;
        let now = FsEventAdapter::now_ms();
        let ttl = req.lease_ttl_ms.unwrap_or(30_000);
        let mut taken = Vec::new();
        let mut ids: Vec<String> = fs::read_dir(self.dir("events"))
            .map_err(storage)?
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                e.path()
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string())
            })
            .collect();
        ids.sort();
        for id in ids {
            if taken.len() as u32 >= req.limit {
                break;
            }
            let lp = self.lease_path(&id);
            if lp.is_file() {
                continue;
            }
            let ep = self.event_path(&id);
            let stored: StoredEvent = FsEventAdapter::read_json(&ep)?;
            let lease = LeaseRec {
                event_id: id.clone(),
                owner: req.lease_owner.clone(),
                until_ms: now.saturating_add(ttl),
            };
            FsEventAdapter::write_atomic_json(&lp, &lease)?;
            taken.push(ClaimedEvent {
                event_id: stored.event_id,
                ecst: stored.ecst,
                trace_id: stored.trace_id,
                lease_owner: req.lease_owner.clone(),
            });
        }
        Ok(ClaimPendingResult { events: taken })
    }

    fn record_delivery(&self, req: RecordDeliveryRequest) -> Result<(), EventManagerError> {
        let del = self.dir("deliveries").join(format!("{}.json", req.delivery_id));
        if del.is_file() {
            let prev: DeliveryStatus = FsEventAdapter::read_json(&del)?;
            if prev == DeliveryStatus::Ack && req.status == DeliveryStatus::Ack {
                return Ok(());
            }
        }
        FsEventAdapter::write_atomic_json(&del, &req.status)?;

        match req.status {
            DeliveryStatus::Ack => {
                let _ = fs::remove_file(self.lease_path(&req.event_id));
                let _ = fs::remove_file(self.event_path(&req.event_id));
            }
            DeliveryStatus::Nack => {
                let _ = fs::remove_file(self.lease_path(&req.event_id));
            }
        }
        Ok(())
    }

    fn move_to_dlq(&self, req: MoveToDlqRequest) -> Result<(), EventManagerError> {
        let ep = self.event_path(&req.event_id);
        let stored: StoredEvent = FsEventAdapter::read_json(&ep)
            .map_err(|_| EventManagerError::NotFound(req.event_id.clone()))?;
        let _ = fs::remove_file(self.lease_path(&req.event_id));
        fs::remove_file(&ep).map_err(storage)?;
        let rec = DlqRec {
            event: stored,
            reason: req.reason,
        };
        FsEventAdapter::write_atomic_json(&self.dlq_path(&req.event_id), &rec)?;
        Ok(())
    }

    fn replay_dlq(&self, req: ReplayDlqRequest) -> Result<(), EventManagerError> {
        let ids: Vec<String> = if req.all.unwrap_or(false) {
            fs::read_dir(self.dir("dlq"))
                .map_err(storage)?
                .filter_map(|e| e.ok())
                .filter_map(|e| {
                    e.path()
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .map(|s| s.to_string())
                })
                .collect()
        } else if let Some(id) = req.event_id.clone() {
            vec![id]
        } else {
            return Err(EventManagerError::InvalidRequest(
                "replay_dlq requires event_id or all=true".into(),
            ));
        };

        for id in ids {
            let dp = self.dlq_path(&id);
            let rec: DlqRec = FsEventAdapter::read_json(&dp)
                .map_err(|_| EventManagerError::NotFound(id.clone()))?;
            fs::remove_file(&dp).map_err(storage)?;
            FsEventAdapter::write_atomic_json(&self.event_path(&id), &rec.event)?;
        }
        Ok(())
    }

    fn append_trace(&self, req: AppendTraceRequest) -> Result<(), EventManagerError> {
        let p = self.dir("traces").join(format!("{}.json", req.trace_id));
        let mut frames: Vec<Value> = if p.is_file() {
            FsEventAdapter::read_json(&p)?
        } else {
            Vec::new()
        };
        frames.push(req.frame);
        FsEventAdapter::write_atomic_json(&p, &frames)?;
        Ok(())
    }

    fn handshake_version(
        &self,
        req: HandshakeVersionRequest,
    ) -> Result<HandshakeVersionResult, EventManagerError> {
        sddia_core::ports::event_manager::version_gate(&req.requested_range, CONTRACT_VERSION)?;
        Ok(HandshakeVersionResult {
            contract: CONTRACT_ID.to_string(),
            contract_version: CONTRACT_VERSION.to_string(),
            compatible: true,
        })
    }

    fn get_event(&self, req: GetEventRequest) -> Result<GetEventResult, EventManagerError> {
        let ep = self.event_path(&req.event_id);
        if ep.is_file() {
            let stored: StoredEvent = FsEventAdapter::read_json(&ep)?;
            return Ok(GetEventResult {
                event_id: stored.event_id,
                ecst: stored.ecst,
                trace_id: stored.trace_id,
            });
        }
        let dp = self.dlq_path(&req.event_id);
        if dp.is_file() {
            let rec: DlqRec = FsEventAdapter::read_json(&dp)?;
            return Ok(GetEventResult {
                event_id: rec.event.event_id,
                ecst: rec.event.ecst,
                trace_id: rec.event.trace_id,
            });
        }
        Err(EventManagerError::NotFound(req.event_id))
    }

    fn list_pending(
        &self,
        req: ListPendingRequest,
    ) -> Result<ListPendingResult, EventManagerError> {
        let limit = req.limit.unwrap_or(u32::MAX) as usize;
        let events_dir = self.dir("events");
        let leases_dir = self.dir("leases");
        let mut out = Vec::new();
        for entry in fs::read_dir(&events_dir).map_err(storage)? {
            if out.len() >= limit {
                break;
            }
            let entry = entry.map_err(storage)?;
            let stem = entry
                .path()
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            if leases_dir.join(format!("{stem}.json")).is_file() {
                continue;
            }
            let stored: StoredEvent = FsEventAdapter::read_json(&entry.path())?;
            out.push(ListedPendingEvent {
                event_id: stored.event_id,
                ecst: stored.ecst,
                trace_id: stored.trace_id,
            });
        }
        Ok(ListPendingResult { events: out })
    }

    fn list_dlq(&self, req: ListDlqRequest) -> Result<ListDlqResult, EventManagerError> {
        let limit = req.limit.unwrap_or(u32::MAX) as usize;
        let dlq_dir = self.dir("dlq");
        let mut out = Vec::new();
        for entry in fs::read_dir(&dlq_dir).map_err(storage)? {
            if out.len() >= limit {
                break;
            }
            let entry = entry.map_err(storage)?;
            let id = entry
                .path()
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            let rec: DlqRec = FsEventAdapter::read_json(&entry.path())?;
            out.push(ListedDlqEvent {
                event_id: id,
                reason: rec.reason,
                ecst: rec.event.ecst,
            });
        }
        Ok(ListDlqResult { events: out })
    }

    fn list_traces(&self, req: ListTracesRequest) -> Result<ListTracesResult, EventManagerError> {
        let p = self.dir("traces").join(format!("{}.json", req.trace_id));
        let frames: Vec<Value> = if p.is_file() {
            FsEventAdapter::read_json(&p)?
        } else {
            Vec::new()
        };
        Ok(ListTracesResult { frames })
    }

    fn purge_event(&self, req: PurgeEventRequest) -> Result<(), EventManagerError> {
        let ep = self.event_path(&req.event_id);
        let dp = self.dlq_path(&req.event_id);
        let lp = self.dir("leases").join(format!("{}.json", req.event_id));
        let removed = ep.is_file() || dp.is_file();
        if !removed {
            return Err(EventManagerError::NotFound(req.event_id));
        }
        if ep.is_file() {
            fs::remove_file(&ep).map_err(storage)?;
        }
        if dp.is_file() {
            fs::remove_file(&dp).map_err(storage)?;
        }
        if lp.is_file() {
            fs::remove_file(&lp).map_err(storage)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sddia_core::ports::event_manager::run_event_manager_contract_suite;
    use tempfile::tempdir;

    #[test]
    fn contract_suite_on_fs_tempdir() {
        let dir = tempdir().expect("tempdir");
        let adapter = FsEventAdapter::open(dir.path()).expect("open");
        run_event_manager_contract_suite(&adapter);
        let adapter2 = FsEventAdapter::open(dir.path()).expect("reopen");
        let _ = adapter2.manifest();
    }
}
