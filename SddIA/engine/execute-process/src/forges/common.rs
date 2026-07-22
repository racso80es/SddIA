//! Utilidades compartidas de forja (paridad `execute_process_forges.py`).

use crate::engine::crypto_broker;
use regex::Regex;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

pub fn required_str(inputs: &Value, key: &str) -> Result<String, String> {
    match inputs.get(key).and_then(|v| v.as_str()) {
        Some(s) if !s.trim().is_empty() => Ok(s.trim().to_string()),
        _ => Err(format!("{key} requerido")),
    }
}

pub fn optional_str(inputs: &Value, key: &str) -> Option<String> {
    inputs
        .get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

pub fn str_field(inputs: &Value, key: &str, default: &str) -> String {
    optional_str(inputs, key).unwrap_or_else(|| default.to_string())
}

/// JSON canónico: claves ordenadas, separadores `,` `:` sin espacios (paridad Python).
pub fn canon_json_sorted(value: &Value) -> String {
    let sorted = sort_json(value);
    serde_json::to_string(&sorted).expect("canon json")
}

fn sort_json(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<_> = map.keys().cloned().collect();
            keys.sort();
            let mut out = Map::new();
            for k in keys {
                if let Some(v) = map.get(&k) {
                    out.insert(k, sort_json(v));
                }
            }
            Value::Object(out)
        }
        Value::Array(arr) => Value::Array(arr.iter().map(sort_json).collect()),
        other => other.clone(),
    }
}

pub fn crypto_raw(repo: &Path, request: &Value) -> Result<Value, String> {
    let out = crypto_broker::run(repo, &json!({ "crypto_request": request }))?;
    out.get("crypto_response")
        .cloned()
        .ok_or_else(|| "crypto_response ausente".into())
}

pub fn generate_uuid(repo: &Path) -> Result<String, String> {
    if let Ok(fixed) = std::env::var("SDDIA_FORGE_LAB_UUID") {
        let t = fixed.trim();
        if !t.is_empty() {
            return Ok(t.to_string());
        }
    }
    let resp = crypto_raw(
        repo,
        &json!({"operation": "GENERATE_UUID", "target_payload": null}),
    )?;
    if let Some(s) = resp.as_str() {
        return Ok(s.to_string());
    }
    resp.get("result")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .ok_or_else(|| "GENERATE_UUID sin resultado".into())
}

pub fn sha256_hex(repo: &Path, payload: &str) -> Result<String, String> {
    if let Ok(fixed) = std::env::var("SDDIA_FORGE_LAB_SHA256") {
        let t = fixed.trim();
        if !t.is_empty() {
            return Ok(t.to_string());
        }
    }
    match crypto_raw(
        repo,
        &json!({
            "operation": "GENERATE_SHA256",
            "target_type": "STRING",
            "target_payload": payload,
        }),
    ) {
        Ok(resp) => {
            if let Some(s) = resp.as_str() {
                return Ok(s.to_string());
            }
            if let Some(s) = resp.get("result").and_then(|v| v.as_str()) {
                return Ok(s.to_string());
            }
            Err("GENERATE_SHA256 sin resultado".into())
        }
        Err(_) => {
            let digest = Sha256::digest(payload.as_bytes());
            Ok(format!("{digest:x}"))
        }
    }
}

pub fn sha256_canon(repo: &Path, canon: &Value) -> Result<String, String> {
    let hex = sha256_hex(repo, &canon_json_sorted(canon))?;
    Ok(format!("sha256:{hex}"))
}

pub fn capability_name(name: &str) -> String {
    let cap = name.replace('-', "_");
    let cap = if cap.len() > 32 { cap[..32].to_string() } else { cap };
    if cap.is_empty() {
        "entity-cap".into()
    } else {
        cap
    }
}

/// Paridad `execute_process_core.parse_frontmatter` (campos usados por forjas).
pub fn parse_frontmatter(path: &Path) -> Result<Map<String, Value>, String> {
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let Some(body) = text.strip_prefix("---") else {
        return Ok(Map::new());
    };
    let Some((yaml, _)) = body.split_once("\n---") else {
        return Ok(Map::new());
    };
    let fm: Value = serde_yaml::from_str(yaml).map_err(|e| e.to_string())?;
    match fm {
        Value::Object(map) => Ok(map),
        _ => Ok(Map::new()),
    }
}

