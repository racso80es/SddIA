//! Handler nativo `sync-client-assets` (PBI-KALMA2-MVP-01B). Tubería H4 + aduana SHA-256.

use super::super::capsules::invoke_tool_capsule_json;
use super::super::progress_trace;
use super::super::thermodynamic;
use super::super::workspace::bootstrap_workspace;
use crate::envelope::OrchestratorEnvelope;
use chrono::Utc;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;
use uuid::Uuid;

fn iso_now() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

fn str_field(inputs: &Value, key: &str) -> Result<String, String> {
    inputs
        .get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .ok_or_else(|| format!("{key} obligatorio"))
}

fn correlation_id(inputs: &Value) -> String {
    inputs
        .get("correlation_id")
        .or_else(|| inputs.get("execution_id"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| Uuid::new_v4().to_string())
}

fn instance_library_root(repo: &Path, asset_family: &str) -> PathBuf {
    let sub = match asset_family {
        "library_norms" => "library/norms",
        _ => "library/codexes",
    };
    repo.join(".SddIA").join(sub)
}

fn genome_codexes_dir(repo: &Path) -> PathBuf {
    repo.join("SddIA/library/codexes")
}

fn sha256_hex(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("sha256:{}", hex::encode(hasher.finalize()))
}

fn parse_index_row(line: &str) -> Option<(String, String, String)> {
    if !line.starts_with('|') || line.contains("---") || line.contains("Archivo fuente") {
        return None;
    }
    let cols: Vec<&str> = line.split('|').map(str::trim).collect();
    if cols.len() < 4 {
        return None;
    }
    let file = cols[1].trim_matches('`').to_string();
    let uuid = cols[2].trim_matches('`').to_string();
    let name = cols[3].to_string();
    if file.is_empty() || uuid.is_empty() {
        return None;
    }
    Some((file, uuid, name))
}

fn resolve_genome_asset(repo: &Path, asset_id: &str, asset_family: &str) -> Result<(String, String), String> {
    if asset_family != "library_codexes" {
        return Err(format!("asset_family no soportado: {asset_family}"));
    }
    let index = genome_codexes_dir(repo).join("index.md");
    let text = fs::read_to_string(&index).map_err(|e| format!("leer codexes index: {e}"))?;
    let needle = asset_id.to_ascii_lowercase();
    for line in text.lines() {
        if let Some((file, uuid, _name)) = parse_index_row(line) {
            if uuid.eq_ignore_ascii_case(&needle)
                || file.trim_end_matches(".md").eq_ignore_ascii_case(&needle)
                || file.contains(&needle)
            {
                let rel = format!("SddIA/library/codexes/{file}");
                return Ok((rel, file));
            }
        }
    }
    if asset_id.ends_with(".md") {
        let rel = format!("SddIA/library/codexes/{asset_id}");
        return Ok((rel, asset_id.to_string()));
    }
    let guess = format!("codex-{asset_id}.md");
    let rel = format!("SddIA/library/codexes/{guess}");
    if repo.join(&rel).is_file() {
        return Ok((rel, guess));
    }
    Err(format!("asset_id no resuelto: {asset_id}"))
}

fn invoke_asset_fetch(repo: &Path, asset_path: &str, git_ref: &str) -> Result<Value, String> {
    let payload = json!({
        "meta": {
            "schemaVersion": "2.0",
            "entityKind": "tool",
            "entityId": "github-raw-fetcher",
        },
        "request": {
            "asset_path": asset_path,
            "ref": git_ref,
        }
    });
    let result = invoke_tool_capsule_json(repo, "github-raw-fetcher", &payload, false)?;
    if result.exit_code != 0 || result.body.get("success") != Some(&json!(true)) {
        return Err(result
            .body
            .get("error")
            .and_then(|v| v.get("message").and_then(|m| m.as_str()))
            .or_else(|| result.body.get("error").and_then(|v| v.as_str()))
            .unwrap_or("asset:fetch failed")
            .to_string());
    }
    Ok(result
        .body
        .get("result")
        .cloned()
        .unwrap_or_else(|| json!({})))
}

fn emit_phase_progress(
    repo: &Path,
    cid: &str,
    phase: &str,
    moment: &str,
    status: Option<&str>,
) {
    progress_trace::emit_progress_trace(
        repo,
        cid,
        phase,
        &[json!("action:download-remote-asset")],
        moment,
        status,
        Some("sync-client-assets"),
    );
}

pub fn run(repo: &Path, process_inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let t0 = Instant::now();
    let asset_id = str_field(process_inputs, "asset_id")?;
    let asset_family = process_inputs
        .get("asset_family")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("library_codexes")
        .to_string();
    let cid = correlation_id(process_inputs);
    let mut process_inputs_mut = process_inputs.clone();
    if process_inputs_mut.get("execution_id").is_none() {
        process_inputs_mut["execution_id"] = json!(cid);
    }
    if process_inputs_mut.get("correlation_id").is_none() {
        process_inputs_mut["correlation_id"] = json!(cid);
    }

    let mut state = json!({
        "execution_id": process_inputs_mut.get("execution_id").and_then(|v| v.as_str()).unwrap_or(&cid),
        "asset_id": asset_id,
    });
    let ws = bootstrap_workspace(
        repo,
        "sync-client-assets",
        ".SddIA/workspaces/{process_name}/{execution_id}/",
        &mut process_inputs_mut,
        &mut state,
    )?;

    let mut phases: Vec<Value> = Vec::new();
    emit_phase_progress(repo, &cid, "Manifiesto-Local", "start", None);

    let (genome_rel, filename) = resolve_genome_asset(repo, &asset_id, &asset_family)?;
    let instance_dir = instance_library_root(repo, &asset_family);
    fs::create_dir_all(&instance_dir).map_err(|e| format!("mkdir instance library: {e}"))?;
    let instance_path = instance_dir.join(&filename);

    let local_manifest = if instance_path.is_file() {
        let content = fs::read_to_string(&instance_path)
            .map_err(|e| format!("leer manifiesto local: {e}"))?;
        Some(json!({
            "path": instance_path.to_string_lossy(),
            "hash": sha256_hex(&content),
            "bytes": content.len(),
        }))
    } else {
        None
    };
    phases.push(json!({
        "phase_name": "Manifiesto-Local",
        "status": "executed",
        "local_present": local_manifest.is_some(),
        "local_manifest": local_manifest,
    }));
    emit_phase_progress(repo, &cid, "Manifiesto-Local", "end", Some("executed"));

    emit_phase_progress(repo, &cid, "Reclamacion", "start", None);
    let git_ref = process_inputs
        .get("ref")
        .and_then(|v| v.as_str())
        .unwrap_or("main");
    let fetched = invoke_asset_fetch(repo, &genome_rel, git_ref)?;
    let content = fetched
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "asset:fetch sin content".to_string())?
        .to_string();
    let declared_hash = fetched
        .get("declared_hash")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let origin_kind = fetched
        .get("origin_kind")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    phases.push(json!({
        "phase_name": "Reclamacion",
        "status": "executed",
        "origin_kind": origin_kind,
        "declared_hash": declared_hash,
    }));
    emit_phase_progress(repo, &cid, "Reclamacion", "end", Some("executed"));

    emit_phase_progress(repo, &cid, "Aduana-Integridad", "start", None);
    let computed = sha256_hex(&content);
    if !declared_hash.is_empty() && computed != declared_hash {
        phases.push(json!({
            "phase_name": "Aduana-Integridad",
            "status": "failed",
            "error": "hash_mismatch",
            "declared_hash": declared_hash,
            "computed_hash": computed,
        }));
        emit_phase_progress(repo, &cid, "Aduana-Integridad", "end", Some("failed"));
        let state = json!({
            "execution_id": process_inputs_mut.get("execution_id"),
            "workspace_path": ws.get("workspace_path"),
            "phase_reports": phases,
            "asset_id": asset_id,
        });
        let _ = thermodynamic::run(
            repo,
            "sync-client-assets",
            &state,
            &process_inputs_mut,
            1,
            t0.elapsed().as_millis() as i64,
            false,
        );
        return Ok(OrchestratorEnvelope {
            success: false,
            status_code: 1,
            data: Some(json!({
                "synced": false,
                "hash_verified": false,
                "error": "hash_mismatch",
            })),
            error: Some("hash_mismatch: aduana abortó sin escribir".into()),
            execution_report: Some(json!({
                "process_name": "sync-client-assets",
                "phases": phases,
            })),
            exit_code: 1,
        });
    }
    phases.push(json!({
        "phase_name": "Aduana-Integridad",
        "status": "executed",
        "hash_verified": true,
        "computed_hash": computed,
    }));
    emit_phase_progress(repo, &cid, "Aduana-Integridad", "end", Some("executed"));

    emit_phase_progress(repo, &cid, "Inyeccion", "start", None);
    fs::write(&instance_path, &content).map_err(|e| format!("inyección fs: {e}"))?;
    phases.push(json!({
        "phase_name": "Inyeccion",
        "status": "executed",
        "target": instance_path.to_string_lossy(),
        "bytes": content.len(),
    }));
    emit_phase_progress(repo, &cid, "Inyeccion", "end", Some("executed"));

    let state = json!({
        "execution_id": process_inputs_mut.get("execution_id"),
        "workspace_path": ws.get("workspace_path"),
        "phase_reports": phases,
        "asset_id": asset_id,
    });
    let toll = thermodynamic::run(
        repo,
        "sync-client-assets",
        &state,
        &process_inputs_mut,
        0,
        t0.elapsed().as_millis() as i64,
        true,
    );

    Ok(OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(json!({
            "synced": true,
            "hash_verified": true,
            "local_version": computed,
            "target": instance_path.to_string_lossy(),
            "correlation_id": cid,
            "thermodynamic": toll,
            "timestamp": iso_now(),
        })),
        error: None,
        execution_report: Some(json!({
            "process_name": "sync-client-assets",
            "phases": phases,
        })),
        exit_code: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repo::find_repo_root;

    #[test]
    fn resolves_codex_kalma2_uuid() {
        let repo = find_repo_root().expect("repo");
        let (rel, file) =
            resolve_genome_asset(&repo, "c43544f3-c557-4cc3-8a03-7175282f2c88", "library_codexes")
                .expect("resolve");
        assert!(rel.contains("codex-kalma2-assistant.md"));
        assert_eq!(file, "codex-kalma2-assistant.md");
    }
}
