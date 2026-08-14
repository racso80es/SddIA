//! Migrador de históricos evolution (EV-AUD-002/007).
//! Modos: manifest | apply | verify.

use crate::validate_evolution_contract::{
    classify_record, fm_get, is_uuid_v4, list_official_filenames, looks_like_uuid,
    parse_date_ok, yaml_str,
};
use chrono::Utc;
use execute_process::core::parser::load_frontmatter_yaml;
use execute_process::core::paths::load_paths_config;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_yaml::Value as YamlValue;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use uuid::Uuid;

const EVO_REL: &str = "SddIA/evolution";
const DRAFT_DIR: &str = "docs/audits/evolution/drafts";
const SKIP_ID: &str = "0bceeb41-64d1-4920-af9d-46a11c0455a2";
const TARGET_CONTRACT: &str = "1.1.1";
const CORRELATION: &str = "4b9de6b3-c400-49c8-86f2-55f08ec64ce4";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Manifest {
    manifest_version: String,
    feature: String,
    correlation_id: String,
    contrato_version_target: String,
    frozen_at: String,
    universe_total_official: usize,
    draft_extractions: usize,
    repo_commit_at_freeze: String,
    blocked_items: Vec<Value>,
    entries: Vec<Entry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Entry {
    seq: usize,
    lote: String,
    old_path: String,
    new_path: String,
    id_cambio: Option<String>,
    accion: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    uuid_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fecha_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fecha_derivacion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tipo_operacion: Option<String>,
    classes_detected: Vec<String>,
    hash_action: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    blocked_reason: Option<String>,
}

fn flag_value<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

fn evo_dir(repo: &Path) -> PathBuf {
    repo.join(EVO_REL)
}

fn git_rev_parse(repo: &Path) -> String {
    Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(repo)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "UNKNOWN".into())
}

fn git_first_add(repo: &Path, rel: &str) -> Option<String> {
    let out = Command::new("git")
        .args([
            "log",
            "--diff-filter=A",
            "--follow",
            "--reverse",
            "--format=%aI",
            "--",
            rel,
        ])
        .current_dir(repo)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    String::from_utf8(out.stdout)
        .ok()?
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .map(|s| s.to_string())
}

fn split_body(raw: &str) -> String {
    let t = raw.trim_start_matches('\u{feff}');
    if !t.starts_with("---") {
        return raw.to_string();
    }
    let rest = t.strip_prefix("---").unwrap_or(t);
    let rest = rest.strip_prefix('\n').or_else(|| rest.strip_prefix('\r')).unwrap_or(rest);
    if let Some(idx) = rest.find("\n---") {
        let after = &rest[idx + 4..];
        return after
            .strip_prefix('\n')
            .or_else(|| after.strip_prefix("\r\n"))
            .unwrap_or(after)
            .to_string();
    }
    raw.to_string()
}

fn h1_title(body: &str) -> Option<String> {
    for line in body.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("# ") {
            let s = rest.trim();
            if !s.is_empty() {
                return Some(s.to_string());
            }
        }
    }
    None
}

