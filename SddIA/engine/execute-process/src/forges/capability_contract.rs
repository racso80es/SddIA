//! Forja nativa de la clase `capability-contract` (sustrato B de la Cicatriz Digital).
//!
//! Norma de Identidad Polimórfica (`.cursorrules` §2): el contrato vive en un único fichero
//! `{name}.schema.json` con cabecera de identidad en la clave raíz
//! `"x-sddia": { "uuid", "version", "hash_signature" }`.
//!
//! - `hash_signature` = sha256 del JSON canónico (claves ordenadas) del schema **sin** `x-sddia`.
//! - `serde_json` del workspace no preserva el orden de claves: la adopción de un schema existente
//!   inserta el bloque **por texto** (el resto del fichero queda byte a byte).
//! - Clase solo nativa: sin proceso `*-creator`, sin fallback.

use super::common::{generate_uuid, required_str, sha256_canon, str_field};
use crate::core::paths::load_paths_config;
use jsonschema::Validator;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

pub const X_SDDIA: &str = "x-sddia";

/// Nombre = stem del fichero: sin separadores de ruta ni `..`.
pub fn validate_contract_name(name: &str) -> Result<(), String> {
    let n = name.trim();
    if n.is_empty()
        || n != name
        || n.contains('/')
        || n.contains('\\')
        || n.contains("..")
        || n.starts_with('.')
        || n.ends_with(".schema")
        || n.ends_with(".json")
    {
        return Err(format!(
            "entity_name inválido para capability-contract: {name:?} (stem sin '/', '\\', '..', '.schema' ni '.json')"
        ));
    }
    Ok(())
}

pub fn is_semver(v: &str) -> bool {
    let core = v.split(['-', '+']).next().unwrap_or("");
    let parts: Vec<&str> = core.split('.').collect();
    parts.len() == 3
        && parts
            .iter()
            .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
}

fn contracts_dir(repo: &Path) -> Result<PathBuf, String> {
    let cfg = load_paths_config(repo)?;
    let rel = cfg
        .pointer("/directories/capability_contracts")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().trim_start_matches("./").trim_end_matches('/'))
        .filter(|s| !s.is_empty())
        .ok_or("directories.capability_contracts ausente en cumulo.paths.json")?;
    Ok(repo.join(rel))
}

/// Hash de identidad: sha256 del JSON canónico del schema sin `x-sddia`.
pub fn contract_hash(repo: &Path, schema: &Value) -> Result<String, String> {
    let mut body = schema.clone();
    body.as_object_mut()
        .ok_or("el schema debe ser un objeto JSON")?
        .remove(X_SDDIA);
    sha256_canon(repo, &body)
}

/// Bloque textual `x-sddia` (insertado justo tras la llave de apertura).
pub fn x_sddia_block(uuid: &str, version: &str, hash: &str) -> String {
    format!(
        "\n  \"{X_SDDIA}\": {{\n    \"uuid\": \"{uuid}\",\n    \"version\": \"{version}\",\n    \"hash_signature\": \"{hash}\"\n  }}"
    )
}

/// Inserta `block` tras la primera `{` del documento. Devuelve el texto resultante.
pub fn insert_x_sddia_text(raw: &str, block: &str) -> Result<String, String> {
    let open = raw
        .find('{')
        .filter(|i| raw[..*i].trim().is_empty())
        .ok_or("el schema debe empezar por '{'")?;
    let rest = &raw[open + 1..];
    let empty_object = rest.trim_start().starts_with('}');
    let mut out = String::with_capacity(raw.len() + block.len() + 1);
    out.push_str(&raw[..=open]);
    out.push_str(block);
    if !empty_object {
        out.push(',');
    }
    out.push_str(rest);
    Ok(out)
}

fn compile(schema: &Value, ctx: &str) -> Result<(), String> {
    Validator::new(schema)
        .map(|_| ())
        .map_err(|e| format!("schema inválido ({ctx}): {e}"))
}

fn unescape_pointer(seg: &str) -> String {
    seg.replace("~1", "/").replace("~0", "~")
}

