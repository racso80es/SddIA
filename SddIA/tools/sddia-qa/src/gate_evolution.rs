//! Aduana evolution: captura árbol (Git nativo), inyecta JSON, invoca WASI, persiste.

use crate::resolve::has_flag;
use execute_process::core::parser::load_frontmatter_yaml;
use execute_process::core::paths::load_paths_config;
use execute_process::engine::capsules::{
    invoke_capsule_subprocess, parse_capsule_stdout, resolve_capsule_native, resolve_capsule_wasm,
};
use serde_json::{json, Value};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

fn git(repo: &Path, args: &[&str]) -> Result<(String, i32), String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .map_err(|e| format!("git spawn: {e}"))?;
    Ok((
        String::from_utf8_lossy(&out.stdout).to_string(),
        out.status.code().unwrap_or(1),
    ))
}

fn range_diff_spec(repo: &Path) -> Result<String, String> {
    for spec in ["origin/main", "main"] {
        if let Ok((stdout, 0)) = git(repo, &["rev-parse", "--verify", "--quiet", spec]) {
            if !stdout.trim().is_empty() {
                return Ok(format!("{spec}...HEAD"));
            }
        }
    }
    Err("faltan refs origin/main y main para --range (CI: fetch origin main)".into())
}

fn diff_paths(repo: &Path, range: bool) -> Result<Vec<(String, String)>, String> {
    let (stdout, code) = if range {
        let spec = range_diff_spec(repo)?;
        git(
            repo,
            &[
                "diff",
                "--name-status",
                "--diff-filter=ACMRD",
                &spec,
            ],
        )?
    } else {
        git(
            repo,
            &[
                "diff",
                "--cached",
                "--name-status",
                "--diff-filter=ACMRD",
            ],
        )?
    };
    if code != 0 && stdout.trim().is_empty() {
        return Err("git diff falló".into());
    }
    let mut rows = Vec::new();
    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut it = line.split('\t');
        let status = it.next().unwrap_or("M").chars().next().unwrap_or('M').to_string();
        if let Some(path) = it.next() {
            rows.push((path.replace('\\', "/"), status));
        }
    }
    Ok(rows)
}

fn read_blob(repo: &Path, rel: &str, staged: bool) -> Option<String> {
    if staged {
        let spec = format!(":{rel}");
        if let Ok((s, c)) = git(repo, &["show", spec.as_str()]) {
            if c == 0 {
                return Some(s);
            }
        }
    }
    fs::read_to_string(repo.join(rel)).ok()
}

fn yaml_json(v: &serde_yaml::Value) -> Value {
    serde_json::to_value(v).unwrap_or(Value::Null)
}

fn fm_json(map: &std::collections::HashMap<String, serde_yaml::Value>) -> Value {
    let mut obj = serde_json::Map::new();
    for (k, v) in map {
        obj.insert(k.clone(), yaml_json(v));
    }
    Value::Object(obj)
}

fn parse_index_rows(content: &str) -> Vec<Value> {
    let mut rows = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if !t.starts_with('|') || t.contains("---") || t.contains("id_cambio") {
            continue;
        }
        let cols: Vec<&str> = t.split('|').map(str::trim).filter(|s| !s.is_empty()).collect();
        if cols.len() < 5 {
            continue;
        }
        let id = cols[0].trim_matches('`');
        rows.push(json!({
            "id_cambio": id,
            "fecha": cols[1],
            "resumen": cols[2],
            "clase_formato": cols[3],
            "ruta_relativa": cols[4].trim_matches('`')
        }));
    }
    rows
}