fn yaml_seq_strings(v: &YamlValue) -> Vec<String> {
    match v {
        YamlValue::String(s) if !s.trim().is_empty() => vec![s.trim().to_string()],
        YamlValue::Sequence(seq) => {
            let mut out = Vec::new();
            for x in seq {
                match x {
                    YamlValue::String(s) if !s.trim().is_empty() => out.push(s.trim().to_string()),
                    YamlValue::Mapping(m) => {
                        let name = m
                            .get(&YamlValue::String("name".into()))
                            .and_then(yaml_str);
                        let uuid = m
                            .get(&YamlValue::String("uuid".into()))
                            .and_then(yaml_str);
                        match (name, uuid) {
                            (Some(n), Some(u)) => out.push(format!("{u}:{n}")),
                            (Some(n), None) => out.push(n),
                            (None, Some(u)) => out.push(u),
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            out
        }
        _ => vec![],
    }
}

fn collect_relacionado(fm: &HashMap<String, YamlValue>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for k in ["relacionado", "related_entities", "artefactos_afectados"] {
        if let Some(v) = fm.get(k) {
            for s in yaml_seq_strings(v) {
                if seen.insert(s.clone()) {
                    out.push(s);
                }
            }
        }
    }
    out
}

fn map_tipo(raw: Option<&str>) -> String {
    let Some(t) = raw.map(str::trim).filter(|s| !s.is_empty()) else {
        return "modificacion".into();
    };
    let l = t.to_ascii_lowercase();
    match l.as_str() {
        "alta" | "baja" | "modificacion" => l,
        _ => "modificacion".into(),
    }
}

fn normalize_fecha(raw: &str) -> String {
    let t = raw.trim().trim_matches('"');
    if RegexDate::ymd(t) {
        return t.to_string();
    }
    if t.len() >= 10 && RegexDate::ymd(&t[..10]) && (t.as_bytes().get(10) == Some(&b'T') || t.as_bytes().get(10) == Some(&b' '))
    {
        return t.to_string();
    }
    t.to_string()
}

struct RegexDate;
impl RegexDate {
    fn ymd(s: &str) -> bool {
        parse_date_ok(s) && s.len() == 10
    }
}

fn yaml_escape(s: &str) -> String {
    if s.is_empty()
        || s.chars().any(|c| {
            matches!(c, ':' | '#' | '"' | '\'' | '{' | '}' | '[' | ']' | ',' | '&' | '*')
                || c.is_whitespace() && c != ' '
        })
        || s.starts_with(['&', '*', '!', '%', '@', '`'])
        || s.contains(": ")
    {
        format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
    } else if s.contains(' ') || s.contains(':') {
        format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
    } else {
        s.to_string()
    }
}

fn emit_record(
    id: &str,
    fecha: &str,
    tipo: &str,
    desc: &str,
    relacionado: &[String],
    origen: Option<&str>,
    extras: &BTreeMap<String, String>,
    body: &str,
    hash_placeholder: bool,
) -> String {
    let mut fm = String::from("---\n");
    fm.push_str(&format!("contrato_version: \"{TARGET_CONTRACT}\"\n"));
    fm.push_str(&format!("id_cambio: \"{id}\"\n"));
    fm.push_str(&format!("fecha: {}\n", yaml_escape(fecha)));
    fm.push_str(&format!("tipo_operacion: {tipo}\n"));
    fm.push_str(&format!("descripcion_breve: {}\n", yaml_escape(desc)));
    if hash_placeholder {
        fm.push_str("hash_integrity: \"\"\n");
    }
    fm.push_str("relacionado:\n");
    if relacionado.is_empty() {
        fm.push_str("  - \"SddIA/evolution\"\n");
    } else {
        for r in relacionado {
            fm.push_str(&format!("  - {}\n", yaml_escape(r)));
        }
    }
    if let Some(o) = origen {
        fm.push_str(&format!("origen_migracion: {}\n", yaml_escape(o)));
    }
    for (k, v) in extras {
        if matches!(
            k.as_str(),
            "contrato_version"
                | "id_cambio"
                | "fecha"
                | "tipo_operacion"
                | "descripcion_breve"
                | "hash_integrity"
                | "relacionado"
                | "origen_migracion"
                | "uuid"
                | "date"
                | "created"
                | "type"
                | "tipo"
                | "operation"
                | "record_kind"
                | "related_entities"
                | "artefactos_afectados"
                | "version"
        ) {
            continue;
        }
        fm.push_str(&format!("{k}: {}\n", yaml_escape(v)));
    }
    fm.push_str("---\n\n");
    fm.push_str(body.trim_start_matches('\n'));
    if !fm.ends_with('\n') {
        fm.push('\n');
    }
    if hash_placeholder {
        let hash = sddia_evolution_register::canonical_hash(&fm);
        fm = fm.replacen("hash_integrity: \"\"\n", &format!("hash_integrity: \"{hash}\"\n"), 1);
    }
    fm
}

fn extras_from_fm(fm: &HashMap<String, YamlValue>) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for k in ["id", "autor", "source_feature", "document_id", "contexto", "impacto", "proyecto_origen_cambio"] {
        if let Some(s) = fm_get(fm, &[k]) {
            out.insert(k.to_string(), s);
        }
    }
    out
}

fn assign_lote(classes: &[String], fname: &str) -> String {
    let stem = fname.trim_end_matches(".md");
    if classes.iter().any(|c| c == "BORRADOR") {
        return "L4".into();
    }
    if stem == SKIP_ID {
        return "SKIP".into();
    }
    if classes.iter().any(|c| matches!(c.as_str(), "NOMBRE" | "UUID-INV" | "SIN_FECHA"))
        || (looks_like_uuid(stem) && !is_uuid_v4(stem))
    {
        return "L3".into();
    }
    if classes.iter().any(|c| c == "INV-L") {
        return "L2".into();
    }
    "L1".into()
}

fn identity_for(
    _repo: &Path,
    fname: &str,
    fm: &HashMap<String, YamlValue>,
    classes: &[String],
    lote: &str,
) -> (String, Option<String>, Option<String>, Option<String>, Option<String>) {
    // returns (accion, id_cambio, uuid_source, new_filename, blocked)
    let stem = fname.trim_end_matches(".md");
    if lote == "L4" {
        return (
            "extract".into(),
            None,
            None,
            Some(format!("{DRAFT_DIR}/{fname}")),
            None,
        );
    }
    if lote == "SKIP" {
        return (
            "skip".into(),
            Some(SKIP_ID.into()),
            Some("filename".into()),
            Some(format!("{EVO_REL}/{fname}")),
            None,
        );
    }

    let fm_id = fm_get(fm, &["id_cambio", "uuid"]);
    let fm_v4 = fm_id.as_ref().filter(|s| is_uuid_v4(s)).cloned();
    let stem_v4 = is_uuid_v4(stem);

    if stem_v4 {
        if let Some(ref fid) = fm_v4 {
            if fid != stem {
                return (
                    "block".into(),
                    None,
                    None,
                    None,
                    Some(format!("filename UUID v4 ≠ frontmatter UUID v4 ({stem} vs {fid})")),
                );
            }
        }
        let accion = if lote == "L3" && classes.iter().any(|c| c == "SIN_FECHA") {
            "normalize_fm"
        } else if fname != format!("{stem}.md") {
            "normalize_and_rename"
        } else {
            "normalize_fm"
        };
        return (
            accion.into(),
            Some(stem.to_string()),
            Some("filename".into()),
            Some(format!("{EVO_REL}/{fname}")),
            None,
        );
    }

    if let Some(fid) = fm_v4 {
        return (
            "normalize_and_rename".into(),
            Some(fid.clone()),
            Some("frontmatter_uuid".into()),
            Some(format!("{EVO_REL}/{fid}.md")),
            None,
        );
    }

    let gen = Uuid::new_v4().to_string();
    (
        "normalize_and_rename".into(),
        Some(gen.clone()),
        Some("manifest_v4".into()),
        Some(format!("{EVO_REL}/{gen}.md")),
        None,
    )
}

fn fecha_for(repo: &Path, rel: &str, fm: &HashMap<String, YamlValue>) -> (Option<String>, Option<String>, Option<String>, Option<String>) {
    // fecha, source, derivacion, blocked
    if let Some(f) = fm_get(fm, &["fecha"]).filter(|s| parse_date_ok(s)) {
        return (Some(normalize_fecha(&f)), Some("fecha".into()), None, None);
    }
    if let Some(f) = fm_get(fm, &["date"]).filter(|s| parse_date_ok(s)) {
        return (Some(normalize_fecha(&f)), Some("date".into()), None, None);
    }
    if let Some(f) = fm_get(fm, &["created"]).filter(|s| parse_date_ok(s)) {
        return (Some(normalize_fecha(&f)), Some("created".into()), None, None);
    }
    if let Some(iso) = git_first_add(repo, rel) {
        let day = if iso.len() >= 10 { iso[..10].to_string() } else { iso.clone() };
        return (
            Some(day),
            Some("git_first_add".into()),
            Some(iso),
            None,
        );
    }
    (
        None,
        None,
        None,
        Some(format!("SIN_FECHA sin evidencia git: {rel}")),
    )
}

fn build_manifest(repo: &Path) -> Result<Manifest, String> {
    let dir = evo_dir(repo);
    let names = list_official_filenames(&dir)?;
    let mut entries = Vec::new();
    let mut blocked = Vec::new();
    let mut used_ids: HashSet<String> = HashSet::new();

    let mut rows: Vec<(String, Value, HashMap<String, YamlValue>)> = Vec::new();
    for fname in &names {
        let path = dir.join(fname);
        let row = classify_record(&path, fname);
        let fm = load_frontmatter_yaml(&path).unwrap_or_default();
        rows.push((fname.clone(), row, fm));
    }

    for (fname, row, fm) in &rows {
        let classes: Vec<String> = row
            .get("classes")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|x| x.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let lote = assign_lote(&classes, fname);
        let old_path = format!("{EVO_REL}/{fname}");
        let (mut accion, mut id_opt, uuid_source, new_path_opt, id_block) =
            identity_for(repo, fname, fm, &classes, &lote);

        let (fecha, fecha_source, fecha_der, fecha_block) = if lote == "L4" || lote == "SKIP" {
            (fm_get(fm, &["fecha", "date", "created"]), None, None, None)
        } else {
            fecha_for(repo, &old_path, fm)
        };

        let mut blocked_reason = id_block.or(fecha_block);
        if let Some(ref id) = id_opt {
            if lote != "L4" && !used_ids.insert(id.clone()) {
                blocked_reason = Some(format!("colisión UUID {id}"));
            }
        }

        if let Some(ref reason) = blocked_reason {
            accion = "block".into();
            id_opt = None;
            blocked.push(json!({ "old_path": old_path, "reason": reason }));
        }

        let tipo = if lote == "L4" {
            None
        } else {
            Some(map_tipo(fm_get(fm, &["tipo_operacion", "type", "tipo", "operation", "record_kind"]).as_deref()))
        };

        let new_path = new_path_opt.unwrap_or_else(|| old_path.clone());
        let hash_action = match accion.as_str() {
            "skip" => "preserve",
            "extract" | "block" => "none",
            _ => "compute",
        };

        let _ = fecha;
        entries.push(Entry {
            seq: 0,
            lote,
            old_path,
            new_path,
            id_cambio: id_opt,
            accion,
            uuid_source,
            fecha_source,
            fecha_derivacion: fecha_der,
            tipo_operacion: tipo,
            classes_detected: classes,
            hash_action: hash_action.into(),
            blocked_reason,
        });
    }

    entries.sort_by(|a, b| (&a.lote, &a.old_path).cmp(&(&b.lote, &b.old_path)));
    for (i, e) in entries.iter_mut().enumerate() {
        e.seq = i + 1;
    }

    let drafts = entries.iter().filter(|e| e.lote == "L4").count();
    let official = entries.iter().filter(|e| e.lote != "L4" && e.accion != "block").count();

    Ok(Manifest {
        manifest_version: "1.0.0".into(),
        feature: "evolution-history-normalization".into(),
        correlation_id: CORRELATION.into(),
        contrato_version_target: TARGET_CONTRACT.into(),
        frozen_at: Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        universe_total_official: official,
        draft_extractions: drafts,
        repo_commit_at_freeze: git_rev_parse(repo),
        blocked_items: blocked,
        entries,
    })
}

fn load_manifest(path: &Path) -> Result<Manifest, String> {
    let raw = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(&raw).map_err(|e| format!("manifest JSON: {e}"))
}

fn apply_entry(repo: &Path, e: &Entry) -> Result<(), String> {
    if e.accion == "block" || e.accion == "skip" {
        return Ok(());
    }
    let src = repo.join(&e.old_path);
    if e.accion == "extract" {
        let dest = repo.join(&e.new_path);
        if dest.is_file() && !src.exists() {
            return Ok(());
        }
        if !src.is_file() {
            return Err(format!("ausente: {}", e.old_path));
        }
        if let Some(p) = dest.parent() {
            fs::create_dir_all(p).map_err(|err| err.to_string())?;
        }
        fs::copy(&src, &dest).map_err(|err| err.to_string())?;
        fs::remove_file(&src).map_err(|err| err.to_string())?;
        return Ok(());
    }
    if !src.is_file() {
        if repo.join(&e.new_path).is_file() {
            return Ok(());
        }
        return Err(format!("ausente: {}", e.old_path));
    }

    let raw = fs::read_to_string(&src).map_err(|err| err.to_string())?;
    let fm = load_frontmatter_yaml(&src).unwrap_or_default();
    let body = split_body(&raw);
    let id = e
        .id_cambio
        .clone()
        .ok_or_else(|| format!("id_cambio ausente: {}", e.old_path))?;

    let fecha = if let Some(d) = &e.fecha_derivacion {
        if d.len() >= 10 {
            d[..10].to_string()
        } else {
            d.clone()
        }
    } else {
        fm_get(&fm, &["fecha", "date", "created"])
            .map(|s| normalize_fecha(&s))
            .ok_or_else(|| format!("fecha ausente: {}", e.old_path))?
    };
    let tipo = e
        .tipo_operacion
        .clone()
        .unwrap_or_else(|| map_tipo(fm_get(&fm, &["tipo_operacion", "type", "tipo"]).as_deref()));
    let desc = fm_get(&fm, &["descripcion_breve", "descripcion"])
        .or_else(|| h1_title(&body))
        .unwrap_or_else(|| id.clone());
    let mut rel = collect_relacionado(&fm);
    let origen = if e.old_path != e.new_path {
        Some(e.old_path.as_str())
    } else {
        None
    };
    if let Some(o) = origen {
        let tag = format!("origen:{o}");
        if !rel.iter().any(|x| x == &tag) {
            rel.push(tag);
        }
    }
    let extras = extras_from_fm(&fm);
    let rendered = emit_record(
        &id,
        &fecha,
        &tipo,
        &desc,
        &rel,
        origen,
        &extras,
        &body,
        e.hash_action == "compute",
    );

    let dest = repo.join(&e.new_path);
    if let Some(p) = dest.parent() {
        fs::create_dir_all(p).map_err(|err| err.to_string())?;
    }
    if e.old_path != e.new_path && src != dest {
        fs::write(&dest, rendered).map_err(|err| err.to_string())?;
        fs::remove_file(&src).map_err(|err| err.to_string())?;
    } else {
        fs::write(&src, rendered).map_err(|err| err.to_string())?;
    }
    Ok(())
}

fn rebuild_index(repo: &Path, man: &Manifest) -> Result<(), String> {
    let cfg = load_paths_config(repo).ok();
    let log_rel = cfg
        .as_ref()
        .and_then(|c| c.pointer("/normative_documents/evolution_log"))
        .and_then(|v| v.as_str())
        .unwrap_or("SddIA/evolution/Evolution_log.md");
    let evo = evo_dir(repo);
    let mut rows: Vec<(String, String, String, String)> = Vec::new();
    for e in &man.entries {
        if e.lote == "L4" || e.accion == "block" {
            continue;
        }
        let fname = Path::new(&e.new_path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        let path = evo.join(fname);
        if !path.is_file() {
            continue;
        }
        let fm = load_frontmatter_yaml(&path).unwrap_or_default();
        let id = fm_get(&fm, &["id_cambio"]).unwrap_or_else(|| fname.trim_end_matches(".md").into());
        let fecha = fm_get(&fm, &["fecha"]).unwrap_or_else(|| "SIN_FECHA".into());
        let desc = fm_get(&fm, &["descripcion_breve"]).unwrap_or_else(|| "—".into());
        rows.push((fecha, id, desc, format!("{EVO_REL}/{fname}")));
    }
    rows.sort_by(|a, b| {
        let a_sf = a.0 == "SIN_FECHA" || !parse_date_ok(&a.0);
        let b_sf = b.0 == "SIN_FECHA" || !parse_date_ok(&b.0);
        match (a_sf, b_sf) {
            (true, false) => std::cmp::Ordering::Greater,
            (false, true) => std::cmp::Ordering::Less,
            (true, true) => a.3.cmp(&b.3),
            (false, false) => b.0.cmp(&a.0).then_with(|| a.3.cmp(&b.3)),
        }
    });

    let mut md = String::from("---\n");
    md.push_str(&format!("contrato_version: \"{TARGET_CONTRACT}\"\n"));
    md.push_str("universe_total: 64\n");
    md.push_str("source_audit: docs/audits/evolution/2026-08-11.md\n");
    md.push_str("migration_manifest: docs/features/evolution-history-normalization/migration-manifest.json\n");
    md.push_str("---\n\n");
    md.push_str("# Evolution_log\n\n");
    md.push_str("| id_cambio | fecha | resumen | clase_formato | ruta_relativa |\n");
    md.push_str("|-----------|-------|---------|---------------|---------------|\n");
    for (fecha, id, desc, ruta) in &rows {
        let brief = desc.replace('|', "/");
        md.push_str(&format!(
            "| `{id}` | {fecha} | {brief} | CANONICO | `{ruta}` |\n"
        ));
    }
    fs::write(repo.join(log_rel), md).map_err(|e| e.to_string())
}

fn verify(repo: &Path, man: &Manifest) -> Result<Value, String> {
    let mut drift = Vec::new();
    for e in &man.entries {
        if e.accion == "block" {
            continue;
        }
        if e.lote == "L4" {
            if repo.join(&e.old_path).exists() {
                drift.push(json!({"kind": "draft_still_in_evolution", "path": e.old_path}));
            }
            if !repo.join(&e.new_path).is_file() {
                drift.push(json!({"kind": "draft_missing", "path": e.new_path}));
            }
            continue;
        }
        if !repo.join(&e.new_path).is_file() {
            drift.push(json!({"kind": "missing", "path": e.new_path}));
        }
    }
    let evo = evo_dir(repo);
    for fname in list_official_filenames(&evo)? {
        if fname.contains("-temp") || fname.contains("_temp") {
            drift.push(json!({"kind": "temp_in_evolution", "file": fname}));
        }
    }
    Ok(json!({
        "success": drift.is_empty(),
        "drift": drift,
        "universe_total_official": man.universe_total_official,
    }))
}

pub fn run(repo: &Path, args: &[String]) -> i32 {
    let json_out = args.iter().any(|a| a == "--json");
    let dry = args.iter().any(|a| a == "--dry-run");
    let mode = args
        .iter()
        .find(|a| matches!(a.as_str(), "manifest" | "apply" | "verify" | "reindex"))
        .map(|s| s.as_str())
        .unwrap_or("");
    if mode.is_empty() {
        eprintln!("uso: migrate-evolution-history manifest|apply|verify|reindex");
        return 1;
    }

    let default_write = "docs/features/evolution-history-normalization/migration-manifest.json";
    let write_path = flag_value(args, "--write").unwrap_or(default_write);
    let manifest_path = flag_value(args, "--manifest").unwrap_or(default_write);
    let lote_filter = flag_value(args, "--lote");

    match mode {
        "manifest" => match build_manifest(repo) {
            Ok(man) => {
                let v = serde_json::to_value(&man).unwrap_or(json!({}));
                if !dry {
                    let p = repo.join(write_path);
                    if let Some(parent) = p.parent() {
                        let _ = fs::create_dir_all(parent);
                    }
                    if let Ok(s) = serde_json::to_string_pretty(&man) {
                        let _ = fs::write(&p, s);
                    }
                }
                if json_out {
                    println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
                } else {
                    println!(
                        "entries={} official={} drafts={} blocked={} frozen_at={}",
                        man.entries.len(),
                        man.universe_total_official,
                        man.draft_extractions,
                        man.blocked_items.len(),
                        man.frozen_at
                    );
                }
                if man.blocked_items.is_empty() {
                    0
                } else {
                    1
                }
            }
            Err(e) => {
                eprintln!("{e}");
                1
            }
        },
        "apply" => match load_manifest(&repo.join(manifest_path)) {
            Ok(man) => {
                if man.frozen_at.trim().is_empty() {
                    eprintln!("apply sin frozen_at");
                    return 1;
                }
                if !man.blocked_items.is_empty() {
                    eprintln!("apply bloqueado: {} ítems", man.blocked_items.len());
                    return 1;
                }
                let mut err = 0;
                for e in &man.entries {
                    if let Some(lf) = lote_filter {
                        if e.lote != lf {
                            continue;
                        }
                    }
                    if dry {
                        continue;
                    }
                    if let Err(er) = apply_entry(repo, e) {
                        eprintln!("{}: {er}", e.old_path);
                        err += 1;
                    }
                }
                if lote_filter.is_none() && err == 0 && !dry {
                    if let Err(e) = rebuild_index(repo, &man) {
                        eprintln!("reindex: {e}");
                        err += 1;
                    }
                }
                if err == 0 {
                    0
                } else {
                    1
                }
            }
            Err(e) => {
                eprintln!("{e}");
                1
            }
        },
        "verify" => match load_manifest(&repo.join(manifest_path)) {
            Ok(man) => match verify(repo, &man) {
                Ok(rep) => {
                    if json_out {
                        println!("{}", serde_json::to_string_pretty(&rep).unwrap_or_default());
                    } else {
                        println!(
                            "success={} drift={}",
                            rep.get("success").and_then(|v| v.as_bool()).unwrap_or(false),
                            rep.get("drift")
                                .and_then(|v| v.as_array())
                                .map(|a| a.len())
                                .unwrap_or(0)
                        );
                    }
                    if rep.get("success") == Some(&json!(true)) {
                        0
                    } else {
                        1
                    }
                }
                Err(e) => {
                    eprintln!("{e}");
                    1
                }
            },
            Err(e) => {
                eprintln!("{e}");
                1
            }
        },
        "reindex" => match load_manifest(&repo.join(manifest_path)) {
            Ok(man) => {
                if let Err(e) = rebuild_index(repo, &man) {
                    eprintln!("reindex: {e}");
                    1
                } else if json_out {
                    println!(
                        "{}",
                        json!({"success": true, "universe_total_official": man.universe_total_official})
                    );
                    0
                } else {
                    println!("reindex ok universe={}", man.universe_total_official);
                    0
                }
            }
            Err(e) => {
                eprintln!("{e}");
                1
            }
        },
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lote_borrador_is_l4() {
        assert_eq!(assign_lote(&["BORRADOR".into(), "INV-A".into()], "x-temp.md"), "L4");
    }

    #[test]
    fn lote_skip_canonical() {
        assert_eq!(
            assign_lote(&["CANONICO".into()], &format!("{SKIP_ID}.md")),
            "SKIP"
        );
    }

    #[test]
    fn lote_nombre_is_l3() {
        assert_eq!(assign_lote(&["NOMBRE".into(), "INV-A".into()], "foo.md"), "L3");
    }

    #[test]
    fn map_tipo_default_modificacion() {
        assert_eq!(map_tipo(Some("feature")), "modificacion");
        assert_eq!(map_tipo(Some("alta")), "alta");
        assert_eq!(map_tipo(None), "modificacion");
    }

    #[test]
    fn hash_parity_with_evolution_register() {
        let rendered = emit_record(
            "aaaaaaaa-1111-4111-8111-aaaaaaaaaaaa",
            "2026-08-14",
            "modificacion",
            "parity",
            &["SddIA/evolution".into()],
            None,
            &BTreeMap::new(),
            "body\n",
            true,
        );
        let computed = sddia_evolution_register::canonical_hash(&rendered);
        assert!(
            rendered.contains(&format!("hash_integrity: \"{computed}\"")),
            "hash_integrity debe coincidir con canonical_hash"
        );
    }
}