/// RFC 6902 mínimo: `add`, `replace`, `remove` sobre JSON Pointer. Otras operaciones = error.
pub fn apply_json_patch(doc: &mut Value, patch: &Value) -> Result<(), String> {
    let ops = patch
        .as_array()
        .ok_or("schema_json_patch debe ser un array RFC 6902")?;
    for (i, op) in ops.iter().enumerate() {
        let kind = op
            .get("op")
            .and_then(|v| v.as_str())
            .ok_or_else(|| format!("patch[{i}]: op ausente"))?;
        let path = op
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| format!("patch[{i}]: path ausente"))?;
        if path == format!("/{X_SDDIA}") || path.starts_with(&format!("/{X_SDDIA}/")) {
            return Err(format!("patch[{i}]: {X_SDDIA} es identidad gestionada; no se parchea"));
        }
        if !path.starts_with('/') {
            return Err(format!("patch[{i}]: path debe ser un JSON Pointer no vacío"));
        }
        let pos = path.rfind('/').ok_or_else(|| format!("patch[{i}]: path inválido"))?;
        let (parent_ptr, last) = (&path[..pos], unescape_pointer(&path[pos + 1..]));
        let parent = doc
            .pointer_mut(parent_ptr)
            .ok_or_else(|| format!("patch[{i}]: padre inexistente para {path}"))?;
        match kind {
            "add" | "replace" => {
                let value = op
                    .get("value")
                    .cloned()
                    .ok_or_else(|| format!("patch[{i}]: value ausente"))?;
                match parent {
                    Value::Object(map) => {
                        if kind == "replace" && !map.contains_key(&last) {
                            return Err(format!("patch[{i}]: replace sobre clave inexistente {path}"));
                        }
                        map.insert(last, value);
                    }
                    Value::Array(arr) => {
                        if last == "-" && kind == "add" {
                            arr.push(value);
                        } else {
                            let idx: usize = last
                                .parse()
                                .map_err(|_| format!("patch[{i}]: índice inválido en {path}"))?;
                            if kind == "add" {
                                if idx > arr.len() {
                                    return Err(format!("patch[{i}]: índice fuera de rango {path}"));
                                }
                                arr.insert(idx, value);
                            } else {
                                if idx >= arr.len() {
                                    return Err(format!("patch[{i}]: índice fuera de rango {path}"));
                                }
                                arr[idx] = value;
                            }
                        }
                    }
                    _ => return Err(format!("patch[{i}]: el padre de {path} no es contenedor")),
                }
            }
            "remove" => match parent {
                Value::Object(map) => {
                    if map.remove(&last).is_none() {
                        return Err(format!("patch[{i}]: remove sobre clave inexistente {path}"));
                    }
                }
                Value::Array(arr) => {
                    let idx: usize = last
                        .parse()
                        .map_err(|_| format!("patch[{i}]: índice inválido en {path}"))?;
                    if idx >= arr.len() {
                        return Err(format!("patch[{i}]: índice fuera de rango {path}"));
                    }
                    arr.remove(idx);
                }
                _ => return Err(format!("patch[{i}]: el padre de {path} no es contenedor")),
            },
            other => {
                return Err(format!(
                    "patch[{i}]: operación RFC 6902 no soportada: {other} (solo add|replace|remove)"
                ))
            }
        }
    }
    Ok(())
}

fn read_schema(path: &Path) -> Result<(String, Value), String> {
    let raw = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let val: Value = serde_json::from_str(&raw).map_err(|e| format!("{}: JSON inválido: {e}", path.display()))?;
    if !val.is_object() {
        return Err(format!("{}: el schema debe ser un objeto JSON", path.display()));
    }
    Ok((raw, val))
}

struct Identity {
    uuid: String,
    version: String,
    hash: String,
}

fn identity_of(schema: &Value) -> Option<Identity> {
    let x = schema.get(X_SDDIA)?;
    Some(Identity {
        uuid: x.get("uuid")?.as_str()?.to_string(),
        version: x.get("version")?.as_str()?.to_string(),
        hash: x.get("hash_signature")?.as_str()?.to_string(),
    })
}

