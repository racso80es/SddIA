//! Dominio puro: cotejo diff×registry y cálculo de registros evolution.
//! Prohibido Git / working tree.

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use uuid::Uuid;

pub const REASON_OK: &str = "EVOL_OK";
pub const REASON_UNREGISTERED: &str = "EVOL_MATERIAL_UNREGISTERED";
pub const REASON_INVALID: &str = "EVOL_RECORD_INVALID";
pub const REASON_NOT_INDEXED: &str = "EVOL_NOT_INDEXED";
pub const REASON_HASH: &str = "EVOL_HASH_MISMATCH";
pub const REASON_DUPLICATE: &str = "EVOL_DUPLICATE";
pub const REASON_ATOMICITY: &str = "EVOL_ATOMICITY";
pub const REASON_CUMULO: &str = "EVOL_CUMULO";

const EVO_PREFIX_DEFAULT: &str = "SddIA/evolution";
const SDDIA_PREFIX: &str = "SddIA/";

#[derive(Debug, Clone)]
pub struct Envelope {
    pub success: bool,
    pub exit_code: i32,
    pub message: String,
    pub result: Value,
}

impl Envelope {
    pub fn to_json(&self) -> Value {
        json!({
            "meta": {
                "schemaVersion": "2.0",
                "entityKind": "skill",
                "entityId": "sddia-evolution-register"
            },
            "success": self.success,
            "exitCode": self.exit_code,
            "message": self.message,
            "result": self.result
        })
    }
}

fn ok_result(result: Value, message: &str) -> Envelope {
    Envelope {
        success: true,
        exit_code: 0,
        message: message.to_string(),
        result,
    }
}

fn err_result(code: &str, message: &str, findings: Value) -> Envelope {
    Envelope {
        success: false,
        exit_code: 2,
        message: message.to_string(),
        result: json!({
            "reason_codes": [code],
            "findings": findings
        }),
    }
}

pub fn canonical_hash(raw: &str) -> String {
    let stripped = strip_hash_integrity_line(raw).replace("\r\n", "\n");
    let mut hasher = Sha256::new();
    hasher.update(stripped.as_bytes());
    format!("sha256:{}", hex::encode(hasher.finalize()))
}

