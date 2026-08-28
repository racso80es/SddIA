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

/// Sincroniza el censo del pie de `daemons/index.md` con las filas del catálogo (L-INDEX-CENSUS).
pub fn sync_daemons_index_census(index_path: &Path) -> Result<(), String> {
    if !index_path.is_file() {
        return Ok(());
    }
    let mut text = fs::read_to_string(index_path).map_err(|e| e.to_string())?;
    let mut names: Vec<String> = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if !t.starts_with("| `") || !t.contains(".md`") {
            continue;
        }
        if let Some(start) = t.find("| `") {
            let rest = &t[start + 3..];
            if let Some(end) = rest.find(".md`") {
                names.push(rest[..end].to_string());
            }
        }
    }
    let n = names.len();
    let list = names.join(", ");
    let sync_line = format!("- **Sincronización:** {n} Centinelas catalogados ({list}).");
    if let Some(idx) = text.find("- **Sincronización:**") {
        if let Some(end) = text[idx..].find('\n') {
            text.replace_range(idx..idx + end, &sync_line);
        }
    }
    fs::write(index_path, text).map_err(|e| e.to_string())
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
    if old_hash.is_some() {
        static RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
        let re = RE.get_or_init(|| Regex::new(r"(?m)^hash_signature:\s*.+$").expect("regex"));
        text = re
            .replace(&text, format!("hash_signature: \"{new_hash}\""))
            .into_owned();
    }
    fs::write(process_path, text).map_err(|e| e.to_string())?;
    Ok((old_hash, new_hash))
}

/// SemVer patch bump (`1.2.0` → `1.2.1`).
pub fn bump_semver_patch(version: &str) -> String {
    let trimmed = version.trim();
    let parts: Vec<&str> = trimmed.split('.').collect();
    match parts.as_slice() {
        [maj, min, pat, ..] => {
            let patch: u64 = pat
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap_or(0);
            format!("{maj}.{min}.{}", patch + 1)
        }
        [maj, min] => format!("{maj}.{min}.1"),
        [maj] if !maj.is_empty() => format!("{maj}.0.1"),
        _ => "1.0.1".into(),
    }
}

fn split_md_frontmatter(text: &str) -> Result<(String, String), String> {
    let rest = text
        .strip_prefix("---")
        .ok_or_else(|| "frontmatter ausente (sin --- inicial)".to_string())?;
    let rest = rest.strip_prefix('\n').unwrap_or(rest);
    let (yaml, body) = rest
        .split_once("\n---")
        .ok_or_else(|| "frontmatter ausente (sin --- de cierre)".to_string())?;
    let body = body.strip_prefix('\n').unwrap_or(body);
    Ok((yaml.to_string(), body.to_string()))
}

/// Resultado de patch update con `process_phases`.
pub struct ProcessPhasesPatchResult {
    pub entity_uuid: String,
    pub old_hash: Option<String>,
    pub new_hash: String,
    pub old_version: String,
    pub new_version: String,
}

