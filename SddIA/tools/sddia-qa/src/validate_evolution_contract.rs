//! Validador de contrato evolution — solo lectura.

use crate::resolve;
use execute_process::core::parser::load_frontmatter_yaml;
use execute_process::core::paths::load_paths_config;
use regex::Regex;
use serde_json::{json, Value};
use serde_yaml::Value as YamlValue;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

pub(crate) fn yaml_str(v: &YamlValue) -> Option<String> {
    match v {
        YamlValue::String(s) => Some(s.clone()),
        YamlValue::Number(n) => Some(n.to_string()),
        YamlValue::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

pub(crate) fn fm_get(fm: &std::collections::HashMap<String, YamlValue>, keys: &[&str]) -> Option<String> {
    for k in keys {
        if let Some(v) = fm.get(*k) {
            if let Some(s) = yaml_str(v) {
                let t = s.trim();
                if !t.is_empty() {
                    return Some(t.to_string());
                }
            }
        }
    }
    None
}

pub(crate) fn is_uuid_v4(s: &str) -> bool {
    let re = Regex::new(
        r"(?i)^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$",
    )
    .expect("uuid regex");
    re.is_match(s.trim())
}

pub(crate) fn looks_like_uuid(s: &str) -> bool {
    let re = Regex::new(
        r"(?i)^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$",
    )
    .expect("uuid-ish regex");
    re.is_match(s.trim())
}

pub(crate) fn parse_date_ok(s: &str) -> bool {
    let t = s.trim().trim_matches('"');
    if Regex::new(r"^\d{4}-\d{2}-\d{2}$")
        .unwrap()
        .is_match(t)
    {
        return true;
    }
    // ISO-8601 datetime prefix
    Regex::new(r"^\d{4}-\d{2}-\d{2}[T ].+")
        .unwrap()
        .is_match(t)
}

fn list_nonempty(fm: &std::collections::HashMap<String, YamlValue>, keys: &[&str]) -> bool {
    for k in keys {
        if let Some(v) = fm.get(*k) {
            match v {
                YamlValue::Sequence(seq) if !seq.is_empty() => return true,
                YamlValue::String(s) if !s.trim().is_empty() => return true,
                YamlValue::Mapping(m) if !m.is_empty() => return true,
                _ => {}
            }
        }
    }
    false
}

pub(crate) fn classify_record(path: &Path, fname: &str) -> Value {
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();
    let fm = load_frontmatter_yaml(path).unwrap_or_default();

    let mut classes: BTreeSet<String> = BTreeSet::new();
    let filename_uuid = looks_like_uuid(&stem);
    let filename_uuid_v4 = is_uuid_v4(&stem);
    if !filename_uuid {
        classes.insert("NOMBRE".into());
    }

    let id = fm_get(&fm, &["id_cambio", "uuid"]);
    let id_ok = id.as_ref().map(|s| looks_like_uuid(s)).unwrap_or(false);
    let _id_v4 = id.as_ref().map(|s| is_uuid_v4(s)).unwrap_or(false);
    if !id_ok && !filename_uuid {
        classes.insert("UUID-INV".into());
    } else if id.as_ref().map(|s| !looks_like_uuid(s)).unwrap_or(false) && !filename_uuid {
        classes.insert("UUID-INV".into());
    } else if !id_ok && filename_uuid && !filename_uuid_v4 {
        classes.insert("UUID-INV".into());
    }

    let fecha = fm_get(&fm, &["fecha", "date", "created"]);
    let fecha_ok = fecha.as_ref().map(|s| parse_date_ok(s)).unwrap_or(false);
    if !fecha_ok {
        classes.insert("SIN_FECHA".into());
    }

    let lower_name = fname.to_lowercase();
    let tipo = fm_get(&fm, &["tipo_operacion", "type", "tipo", "operation", "record_kind"]);
    let estado = fm_get(&fm, &["estado", "status"]);
    if lower_name.contains("-temp")
        || lower_name.contains("_temp")
        || tipo
            .as_ref()
            .map(|t| t.to_lowercase().contains("analisis") || t.to_lowercase().contains("temp"))
            .unwrap_or(false)
        || estado
            .as_ref()
            .map(|s| s.to_lowercase().contains("borrador") || s.to_lowercase() == "draft")
            .unwrap_or(false)
    {
        classes.insert("BORRADOR".into());
    }

    let has_contract_ver = fm_get(&fm, &["contrato_version"]).is_some();
    let has_id_cambio = fm_get(&fm, &["id_cambio"]).is_some();
    let has_tipo_op = fm_get(&fm, &["tipo_operacion"]).is_some();
    let has_fecha_legacy = fm_get(&fm, &["fecha"]).is_some();
    if has_contract_ver && has_id_cambio && has_fecha_legacy && has_tipo_op {
        classes.insert("INV-L".into());
    }

    let has_atomic_uuid = fm_get(&fm, &["uuid"]).is_some();
    let has_type = fm_get(&fm, &["type"]).is_some();
    let has_version = fm_get(&fm, &["version"]).is_some();
    let has_id_kebab = fm_get(&fm, &["id"]).is_some();
    if (has_atomic_uuid || has_type || has_version || has_id_kebab)
        && !classes.contains("INV-L")
        && !has_contract_ver
    {
        classes.insert("INV-A".into());
    }

    let tipo_canon = matches!(
        tipo.as_deref(),
        Some("alta") | Some("baja") | Some("modificacion")
    );
    let hash = fm_get(&fm, &["hash_integrity"]);
    let hash_ok = hash.as_ref().map(|h| !h.trim().is_empty()).unwrap_or(false);
    let desc_ok = fm_get(&fm, &["descripcion_breve", "descripcion"]).is_some();
    let refs_ok = list_nonempty(
        &fm,
        &["relacionado", "related_entities", "artefactos_afectados"],
    );
    let canon = has_contract_ver
        && has_id_cambio
        && fecha_ok
        && tipo_canon
        && desc_ok
        && hash_ok
        && refs_ok
        && filename_uuid_v4
        && id.as_ref().map(|s| s == &stem).unwrap_or(false);

    if canon {
        classes.insert("CANONICO".into());
    }

    if classes.is_empty() {
        classes.insert("INCOMPLETO".into());
    }

    json!({
        "file": fname,
        "path": format!("SddIA/evolution/{fname}"),
        "classes": classes.iter().cloned().collect::<Vec<_>>(),
        "id": id,
        "fecha": fecha,
        "tipo": tipo,
        "hash_present": hash_ok,
        "tipo_canonico": tipo_canon,
        "canonical_v11": canon,
        "filename_uuid_v4": filename_uuid_v4,
    })
}

fn parse_audit_universe(audit_path: &Path) -> Result<Vec<String>, String> {
    let text = fs::read_to_string(audit_path).map_err(|e| format!("{}: {e}", audit_path.display()))?;
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for line in text.lines() {
        if !line.starts_with('|') {
            continue;
        }
        let parts: Vec<&str> = line.split('|').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        if parts.len() < 2 {
            continue;
        }
        let fecha = parts[0];
        if fecha == "Fecha" || fecha.starts_with('-') {
            continue;
        }
        if fecha != "SIN_FECHA" && !Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap().is_match(fecha) {
            continue;
        }
        let registro = parts[1];
        let re = Regex::new(r"`([^`]+?\.md)`").unwrap();
        if let Some(caps) = re.captures(registro) {
            let fname = caps.get(1).unwrap().as_str().to_string();
            if seen.insert(fname.clone()) {
                out.push(fname);
            }
        }
    }
    if out.is_empty() {
        return Err("universo vacío: no se parsearon filas del audit".into());
    }
    Ok(out)
}

pub(crate) fn count_log_rows(log_path: &Path) -> Result<usize, String> {
    let text = fs::read_to_string(log_path).map_err(|e| e.to_string())?;
    let re = Regex::new(r"(?m)^\| `(?:[0-9a-fA-F-]{36}|UUID-INV)` \|").unwrap();
    Ok(re.find_iter(&text).count())
}

pub(crate) fn is_evolution_meta(fname: &str) -> bool {
    let l = fname.to_ascii_lowercase();
    l == "evolution_log.md" || l == "evolution_contract.md"
}

pub(crate) fn list_official_filenames(evo_dir: &Path) -> Result<Vec<String>, String> {
    let mut names: Vec<String> = Vec::new();
    let rd = fs::read_dir(evo_dir).map_err(|e| format!("{}: {e}", evo_dir.display()))?;
    for ent in rd.flatten() {
        let p = ent.path();
        if p.extension().and_then(|x| x.to_str()) != Some("md") {
            continue;
        }
        let fname = p
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        if is_evolution_meta(&fname) {
            continue;
        }
        names.push(fname);
    }
    names.sort();
    Ok(names)
}

fn classes_for_universe(universe: &str, row: &Value) -> Vec<String> {
    let raw: Vec<String> = row
        .get("classes")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();
    if universe != "official" {
        return raw;
    }
    let canon = row
        .get("canonical_v11")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    if canon {
        vec!["CANONICO".into()]
    } else {
        raw
    }
}

fn hash_matches_canonical(path: &Path) -> bool {
    let Ok(raw) = fs::read_to_string(path) else {
        return false;
    };
    let fm = load_frontmatter_yaml(path).unwrap_or_default();
    let stored = fm_get(&fm, &["hash_integrity"]).unwrap_or_default();
    if stored.trim().is_empty() {
        return false;
    }
    let computed = sddia_evolution_register::canonical_hash(&raw);
    stored.trim() == computed
}

pub fn run(repo: &Path, args: &[String]) -> i32 {
    let json_out = args.iter().any(|a| a == "--json");
    let universe = resolve::flag_value(args, "--universe").unwrap_or("audit-cut");
    let audit_ref = resolve::flag_value(args, "--audit-ref")
        .unwrap_or("docs/audits/evolution/2026-08-11.md");

    let cfg = match load_paths_config(repo) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("cumulo: {e}");
            return 1;
        }
    };

    let contract_rel = cfg
        .pointer("/normative_documents/evolution_contract")
        .and_then(|v| v.as_str())
        .unwrap_or("SddIA/evolution/evolution_contract.md");
    let log_rel = cfg
        .pointer("/normative_documents/evolution_log")
        .and_then(|v| v.as_str())
        .unwrap_or("SddIA/evolution/Evolution_log.md");
    let evo_rel = cfg
        .pointer("/directories/evolution")
        .and_then(|v| v.as_str())
        .unwrap_or("SddIA/evolution");

    let contract_path = repo.join(contract_rel);
    let log_path = repo.join(log_rel);
    let evo_dir = repo.join(evo_rel);

    let mut errors: Vec<String> = Vec::new();
    if !contract_path.is_file() {
        errors.push(format!("contrato ausente: {contract_rel}"));
    }
    if !log_path.is_file() {
        errors.push(format!("índice ausente: {log_rel}"));
    }
    if !evo_dir.is_dir() {
        errors.push(format!("directories.evolution ausente: {evo_rel}"));
    }

    if !errors.is_empty() {
        let report = json!({
            "success": false,
            "errors": errors,
            "contract": contract_rel,
            "evolution_log": log_rel,
        });
        if json_out {
            println!("{}", serde_json::to_string_pretty(&report).unwrap_or_default());
        } else {
            for e in &errors {
                eprintln!("{e}");
            }
        }
        return 1;
    }

    let manifest_rel = resolve::flag_value(args, "--manifest");
    let extract_stems: BTreeSet<String> = manifest_rel
        .and_then(|p| fs::read_to_string(repo.join(p)).ok())
        .and_then(|raw| serde_json::from_str::<Value>(&raw).ok())
        .and_then(|v| v.get("entries").and_then(|e| e.as_array()).cloned())
        .map(|arr| {
            arr.iter()
                .filter(|e| {
                    e.get("lote").and_then(|x| x.as_str()) == Some("L4")
                        || e.get("accion").and_then(|x| x.as_str()) == Some("extract")
                })
                .filter_map(|e| {
                    e.get("old_path")
                        .and_then(|x| x.as_str())
                        .and_then(|p| Path::new(p).file_name())
                        .and_then(|s| s.to_str())
                        .map(|s| s.to_string())
                })
                .collect()
        })
        .unwrap_or_default();

    let files: Vec<String> = match universe {
        "audit-cut" => {
            let audit_path = if Path::new(audit_ref).is_absolute() {
                PathBuf::from(audit_ref)
            } else {
                repo.join(audit_ref)
            };
            match parse_audit_universe(&audit_path) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{e}");
                    return 1;
                }
            }
        }
        "official" => match list_official_filenames(&evo_dir) {
            Ok(v) => v
                .into_iter()
                .filter(|n| !extract_stems.contains(n))
                .collect(),
            Err(e) => {
                eprintln!("{e}");
                return 1;
            }
        },
        other => {
            eprintln!("universe no soportado: {other} (use audit-cut|official)");
            return 1;
        }
    };

    let mut rows = Vec::new();
    let mut by_class: BTreeMap<String, usize> = BTreeMap::new();
    let mut missing = Vec::new();
    let mut non_exclusive = Vec::new();
    let mut hash_mismatch = Vec::new();

    for fname in &files {
        let path = evo_dir.join(fname);
        if !path.is_file() {
            missing.push(fname.clone());
            continue;
        }
        // Solo lectura: load_frontmatter_yaml / metadata; no write.
        let mut row = classify_record(&path, fname);
        let classes = classes_for_universe(universe, &row);
        if universe == "official" {
            if let Some(obj) = row.as_object_mut() {
                obj.insert("classes".into(), json!(classes.clone()));
            }
            let exclusive = classes.len() == 1 && classes[0] == "CANONICO";
            if !exclusive {
                non_exclusive.push(fname.clone());
            }
            if exclusive && !hash_matches_canonical(&path) {
                hash_mismatch.push(fname.clone());
            }
        }
        for s in &classes {
            *by_class.entry(s.clone()).or_insert(0) += 1;
        }
        rows.push(row);
    }

    let log_rows = count_log_rows(&log_path).unwrap_or(0);
    let canon_count = *by_class.get("CANONICO").unwrap_or(&0);
    let official_ok = universe != "official"
        || (missing.is_empty()
            && non_exclusive.is_empty()
            && hash_mismatch.is_empty()
            && canon_count == rows.len()
            && log_rows == rows.len());
    let success = missing.is_empty() && official_ok;
    let report = json!({
        "success": success,
        "mode": "read-only",
        "universe": universe,
        "audit_ref": audit_ref,
        "contract": contract_rel,
        "evolution_log": log_rel,
        "evolution_dir": evo_rel,
        "universe_total": files.len(),
        "classified_total": rows.len(),
        "missing": missing,
        "non_exclusive_canonico": non_exclusive,
        "hash_mismatch": hash_mismatch,
        "evolution_log_rows": log_rows,
        "log_matches_universe": log_rows == files.len(),
        "by_class": by_class,
        "rows": rows,
    });

    if json_out {
        println!("{}", serde_json::to_string_pretty(&report).unwrap_or_default());
    } else {
        println!(
            "classified_total={} universe_total={} log_rows={} missing={}",
            rows.len(),
            files.len(),
            log_rows,
            missing.len()
        );
        for (k, v) in &by_class {
            println!("  {k}={v}");
        }
    }

    if success {
        0
    } else {
        1
    }
}