fn strip_hash_integrity_line(raw: &str) -> String {
    raw.lines()
        .filter(|l| !l.trim_start().starts_with("hash_integrity:"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn is_uuid_v4(s: &str) -> bool {
    Uuid::parse_str(s.trim())
        .map(|u| u.get_version() == Some(uuid::Version::Random))
        .unwrap_or(false)
}

fn evo_prefix(registry: &Value) -> String {
    registry
        .get("evolution_dir")
        .and_then(|v| v.as_str())
        .unwrap_or(EVO_PREFIX_DEFAULT)
        .trim_end_matches('/')
        .to_string()
}

fn path_under(path: &str, prefix: &str) -> bool {
    let p = path.replace('\\', "/");
    p == prefix || p.starts_with(&format!("{prefix}/"))
}

fn covers(reference: &str, path: &str) -> bool {
    let r = reference.trim().trim_end_matches('/').replace('\\', "/");
    let p = path.replace('\\', "/");
    if r.is_empty() {
        return false;
    }
    p == r || p.starts_with(&format!("{r}/"))
}

fn is_borrador(rec: &Value) -> bool {
    let fname = rec
        .get("filename")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if fname.contains("-temp") || fname.contains("analisis-temp") {
        return true;
    }
    let fm = rec.get("frontmatter").unwrap_or(&Value::Null);
    if fm.get("estado").and_then(|v| v.as_str()) == Some("borrador") {
        return true;
    }
    matches!(
        fm.get("tipo_operacion")
            .or_else(|| fm.get("tipo"))
            .and_then(|v| v.as_str()),
        Some("analisis-temporal")
    )
}

fn is_record_file(path: &str, evo: &str) -> bool {
    let p = path.replace('\\', "/");
    if !path_under(&p, evo) || !p.ends_with(".md") {
        return false;
    }
    let fname = p.rsplit('/').next().unwrap_or("");
    if fname.eq_ignore_ascii_case("evolution_contract.md")
        || fname.eq_ignore_ascii_case("Evolution_log.md")
    {
        return false;
    }
    let stem = fname.trim_end_matches(".md");
    is_uuid_v4(stem)
}

fn list_refs(fm: &Value, keys: &[&str]) -> Vec<String> {
    let mut out = Vec::new();
    for k in keys {
        match fm.get(*k) {
            Some(Value::Array(arr)) => {
                for x in arr {
                    if let Some(s) = x.as_str() {
                        if !s.trim().is_empty() {
                            out.push(s.trim().to_string());
                        }
                    }
                }
            }
            Some(Value::String(s)) if !s.trim().is_empty() => out.push(s.trim().to_string()),
            _ => {}
        }
    }
    out
}

fn fm_str(fm: &Value, keys: &[&str]) -> Option<String> {
    for k in keys {
        if let Some(s) = fm.get(*k).and_then(|v| v.as_str()) {
            let t = s.trim();
            if !t.is_empty() {
                return Some(t.to_string());
            }
        }
    }
    None
}

fn is_valid_hash_integrity(declared: &str) -> bool {
    let declared = declared.trim();
    if !declared.starts_with("sha256:") {
        return false;
    }
    let hex = declared.strip_prefix("sha256:").unwrap_or("");
    hex.len() == 64 && hex.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f'))
}

fn validate_canonical(rec: &Value) -> Option<(String, String)> {
    let fm = rec.get("frontmatter").unwrap_or(&Value::Null);
    let fname = rec
        .get("filename")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let stem = fname.trim_end_matches(".md");
    let id = fm_str(fm, &["id_cambio", "uuid"]).unwrap_or_default();
    if !is_uuid_v4(&id) || id != stem {
        return Some((
            REASON_INVALID.to_string(),
            format!("{fname}: id_cambio no UUID v4 o distinto del stem"),
        ));
    }
    let tipo = fm_str(fm, &["tipo_operacion"]).unwrap_or_default();
    if !matches!(tipo.as_str(), "alta" | "baja" | "modificacion") {
        return Some((
            REASON_INVALID.to_string(),
            format!("{fname}: tipo_operacion no canónico"),
        ));
    }
    if fm_str(fm, &["fecha", "date", "created"]).is_none() {
        return Some((
            REASON_INVALID.to_string(),
            format!("{fname}: fecha ausente"),
        ));
    }
    if fm_str(fm, &["descripcion_breve", "descripcion"]).is_none() {
        return Some((
            REASON_INVALID.to_string(),
            format!("{fname}: descripcion_breve vacía"),
        ));
    }
    if list_refs(fm, &["relacionado", "related_entities", "artefactos_afectados"]).is_empty() {
        return Some((
            REASON_INVALID.to_string(),
            format!("{fname}: relacionado vacío"),
        ));
    }
    let declared = fm_str(fm, &["hash_integrity"]).unwrap_or_default();
    if declared.is_empty() {
        return Some((
            REASON_HASH.to_string(),
            format!("{fname}: hash_integrity vacío"),
        ));
    }
    if !is_valid_hash_integrity(&declared) {
        return Some((
            REASON_HASH.to_string(),
            format!(
                "{fname}: placeholder/formato inválido; sddia-qa evolution-rehash --id {id}"
            ),
        ));
    }
    if let Some(raw) = rec.get("raw").and_then(|v| v.as_str()) {
        let recomputed = canonical_hash(raw);
        if recomputed != declared {
            return Some((
                REASON_HASH.to_string(),
                format!("{fname}: hash mismatch"),
            ));
        }
    }
    None
}

fn index_ids(registry: &Value) -> Vec<String> {
    let mut ids = Vec::new();
    if let Some(rows) = registry.pointer("/index/rows").and_then(|v| v.as_array()) {
        for r in rows {
            if let Some(id) = r.get("id_cambio").and_then(|v| v.as_str()) {
                ids.push(id.trim().to_string());
            }
        }
    }
    ids
}

fn indexed(registry: &Value, id: &str) -> bool {
    index_ids(registry).iter().any(|x| x == id)
}

fn correlators<'a>(records: &'a [Value], evo: &str) -> Vec<&'a Value> {
    records
        .iter()
        .filter(|r| r.get("in_diff").and_then(|v| v.as_bool()) == Some(true))
        .filter(|r| {
            let p = r.get("path").and_then(|v| v.as_str()).unwrap_or("");
            is_record_file(p, evo) && !is_borrador(r)
        })
        .collect()
}

fn universe_records<'a>(records: &'a [Value], evo: &str) -> Vec<&'a Value> {
    records
        .iter()
        .filter(|r| {
            let p = r.get("path").and_then(|v| v.as_str()).unwrap_or("");
            is_record_file(p, evo) && !is_borrador(r)
        })
        .collect()
}