fn build_registry(repo: &Path, evo_rel: &str, log_rel: &str, paths: &[(String, String)], staged: bool) -> Value {
    let evo_dir = repo.join(evo_rel);
    let mut records = Vec::new();
    if let Ok(rd) = fs::read_dir(&evo_dir) {
        for ent in rd.flatten() {
            let p = ent.path();
            if p.extension().and_then(|x| x.to_str()) != Some("md") {
                continue;
            }
            let rel = p
                .strip_prefix(repo)
                .map(|x| x.to_string_lossy().replace('\\', "/"))
                .unwrap_or_default();
            let fname = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
            let in_diff = paths.iter().any(|(x, _)| x == &rel);
            let status = paths
                .iter()
                .find(|(x, _)| x == &rel)
                .map(|(_, s)| s.as_str())
                .unwrap_or("");
            let raw = if in_diff {
                read_blob(repo, &rel, staged).unwrap_or_default()
            } else {
                fs::read_to_string(&p).unwrap_or_default()
            };
            if raw.is_empty() {
                continue;
            }
            let fm = load_frontmatter_yaml(&p).unwrap_or_default();
            records.push(json!({
                "path": rel,
                "filename": fname,
                "in_diff": in_diff,
                "diff_status": status,
                "frontmatter": fm_json(&fm),
                "raw": raw
            }));
        }
    }
    let index_raw = read_blob(repo, log_rel, staged)
        .or_else(|| fs::read_to_string(repo.join(log_rel)).ok())
        .unwrap_or_default();
    json!({
        "evolution_dir": evo_rel,
        "records": records,
        "index": {
            "path": log_rel,
            "content": index_raw,
            "rows": parse_index_rows(&index_raw)
        }
    })
}

fn invoke_register(repo: &Path, request: Value) -> Result<Value, String> {
    let payload = json!({
        "meta": {
            "schemaVersion": "2.0",
            "entityKind": "skill",
            "entityId": "sddia-evolution-register"
        },
        "request": request
    });
    let stdin = serde_json::to_string(&payload).map_err(|e| e.to_string())?;
    let wasm = resolve_capsule_wasm(repo, "sddia-evolution-register");
    let native = resolve_capsule_native(repo, "sddia-evolution-register");
    let (kind, path) = if let Some(w) = wasm {
        ("wasm", w)
    } else if let Some(n) = native {
        ("native", n)
    } else {
        return Err("EVOL_CUMULO: cápsula sddia-evolution-register ausente".into());
    };
    let (stdout, stderr, _code) = invoke_capsule_subprocess(repo, kind, &path, &stdin)?;
    parse_capsule_stdout(&stdout).or_else(|e| {
        Err(format!("{e}; stderr={}", stderr.trim()))
    })
}

fn persist(repo: &Path, evo_rel: &str, log_rel: &str, body: &Value) -> Result<(), String> {
    let id = body
        .pointer("/result/id_cambio")
        .and_then(|v| v.as_str())
        .ok_or("result.id_cambio ausente")?;
    let detail = body
        .pointer("/result/detail")
        .and_then(|v| v.as_str())
        .ok_or("result.detail ausente")?;
    let index = body
        .pointer("/result/index")
        .and_then(|v| v.as_str())
        .ok_or("result.index ausente")?;
    let detail_path = repo.join(evo_rel).join(format!("{id}.md"));
    let index_path = repo.join(log_rel);
    let bak = PathBuf::from(format!("{}.bak", index_path.display()));
    if index_path.is_file() {
        fs::copy(&index_path, &bak).map_err(|e| e.to_string())?;
    }
    if let Err(e) = fs::write(&detail_path, detail) {
        let _ = fs::remove_file(&detail_path);
        return Err(format!("EVOL_ATOMICITY: {e}"));
    }
    if let Err(e) = fs::write(&index_path, index) {
        let _ = fs::remove_file(&detail_path);
        if bak.is_file() {
            let _ = fs::rename(&bak, &index_path);
        }
        return Err(format!("EVOL_ATOMICITY: {e}"));
    }
    let _ = fs::remove_file(&bak);
    Ok(())
}

fn emit(body: &Value, json_out: bool, code: i32) -> i32 {
    if json_out {
        println!("{}", serde_json::to_string(body).unwrap_or_else(|_| "{}".into()));
    } else if let Some(msg) = body.get("message").and_then(|v| v.as_str()) {
        eprintln!("{msg}");
    }
    code
}

