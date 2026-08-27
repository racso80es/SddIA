//! Aduana evolution: captura árbol (Git nativo), inyecta JSON, invoca WASI, persiste.

use crate::resolve::{flag_value, has_flag};
use execute_process::core::parser::{frontmatter_yaml_to_json, parse_frontmatter_from_str};
use execute_process::core::paths::load_paths_config;
use execute_process::engine::capsules::{
    invoke_capsule_subprocess, parse_capsule_stdout, resolve_capsule_native, resolve_capsule_wasm,
};
use serde_json::{json, Value};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;
use uuid::Uuid;

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

fn read_blob(repo: &Path, rel: &str, rev: &str) -> Option<String> {
    let spec = if rev == ":" {
        format!(":{rel}")
    } else {
        format!("{rev}:{rel}")
    };
    if let Ok((s, c)) = git(repo, &["show", spec.as_str()]) {
        if c == 0 {
            return Some(s);
        }
    }
    None
}

fn is_uuid_v4_stem(stem: &str) -> bool {
    Uuid::parse_str(stem.trim())
        .map(|u| u.get_version() == Some(uuid::Version::Random))
        .unwrap_or(false)
}

fn is_evolution_record_file(fname: &str) -> bool {
    if !fname.ends_with(".md") {
        return false;
    }
    if fname.eq_ignore_ascii_case("evolution_contract.md")
        || fname.eq_ignore_ascii_case("Evolution_log.md")
    {
        return false;
    }
    let stem = fname.trim_end_matches(".md");
    is_uuid_v4_stem(stem)
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

fn build_registry(
    repo: &Path,
    evo_rel: &str,
    log_rel: &str,
    paths: &[(String, String)],
    rev: &str,
    all: bool,
) -> Value {
    let evo_dir = repo.join(evo_rel);
    let evo_prefix = evo_rel.trim_end_matches('/');
    let mut records = Vec::new();
    if let Ok(rd) = fs::read_dir(&evo_dir) {
        for ent in rd.flatten() {
            let p = ent.path();
            if p.extension().and_then(|x| x.to_str()) != Some("md") {
                continue;
            }
            let fname = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if !is_evolution_record_file(fname) {
                continue;
            }
            let rel = p
                .strip_prefix(repo)
                .map(|x| x.to_string_lossy().replace('\\', "/"))
                .unwrap_or_default();
            let in_diff = if all {
                true
            } else {
                paths.iter().any(|(x, _)| x == &rel)
            };
            let status = paths
                .iter()
                .find(|(x, _)| x == &rel)
                .map(|(_, s)| s.as_str())
                .unwrap_or("");
            let raw = if all || in_diff {
                read_blob(repo, &rel, rev).unwrap_or_default()
            } else {
                fs::read_to_string(&p).unwrap_or_default()
            };
            if raw.is_empty() {
                continue;
            }
            let fm = parse_frontmatter_from_str(&raw).unwrap_or_default();
            records.push(json!({
                "path": rel,
                "filename": fname,
                "in_diff": in_diff,
                "diff_status": status,
                "frontmatter": frontmatter_yaml_to_json(&fm),
                "raw": raw
            }));
        }
    }
    let index_raw = read_blob(repo, log_rel, rev)
        .or_else(|| fs::read_to_string(repo.join(log_rel)).ok())
        .unwrap_or_default();
    json!({
        "evolution_dir": evo_prefix,
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

fn persist_detail_only(repo: &Path, evo_rel: &str, body: &Value) -> Result<(), String> {
    let id = body
        .pointer("/result/id_cambio")
        .and_then(|v| v.as_str())
        .ok_or("result.id_cambio ausente")?;
    let detail = body
        .pointer("/result/detail")
        .and_then(|v| v.as_str())
        .ok_or("result.detail ausente")?;
    let detail_path = repo.join(evo_rel).join(format!("{id}.md"));
    fs::write(&detail_path, detail).map_err(|e| format!("EVOL_ATOMICITY: {e}"))?;
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

fn range_touches_evolution(evo_rel: &str, paths: &[(String, String)]) -> bool {
    let prefix = evo_rel.trim_end_matches('/');
    paths.iter().any(|(p, _)| p == prefix || p.starts_with(&format!("{prefix}/")))
}

pub fn run_gate(repo: &Path, args: &[String]) -> i32 {
    let json_out = true;
    let range = has_flag(args, "--range");
    let all = has_flag(args, "--all");
    let if_touched = has_flag(args, "--if-touched");

    if range && all {
        let body = json!({
            "success": false,
            "exitCode": 2,
            "message": "EVOL_CUMULO: --range y --all son mutuamente excluyentes",
            "result": {"reason_codes": ["EVOL_CUMULO"]}
        });
        return emit(&body, json_out, 2);
    }

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

    let rev = if all || range {
        "HEAD"
    } else {
        ":"
    };

    let paths = if all {
        Vec::new()
    } else {
        match diff_paths(repo, range) {
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
        }
    };

    if if_touched && !all && !range_touches_evolution(evo, &paths) {
        let body = json!({
            "meta": {"schemaVersion": "2.0", "entityKind": "tool", "entityId": "sddia-qa"},
            "success": true,
            "exitCode": 0,
            "message": "skipped: evolution no tocado en rango",
            "result": {
                "reason_codes": ["EVOL_OK"],
                "skipped": "if-touched"
            }
        });
        return emit(&body, json_out, 0);
    }

    let diff = json!({
        "paths": paths.iter().map(|(p, s)| json!({"path": p, "status": s})).collect::<Vec<_>>()
    });
    let registry = build_registry(repo, evo, log, &paths, rev, all);
    let request = if all {
        json!({
            "operation": "verdict",
            "audit": "universe",
            "diff": diff,
            "registry": registry
        })
    } else {
        json!({
            "operation": "verdict",
            "diff": diff,
            "registry": registry
        })
    };
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

fn registry_for_rehash(repo: &Path, evo_rel: &str, log_rel: &str, id: &str) -> Value {
    let evo_prefix = evo_rel.trim_end_matches('/');
    let rel = format!("{evo_prefix}/{id}.md");
    let detail_path = repo.join(evo_rel).join(format!("{id}.md"));
    let raw = fs::read_to_string(&detail_path).unwrap_or_default();
    let fm = parse_frontmatter_from_str(&raw).unwrap_or_default();
    let index_raw = fs::read_to_string(repo.join(log_rel)).unwrap_or_default();
    json!({
        "evolution_dir": evo_prefix,
        "records": [json!({
            "path": rel,
            "filename": format!("{id}.md"),
            "in_diff": false,
            "diff_status": "",
            "frontmatter": frontmatter_yaml_to_json(&fm),
            "raw": raw
        })],
        "index": {
            "path": log_rel,
            "content": index_raw,
            "rows": parse_index_rows(&index_raw)
        }
    })
}

pub fn run_rehash(repo: &Path, args: &[String]) -> i32 {
    let json_out = true;
    let dry = has_flag(args, "--dry-run");
    let id = match flag_value(args, "--id") {
        Some(s) if !s.trim().is_empty() => s.trim().to_string(),
        _ => {
            let body = json!({
                "success": false,
                "exitCode": 1,
                "message": "evolution-rehash requiere --id <uuid>"
            });
            return emit(&body, json_out, 1);
        }
    };

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

    let detail_path = repo.join(evo).join(format!("{id}.md"));
    if !detail_path.is_file() {
        let body = json!({
            "success": false,
            "exitCode": 2,
            "message": format!("EVOL_CUMULO: registro {id}.md ausente")
        });
        return emit(&body, json_out, 2);
    }

    let registry = registry_for_rehash(repo, evo, log, &id);
    let request = json!({
        "operation": "rehash",
        "id_cambio": id,
        "registry": registry
    });
    match invoke_register(repo, request) {
        Ok(body) => {
            let success = body.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
            let idem = body
                .pointer("/result/idempotent")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            if success && !dry && !idem {
                if let Err(e) = persist_detail_only(repo, evo, &body) {
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
    let registry = build_registry(repo, evo, log, &[], ":", false);
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