fn patch_hash_integrity_line(raw: &str, hash: &str) -> String {
    let new_line = format!("hash_integrity: \"{hash}\"");
    let mut replaced = false;
    let mut out: Vec<String> = Vec::new();
    for line in raw.split_inclusive('\n') {
        let trimmed = line.trim_end_matches('\n');
        if trimmed.trim_start().starts_with("hash_integrity:") {
            out.push(new_line.clone());
            replaced = true;
        } else if !trimmed.is_empty() || line.ends_with('\n') {
            out.push(trimmed.to_string());
        }
    }
    if replaced {
        return out.join("\n");
    }
  // Insertar tras descripcion_breve si existe
    let mut inserted = false;
    out.clear();
    for line in raw.split_inclusive('\n') {
        let trimmed = line.trim_end_matches('\n');
        out.push(trimmed.to_string());
        if !inserted && trimmed.trim_start().starts_with("descripcion_breve:") {
            out.push(new_line.clone());
            inserted = true;
        }
    }
    if inserted {
        out.join("\n")
    } else {
        format!("{raw}\n{new_line}\n")
    }
}

fn verdict_universe(registry: &Value, evo: &str) -> Envelope {
    let records = registry
        .get("records")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let mut findings = Vec::new();
    for rec in universe_records(&records, evo) {
        let path = rec.get("path").and_then(|v| v.as_str()).unwrap_or("");
        if let Some((code, detail)) = validate_canonical(rec) {
            findings.push(json!({"path": path, "reason_code": code, "detail": detail}));
        }
    }
    if findings.is_empty() {
        ok_result(
            json!({
                "operation": "verdict",
                "audit": "universe",
                "reason_codes": [REASON_OK],
                "findings": []
            }),
            "universo conforme",
        )
    } else {
        err_result(
            REASON_HASH,
            "universo evolution no conforme",
            json!(findings),
        )
    }
}

fn rehash(request: &Value) -> Envelope {
    let id = match request.get("id_cambio").and_then(|v| v.as_str()) {
        Some(s) if !s.trim().is_empty() => s.trim().to_string(),
        _ => {
            return err_result(REASON_INVALID, "id_cambio obligatorio", json!([]));
        }
    };
    let registry = request.get("registry").cloned().unwrap_or(json!({}));
    let evo = evo_prefix(&registry);
    let ruta = format!("{evo}/{id}.md");
    let records = registry
        .get("records")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let rec = records
        .iter()
        .find(|r| r.get("path").and_then(|v| v.as_str()) == Some(ruta.as_str()));
    let Some(rec) = rec else {
        return err_result(
            REASON_INVALID,
            &format!("registro {id} no encontrado en registry"),
            json!([]),
        );
    };
    let raw = rec
        .get("raw")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if raw.is_empty() {
        return err_result(REASON_INVALID, "raw vacío", json!([]));
    }
    let hash = canonical_hash(raw);
    let detail = patch_hash_integrity_line(raw, &hash);
    let current_index = registry
        .pointer("/index/content")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    ok_result(
        json!({
            "operation": "rehash",
            "reason_codes": [REASON_OK],
            "findings": [],
            "id_cambio": id,
            "detail": detail,
            "index": current_index,
            "hash_integrity": hash,
            "idempotent": detail == raw
        }),
        "hash re-anclado",
    )
}

fn record_covers(rec: &Value, path: &str) -> bool {
    let fm = rec.get("frontmatter").unwrap_or(&Value::Null);
    let tipo = fm_str(fm, &["tipo_operacion"]).unwrap_or_default();
    let mut refs = list_refs(fm, &["relacionado", "related_entities", "artefactos_afectados"]);
    if tipo == "baja" {
        refs.extend(list_refs(fm, &["rutas_eliminadas"]));
    }
    refs.iter().any(|r| covers(r, path))
}