/// Update canónico: reemplaza `phases` + bump version + `hash_signature`.
/// Preserva uuid/name/outputs/phase_invocations/context/workspace_template y cuerpo MD.
/// Si `inputs` es `Some`, reemplaza la clave `inputs` del frontmatter.
pub fn patch_process_phases_update(
    process_path: &Path,
    phases: &Value,
    process_version: Option<&str>,
    inputs: Option<&Value>,
) -> Result<ProcessPhasesPatchResult, String> {
    if !phases.as_array().map(|a| !a.is_empty()).unwrap_or(false) {
        return Err("process_phases debe ser array no vacío".into());
    }
    let text = fs::read_to_string(process_path).map_err(|e| e.to_string())?;
    let (yaml, body) = split_md_frontmatter(&text)?;
    let fm_val: Value = serde_yaml::from_str(&yaml).map_err(|e| e.to_string())?;
    let mut map = match fm_val {
        Value::Object(m) => m,
        _ => return Err("frontmatter no es objeto YAML".into()),
    };

    let entity_uuid = map
        .get("uuid")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if entity_uuid.is_empty() {
        return Err("uuid ausente en frontmatter".into());
    }
    let old_hash = map
        .get("hash_signature")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let old_version = map
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("1.0.0")
        .to_string();
    let new_version = process_version
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| bump_semver_patch(&old_version));
    let new_hash = sha256_phases_integrity(phases);

    map.insert("phases".into(), phases.clone());
    if let Some(ins) = inputs {
        map.insert("inputs".into(), ins.clone());
    }
    map.insert("version".into(), Value::String(new_version.clone()));
    map.insert("hash_signature".into(), Value::String(new_hash.clone()));

    let yaml_out = serde_yaml::to_string(&Value::Object(map)).map_err(|e| e.to_string())?;
    // serde_yaml may emit a leading "---\n"; strip for double-fence safety
    let yaml_out = yaml_out
        .strip_prefix("---\n")
        .unwrap_or(&yaml_out)
        .trim_end()
        .to_string();
    let body = body.trim_start_matches('\n');
    let mut out = format!("---\n{yaml_out}\n---\n\n{body}");
    if !out.ends_with('\n') {
        out.push('\n');
    }
    fs::write(process_path, out).map_err(|e| e.to_string())?;

    Ok(ProcessPhasesPatchResult {
        entity_uuid,
        old_hash,
        new_hash,
        old_version,
        new_version,
    })
}