pub fn run_gate(repo: &Path, args: &[String]) -> i32 {
    let json_out = true;
    let range = has_flag(args, "--range");
    let cfg = match load_paths_config(repo) {
        Ok(c) => c,
        Err(e) => {
            let body = json!({
                "meta": {"schemaVersion": "2.0", "entityKind": "tool", "entityId": "sddia-qa"},
                "success": false,
                "exitCode": 2,
                "message": format!("EVOL_CUMULO: {e}"),
                "result": {"reason_codes": ["EVOL_CUMULO"]}
            });
            return emit(&body, json_out, 2);
        }
    };
    let evo = cfg
        .pointer("/directories/evolution")
        .and_then(|v| v.as_str())
        .unwrap_or("SddIA/evolution");
    let log = cfg
        .pointer("/normative_documents/evolution_log")
        .and_then(|v| v.as_str())
        .unwrap_or("SddIA/evolution/Evolution_log.md");
    let contract = cfg
        .pointer("/normative_documents/evolution_contract")
        .and_then(|v| v.as_str())
        .unwrap_or("SddIA/evolution/evolution_contract.md");
    if !repo.join(contract).is_file() || !repo.join(log).is_file() {
        let body = json!({
            "success": false,
            "exitCode": 2,
            "message": "EVOL_CUMULO: contrato o índice ausente",
            "result": {"reason_codes": ["EVOL_CUMULO"]}
        });
        return emit(&body, json_out, 2);
    }
    let paths = match diff_paths(repo, range) {
        Ok(p) => p,
        Err(e) => {
            let body = json!({
                "success": false,
                "exitCode": 2,
                "message": format!("EVOL_CUMULO: {e}"),
                "result": {"reason_codes": ["EVOL_CUMULO"]}
            });
            return emit(&body, json_out, 2);
        }
    };
    let diff = json!({
        "paths": paths.iter().map(|(p, s)| json!({"path": p, "status": s})).collect::<Vec<_>>()
    });
    let registry = build_registry(repo, evo, log, &paths, !range);
    let request = json!({
        "operation": "verdict",
        "diff": diff,
        "registry": registry
    });
    match invoke_register(repo, request) {
        Ok(body) => {
            let success = body.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
            let code = body
                .get("exitCode")
                .and_then(|v| v.as_i64())
                .unwrap_or(if success { 0 } else { 2 }) as i32;
            emit(&body, json_out, code)
        }
        Err(e) => {
            let body = json!({
                "success": false,
                "exitCode": 2,
                "message": e,
                "result": {"reason_codes": ["EVOL_CUMULO"]}
            });
            emit(&body, json_out, 2)
        }
    }
}

pub fn run_mutate(repo: &Path, args: &[String]) -> i32 {
    let json_out = true;
    let dry = has_flag(args, "--dry-run");
    let cfg = match load_paths_config(repo) {
        Ok(c) => c,
        Err(e) => {
            let body = json!({"success": false, "exitCode": 2, "message": e});
            return emit(&body, json_out, 2);
        }
    };
    let evo = cfg
        .pointer("/directories/evolution")
        .and_then(|v| v.as_str())
        .unwrap_or("SddIA/evolution");
    let log = cfg
        .pointer("/normative_documents/evolution_log")
        .and_then(|v| v.as_str())
        .unwrap_or("SddIA/evolution/Evolution_log.md");
    let mut raw = String::new();
    if std::io::stdin().read_to_string(&mut raw).is_err() {
        return 1;
    }
    let mut payload: Value = match serde_json::from_str(raw.trim()) {
        Ok(v) => v,
        Err(e) => {
            let body = json!({"success": false, "exitCode": 1, "message": e.to_string()});
            return emit(&body, json_out, 1);
        }
    };
    let registry = build_registry(repo, evo, log, &[], false);
    if let Some(req) = payload.get_mut("request") {
        req.as_object_mut()
            .map(|o| o.insert("registry".into(), registry.clone()));
    } else {
        payload
            .as_object_mut()
            .map(|o| o.insert("registry".into(), registry));
    }
    let request = payload.get("request").cloned().unwrap_or(payload);
    match invoke_register(repo, request) {
        Ok(body) => {
            let success = body.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
            let idem = body
                .pointer("/result/idempotent")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            if success && !dry && !idem {
                if let Err(e) = persist(repo, evo, log, &body) {
                    let fail = json!({
                        "success": false,
                        "exitCode": 2,
                        "message": e,
                        "result": {"reason_codes": ["EVOL_ATOMICITY"]}
                    });
                    return emit(&fail, json_out, 2);
                }
            }
            let code = body
                .get("exitCode")
                .and_then(|v| v.as_i64())
                .unwrap_or(if success { 0 } else { 2 }) as i32;
            emit(&body, json_out, code)
        }
        Err(e) => emit(
            &json!({"success": false, "exitCode": 2, "message": e, "result": {"reason_codes": ["EVOL_CUMULO"]}}),
            json_out,
            2,
        ),
    }
}
