//! Reactor piloto EDA DI — CapabilityDi_Requested → resolve→gate→Cerbero (PBI-042 Hito 3 — R6).

use super::capability_di_gate;
use super::capability_di_resolver::{self, ResolvedBinding};
use super::cerbero_di_envelope;
use super::cerbero_di_rbac::{self, resolve_requester_policies};
use super::eda_bus_topology::{
    ensure_event_bus_topology, iso_now, load_event_bus_topology, write_json_atomic,
};
use crate::core::resolver::ProcessDef;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

const EDA_PILOT_ENV: &str = "SDDIA_DI_EDA_PILOT";

pub fn env_eda_pilot() -> bool {
    std::env::var(EDA_PILOT_ENV)
        .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

pub fn is_eda_pilot_phase(phase: &Value) -> bool {
    if env_eda_pilot() {
        return phase_has_di_requirement(phase);
    }
    phase
        .get("di_composition")
        .and_then(|v| v.as_str())
        .map(|s| s.trim() == "eda_pilot")
        .unwrap_or(false)
        && phase_has_di_requirement(phase)
}

fn phase_has_di_requirement(phase: &Value) -> bool {
    phase
        .get("requires_capability")
        .and_then(|v| v.as_array())
        .map(|a| !a.is_empty())
        .unwrap_or(false)
}

fn pending_dir(repo: &Path) -> Result<PathBuf, String> {
    let topo = load_event_bus_topology(repo)?;
    Ok(repo.join(topo.pending.trim_start_matches("./")))
}

fn processed_dir(repo: &Path) -> Result<PathBuf, String> {
    let topo = load_event_bus_topology(repo)?;
    Ok(repo.join(topo.processed.trim_start_matches("./")))
}

/// Emite `CapabilityDi_Requested` en `./.events/pending/`. Retorna `event_id`.
pub fn emit_di_requested(
    repo: &Path,
    phase: &Value,
    process_name: &str,
    process_inputs: &Value,
) -> Result<String, String> {
    let _ = ensure_event_bus_topology(repo)?;
    let event_id = Uuid::new_v4().to_string();
    let phase_name = phase.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let event = json!({
        "event_id": event_id,
        "event_type": "CapabilityDi_Requested",
        "timestamp": iso_now(),
        "emitter_agent": "execute-process",
        "payload": {
            "correlation_id": process_inputs.get("correlation_id").cloned().unwrap_or(Value::Null),
            "process_name": process_name,
            "phase_name": phase_name,
            "execution_id": process_inputs.get("execution_id").cloned().unwrap_or(Value::Null),
            "persist_ref": process_inputs.get("persist_ref").cloned().unwrap_or(Value::Null),
            "requires_capability": phase.get("requires_capability").cloned().unwrap_or(json!([])),
            "di_composition": "eda_pilot"
        },
        "delivery_state": {}
    });
    let path = pending_dir(repo)?.join(format!("{event_id}.json"));
    write_json_atomic(&path, &event)?;
    Ok(event_id)
}

struct ChainOutcome {
    bindings: Vec<ResolvedBinding>,
    chain_status: &'static str,
    di_gate_code: Option<String>,
    cerbero_di_code: Option<String>,
    cerbero_envelope_di_code: Option<String>,
    error: Option<String>,
}

fn run_sync_chain(
    repo: &Path,
    phase: &Value,
    process_name: &str,
    process_def: &ProcessDef,
    process_inputs: &Value,
) -> ChainOutcome {
    let phase_name = phase.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let policies = resolve_requester_policies(process_def, process_inputs);

    let bindings = match capability_di_resolver::resolve_phase_bindings(repo, phase) {
        Ok(b) => b,
        Err(e) => {
            capability_di_resolver::write_resolve_dead_letter(repo, &e, phase_name, process_name);
            return ChainOutcome {
                bindings: vec![],
                chain_status: "failed",
                di_gate_code: None,
                cerbero_di_code: None,
                cerbero_envelope_di_code: None,
                error: Some(e.message),
            };
        }
    };

    let effective = capability_di_resolver::phase_with_effective_delegates(phase, &bindings);

    if let Err(e) =
        capability_di_gate::validate_phase_capability_di(repo, &effective, process_name)
    {
        capability_di_gate::write_di_dead_letter(repo, &e, phase_name, process_name);
        return ChainOutcome {
            bindings,
            chain_status: "failed",
            di_gate_code: Some(e.code.as_str().to_string()),
            cerbero_di_code: None,
            cerbero_envelope_di_code: None,
            error: Some(e.message),
        };
    }

    if let Err(e) = cerbero_di_rbac::validate_di_rbac(
        repo,
        process_name,
        phase_name,
        &policies,
        &bindings,
    ) {
        cerbero_di_rbac::write_cerbero_di_dead_letter(repo, &e, phase_name, process_name);
        return ChainOutcome {
            bindings,
            chain_status: "failed",
            di_gate_code: None,
            cerbero_di_code: Some(e.code.as_str().to_string()),
            cerbero_envelope_di_code: None,
            error: Some(e.message),
        };
    }

    let packaged: Vec<Value> = bindings
        .iter()
        .map(capability_di_resolver::di_binding_object)
        .collect();
    if let Err(e) = cerbero_di_envelope::validate_packaged_bindings(repo, &bindings, &packaged) {
        cerbero_di_envelope::write_cerbero_envelope_dead_letter(repo, &e, phase_name, process_name);
        return ChainOutcome {
            bindings,
            chain_status: "failed",
            di_gate_code: None,
            cerbero_di_code: None,
            cerbero_envelope_di_code: Some(e.code.as_str().to_string()),
            error: Some(e.message),
        };
    }

    ChainOutcome {
        bindings,
        chain_status: "resolved",
        di_gate_code: None,
        cerbero_di_code: None,
        cerbero_envelope_di_code: None,
        error: None,
    }
}

fn emit_di_resolved(repo: &Path, request_event_id: &str, outcome: &ChainOutcome) -> Result<(), String> {
    let event_id = Uuid::new_v4().to_string();
    let di_bindings: Vec<Value> = outcome
        .bindings
        .iter()
        .map(capability_di_resolver::di_binding_object)
        .collect();
    let event = json!({
        "event_id": event_id,
        "event_type": "CapabilityDi_Resolved",
        "timestamp": iso_now(),
        "emitter_agent": "capability-di-reactor",
        "payload": {
            "request_event_id": request_event_id,
            "di_bindings": di_bindings,
            "chain_status": outcome.chain_status,
            "di_gate_code": outcome.di_gate_code,
            "cerbero_di_code": outcome.cerbero_di_code,
            "cerbero_envelope_di_code": outcome.cerbero_envelope_di_code,
            "error": outcome.error,
        },
        "delivery_state": { "ecst_ack": true }
    });
    let path = processed_dir(repo)?.join(format!("{event_id}.json"));
    write_json_atomic(&path, &event)
}

/// Procesa eventos `CapabilityDi_Requested` pendientes (helper test/CI).
pub fn drain_di_reactor_once(repo: &Path, process_def: &ProcessDef) -> Result<Value, String> {
    let pending = pending_dir(repo)?;
    if !pending.is_dir() {
        return Ok(json!({"processed": 0}));
    }
    let mut processed = 0;
    let mut last = json!({});
    let entries: Vec<PathBuf> = fs::read_dir(&pending)
        .map_err(|e| e.to_string())?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("json"))
        .collect();

    for path in entries {
        let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let event: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
        if event.get("event_type").and_then(|v| v.as_str()) != Some("CapabilityDi_Requested") {
            continue;
        }
        let event_id = event
            .get("event_id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let payload = event.get("payload").cloned().unwrap_or(json!({}));
        let process_name = payload
            .get("process_name")
            .and_then(|v| v.as_str())
            .unwrap_or("feature");
        let phase = json!({
            "name": payload.get("phase_name").unwrap_or(&json!("")),
            "requires_capability": payload.get("requires_capability").unwrap_or(&json!([])),
        });
        let inputs = json!({
            "execution_id": payload.get("execution_id"),
            "persist_ref": payload.get("persist_ref"),
            "correlation_id": payload.get("correlation_id"),
        });
        let outcome = run_sync_chain(repo, &phase, process_name, process_def, &inputs);
        emit_di_resolved(repo, &event_id, &outcome)?;
        fs::remove_file(&path).ok();
        processed += 1;
        last = json!({
            "request_event_id": event_id,
            "chain_status": outcome.chain_status,
            "di_gate_code": outcome.di_gate_code,
            "cerbero_di_code": outcome.cerbero_di_code,
            "cerbero_envelope_di_code": outcome.cerbero_envelope_di_code,
        });
    }
    Ok(json!({"processed": processed, "last": last}))
}

/// Dispara reactor en hilo separado (non-blocking para orquestador).
pub fn spawn_reactor_background(repo: PathBuf, process_def: ProcessDef) {
    std::thread::spawn(move || {
        let _ = drain_di_reactor_once(&repo, &process_def);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::parser::parse_frontmatter;
    use std::io::Write;

    fn write_file(dir: &Path, rel: &str, body: &str) {
        let p = dir.join(rel);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let mut f = fs::File::create(p).unwrap();
        f.write_all(body.as_bytes()).unwrap();
    }

    fn fixture_repo() -> tempfile::TempDir {
        let td = tempfile::tempdir().unwrap();
        let root = td.path();
        write_file(
            root,
            "SddIA/core/cumulo.paths.json",
            r#"{
  "version": "1.5.3",
  "directories": {
    "library_norms": "SddIA/library/norms/",
    "capability_contracts": "SddIA/library/norms/capability-contracts/"
  },
  "capability_di": {
    "bindings": "SddIA/core/capability-bindings.md"
  },
  "normative_documents": {
    "capability_taxonomy": "SddIA/library/norms/capability-taxonomy.md"
  },
  "event_bus": "./.events",
  "eda_bus": {
    "pending": "./.events/pending",
    "processed": "./.events/processed",
    "dead_letter": "./.events/dead-letter"
  }
}"#,
        );
        write_file(
            root,
            "SddIA/core/capability-bindings.md",
            r#"---
bindings:
  - capability_id: "doc:closure"
    contract: "doc.closure"
    provider: "skill:filesystem-manager"
---
"#,
        );
        write_file(
            root,
            "SddIA/library/norms/capability-taxonomy.md",
            r#"---
catalog:
  - id: "doc:closure"
    contract: "doc.closure"
    version: "1.0.0"
---
"#,
        );
        write_file(
            root,
            "SddIA/library/norms/capability-contracts/doc.closure.schema.json",
            r#"{"type":"object","required":["exitCode","data"],"properties":{"exitCode":{"type":"integer"},"data":{"type":"object"}}}"#,
        );
        write_file(
            root,
            "SddIA/library/norms/capability-contracts/di.binding.schema.json",
            include_str!("../../../../library/norms/capability-contracts/di.binding.schema.json"),
        );
        write_file(
            root,
            "SddIA/skills/filesystem-manager.md",
            r#"---
context: filesystem-ops
provides:
  - id: "doc:closure"
    contract: "doc.closure"
    version: "1.0.0"
outputs:
  - exitCode: "0"
  - data: "x"
---
"#,
        );
        td
    }

    fn process_def() -> ProcessDef {
        let repo = crate::core::repo::find_repo_root().expect("repo");
        let path = crate::core::resolver::resolve_process_path(&repo, "feature").expect("feature");
        parse_frontmatter(&path).expect("feature fm")
    }

    #[test]
    fn ac_r6_emit_pending_non_blocking() {
        let td = fixture_repo();
        std::env::set_var(EDA_PILOT_ENV, "1");
        let phase = json!({
            "name": "lab-eda",
            "requires_capability": [{
                "id": "doc:closure",
                "contract": "doc.closure",
                "version": ">=1.0.0"
            }],
            "di_composition": "eda_pilot"
        });
        let inputs = json!({"execution_id": "exec-1", "persist_ref": "docs/features/x"});
        let event_id = emit_di_requested(td.path(), &phase, "feature", &inputs).expect("emit");
        let pending_file = td.path().join(format!(".events/pending/{event_id}.json"));
        assert!(pending_file.is_file(), "evento en pending");
        let processed_json = fs::read_dir(td.path().join(".events/processed"))
            .map(|d| {
                d.flatten()
                    .filter(|e| {
                        e.path()
                            .extension()
                            .and_then(|x| x.to_str())
                            == Some("json")
                    })
                    .count()
            })
            .unwrap_or(0);
        assert_eq!(processed_json, 0, "orquestador no espera reactor");
        std::env::remove_var(EDA_PILOT_ENV);
    }

    #[test]
    fn ac_r6_drain_resolved_ecst_ack() {
        let td = fixture_repo();
        let phase = json!({
            "name": "lab",
            "requires_capability": [{"id": "doc:closure", "contract": "doc.closure"}]
        });
        let event_id = emit_di_requested(td.path(), &phase, "feature", &json!({})).expect("emit");
        let def = process_def();
        let out = drain_di_reactor_once(td.path(), &def).expect("drain");
        assert_eq!(out["processed"], 1);
        assert!(
            !td.path()
                .join(format!(".events/pending/{event_id}.json"))
                .is_file()
        );
        let processed: Vec<_> = fs::read_dir(td.path().join(".events/processed"))
            .unwrap()
            .flatten()
            .map(|e| e.path())
            .collect();
        assert!(!processed.is_empty());
        let text = fs::read_to_string(&processed[0]).unwrap();
        let ev: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(ev["event_type"], "CapabilityDi_Resolved");
        assert_eq!(ev["delivery_state"]["ecst_ack"], true);
    }
}