/// Actualiza columna Versión en `process/index.md` para `name`.
pub fn update_process_index_version(
    index_path: &Path,
    name: &str,
    new_version: &str,
) -> Result<(), String> {
    if !index_path.is_file() {
        return Ok(());
    }
    let idx = fs::read_to_string(index_path).map_err(|e| e.to_string())?;
    let mut changed = false;
    let mut lines: Vec<String> = Vec::new();
    for line in idx.lines() {
        if !line.starts_with('|') {
            lines.push(line.to_string());
            continue;
        }
        let cols: Vec<&str> = line.split('|').collect();
        // | Name | UUID | Versión | ... → cols[1]=Name
        if cols.len() >= 4 && cols[1].trim() == name {
            let mut new_cols: Vec<String> = cols.iter().map(|c| (*c).to_string()).collect();
            new_cols[3] = format!(" {new_version} ");
            let rebuilt = new_cols.join("|");
            if rebuilt != *line {
                changed = true;
            }
            lines.push(rebuilt);
        } else {
            lines.push(line.to_string());
        }
    }
    if changed {
        let mut out = lines.join("\n");
        if idx.ends_with('\n') && !out.ends_with('\n') {
            out.push('\n');
        }
        fs::write(index_path, out).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn format_action_io_yaml(key: &str, items: &Value) -> String {
    let Some(arr) = items.as_array() else {
        return format!("{key}: []\n");
    };
    if arr.is_empty() {
        return format!("{key}: []\n");
    }
    let mut lines = vec![format!("{key}:")];
    for item in arr {
        if let Some(obj) = item.as_object() {
            for (k, v) in obj {
                let desc = v.as_str().unwrap_or("");
                lines.push(format!("  - \"{k}\": \"{desc}\""));
            }
        }
    }
    lines.join("\n") + "\n"
}

fn format_capabilities_yaml(caps: &Value) -> String {
    let Some(arr) = caps.as_array() else {
        return "capabilities: []\n".to_string();
    };
    if arr.is_empty() {
        return "capabilities: []\n".to_string();
    }
    let mut lines = vec!["capabilities:".to_string()];
    for c in arr {
        if let Some(s) = c.as_str() {
            lines.push(format!("  - \"{s}\""));
        }
    }
    lines.join("\n") + "\n"
}

fn action_integrity_hash(
    name: &str,
    version: &str,
    context: &str,
    inputs: &Value,
    outputs: &Value,
    body: &str,
) -> String {
    let payload = json!({
        "name": name,
        "version": version,
        "context": context,
        "inputs": inputs,
        "outputs": outputs,
        "body": body.trim(),
    });
    let canon = canon_json_sorted(&payload);
    let mut hasher = Sha256::new();
    hasher.update(canon.as_bytes());
    format!("sha256:{:x}", hasher.finalize())
}

fn hash_from_existing_action(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    format!("sha256:{:x}", hasher.finalize())
}

/// Resultado de patch update de acción.
pub struct ActionContentPatchResult {
    pub entity_uuid: String,
    pub old_hash: String,
    pub new_hash: String,
    pub old_version: String,
    pub new_version: String,
}

/// Update canónico de acción: reemplaza inputs/outputs/capabilities y cuerpo MD; preserva uuid.
pub fn patch_action_content_update(
    action_path: &Path,
    inputs: &Value,
) -> Result<ActionContentPatchResult, String> {
    let text = fs::read_to_string(action_path).map_err(|e| e.to_string())?;
    let (yaml, _old_body) = split_md_frontmatter(&text)?;
    let fm_val: Value = serde_yaml::from_str(&yaml).map_err(|e| e.to_string())?;
    let map = match fm_val {
        Value::Object(m) => m,
        _ => return Err("frontmatter no es objeto YAML".into()),
    };

    let name = map
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if name.is_empty() {
        return Err("name ausente en frontmatter".into());
    }
    let entity_uuid = map
        .get("uuid")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if entity_uuid.is_empty() {
        return Err("uuid ausente en frontmatter".into());
    }

    let old_version = map
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("1.0.0")
        .to_string();
    let new_version = optional_str(inputs, "action_version")
        .unwrap_or_else(|| bump_semver_patch(&old_version));

    let old_hash = map
        .get("hash_signature")
        .and_then(|v| v.as_str())
        .filter(|s| s.starts_with("sha256:"))
        .map(str::to_string)
        .unwrap_or_else(|| hash_from_existing_action(&text));

    let context = optional_str(inputs, "action_context")
        .or_else(|| map.get("context").and_then(|v| v.as_str()).map(str::to_string))
        .unwrap_or_else(|| "ecosystem-evolution".to_string());
    let contract_ver = optional_str(inputs, "actions_contract_version")
        .unwrap_or_else(|| "1.2.0".to_string());

    let action_inputs = inputs
        .get("action_inputs")
        .cloned()
        .unwrap_or_else(|| json!([]));
    let action_outputs = inputs
        .get("action_outputs")
        .cloned()
        .unwrap_or_else(|| json!([]));
    let capabilities = inputs
        .get("action_capabilities")
        .cloned()
        .or_else(|| map.get("capabilities").cloned())
        .unwrap_or_else(|| json!([]));

    let body = inputs
        .get("action_body")
        .and_then(|v| v.as_str())
        .or_else(|| inputs.get("orchestration_logic").and_then(|v| v.as_str()))
        .ok_or("action_body u orchestration_logic requerido para update")?
        .trim()
        .to_string();

    let new_hash = action_integrity_hash(
        &name,
        &new_version,
        &context,
        &action_inputs,
        &action_outputs,
        &body,
    );

    let minteo = map
        .get("minteo_maximo")
        .map(|v| {
            if v.is_null() {
                "null".to_string()
            } else {
                v.to_string()
            }
        })
        .unwrap_or_else(|| "null".to_string());
    let pct = map
        .get("porcentaje_de_exito")
        .map(|v| {
            if v.is_null() {
                "null".to_string()
            } else {
                v.to_string()
            }
        })
        .unwrap_or_else(|| "null".to_string());

    let fm_block = format!(
        r#"---
uuid: "{entity_uuid}"
name: "{name}"
version: "{new_version}"
contract: "actions-contract v{contract_ver}"
context: "{context}"
{capabilities_yaml}{inputs_yaml}{outputs_yaml}hash_signature: "{new_hash}"
minteo_maximo: {minteo}
porcentaje_de_exito: {pct}
---

{body}
"#,
        capabilities_yaml = format_capabilities_yaml(&capabilities),
        inputs_yaml = format_action_io_yaml("inputs", &action_inputs),
        outputs_yaml = format_action_io_yaml("outputs", &action_outputs),
    );

    fs::write(action_path, fm_block).map_err(|e| e.to_string())?;

    Ok(ActionContentPatchResult {
        entity_uuid,
        old_hash,
        new_hash,
        old_version,
        new_version,
    })
}

fn format_index_capabilities(capabilities: &Value) -> String {
    capabilities
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(|c| format!("`{c}`"))
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_default()
}

/// Sincroniza fila completa en `actions/index.md` (Name, UUID, Versión, Context, Descripción, Capabilities).
pub fn sync_action_index_row(
    index_path: &Path,
    name: &str,
    uuid: &str,
    version: &str,
    context: &str,
    description: &str,
    capabilities: &Value,
) -> Result<(), String> {
    if !index_path.is_file() {
        return Ok(());
    }
    let caps = format_index_capabilities(capabilities);
    let new_row = format!(
        "| {name} | {uuid} | {version} | {context} | {description} | {caps} |"
    );
    let idx = fs::read_to_string(index_path).map_err(|e| e.to_string())?;
    let mut changed = false;
    let mut lines: Vec<String> = Vec::new();
    for line in idx.lines() {
        if line.starts_with('|') {
            let cols: Vec<&str> = line.split('|').collect();
            if cols.len() >= 3 && cols[1].trim() == name {
                if line != new_row {
                    changed = true;
                }
                lines.push(new_row.clone());
                continue;
            }
        }
        lines.push(line.to_string());
    }
    if changed {
        let mut out = lines.join("\n");
        if idx.ends_with('\n') && !out.ends_with('\n') {
            out.push('\n');
        }
        fs::write(index_path, out).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Actualiza columna Versión en `actions/index.md` para `name`.
pub fn update_action_index_version(
    index_path: &Path,
    name: &str,
    new_version: &str,
) -> Result<(), String> {
    update_process_index_version(index_path, name, new_version)
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
    fn sync_daemons_index_census_updates_footer() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let index = tmp.path().join("index.md");
        fs::write(
            &index,
            "| Archivo fuente | uuid | name |\n|------|------|------|\n| `a.md` | u1 | a |\n| `b.md` | u2 | b |\n\n## Integridad\n\n- **Sincronización:** uno Centinelas.\n",
        )
        .unwrap();
        sync_daemons_index_census(&index).unwrap();
        let text = fs::read_to_string(&index).unwrap();
        assert!(text.contains("2 Centinelas catalogados (a, b)"));
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
    fn bump_semver_patch_increments() {
        assert_eq!(bump_semver_patch("1.2.0"), "1.2.1");
        assert_eq!(bump_semver_patch("1.1.0"), "1.1.1");
        assert_eq!(bump_semver_patch("1.0.0"), "1.0.1");
    }

    #[test]
    fn refresh_process_hash_replaces_quoted_value() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let process = tmp.path().join("quoted-process.md");
        fs::write(
            &process,
            r#"---
uuid: "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee"
name: "quoted-process"
version: "1.0.0"
hash_signature: "sha256:old"
phases:
  - name: "Fase"
    intent: "Salida explícita."
---
# quoted-process
"#,
        )
        .unwrap();

        let (_, expected) = refresh_process_hash(&process).expect("refresh");
        let text = fs::read_to_string(process).expect("read");
        assert!(text.contains(&format!("hash_signature: \"{expected}\"")));
        assert!(!text.contains("sha256:old"));
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