fn verdict(request: &Value) -> Envelope {
    let registry = request.get("registry").cloned().unwrap_or(json!({}));
    let evo = evo_prefix(&registry);
    let audit = request
        .get("audit")
        .and_then(|v| v.as_str())
        .unwrap_or("delta");
    if audit == "universe" {
        return verdict_universe(&registry, &evo);
    }

    let diff = request.get("diff").cloned().unwrap_or(json!({}));
    let paths: Vec<String> = diff
        .get("paths")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|p| {
                    p.get("path")
                        .and_then(|x| x.as_str())
                        .map(|s| s.replace('\\', "/"))
                })
                .collect()
        })
        .unwrap_or_default();

    let material: Vec<String> = paths
        .iter()
        .filter(|p| path_under(p, SDDIA_PREFIX.trim_end_matches('/')) && !path_under(p, &evo))
        .cloned()
        .collect();

    let records = registry
        .get("records")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let cors = correlators(&records, &evo);

    let mut findings = Vec::new();
    for rec in &cors {
        let path = rec.get("path").and_then(|v| v.as_str()).unwrap_or("");
        if let Some((code, detail)) = validate_canonical(rec) {
            findings.push(json!({"path": path, "reason_code": code, "detail": detail}));
            continue;
        }
        let id = fm_str(
            rec.get("frontmatter").unwrap_or(&Value::Null),
            &["id_cambio", "uuid"],
        )
        .unwrap_or_default();
        if !indexed(&registry, &id) {
            findings.push(json!({
                "path": path,
                "reason_code": REASON_NOT_INDEXED,
                "detail": format!("{id} no está en el índice")
            }));
        }
    }
    if !findings.is_empty() {
        let code = findings[0]
            .get("reason_code")
            .and_then(|v| v.as_str())
            .unwrap_or(REASON_INVALID);
        return err_result(code, "registro del diff inválido", json!(findings));
    }

    if material.is_empty() {
        return ok_result(
            json!({
                "operation": "verdict",
                "reason_codes": [REASON_OK],
                "findings": []
            }),
            "L-SELF / sin material",
        );
    }

    for p in &material {
        let covered = cors.iter().any(|r| record_covers(r, p));
        if !covered {
            findings.push(json!({
                "path": p,
                "reason_code": REASON_UNREGISTERED,
                "detail": "cambio material sin correlato evolution en el diff"
            }));
        }
    }
    if !findings.is_empty() {
        return err_result(
            REASON_UNREGISTERED,
            "diff material sin evolution correlacionada",
            json!(findings),
        );
    }

    ok_result(
        json!({
            "operation": "verdict",
            "reason_codes": [REASON_OK],
            "findings": []
        }),
        "delta cubierto",
    )
}

fn existing_ids(registry: &Value) -> Vec<String> {
    let mut ids = index_ids(registry);
    if let Some(recs) = registry.get("records").and_then(|v| v.as_array()) {
        for r in recs {
            if let Some(id) = fm_str(
                r.get("frontmatter").unwrap_or(&Value::Null),
                &["id_cambio", "uuid"],
            ) {
                ids.push(id);
            }
            if let Some(f) = r.get("filename").and_then(|v| v.as_str()) {
                let stem = f.trim_end_matches(".md");
                if is_uuid_v4(stem) {
                    ids.push(stem.to_string());
                }
            }
        }
    }
    ids
}

fn render_detail(
    id: &str,
    fecha: &str,
    tipo: &str,
    desc: &str,
    relacionado: &[String],
    extra_yaml: &str,
) -> (String, String) {
    let rel = relacionado
        .iter()
        .map(|r| format!("  - \"{}\"", r.replace('"', "\\\"")))
        .collect::<Vec<_>>()
        .join("\n");
    let draft = format!(
        "---\ncontrato_version: \"1.1.1\"\nid_cambio: \"{id}\"\nfecha: \"{fecha}\"\ntipo_operacion: {tipo}\ndescripcion_breve: \"{desc}\"\n{extra_yaml}relacionado:\n{rel}\n---\n\n# {desc}\n"
    );
    let hash = canonical_hash(&draft);
    let detail = format!(
        "---\ncontrato_version: \"1.1.1\"\nid_cambio: \"{id}\"\nfecha: \"{fecha}\"\ntipo_operacion: {tipo}\ndescripcion_breve: \"{desc}\"\nhash_integrity: \"{hash}\"\n{extra_yaml}relacionado:\n{rel}\n---\n\n# {desc}\n"
    );
    (detail, hash)
}

fn patch_index(current: &str, id: &str, fecha: &str, resumen: &str, ruta: &str) -> String {
    let row = format!(
        "| `{id}` | {fecha} | {resumen} | CANONICO | `{ruta}` |"
    );
    if current.contains(&format!("`{id}`")) {
        return current.to_string();
    }
    let marker = "|-----------|-------|---------|---------------|---------------|";
    if let Some(pos) = current.find(marker) {
        let insert_at = pos + marker.len();
        let mut out = String::new();
        out.push_str(&current[..insert_at]);
        out.push('\n');
        out.push_str(&row);
        out.push_str(&current[insert_at..]);
        return out;
    }
    format!("{}\n{row}\n", current.trim_end())
}

