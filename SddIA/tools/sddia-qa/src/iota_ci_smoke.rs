use execute_process::core::env::load_hierarchical_env;
use execute_process::engine::eda_bus_topology::{
    ensure_event_bus_topology, list_witnesses, EventBusTopology,
};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use std::process::Command;
use uuid::Uuid;

use crate::resolve::{has_flag, print_json_report, resolve_event_watcher, run_json_cmd};

const FIXTURE_REL: &str = "docs/features/e1-iota-ci/_smoke-iota-ci-merged.json";
const IOTA_SUBSCRIBER: &str = "cumulo.iota-immutable-publisher";

fn iso_now() -> String {
    chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

fn truthy_env(name: &str) -> bool {
    matches!(
        std::env::var(name).unwrap_or_default().to_lowercase().as_str(),
        "1" | "true" | "yes"
    )
}

fn wallet_available(repo: &Path) -> bool {
    if !std::env::var("IOTA_WALLET_SECRET").unwrap_or_default().trim().is_empty() {
        return true;
    }
    let wallet = repo.join(".SddIA/.dev/wallet.key");
    wallet.is_file()
        && fs::read_to_string(&wallet)
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false)
}

fn load_fixture(repo: &Path) -> Result<Value, String> {
    let path = repo.join(FIXTURE_REL);
    let text = fs::read_to_string(&path).map_err(|_| format!("fixture ausente: {FIXTURE_REL}"))?;
    let template: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let event_id = Uuid::new_v4().to_string();
    let correlation = template
        .get("correlation_id")
        .and_then(|v| v.as_str())
        .unwrap_or("iota-ci-smoke");
    let mut payload = template
        .get("payload")
        .cloned()
        .unwrap_or(json!({}));
    if let Some(obj) = payload.as_object_mut() {
        obj.entry("merge_commit_hash")
            .or_insert(json!("0".repeat(40)));
        obj.entry("source_branch")
            .or_insert(json!("feat/e1-iota-ci-smoke"));
        obj.entry("target_branch").or_insert(json!("main"));
        obj.entry("author")
            .or_insert(json!("iota-ci-smoke@sddia.local"));
        obj.entry("security_clearance")
            .or_insert(json!("lab-smoke"));
    }
    Ok(json!({
        "event_id": event_id,
        "event_type": "PullRequest_Merged",
        "timestamp": iso_now(),
        "emitter_agent": "iota-ci-smoke",
        "correlation_id": format!("{correlation}-{}", &event_id[..8.min(event_id.len())]),
        "payload": payload,
        "delivery_state": {},
    }))
}

fn write_pending(repo: &Path, bus: &EventBusTopology, event: &Value) -> Result<String, String> {
    let event_id = event
        .get("event_id")
        .and_then(|v| v.as_str())
        .ok_or("event_id requerido")?;
    let pending = repo.join(&bus.pending);
    fs::create_dir_all(&pending).map_err(|e| e.to_string())?;
    let dest = pending.join(format!("{event_id}.json"));
    fs::write(
        &dest,
        format!(
            "{}\n",
            serde_json::to_string_pretty(event).map_err(|e| e.to_string())?
        ),
    )
    .map_err(|e| e.to_string())?;
    Ok(dest
        .strip_prefix(repo)
        .unwrap_or(&dest)
        .to_string_lossy()
        .replace('\\', "/"))
}

fn route_event(repo: &Path, rel_path: &str) -> Result<Value, String> {
    let watcher = resolve_event_watcher(repo)?;
    let mut cmd = Command::new(&watcher);
    cmd.args(["--event-file-path", rel_path]).current_dir(repo);
    run_json_cmd(&mut cmd)
}

fn cleanup_smoke(repo: &Path, bus: &EventBusTopology, event_id: &str) {
    for key in ["pending", "processing", "processed", "dead_letter"] {
        let rel = match key {
            "pending" => &bus.pending,
            "processing" => &bus.processing,
            "processed" => &bus.processed,
            "dead_letter" => &bus.dead_letter,
            _ => continue,
        };
        let _ = fs::remove_file(repo.join(rel).join(format!("{event_id}.json")));
    }
    for state_key in [
        "processing_subscribers",
        "processed_subscribers",
        "dead_letter_subscribers",
    ] {
        for p in list_witnesses(repo, bus, state_key, event_id) {
            let _ = fs::remove_file(p);
        }
    }
}

pub fn run(repo: &Path, args: &[String]) -> i32 {
    let simulate = has_flag(args, "--simulate");
    let require_physical = has_flag(args, "--require-physical");
    let json_out = has_flag(args, "--json") || !has_flag(args, "--no-json");

    let result = (|| -> Result<Value, String> {
        load_hierarchical_env(repo)?;
        if require_physical {
            if simulate {
                return Err("--simulate incompatible con --require-physical".into());
            }
            std::env::remove_var("SDDIA_LAB_SIMULATE_IOTA");
            if truthy_env("SDDIA_LAB_SIMULATE_IOTA") {
                return Err("SDDIA_LAB_SIMULATE_IOTA activo — aborto modo físico".into());
            }
            if !wallet_available(repo) {
                return Err("IOTA_WALLET_SECRET o .SddIA/.dev/wallet.key requeridos".into());
            }
        } else if simulate {
            std::env::set_var("SDDIA_LAB_SIMULATE_IOTA", "1");
        }

        let event = load_fixture(repo)?;
        let event_id = event
            .get("event_id")
            .and_then(|v| v.as_str())
            .ok_or("event_id")?
            .to_string();
        let bus = ensure_event_bus_topology(repo)?;
        let rel = write_pending(repo, &bus, &event)?;

        let route_out = route_event(repo, &rel)?;
        let data = route_out.get("data").cloned().unwrap_or(json!({}));
        let delivery_status = data.get("delivery_status").cloned().unwrap_or(json!({}));
        let cumulo_status = delivery_status.get(IOTA_SUBSCRIBER).and_then(|v| v.as_str());
        if cumulo_status != Some("success") {
            return Err(format!(
                "{IOTA_SUBSCRIBER} status={cumulo_status:?} delivery_status={delivery_status}"
            ));
        }
        let digest = data.get("transaction_digest").and_then(|v| v.as_str()).map(str::to_string);
        let mode = if require_physical {
            "physical"
        } else if simulate {
            "simulate"
        } else {
            "default"
        };
        if require_physical {
            let d = digest.as_deref().unwrap_or("");
            if d.is_empty() {
                return Err("transaction_digest ausente en modo físico".into());
            }
            if d.starts_with("lab-sim-") {
                return Err(format!("digest simulado en modo físico: {d}"));
            }
        }
        let witnesses = list_witnesses(repo, &bus, "processed_subscribers", &event_id);
        let witness_ok = witnesses.iter().any(|p| p.to_string_lossy().contains(IOTA_SUBSCRIBER));
        cleanup_smoke(repo, &bus, &event_id);
        Ok(json!({
            "success": true,
            "mode": mode,
            "event_id": event_id,
            "event_type": event.get("event_type"),
            "delivery_status": delivery_status,
            "transaction_digest": digest,
            "witness_processed": witness_ok,
            "parent_path": data.get("parent_path"),
        }))
    })();

    match result {
        Ok(report) => {
            print_json_report(&report, json_out);
            0
        }
        Err(e) => {
            let err = json!({"success": false, "error": e});
            print_json_report(&err, json_out);
            eprintln!("run-iota-ci-smoke: {e}");
            1
        }
    }
}