fn handoff(uuid: &str, new_hash: &str, old_hash: Option<&str>, version: &str, extra: Value) -> Value {
    let mut out = json!({
        "handoff_entity_uuid": uuid,
        "handoff_hash_signature_new": new_hash,
        "handoff_hash_signature_old": old_hash,
        "handoff_version": version,
    });
    if let (Value::Object(base), Value::Object(ext)) = (&mut out, extra) {
        for (k, v) in ext {
            base.insert(k, v);
        }
    }
    out
}

fn write_atomic(path: &Path, text: &str) -> Result<(), String> {
    let tmp = path.with_extension("schema.json.tmp");
    fs::write(&tmp, text).map_err(|e| e.to_string())?;
    fs::rename(&tmp, path).map_err(|e| {
        let _ = fs::remove_file(&tmp);
        e.to_string()
    })
}

fn pretty(value: &Value) -> Result<String, String> {
    let mut s = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    s.push('\n');
    Ok(s)
}

/// Serializa `body` (sin `x-sddia`) y le antepone el bloque de identidad por texto.
fn render_with_identity(body: &Value, uuid: &str, version: &str, hash: &str) -> Result<String, String> {
    insert_x_sddia_text(&pretty(body)?, &x_sddia_block(uuid, version, hash))
}

pub fn run_capability_contract_forge(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let name = required_str(inputs, "entity_name")?;
    validate_contract_name(&name)?;
    let dir = contracts_dir(repo)?;
    let path = dir.join(format!("{name}.schema.json"));
    let lifecycle = str_field(inputs, "lifecycle_operation", "create");

    match lifecycle.as_str() {
        "create" => forge_create(repo, inputs, &path),
        "update" => forge_update(repo, inputs, &path, &name),
        "delete" => Err("delete de capability-contract no soportado".into()),
        other => Err(format!("lifecycle_operation no soportada: {other}")),
    }
}

fn forge_create(repo: &Path, inputs: &Value, path: &Path) -> Result<Value, String> {
    if path.is_file() {
        let (_, existing) = read_schema(path)?;
        return match identity_of(&existing) {
            Some(id) => Ok(handoff(
                &id.uuid,
                &id.hash,
                None,
                &id.version,
                json!({"idempotent": true}),
            )),
            None => Err(format!(
                "{} existe sin {X_SDDIA}: usar update con semantic_seed.adopt=true (no se sobrescribe)",
                path.display()
            )),
        };
    }
    let body = inputs
        .get("schema_json")
        .cloned()
        .filter(Value::is_object)
        .ok_or("create requiere schema_json (objeto)")?;
    if body.get(X_SDDIA).is_some() {
        return Err(format!("schema_json no debe traer {X_SDDIA}: lo gestiona la forja"));
    }
    let version = str_field(inputs, "schema_version", "1.0.0");
    if !is_semver(&version) {
        return Err(format!("schema_version no es SemVer: {version}"));
    }
    compile(&body, "create")?;
    let hash = contract_hash(repo, &body)?;
    let uuid = generate_uuid(repo)?;
    let text = render_with_identity(&body, &uuid, &version, &hash)?;
    // Defensa: lo escrito debe volver a compilar y conservar el hash.
    let written: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    compile(&written, "create/resultado")?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    write_atomic(path, &text)?;
    Ok(handoff(&uuid, &hash, None, &version, json!({})))
}