fn mutate(request: &Value, op: &str) -> Envelope {
    let registry = request.get("registry").cloned().unwrap_or(json!({}));
    let fecha = request
        .get("fecha")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty());
    let Some(fecha) = fecha else {
        return err_result(
            REASON_INVALID,
            "fecha obligatoria; la cápsula no inventa",
            json!([]),
        );
    };
    let desc = request
        .get("descripcion_breve")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("");
    if desc.is_empty() {
        return err_result(REASON_INVALID, "descripcion_breve vacía", json!([]));
    }
    let relacionado: Vec<String> = request
        .get("relacionado")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(|s| s.to_string()))
                .filter(|s| !s.trim().is_empty())
                .collect()
        })
        .unwrap_or_default();
    if relacionado.is_empty() {
        return err_result(REASON_INVALID, "relacionado vacío", json!([]));
    }

    let mut extra = String::new();
    if op == "baja" {
        let rutas = list_refs(request, &["rutas_eliminadas"]);
        if rutas.is_empty() {
            return err_result(
                REASON_INVALID,
                "baja requiere rutas_eliminadas",
                json!([]),
            );
        }
        extra.push_str("rutas_eliminadas:\n");
        for r in &rutas {
            extra.push_str(&format!("  - \"{}\"\n", r.replace('"', "\\\"")));
        }
        if let Some(c) = request
            .get("commit_referencia_previo")
            .and_then(|v| v.as_str())
        {
            extra.push_str(&format!("commit_referencia_previo: \"{c}\"\n"));
        }
    }

    let id = match request.get("id_cambio").and_then(|v| v.as_str()) {
        Some(s) if is_uuid_v4(s) => s.trim().to_string(),
        Some(_) => {
            return err_result(REASON_INVALID, "id_cambio no es UUID v4", json!([]));
        }
        None if op == "alta" => Uuid::new_v4().to_string(),
        None => {
            return err_result(
                REASON_INVALID,
                "id_cambio obligatorio en modificacion/baja",
                json!([]),
            );
        }
    };

    let evo = evo_prefix(&registry);
    let ruta = format!("{evo}/{id}.md");
    let (detail, hash) = render_detail(&id, fecha, op, desc, &relacionado, &extra);
    let current_index = registry
        .pointer("/index/content")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let index = patch_index(current_index, &id, fecha, desc, &ruta);

    let idempotent = registry
        .get("records")
        .and_then(|v| v.as_array())
        .and_then(|arr| {
            arr.iter().find(|r| {
                r.get("path").and_then(|p| p.as_str()) == Some(ruta.as_str())
                    && r.get("raw").and_then(|p| p.as_str()) == Some(detail.as_str())
            })
        })
        .is_some()
        && current_index == index;

    if op == "alta" && !idempotent && existing_ids(&registry).iter().any(|x| x == &id) {
        return err_result(
            REASON_DUPLICATE,
            "id_cambio duplicado",
            json!([{"path": id, "reason_code": REASON_DUPLICATE, "detail": "alta duplicada"}]),
        );
    }

    ok_result(
        json!({
            "operation": op,
            "reason_codes": [REASON_OK],
            "findings": [],
            "id_cambio": id,
            "detail": detail,
            "index": index,
            "hash_integrity": hash,
            "idempotent": idempotent
        }),
        "estado propuesto",
    )
}