pub fn idempotent_forge_handoff(
    artifact: &Path,
    lifecycle: &str,
) -> Result<Option<Value>, String> {
    if lifecycle != "create" || !artifact.is_file() {
        return Ok(None);
    }
    let fm = parse_frontmatter(artifact)?;
    let uuid = fm
        .get("uuid")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty());
    let Some(uuid) = uuid else {
        return Ok(None);
    };
    Ok(Some(json!({
        "handoff_entity_uuid": uuid,
        "handoff_hash_signature_new": fm.get("hash_signature"),
        "handoff_hash_signature_old": null,
        "handoff_version": fm.get("version"),
        "idempotent": true,
    })))
}

/// Paridad `execute_process_forges._append_row`.
pub fn append_row(index_path: &Path, row: &str, name: &str) -> Result<(), String> {
    if !index_path.is_file() {
        return Ok(());
    }
    let idx = fs::read_to_string(index_path).map_err(|e| e.to_string())?;
    if idx.contains(name) {
        return Ok(());
    }
    let lines: Vec<&str> = idx.split('\n').collect();
    for i in 0..lines.len() {
        let line = lines[i];
        if !line.starts_with('|') {
            continue;
        }
        if i + 1 >= lines.len() {
            break;
        }
        let sep_line = lines[i + 1].trim();
        if sep_line.chars().all(|c| matches!(c, '|' | '-' | ' ' | ':')) {
            let mut out: Vec<String> = lines.iter().map(|s| (*s).to_string()).collect();
            out.insert(i + 2, row.to_string());
            fs::write(index_path, out.join("\n") + "\n").map_err(|e| e.to_string())?;
            return Ok(());
        }
    }
    fs::write(index_path, format!("{}\n{row}\n", idx.trim_end()))
        .map_err(|e| e.to_string())
}

pub fn sha256_phases_integrity(phases: &Value) -> String {
    let canon = canon_json_sorted(phases);
    let digest = Sha256::digest(canon.as_bytes());
    format!("sha256:{digest:x}")
}

pub fn refresh_process_hash(process_path: &Path) -> Result<(Option<String>, String), String> {
    let fm = parse_frontmatter(process_path)?;
    let old_hash = fm
        .get("hash_signature")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let phases = fm
        .get("phases")
        .cloned()
        .unwrap_or_else(|| json!([{"name": "Fase inicial", "intent": "update"}]));
    let new_hash = sha256_phases_integrity(&phases);
    let mut text = fs::read_to_string(process_path).map_err(|e| e.to_string())?;
    if let Some(ref old) = old_hash {
        if text.contains(old) {
            text = text.replacen(&format!("hash_signature: {old}"), &format!("hash_signature: {new_hash}"), 1);
        } else {
            static RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| Regex::new(r"(?m)^hash_signature:\s*.+$").expect("regex"));
            text = re
                .replace(&text, format!("hash_signature: {new_hash}"))
                .into_owned();
        }
    }
    fs::write(process_path, text).map_err(|e| e.to_string())?;
    Ok((old_hash, new_hash))
}

pub fn handoff_create(
    entity_uuid: &str,
    hash_sig: &str,
    version: &str,
    extra: Value,
) -> Value {
    let mut out = json!({
        "handoff_entity_uuid": entity_uuid,
        "handoff_hash_signature_new": hash_sig,
        "handoff_hash_signature_old": null,
        "handoff_version": version,
    });
    if let (Value::Object(base), Value::Object(ext)) = (&mut out, extra) {
        for (k, v) in ext {
            base.insert(k, v);
        }
    }
    out
}

pub fn repo_tool_base(repo: &Path, scope: &str) -> PathBuf {
    if scope == "local" {
        repo.join(".SddIA/tools")
    } else {
        repo.join("SddIA/tools")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canon_json_sorted_matches_python_separators() {
        let v = json!({"b": 1, "a": {"z": 2, "y": 3}});
        assert_eq!(canon_json_sorted(&v), r#"{"a":{"y":3,"z":2},"b":1}"#);
    }

    #[test]
    fn append_row_idempotent_by_name() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let index = tmp.path().join("index.md");
        fs::write(
            &index,
            "| Col |\n|-----|\n| existing |\n",
        )
        .unwrap();
        append_row(&index, "| new-row |", "new-row").unwrap();
        let text = fs::read_to_string(&index).unwrap();
        assert!(text.contains("| new-row |"));
        append_row(&index, "| new-row-duplicate |", "new-row").unwrap();
        assert_eq!(text.matches("| new-row |").count(), 1);
    }

    #[test]
    #[ignore = "one-off: refresh hash_signature on process YAMLs"]
    fn refresh_process_hash_feature_bugfix() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("repo root");
        for name in ["feature.md", "bug-fix.md"] {
            let p = root.join("process").join(name);
            let (old, new) = refresh_process_hash(&p).expect("refresh");
            eprintln!("{}: {:?} -> {}", name, old, new);
        }
    }
}