fn forge_update(repo: &Path, inputs: &Value, path: &Path, name: &str) -> Result<Value, String> {
    if !path.is_file() {
        return Err(format!("capability-contract inexistente: {}", path.display()));
    }
    let (raw, mut schema) = read_schema(path)?;
    let version = optional_version(inputs)?;
    let adopt = inputs.get("adopt").and_then(|v| v.as_bool()).unwrap_or(false);
    let patch = inputs.get("schema_json_patch").filter(|v| !v.is_null());

    if adopt && patch.is_some() {
        return Err("adopt y schema_json_patch son excluyentes (adoptar primero, parchear después)".into());
    }

    if adopt {
        let version = version.ok_or("adopt requiere schema_version")?;
        let current_hash = contract_hash(repo, &schema)?;
        if let Some(id) = identity_of(&schema) {
            if id.hash != current_hash {
                return Err(format!(
                    "{name}: hash_signature desalineada con el contenido (edit manual); usar schema_json_patch por cauce oficial"
                ));
            }
            return Ok(handoff(
                &id.uuid,
                &id.hash,
                Some(&id.hash),
                &id.version,
                json!({"idempotent": true}),
            ));
        }
        if schema.get(X_SDDIA).is_some() {
            return Err(format!("{name}: {X_SDDIA} presente pero incompleto (uuid/version/hash_signature)"));
        }
        compile(&schema, "adopt/origen")?;
        let uuid = generate_uuid(repo)?;
        let text = insert_x_sddia_text(&raw, &x_sddia_block(&uuid, &version, &current_hash))?;
        let after: Value = serde_json::from_str(&text).map_err(|e| format!("adopt produjo JSON inválido: {e}"))?;
        compile(&after, "adopt/resultado")?;
        if contract_hash(repo, &after)? != current_hash {
            return Err("adopt alteró el contenido del schema (hash distinto)".into());
        }
        write_atomic(path, &text)?;
        return Ok(handoff(&uuid, &current_hash, None, &version, json!({"adopted": true})));
    }

    if let Some(patch) = patch {
        let id = identity_of(&schema)
            .ok_or_else(|| format!("{name}: sin {X_SDDIA}; adoptar antes de parchear"))?;
        let new_version = version.ok_or("schema_json_patch requiere schema_version")?;
        if new_version == id.version {
            return Err(format!(
                "schema_version debe cambiar al parchear (actual {})",
                id.version
            ));
        }
        let current_hash = contract_hash(repo, &schema)?;
        if id.hash != current_hash {
            return Err(format!("{name}: hash_signature desalineada con el contenido; revisar antes de parchear"));
        }
        if let Some(m) = schema.as_object_mut() {
            m.remove(X_SDDIA);
        }
        apply_json_patch(&mut schema, patch)?;
        compile(&schema, "update/resultado")?;
        let new_hash = contract_hash(repo, &schema)?;
        let text = render_with_identity(&schema, &id.uuid, &new_version, &new_hash)?;
        let after: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
        compile(&after, "update/render")?;
        write_atomic(path, &text)?;
        return Ok(handoff(&id.uuid, &new_hash, Some(&id.hash), &new_version, json!({})));
    }

    Err(format!(
        "capability-contract update sin adopt ni schema_json_patch; no se regenera {}",
        path.display()
    ))
}