/// Punto de entrada de dominio. `payload` = envelope completo o `request` suelto.
pub fn execute(payload: &Value) -> Envelope {
    let request = payload.get("request").unwrap_or(payload);
    let op = request
        .get("operation")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim();
    match op {
        "verdict" => verdict(request),
        "rehash" => rehash(request),
        "alta" | "modificacion" | "baja" => mutate(request, op),
        "" => err_result(REASON_INVALID, "operation ausente", json!([])),
        other => err_result(
            REASON_INVALID,
            &format!("operation desconocida: {other}"),
            json!([]),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec(id: &str, rel: &[&str], in_diff: bool, raw_hash: bool) -> Value {
        let relacionado: Vec<Value> = rel.iter().map(|s| json!(s)).collect();
        let mut body = format!(
            "---\ncontrato_version: \"1.1.1\"\nid_cambio: \"{id}\"\nfecha: \"2026-08-13\"\ntipo_operacion: alta\ndescripcion_breve: \"t\"\nrelacionado:\n  - \"{}\"\n---\n\n# t\n",
            rel.first().copied().unwrap_or("SddIA/x")
        );
        if raw_hash {
            let h = canonical_hash(&body);
            body = format!(
                "---\ncontrato_version: \"1.1.1\"\nid_cambio: \"{id}\"\nfecha: \"2026-08-13\"\ntipo_operacion: alta\ndescripcion_breve: \"t\"\nhash_integrity: \"{h}\"\nrelacionado:\n  - \"{}\"\n---\n\n# t\n",
                rel.first().copied().unwrap_or("SddIA/x")
            );
        }
        let hash = fm_from_raw(&body);
        json!({
            "path": format!("SddIA/evolution/{id}.md"),
            "filename": format!("{id}.md"),
            "in_diff": in_diff,
            "frontmatter": {
                "contrato_version": "1.1.1",
                "id_cambio": id,
                "fecha": "2026-08-13",
                "tipo_operacion": "alta",
                "descripcion_breve": "t",
                "hash_integrity": hash,
                "relacionado": relacionado
            },
            "raw": body
        })
    }

    fn fm_from_raw(raw: &str) -> String {
        for line in raw.lines() {
            if let Some(rest) = line.trim().strip_prefix("hash_integrity:") {
                return rest.trim().trim_matches('"').to_string();
            }
        }
        String::new()
    }

    fn registry(records: Vec<Value>, ids: &[&str]) -> Value {
        let rows: Vec<Value> = ids
            .iter()
            .map(|id| json!({"id_cambio": id}))
            .collect();
        json!({
            "evolution_dir": "SddIA/evolution",
            "records": records,
            "index": { "content": "", "rows": rows }
        })
    }

    #[test]
    fn self_only_evolution_ok() {
        let req = json!({
            "operation": "verdict",
            "diff": {"paths": [{"path": "SddIA/evolution/Evolution_log.md", "status": "M"}]},
            "registry": registry(vec![], &[])
        });
        let env = execute(&req);
        assert!(env.success);
    }

    #[test]
    fn material_unregistered() {
        let req = json!({
            "operation": "verdict",
            "diff": {"paths": [{"path": "SddIA/norms/foo.md", "status": "M"}]},
            "registry": registry(vec![], &[])
        });
        let env = execute(&req);
        assert!(!env.success);
        assert_eq!(
            env.result["reason_codes"][0],
            REASON_UNREGISTERED
        );
    }

    #[test]
    fn material_covered() {
        let id = "aaaaaaaa-1111-4111-8111-aaaaaaaaaaaa";
        let rec = rec(id, &["SddIA/norms/foo.md"], true, true);
        let req = json!({
            "operation": "verdict",
            "diff": {"paths": [
                {"path": "SddIA/norms/foo.md", "status": "M"},
                {"path": format!("SddIA/evolution/{id}.md"), "status": "A"}
            ]},
            "registry": registry(vec![rec], &[id])
        });
        let env = execute(&req);
        assert!(env.success, "{}", env.message);
    }

    #[test]
    fn prefix_cover() {
        let id = "bbbbbbbb-1111-4111-8111-bbbbbbbbbbbb";
        let rec = rec(id, &["SddIA/tools/sddia-qa"], true, true);
        let req = json!({
            "operation": "verdict",
            "diff": {"paths": [
                {"path": "SddIA/tools/sddia-qa/src/main.rs", "status": "M"},
                {"path": format!("SddIA/evolution/{id}.md"), "status": "A"}
            ]},
            "registry": registry(vec![rec], &[id])
        });
        assert!(execute(&req).success);
    }

    #[test]
    fn hash_mismatch() {
        let id = "cccccccc-1111-4111-8111-cccccccccccc";
        let mut rec = rec(id, &["SddIA/norms/foo.md"], true, true);
        rec["frontmatter"]["hash_integrity"] = json!("sha256:deadbeef");
        rec["raw"] = json!("---\nhash_integrity: \"sha256:deadbeef\"\nid_cambio: x\n---\n");
        rec["frontmatter"]["id_cambio"] = json!(id);
        rec["filename"] = json!(format!("{id}.md"));
        let req = json!({
            "operation": "verdict",
            "diff": {"paths": [
                {"path": "SddIA/norms/foo.md", "status": "M"},
                {"path": format!("SddIA/evolution/{id}.md"), "status": "A"}
            ]},
            "registry": registry(vec![rec], &[id])
        });
        let env = execute(&req);
        assert!(!env.success);
        assert_eq!(env.result["reason_codes"][0], REASON_HASH);
    }

    #[test]
    fn not_indexed() {
        let id = "dddddddd-1111-4111-8111-dddddddddddd";
        let rec = rec(id, &["SddIA/norms/foo.md"], true, true);
        let req = json!({
            "operation": "verdict",
            "diff": {"paths": [
                {"path": "SddIA/norms/foo.md", "status": "M"},
                {"path": format!("SddIA/evolution/{id}.md"), "status": "A"}
            ]},
            "registry": registry(vec![rec], &[])
        });
        let env = execute(&req);
        assert!(!env.success);
        assert_eq!(env.result["reason_codes"][0], REASON_NOT_INDEXED);
    }

    #[test]
    fn alta_idempotent_and_duplicate() {
        let alta = json!({
            "operation": "alta",
            "fecha": "2026-08-13",
            "descripcion_breve": "hito",
            "relacionado": ["SddIA/tools/sddia-qa"],
            "registry": registry(vec![], &[])
        });
        let env = execute(&alta);
        assert!(env.success);
        let id = env.result["id_cambio"].as_str().unwrap().to_string();
        let dup = json!({
            "operation": "alta",
            "id_cambio": id,
            "fecha": "2026-08-13",
            "descripcion_breve": "hito",
            "relacionado": ["SddIA/tools/sddia-qa"],
            "registry": registry(vec![], &[id.as_str()])
        });
        let env2 = execute(&dup);
        assert!(!env2.success);
        assert_eq!(env2.result["reason_codes"][0], REASON_DUPLICATE);
    }

    #[test]
    fn alta_requires_fecha() {
        let env = execute(&json!({
            "operation": "alta",
            "descripcion_breve": "x",
            "relacionado": ["SddIA/x"]
        }));
        assert!(!env.success);
    }

    #[test]
    fn hash_stable() {
        let a = canonical_hash("---\nid: 1\n---\n\nbody\n");
        let b = canonical_hash("---\nid: 1\nhash_integrity: \"sha256:ff\"\n---\n\nbody\n");
        assert_eq!(a, b);
        assert!(a.starts_with("sha256:"));
    }

    #[test]
    fn modificacion_emits_detail_and_index() {
        let id = "eeeeeeee-1111-4111-8111-eeeeeeeeeeee";
        let env = execute(&json!({
            "operation": "modificacion",
            "id_cambio": id,
            "fecha": "2026-08-13",
            "descripcion_breve": "mod",
            "relacionado": ["SddIA/tools/sddia-qa"],
            "registry": registry(vec![], &[])
        }));
        assert!(env.success, "{}", env.message);
        assert!(env.result["detail"].as_str().unwrap().contains("tipo_operacion: modificacion"));
        assert!(env.result["index"].as_str().unwrap().contains(id));
    }

    #[test]
    fn baja_requires_rutas_eliminadas() {
        let env = execute(&json!({
            "operation": "baja",
            "id_cambio": "ffffffff-1111-4111-8111-ffffffffffff",
            "fecha": "2026-08-13",
            "descripcion_breve": "baja",
            "relacionado": ["SddIA/x"],
            "registry": registry(vec![], &[])
        }));
        assert!(!env.success);
        assert_eq!(env.result["reason_codes"][0], REASON_INVALID);
    }

    #[test]
    fn baja_emits_rutas() {
        let id = "ffffffff-1111-4111-8111-ffffffffffff";
        let env = execute(&json!({
            "operation": "baja",
            "id_cambio": id,
            "fecha": "2026-08-13",
            "descripcion_breve": "retiro",
            "relacionado": ["SddIA/x"],
            "rutas_eliminadas": ["SddIA/x/old.md"],
            "commit_referencia_previo": "abc123",
            "registry": registry(vec![], &[])
        }));
        assert!(env.success, "{}", env.message);
        let detail = env.result["detail"].as_str().unwrap();
        assert!(detail.contains("tipo_operacion: baja"));
        assert!(detail.contains("SddIA/x/old.md"));
    }

    #[test]
    fn alta_idempotent_when_already_persisted() {
        let alta = json!({
            "operation": "alta",
            "id_cambio": "aaaaaaaa-2222-4222-8222-aaaaaaaaaaaa",
            "fecha": "2026-08-13",
            "descripcion_breve": "hito",
            "relacionado": ["SddIA/tools/sddia-qa"],
            "registry": registry(vec![], &[])
        });
        let env = execute(&alta);
        assert!(env.success);
        let detail = env.result["detail"].as_str().unwrap().to_string();
        let index = env.result["index"].as_str().unwrap().to_string();
        let id = env.result["id_cambio"].as_str().unwrap();
        let rec = json!({
            "path": format!("SddIA/evolution/{id}.md"),
            "filename": format!("{id}.md"),
            "raw": detail,
            "frontmatter": {"id_cambio": id}
        });
        let again = execute(&json!({
            "operation": "alta",
            "id_cambio": id,
            "fecha": "2026-08-13",
            "descripcion_breve": "hito",
            "relacionado": ["SddIA/tools/sddia-qa"],
            "registry": {
                "evolution_dir": "SddIA/evolution",
                "records": [rec],
                "index": {"content": index, "rows": [{"id_cambio": id}]}
            }
        }));
        assert!(again.success);
        assert_eq!(again.result["idempotent"], true);
    }

    #[test]
    fn placeholder_format_rejected() {
        let id = "11111111-1111-4111-8111-111111111111";
        let mut rec = rec(id, &["SddIA/norms/foo.md"], true, false);
        rec["frontmatter"]["hash_integrity"] = json!("sha256:pending");
        rec["raw"] = json!(rec["raw"]);
        let req = json!({
            "operation": "verdict",
            "diff": {"paths": [{"path": format!("SddIA/evolution/{id}.md"), "status": "M"}]},
            "registry": registry(vec![rec], &[id])
        });
        let env = execute(&req);
        assert!(!env.success);
        assert_eq!(env.result["reason_codes"][0], REASON_HASH);
        let detail = env.result["findings"][0]["detail"].as_str().unwrap_or("");
        assert!(detail.contains("placeholder/formato"));
    }

    #[test]
    fn evolution_only_placeholder_fails_without_material() {
        let id = "22222222-2222-4222-8222-222222222222";
        let mut rec = rec(id, &["SddIA/x"], true, false);
        rec["frontmatter"]["hash_integrity"] = json!("sha256:pending");
        let req = json!({
            "operation": "verdict",
            "diff": {"paths": [{"path": format!("SddIA/evolution/{id}.md"), "status": "M"}]},
            "registry": registry(vec![rec], &[id])
        });
        let env = execute(&req);
        assert!(!env.success, "{}", env.message);
    }

    #[test]
    fn hash_newline_and_crlf_stable() {
        let base = "---\ncontrato_version: \"1.1.1\"\nid_cambio: \"x\"\nfecha: \"2026-08-13\"\ntipo_operacion: alta\ndescripcion_breve: \"t\"\nrelacionado:\n  - \"SddIA/x\"\n---\n\n# t";
        let with_nl = format!("{base}\n");
        let with_crlf = base.replace("\n", "\r\n");
        let h1 = canonical_hash(&with_nl);
        let h2 = canonical_hash(with_nl.trim_end());
        let h3 = canonical_hash(&with_crlf);
        assert_eq!(h1, h2);
        assert_eq!(h1, h3);
    }

    #[test]
    fn universe_audit_validates_not_in_diff() {
        let id = "33333333-3333-4333-8333-333333333333";
        let rec = rec(id, &["SddIA/x"], false, true);
        let req = json!({
            "operation": "verdict",
            "audit": "universe",
            "registry": registry(vec![rec], &[id])
        });
        assert!(execute(&req).success);
    }

    #[test]
    fn rehash_patches_placeholder() {
        let id = "44444444-4444-4444-8444-444444444444";
        let mut rec = rec(id, &["SddIA/x"], false, false);
        rec["frontmatter"]["hash_integrity"] = json!("sha256:pending");
        let raw = rec["raw"].as_str().unwrap().to_string();
        rec["raw"] = json!(raw);
        let env = execute(&json!({
            "operation": "rehash",
            "id_cambio": id,
            "registry": registry(vec![rec], &[id])
        }));
        assert!(env.success, "{}", env.message);
        let detail = env.result["detail"].as_str().unwrap();
        assert!(detail.contains("hash_integrity: \"sha256:"));
        assert!(!detail.contains("pending"));
        assert!(is_valid_hash_integrity(
            env.result["hash_integrity"].as_str().unwrap()
        ));
    }

    #[test]
    fn lib_source_has_no_git() {
        let src = include_str!("lib.rs");
        let prod = src.split("#[cfg(test)]").next().unwrap();
        assert!(!prod.contains("Command::new(\"git\")"));
        assert!(!prod.contains("git diff"));
    }
}
