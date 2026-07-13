//! fix-tool-process — sandbox Self-Healing (paridad `fix_tool_process_core.py`).

use super::fractal_bus::load_radamanto_config;
use super::radamanto_batch_core::set_structure_valid;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

fn write_json_atomic(path: &Path, data: &Value) -> Result<(), String> {
    super::eda_bus_topology::write_json_atomic(path, data)
}

fn sandbox_root(repo: &Path) -> Result<PathBuf, String> {
    let cfg = load_radamanto_config(repo)?;
    let rel = cfg
        .get("sandbox_root")
        .and_then(|v| v.as_str())
        .unwrap_or(".SddIA/sandbox/");
    Ok(repo.join(rel.trim_start_matches("./")))
}

fn materialize_sandbox(repo: &Path, entity_id: &str, recovery_attempt: i64) -> Result<PathBuf, String> {
    let root = sandbox_root(repo)?
        .join(entity_id.replace(':', "_"))
        .join(recovery_attempt.to_string());
    fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    fs::write(
        root.join("fix-artifact.md"),
        format!("# Fix sandbox\n\nentity={entity_id}\nattempt={recovery_attempt}\n"),
    )
    .map_err(|e| e.to_string())?;
    Ok(root)
}

fn production_roots(repo: &Path) -> Vec<PathBuf> {
    vec![
        repo.join("SddIA/tools").canonicalize().unwrap_or_else(|_| repo.join("SddIA/tools")),
        repo.join("SddIA/skills")
            .canonicalize()
            .unwrap_or_else(|_| repo.join("SddIA/skills")),
    ]
}

fn assert_sandbox_write(repo: &Path, target: &Path, sandbox: &Path) -> Result<(), String> {
    let strict = !matches!(
        std::env::var("SDDIA_SANDBOX_STRICT")
            .unwrap_or_else(|_| "1".into())
            .trim()
            .to_lowercase()
            .as_str(),
        "0" | "false" | "no"
    );
    if !strict {
        return Ok(());
    }
    if target
        .canonicalize()
        .ok()
        .and_then(|t| sandbox.canonicalize().ok().map(|s| t.starts_with(&s)))
        .unwrap_or(false)
    {
        return Ok(());
    }
    for prod in production_roots(repo) {
        if target
            .canonicalize()
            .ok()
            .and_then(|t| prod.canonicalize().ok().map(|p| t.starts_with(&p)))
            .unwrap_or(false)
        {
            return Err(format!("write prohibido en produccion: {}", target.display()));
        }
    }
    Ok(())
}

fn run_argos_structure_gate(repo: &Path, sandbox: &Path, entity_id: &str) -> Result<Value, String> {
    let artifact = sandbox.join("fix-artifact.md");
    let structure_valid = artifact.is_file() && fs::metadata(&artifact).map(|m| m.len() > 0).unwrap_or(false);
    let gate = json!({
        "structure_valid": structure_valid,
        "entity_id": entity_id,
        "sandbox": sandbox.strip_prefix(repo).unwrap_or(sandbox).to_string_lossy().replace('\\', "/"),
        "emitter": "argos",
        "emits_status_restored": false,
    });
    write_json_atomic(&sandbox.join("argos_gate.json"), &gate)?;
    if structure_valid {
        set_structure_valid(repo, entity_id, true)?;
    }
    Ok(gate)
}

pub fn process_fix_tool(repo: &Path, rel_path: &str) -> Value {
    match process_fix_tool_inner(repo, rel_path) {
        Ok(v) => v,
        Err(e) => json!({"ok": false, "error": e}),
    }
}

fn process_fix_tool_inner(repo: &Path, rel_path: &str) -> Result<Value, String> {
    let event_path = repo.join(rel_path.trim());
    if !event_path.is_file() {
        return Err(format!("no existe: {rel_path}"));
    }
    let event: Value =
        serde_json::from_str(&fs::read_to_string(&event_path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;

    if event.get("event_type").and_then(|v| v.as_str()) != Some("Domain_Entity_Degraded") {
        return Err("solo Domain_Entity_Degraded inicia fix-tool-process".into());
    }
    let payload = event.get("payload").cloned().unwrap_or(json!({}));
    if !payload.is_object() {
        return Err("payload invalido".into());
    }
    let entity_id = payload
        .get("entity_id")
        .or_else(|| payload.get("target_entity_id"))
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    let entity_type = payload
        .get("entity_type")
        .and_then(|v| v.as_str())
        .unwrap_or("tool");
    if entity_type != "tool" {
        return Ok(json!({
            "ok": true,
            "skipped": true,
            "reason": format!("entity_type={entity_type} fuera de alcance fix-tool-process"),
            "entity_id": entity_id,
        }));
    }
    let attempt = payload
        .get("recovery_attempt")
        .and_then(|v| v.as_i64())
        .unwrap_or(1);
    let sandbox = materialize_sandbox(repo, &entity_id, attempt)?;
    assert_sandbox_write(repo, &sandbox.join("fix-artifact.md"), &sandbox)?;
    let gate = run_argos_structure_gate(repo, &sandbox, &entity_id)?;
    Ok(json!({
        "ok": true,
        "entity_id": entity_id,
        "sandbox": sandbox.strip_prefix(repo).unwrap_or(&sandbox).to_string_lossy().replace('\\', "/"),
        "argos_gate": gate,
        "status_restored_emitted": false,
    }))
}