fn optional_version(inputs: &Value) -> Result<Option<String>, String> {
    match inputs.get("schema_version").and_then(|v| v.as_str()) {
        None => Ok(None),
        Some(v) if is_semver(v.trim()) => Ok(Some(v.trim().to_string())),
        Some(v) => Err(format!("schema_version no es SemVer: {v}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Serializa con el resto de tests de forja y fija un UUID de laboratorio (en un repo temporal
    /// no hay cápsula `cryptography-manager`). Restaura el entorno al soltarse, incluso con pánico.
    struct Lab {
        _g: crate::engine::eda_port::test_lab::ParallelLabGuard,
    }

    impl Drop for Lab {
        fn drop(&mut self) {
            std::env::remove_var("SDDIA_FORGE_LAB_UUID");
        }
    }

    fn guard() -> Lab {
        let g = crate::engine::eda_port::test_lab::ParallelLabGuard::acquire();
        std::env::set_var("SDDIA_FORGE_LAB_UUID", "5e5e5e5e-1111-4222-8333-444444444444");
        Lab { _g: g }
    }

    fn repo_with(dir_rel: &str) -> tempfile::TempDir {
        let td = tempfile::tempdir().unwrap();
        let core = td.path().join("SddIA/core");
        fs::create_dir_all(&core).unwrap();
        fs::write(
            core.join("cumulo.paths.json"),
            serde_json::to_string(&json!({"directories": {"capability_contracts": dir_rel}})).unwrap(),
        )
        .unwrap();
        fs::create_dir_all(td.path().join(dir_rel.trim_end_matches('/'))).unwrap();
        td
    }

    const ORIGINAL: &str = "{\n  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\",\n  \"title\": \"demo\",\n  \"type\": \"object\",\n  \"required\": [\"a\"],\n  \"properties\": {\n    \"a\": {\"type\": \"string\", \"description\": \"caf\\u00e9\"}\n  }\n}\n";

    fn put(td: &tempfile::TempDir, rel: &str, text: &str) -> PathBuf {
        let p = td.path().join(rel);
        fs::write(&p, text).unwrap();
        p
    }

    fn adopt_inputs(name: &str, version: &str) -> Value {
        json!({
            "entity_class": "capability-contract",
            "entity_name": name,
            "lifecycle_operation": "update",
            "adopt": true,
            "schema_version": version,
        })
    }

    #[test]
    fn adopt_inserts_block_and_is_byte_reversible() {
        let _g = guard();
        let td = repo_with("SddIA/contracts/");
        let p = put(&td, "SddIA/contracts/demo.schema.json", ORIGINAL);
        let out = run_capability_contract_forge(td.path(), &adopt_inputs("demo", "1.0.0")).unwrap();
        let uuid = out["handoff_entity_uuid"].as_str().unwrap().to_string();
        let hash = out["handoff_hash_signature_new"].as_str().unwrap().to_string();
        assert!(out["handoff_hash_signature_old"].is_null());
        let after = fs::read_to_string(&p).unwrap();
        let block = x_sddia_block(&uuid, "1.0.0", &hash);
        assert!(after.contains(&block));
        assert_eq!(after.replacen(&format!("{block},"), "", 1), ORIGINAL, "diff = solo el bloque");
        let v: Value = serde_json::from_str(&after).unwrap();
        assert_eq!(v[X_SDDIA]["uuid"], json!(uuid));
        assert_eq!(contract_hash(td.path(), &v).unwrap(), hash);
    }

    #[test]
    fn adopt_is_idempotent_and_keeps_uuid() {
        let _g = guard();
        let td = repo_with("SddIA/contracts/");
        let p = put(&td, "SddIA/contracts/demo.schema.json", ORIGINAL);
        let first = run_capability_contract_forge(td.path(), &adopt_inputs("demo", "1.0.0")).unwrap();
        let bytes = fs::read(&p).unwrap();
        let second = run_capability_contract_forge(td.path(), &adopt_inputs("demo", "1.0.0")).unwrap();
        assert_eq!(first["handoff_entity_uuid"], second["handoff_entity_uuid"]);
        assert_eq!(second["idempotent"], json!(true));
        assert_eq!(bytes, fs::read(&p).unwrap());
    }

    #[test]
    fn adopt_into_empty_object_is_valid_json() {
        let text = insert_x_sddia_text("{}", &x_sddia_block("u", "1.0.0", "sha256:x")).unwrap();
        let v: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(v[X_SDDIA]["uuid"], json!("u"));
    }

    #[test]
    fn update_without_patch_or_adopt_is_error() {
        let _g = guard();
        let td = repo_with("SddIA/contracts/");
        let p = put(&td, "SddIA/contracts/demo.schema.json", ORIGINAL);
        let err = run_capability_contract_forge(
            td.path(),
            &json!({"entity_name": "demo", "lifecycle_operation": "update", "schema_version": "1.0.1"}),
        )
        .unwrap_err();
        assert!(err.contains("sin adopt ni schema_json_patch"), "{err}");
        assert_eq!(fs::read_to_string(&p).unwrap(), ORIGINAL);
    }

    fn patch_inputs(version: &str, patch: Value) -> Value {
        json!({
            "entity_name": "demo",
            "lifecycle_operation": "update",
            "schema_version": version,
            "schema_json_patch": patch,
        })
    }

    #[test]
    fn patch_keeps_uuid_updates_hash_and_version() {
        let _g = guard();
        let td = repo_with("SddIA/contracts/");
        let p = put(&td, "SddIA/contracts/demo.schema.json", ORIGINAL);
        let adopted = run_capability_contract_forge(td.path(), &adopt_inputs("demo", "1.0.0")).unwrap();
        let out = run_capability_contract_forge(
            td.path(),
            &patch_inputs(
                "2.0.0",
                json!([
                    {"op": "add", "path": "/properties/b", "value": {"type": "integer"}},
                    {"op": "replace", "path": "/title", "value": "demo2"},
                    {"op": "add", "path": "/required/-", "value": "b"}
                ]),
            ),
        )
        .unwrap();
        assert_eq!(out["handoff_entity_uuid"], adopted["handoff_entity_uuid"]);
        assert_eq!(out["handoff_hash_signature_old"], adopted["handoff_hash_signature_new"]);
        assert_ne!(out["handoff_hash_signature_new"], adopted["handoff_hash_signature_new"]);
        assert_eq!(out["handoff_version"], json!("2.0.0"));
        let v: Value = serde_json::from_str(&fs::read_to_string(&p).unwrap()).unwrap();
        assert_eq!(v["title"], json!("demo2"));
        assert_eq!(v["required"], json!(["a", "b"]));
        assert_eq!(v[X_SDDIA]["version"], json!("2.0.0"));
        assert_eq!(
            contract_hash(td.path(), &v).unwrap(),
            out["handoff_hash_signature_new"].as_str().unwrap()
        );
    }

    #[test]
    fn patch_on_identity_is_error() {
        let _g = guard();
        let td = repo_with("SddIA/contracts/");
        let p = put(&td, "SddIA/contracts/demo.schema.json", ORIGINAL);
        run_capability_contract_forge(td.path(), &adopt_inputs("demo", "1.0.0")).unwrap();
        let before = fs::read(&p).unwrap();
        let err = run_capability_contract_forge(
            td.path(),
            &patch_inputs("2.0.0", json!([{"op": "replace", "path": "/x-sddia/uuid", "value": "x"}])),
        )
        .unwrap_err();
        assert!(err.contains("identidad gestionada"), "{err}");
        assert_eq!(before, fs::read(&p).unwrap());
    }

    #[test]
    fn patch_producing_invalid_schema_writes_nothing() {
        let _g = guard();
        let td = repo_with("SddIA/contracts/");
        let p = put(&td, "SddIA/contracts/demo.schema.json", ORIGINAL);
        run_capability_contract_forge(td.path(), &adopt_inputs("demo", "1.0.0")).unwrap();
        let before = fs::read(&p).unwrap();
        let err = run_capability_contract_forge(
            td.path(),
            &patch_inputs("2.0.0", json!([{"op": "replace", "path": "/type", "value": 42}])),
        )
        .unwrap_err();
        assert!(err.contains("schema inválido"), "{err}");
        assert_eq!(before, fs::read(&p).unwrap());
    }

    #[test]
    fn patch_requires_adopted_contract_and_new_version() {
        let _g = guard();
        let td = repo_with("SddIA/contracts/");
        put(&td, "SddIA/contracts/demo.schema.json", ORIGINAL);
        let e1 = run_capability_contract_forge(
            td.path(),
            &patch_inputs("2.0.0", json!([{"op": "replace", "path": "/title", "value": "z"}])),
        )
        .unwrap_err();
        assert!(e1.contains("adoptar antes"), "{e1}");
        run_capability_contract_forge(td.path(), &adopt_inputs("demo", "1.0.0")).unwrap();
        let e2 = run_capability_contract_forge(
            td.path(),
            &patch_inputs("1.0.0", json!([{"op": "replace", "path": "/title", "value": "z"}])),
        )
        .unwrap_err();
        assert!(e2.contains("debe cambiar"), "{e2}");
    }

    #[test]
    fn manual_edit_is_detected_as_hash_drift() {
        let _g = guard();
        let td = repo_with("SddIA/contracts/");
        let p = put(&td, "SddIA/contracts/demo.schema.json", ORIGINAL);
        run_capability_contract_forge(td.path(), &adopt_inputs("demo", "1.0.0")).unwrap();
        let tampered = fs::read_to_string(&p).unwrap().replace("\"demo\"", "\"hack\"");
        fs::write(&p, tampered).unwrap();
        let err = run_capability_contract_forge(td.path(), &adopt_inputs("demo", "1.0.0")).unwrap_err();
        assert!(err.contains("desalineada"), "{err}");
    }

    #[test]
    fn create_writes_identity_and_is_idempotent() {
        let _g = guard();
        let td = repo_with("SddIA/contracts/");
        let inputs = json!({
            "entity_name": "fresh.v1",
            "lifecycle_operation": "create",
            "schema_version": "1.0.0",
            "schema_json": {"type": "object", "title": "fresh"},
        });
        let out = run_capability_contract_forge(td.path(), &inputs).unwrap();
        let p = td.path().join("SddIA/contracts/fresh.v1.schema.json");
        let v: Value = serde_json::from_str(&fs::read_to_string(&p).unwrap()).unwrap();
        assert_eq!(v[X_SDDIA]["uuid"], out["handoff_entity_uuid"]);
        let again = run_capability_contract_forge(td.path(), &inputs).unwrap();
        assert_eq!(again["idempotent"], json!(true));
        assert_eq!(again["handoff_entity_uuid"], out["handoff_entity_uuid"]);
    }

    #[test]
    fn create_rejects_identity_in_seed_and_invalid_schema() {
        let _g = guard();
        let td = repo_with("SddIA/contracts/");
        let e1 = run_capability_contract_forge(
            td.path(),
            &json!({"entity_name": "a", "lifecycle_operation": "create",
                    "schema_json": {"type": "object", "x-sddia": {}}}),
        )
        .unwrap_err();
        assert!(e1.contains("lo gestiona la forja"), "{e1}");
        let e2 = run_capability_contract_forge(
            td.path(),
            &json!({"entity_name": "b", "lifecycle_operation": "create", "schema_json": {"type": 7}}),
        )
        .unwrap_err();
        assert!(e2.contains("schema inválido"), "{e2}");
        assert!(!td.path().join("SddIA/contracts/b.schema.json").exists());
    }

    #[test]
    fn create_over_existing_without_identity_is_error() {
        let _g = guard();
        let td = repo_with("SddIA/contracts/");
        put(&td, "SddIA/contracts/demo.schema.json", ORIGINAL);
        let err = run_capability_contract_forge(
            td.path(),
            &json!({"entity_name": "demo", "lifecycle_operation": "create",
                    "schema_json": {"type": "object"}}),
        )
        .unwrap_err();
        assert!(err.contains("adopt"), "{err}");
    }

    #[test]
    fn delete_is_error() {
        let _g = guard();
        let td = repo_with("SddIA/contracts/");
        let err = run_capability_contract_forge(
            td.path(),
            &json!({"entity_name": "demo", "lifecycle_operation": "delete"}),
        )
        .unwrap_err();
        assert!(err.contains("no soportado"), "{err}");
    }

    #[test]
    fn dangerous_names_are_rejected() {
        for n in ["../x", "a/b", "a\\b", "..", ".hidden", "x.schema", "x.json", " x", ""] {
            assert!(validate_contract_name(n).is_err(), "{n:?}");
        }
        for n in ["eda.event_manager.v1", "doc.closure", "llm-registry"] {
            assert!(validate_contract_name(n).is_ok(), "{n:?}");
        }
    }

    #[test]
    fn directory_comes_from_cumulo_not_hardwired() {
        let _g = guard();
        let td = repo_with("elsewhere/contratos/");
        put(&td, "elsewhere/contratos/demo.schema.json", ORIGINAL);
        run_capability_contract_forge(td.path(), &adopt_inputs("demo", "1.0.0")).unwrap();
        let v: Value =
            serde_json::from_str(&fs::read_to_string(td.path().join("elsewhere/contratos/demo.schema.json")).unwrap())
                .unwrap();
        assert!(v.get(X_SDDIA).is_some());
    }

    /// Contratos auxiliares: existen como schema pero no figuran como capacidad en el catálogo
    /// (`capability-taxonomy.md`); su `x-sddia.version` no se contrasta con él. Lista cerrada.
    const AUXILIARY_CONTRACTS: &[&str] = &["di.binding", "llm-registry"];

    /// El contrato `eda.event_manager` vive en `eda.event_manager.v1.schema.json`
    /// (`contract_schema` en `capability-bindings.md`).
    fn contract_file_stem(contract: &str) -> String {
        if contract == "eda.event_manager" {
            "eda.event_manager.v1".into()
        } else {
            contract.into()
        }
    }

    fn catalog_versions(repo: &Path) -> Vec<(String, String)> {
        let text = fs::read_to_string(repo.join("SddIA/library/norms/capability-taxonomy.md")).unwrap();
        let re = regex::Regex::new(r"^\|\s*`[^`]+`\s*\|\s*`([^`]+)`\s*\|\s*(\d+\.\d+\.\d+)\s*\|").unwrap();
        text.lines()
            .filter_map(|l| re.captures(l).map(|c| (c[1].to_string(), c[2].to_string())))
            .collect()
    }

    /// Guardián de la Norma de Identidad Polimórfica (sustrato B) sobre el repo real.
    #[test]
    fn all_repo_capability_contracts_carry_valid_identity() {
        let _g = guard();
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap();
        let dir = contracts_dir(&repo).unwrap();
        let mut files: Vec<PathBuf> = fs::read_dir(&dir)
            .unwrap()
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.ends_with(".schema.json")))
            .collect();
        files.sort();
        assert_eq!(files.len(), 17, "nº de contratos: {files:?}");

        let mut uuids = std::collections::BTreeSet::new();
        let mut problems = Vec::new();
        let mut versions = std::collections::BTreeMap::new();
        for f in &files {
            let stem = f.file_name().unwrap().to_str().unwrap().trim_end_matches(".schema.json").to_string();
            let (_, schema) = read_schema(f).unwrap();
            if let Err(e) = Validator::new(&schema) {
                problems.push(format!("{stem}: no compila: {e}"));
                continue;
            }
            let Some(id) = identity_of(&schema) else {
                problems.push(format!("{stem}: sin x-sddia completo"));
                continue;
            };
            match uuid::Uuid::parse_str(&id.uuid) {
                Ok(u) if u.get_version_num() == 4 => {}
                _ => problems.push(format!("{stem}: uuid no es v4: {}", id.uuid)),
            }
            if !uuids.insert(id.uuid.clone()) {
                problems.push(format!("{stem}: uuid duplicado {}", id.uuid));
            }
            if !is_semver(&id.version) {
                problems.push(format!("{stem}: version no SemVer {}", id.version));
            }
            match contract_hash(&repo, &schema) {
                Ok(h) if h == id.hash => {}
                Ok(h) => problems.push(format!("{stem}: hash_signature desalineada ({} != {h})", id.hash)),
                Err(e) => problems.push(format!("{stem}: {e}")),
            }
            versions.insert(stem, id.version);
        }
        assert!(problems.is_empty(), "identidad inválida:\n{}", problems.join("\n"));

        // Versión x-sddia == catálogo; auxiliares excluidos de forma explícita (no se saltan en silencio).
        let catalog = catalog_versions(&repo);
        assert_eq!(catalog.len(), 15, "filas de catálogo: {catalog:?}");
        for (contract, ver) in &catalog {
            let stem = contract_file_stem(contract);
            assert_eq!(versions.get(&stem), Some(ver), "x-sddia.version de {stem} != catálogo {ver}");
        }
        let in_catalog: std::collections::BTreeSet<String> =
            catalog.iter().map(|(c, _)| contract_file_stem(c)).collect();
        let not_in_catalog: std::collections::BTreeSet<String> =
            versions.keys().filter(|k| !in_catalog.contains(*k)).cloned().collect();
        let expected_aux: std::collections::BTreeSet<String> =
            AUXILIARY_CONTRACTS.iter().map(|s| (*s).to_string()).collect();
        assert_eq!(not_in_catalog, expected_aux, "contratos fuera de catálogo no declarados como auxiliares");
        let bindings = fs::read_to_string(repo.join("SddIA/core/capability-bindings.md")).unwrap();
        assert!(bindings.contains("contract_schema: \"eda.event_manager.v1.schema.json\""));
    }

    #[test]
    fn semver_validation() {
        assert!(is_semver("2.0.0") && is_semver("1.2.3-rc.1"));
        assert!(!is_semver("2.0") && !is_semver("a.b.c") && !is_semver(""));
    }
}
