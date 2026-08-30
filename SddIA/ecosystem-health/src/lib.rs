//! Fusión map-snapshot × territorio (Argos / Radamanto / Cerbero) para el Espejo de Consciencia.

use chrono::Utc;
use regex::Regex;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const MISSED_CYCLES_THRESHOLD: i64 = 3;
const STATS_META_KEYS: &[&str] = &["cognitive", "entities"];

#[derive(Debug, Clone, Copy)]
pub struct FuseOptions {
    pub compile_map: bool,
    pub persist: bool,
}

impl Default for FuseOptions {
    fn default() -> Self {
        Self {
            compile_map: false,
            persist: true,
        }
    }
}

#[derive(Clone)]
struct MapEntry {
    id: String,
    uuid: String,
}

fn iso_now() -> String {
    Utc::now().to_rfc3339()
}

fn normalize_rel(rel: &str) -> String {
    rel.trim().trim_start_matches("./").to_string()
}

pub fn load_cumulo(repo: &Path) -> Result<Value, String> {
    let path = repo.join("SddIA/core/cumulo.paths.json");
    let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&text).map_err(|e| e.to_string())
}

fn observability_path(repo: &Path, cfg: &Value, key: &str, default: &str) -> PathBuf {
    cfg.get("observability")
        .and_then(|o| o.get(key))
        .and_then(|v| v.as_str())
        .map(normalize_rel)
        .map(|r| repo.join(r))
        .unwrap_or_else(|| repo.join(default.trim_start_matches("./")))
}

fn map_snapshot_path(repo: &Path, cfg: &Value) -> PathBuf {
    observability_path(
        repo,
        cfg,
        "map_snapshot",
        ".SddIA/observability/map-snapshot.json",
    )
}

fn ecosystem_health_path(repo: &Path, cfg: &Value) -> PathBuf {
    observability_path(
        repo,
        cfg,
        "ecosystem_health",
        ".SddIA/observability/ecosystem-health.json",
    )
}

fn heartbeat_audit_path(repo: &Path, cfg: &Value) -> PathBuf {
    let rel = cfg
        .get("daemons_instance")
        .and_then(|d| d.get("state"))
        .and_then(|v| v.as_str())
        .unwrap_or(".SddIA/daemons/state");
    repo.join(normalize_rel(rel)).join("heartbeat-audit.json")
}

fn stats_path(repo: &Path, cfg: &Value) -> PathBuf {
    let rel = cfg
        .get("radamanto")
        .and_then(|r| r.get("stats"))
        .and_then(|v| v.as_str())
        .unwrap_or(".SddIA/radamanto/stats.json");
    repo.join(normalize_rel(rel))
}

fn revoked_path(repo: &Path, cfg: &Value) -> PathBuf {
    let rel = cfg
        .get("radamanto")
        .and_then(|r| r.get("revoked_entities"))
        .and_then(|v| v.as_str())
        .unwrap_or(".SddIA/cerbero/revoked_entities.json");
    repo.join(normalize_rel(rel))
}

fn read_json_file(path: &Path) -> Option<Value> {
    if !path.is_file() {
        return None;
    }
    let text = fs::read_to_string(path).ok()?;
    serde_json::from_str(&text).ok()
}

fn write_json_atomic(path: &Path, data: &Value) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let serialized = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    let tmp = path.with_extension("json.tmp");
    {
        let mut file = fs::File::create(&tmp).map_err(|e| e.to_string())?;
        file.write_all(serialized.as_bytes())
            .map_err(|e| e.to_string())?;
        file.sync_all().map_err(|e| e.to_string())?;
    }
    fs::rename(&tmp, path).map_err(|e| e.to_string())
}

fn looks_like_uuid(s: &str) -> bool {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        Regex::new(r"(?i)^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$")
            .expect("uuid regex")
    });
    re.is_match(s.trim())
}

fn stem_from_archivo(archivo: &str) -> String {
    archivo
        .trim()
        .trim_matches('`')
        .replace(".md", "")
}

/// Parsea filas de catálogo `index.md` (tools/skills/daemons).
pub(crate) fn parse_catalog_index(index_path: &Path) -> Vec<MapEntry> {
    let Ok(text) = fs::read_to_string(index_path) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for line in text.lines() {
        if !line.starts_with('|') || line.contains("---") {
            continue;
        }
        let cols: Vec<&str> = line.split('|').map(str::trim).collect();
        if cols.len() < 4 {
            continue;
        }
        let archivo = cols[1];
        if archivo.is_empty()
            || archivo.contains("Archivo")
            || archivo.eq_ignore_ascii_case("uuid")
        {
            continue;
        }
        let uuid_col = cols[2].trim().trim_matches('`');
        if !looks_like_uuid(uuid_col) {
            continue;
        }
        let name_col = cols.get(3).copied().unwrap_or("");
        let id = if !name_col.is_empty() && !looks_like_uuid(name_col) && !name_col.contains('.') {
            name_col.to_string()
        } else {
            stem_from_archivo(archivo)
        };
        if id.is_empty() {
            continue;
        }
        out.push(MapEntry {
            id,
            uuid: uuid_col.to_string(),
        });
    }
    out
}

fn directory_rel(cfg: &Value, key: &str, default: &str) -> String {
    cfg.get("directories")
        .and_then(|d| d.get(key))
        .and_then(|v| v.as_str())
        .map(normalize_rel)
        .unwrap_or_else(|| normalize_rel(default))
}

/// Compila inventario esperado desde índices Cúmulo (sin infrastructure_adapters).
pub fn compile_map_snapshot(repo: &Path) -> Result<Value, String> {
    let cfg = load_cumulo(repo)?;
    let tools_dir = directory_rel(&cfg, "tools", "SddIA/tools");
    let skills_dir = directory_rel(&cfg, "skills", "SddIA/skills");
    let daemons_dir = directory_rel(&cfg, "daemons", "SddIA/daemons");

    let families = |family: &str, dir: &str| -> Vec<Value> {
        parse_catalog_index(&repo.join(dir).join("index.md"))
            .into_iter()
            .map(|e| json!({"id": e.id, "uuid": e.uuid, "family": family}))
            .collect()
    };

    let snapshot = json!({
        "compiled_at": iso_now(),
        "compiler": "compile-ecosystem-map-snapshot",
        "families": {
            "tool": families("tool", &tools_dir),
            "skill": families("skill", &skills_dir),
            "daemon": families("daemon", &daemons_dir),
        }
    });

    let path = map_snapshot_path(repo, &cfg);
    write_json_atomic(&path, &snapshot)?;
    Ok(json!({
        "ok": true,
        "map_snapshot_path": path.strip_prefix(repo).unwrap_or(&path).to_string_lossy().replace('\\', "/"),
        "snapshot": snapshot,
    }))
}

fn load_map_snapshot(repo: &Path, cfg: &Value) -> (Value, String) {
    let path = map_snapshot_path(repo, cfg);
    let Some(body) = read_json_file(&path) else {
        return (Value::Null, "absent".into());
    };
    if body.get("families").is_some() {
        (body, "ok".into())
    } else {
        (body, "stale".into())
    }
}

fn load_heartbeat_state(repo: &Path, cfg: &Value) -> Value {
    read_json_file(&heartbeat_audit_path(repo, cfg))
        .map(|mut body| {
            if !body.get("daemons").and_then(|v| v.as_object()).is_some() {
                body["daemons"] = json!({});
            }
            body
        })
        .unwrap_or(json!({"daemons": {}}))
}

fn load_stats(repo: &Path, cfg: &Value) -> Value {
    read_json_file(&stats_path(repo, cfg)).unwrap_or(json!({}))
}

fn load_revoked(repo: &Path, cfg: &Value) -> Value {
    read_json_file(&revoked_path(repo, cfg))
        .map(|mut data| {
            if !data.get("revoked").and_then(|v| v.as_object()).is_some() {
                data["revoked"] = json!({});
            }
            if !data.get("permanent").and_then(|v| v.as_object()).is_some() {
                data["permanent"] = json!({});
            }
            data
        })
        .unwrap_or(json!({"revoked": {}, "permanent": {}}))
}

fn collect_revoked_keys(revoked: &Value) -> HashSet<String> {
    let mut set = HashSet::new();
    for bucket in ["revoked", "permanent"] {
        if let Some(obj) = revoked.get(bucket).and_then(|v| v.as_object()) {
            for k in obj.keys() {
                set.insert(k.clone());
            }
        }
    }
    set
}

fn is_revoked(id: &str, uuid: &str, family: &str, revoked: &HashSet<String>) -> bool {
    if revoked.contains(id) || revoked.contains(uuid) {
        return true;
    }
    let prefixed = format!("{family}:{id}");
    if revoked.contains(&prefixed) {
        return true;
    }
    for alt in ["skill", "tool", "process", "daemon"] {
        if revoked.contains(&format!("{alt}:{id}")) {
            return true;
        }
    }
    false
}

fn thermo_entities(stats: &Value) -> HashMap<String, Value> {
    let Some(obj) = stats.as_object() else {
        return HashMap::new();
    };
    obj.iter()
        .filter(|(k, v)| !STATS_META_KEYS.contains(&k.as_str()) && v.is_object())
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

fn normalize_entity_key(id: &str) -> String {
    id.rsplit_once(':')
        .map(|(_, tail)| tail.to_string())
        .unwrap_or_else(|| id.to_string())
}

fn entity_matches_map(id: &str, map_id: &str) -> bool {
    id == map_id || normalize_entity_key(id) == map_id
}

fn has_samples(bucket: &Value) -> bool {
    bucket
        .get("samples")
        .and_then(|v| v.as_array())
        .map(|a| !a.is_empty())
        .unwrap_or(false)
}

fn color_daemon(
    daemon_id: &str,
    heartbeat: &Value,
    revoked: &HashSet<String>,
    uuid: &str,
) -> (String, String, i64, Option<String>, bool) {
    if is_revoked(daemon_id, uuid, "daemon", revoked) {
        return (
            "red".into(),
            "revoked".into(),
            0,
            None,
            true,
        );
    }
    let entry = heartbeat
        .get("daemons")
        .and_then(|d| d.get(daemon_id));
    let missed = entry
        .and_then(|e| e.get("missed_cycles"))
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    if missed >= MISSED_CYCLES_THRESHOLD {
        return ("red".into(), "missed_cycles".into(), missed, None, false);
    }
    let hb_status = entry
        .and_then(|e| e.get("status"))
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if hb_status == "degraded" {
        return (
            "yellow".into(),
            "heartbeat_degraded".into(),
            missed,
            None,
            false,
        );
    }
    if hb_status == "shutting_down" {
        return (
            "yellow".into(),
            "heartbeat_shutting_down".into(),
            missed,
            None,
            false,
        );
    }
    let has_hb = entry
        .and_then(|e| e.get("last_heartbeat_at"))
        .and_then(|v| v.as_str())
        .map(|s| !s.is_empty())
        .unwrap_or(false);
    if has_hb && missed < MISSED_CYCLES_THRESHOLD {
        return (
            "green".into(),
            "heartbeat_ok".into(),
            missed,
            None,
            false,
        );
    }
    ("gray".into(), "no_heartbeat".into(), missed, None, false)
}

fn color_skill_tool(
    id: &str,
    uuid: &str,
    family: &str,
    stats: &HashMap<String, Value>,
    revoked: &HashSet<String>,
) -> (String, String, Option<String>, bool) {
    if is_revoked(id, uuid, family, revoked) {
        return ("red".into(), "revoked".into(), None, true);
    }
    let bucket = stats
        .iter()
        .find(|(k, _)| entity_matches_map(k, id))
        .map(|(_, v)| v);
    let Some(bucket) = bucket else {
        return ("gray".into(), "no_executions".into(), None, false);
    };
    let status = bucket
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let thermo = if status.is_empty() {
        None
    } else {
        Some(status.to_string())
    };
    if status == "deprecated" {
        return ("red".into(), "deprecated".into(), thermo, false);
    }
    if status == "degraded" || status == "pending_redemption" {
        return ("yellow".into(), "degraded".into(), thermo, false);
    }
    if status == "healthy" && has_samples(bucket) {
        return ("green".into(), "healthy".into(), thermo, false);
    }
    if has_samples(bucket) && status == "healthy" {
        return ("green".into(), "healthy".into(), thermo, false);
    }
    ("gray".into(), "no_executions".into(), thermo, false)
}

fn make_row(
    family: &str,
    id: &str,
    uuid: &str,
    on_map: bool,
    color: &str,
    reason: &str,
    missed_cycles: i64,
    thermo_status: Option<String>,
    revoked: bool,
) -> Value {
    json!({
        "family": family,
        "id": id,
        "uuid": uuid,
        "on_map": on_map,
        "color": color,
        "reason": reason,
        "missed_cycles": missed_cycles,
        "thermo_status": thermo_status,
        "revoked": revoked,
    })
}

fn families_from_snapshot(snapshot: &Value) -> HashMap<String, Vec<MapEntry>> {
    let mut out: HashMap<String, Vec<MapEntry>> = HashMap::new();
    let Some(families) = snapshot.get("families").and_then(|v| v.as_object()) else {
        return out;
    };
    for (family, list) in families {
        let entries = list
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| {
                        let id = item.get("id")?.as_str()?.to_string();
                        let uuid = item
                            .get("uuid")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        Some(MapEntry { id, uuid })
                    })
                    .collect()
            })
            .unwrap_or_default();
        out.insert(family.clone(), entries);
    }
    out
}

/// Fusiona map-snapshot × territorio y opcionalmente persiste el Read Model.
pub fn fuse_ecosystem_health(repo: &Path, opts: FuseOptions) -> Result<Value, String> {
    if opts.compile_map {
        compile_map_snapshot(repo)?;
    }

    let cfg = load_cumulo(repo)?;
    let (snapshot, map_status) = load_map_snapshot(repo, &cfg);
    let compiled_at = snapshot
        .get("compiled_at")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let heartbeat = load_heartbeat_state(repo, &cfg);
    let stats_raw = load_stats(repo, &cfg);
    let thermo = thermo_entities(&stats_raw);
    let revoked_data = load_revoked(repo, &cfg);
    let revoked_keys = collect_revoked_keys(&revoked_data);

    let mut rows: Vec<Value> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    let families = if snapshot.is_null() {
        HashMap::new()
    } else {
        families_from_snapshot(&snapshot)
    };

    for (family, entries) in &families {
        for entry in entries {
            let key = format!("{family}:{}", entry.id);
            seen.insert(key.clone());
            let row = match family.as_str() {
                "daemon" => {
                    let (color, reason, missed, thermo_status, revoked) =
                        color_daemon(&entry.id, &heartbeat, &revoked_keys, &entry.uuid);
                    make_row(
                        family,
                        &entry.id,
                        &entry.uuid,
                        true,
                        &color,
                        &reason,
                        missed,
                        thermo_status,
                        revoked,
                    )
                }
                "skill" | "tool" => {
                    let (color, reason, thermo_status, revoked) =
                        color_skill_tool(&entry.id, &entry.uuid, family, &thermo, &revoked_keys);
                    make_row(
                        family,
                        &entry.id,
                        &entry.uuid,
                        true,
                        &color,
                        &reason,
                        0,
                        thermo_status,
                        revoked,
                    )
                }
                _ => continue,
            };
            rows.push(row);
        }
    }

    // Huérfanos territorio (L10)
    if let Some(daemons) = heartbeat.get("daemons").and_then(|v| v.as_object()) {
        for (daemon_id, _) in daemons {
            let key = format!("daemon:{daemon_id}");
            if seen.contains(&key) {
                continue;
            }
            let (color, reason, missed, thermo_status, revoked) =
                color_daemon(daemon_id, &heartbeat, &revoked_keys, "");
            rows.push(make_row(
                "daemon",
                daemon_id,
                "",
                false,
                &color,
                if on_map_false_reason(&reason) {
                    "off_map"
                } else {
                    &reason
                },
                missed,
                thermo_status,
                revoked,
            ));
            seen.insert(key);
        }
    }

    for (entity_id, bucket) in &thermo {
        let bare = normalize_entity_key(entity_id);
        let family = infer_family_from_key(entity_id);
        let key = format!("{family}:{bare}");
        if seen.contains(&key) {
            continue;
        }
        let (color, _reason, thermo_status, revoked) = match family.as_str() {
            "daemon" => {
                let (c, _r, m, t, rev) =
                    color_daemon(&bare, &heartbeat, &revoked_keys, "");
                rows.push(make_row(
                    "daemon",
                    &bare,
                    "",
                    false,
                    &c,
                    "off_map",
                    m,
                    t,
                    rev,
                ));
                seen.insert(key);
                continue;
            }
            "skill" | "tool" => color_skill_tool(&bare, "", &family, &thermo, &revoked_keys),
            _ => {
                let status = bucket
                    .get("status")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if status == "deprecated" {
                    ("red".into(), "deprecated".into(), Some(status.into()), false)
                } else if status == "degraded" || status == "pending_redemption" {
                    ("yellow".into(), "degraded".into(), Some(status.into()), false)
                } else if has_samples(bucket) {
                    ("green".into(), "healthy".into(), Some(status.into()), false)
                } else {
                    ("gray".into(), "no_executions".into(), None, false)
                }
            }
        };
        rows.push(make_row(
            &family,
            &bare,
            "",
            false,
            &color,
            "off_map",
            0,
            thermo_status,
            revoked,
        ));
        seen.insert(key);
    }

    for revoked_key in &revoked_keys {
        let bare = normalize_entity_key(revoked_key);
        let family = infer_family_from_key(revoked_key);
        let key = format!("{family}:{bare}");
        if seen.contains(&key) {
            continue;
        }
        rows.push(make_row(
            &family,
            &bare,
            "",
            false,
            "red",
            "off_map",
            0,
            None,
            true,
        ));
        seen.insert(key);
    }

    rows.sort_by(|a, b| {
        let fa = a.get("family").and_then(|v| v.as_str()).unwrap_or("");
        let fb = b.get("family").and_then(|v| v.as_str()).unwrap_or("");
        fa.cmp(fb).then_with(|| {
            a.get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .cmp(b.get("id").and_then(|v| v.as_str()).unwrap_or(""))
        })
    });

    let mut response = json!({
        "success": true,
        "exit_code": 0,
        "map_status": map_status,
        "compiled_at": compiled_at,
        "fused_at": iso_now(),
        "rows": rows,
    });
    if map_status == "absent" {
        response["warning"] = json!("map-snapshot ausente; ejecutar compile-ecosystem-map-snapshot");
    }

    if opts.persist {
        let path = ecosystem_health_path(repo, &cfg);
        if let Err(e) = write_json_atomic(&path, &response) {
            response["persist_warning"] = json!(e);
        } else {
            response["ecosystem_health_path"] = json!(
                path.strip_prefix(repo)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .replace('\\', "/")
            );
        }
    }

    Ok(response)
}

fn on_map_false_reason(reason: &str) -> bool {
    reason == "off_map"
}

fn infer_family_from_key(key: &str) -> String {
    if let Some((prefix, _)) = key.split_once(':') {
        match prefix {
            "skill" => return "skill".into(),
            "tool" => return "tool".into(),
            "daemon" => return "daemon".into(),
            "process" => return "tool".into(),
            _ => {}
        }
    }
    "tool".into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn write_repo_layout(repo: &Path) {
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::create_dir_all(repo.join("SddIA/tools")).unwrap();
        fs::create_dir_all(repo.join("SddIA/skills")).unwrap();
        fs::create_dir_all(repo.join("SddIA/daemons")).unwrap();
        fs::create_dir_all(repo.join(".SddIA/daemons/state")).unwrap();
        fs::create_dir_all(repo.join(".SddIA/radamanto")).unwrap();
        fs::create_dir_all(repo.join(".SddIA/cerbero")).unwrap();
        fs::create_dir_all(repo.join(".SddIA/observability")).unwrap();

        let cumulo = json!({
            "version": "1.8.0",
            "directories": {
                "tools": "SddIA/tools",
                "skills": "SddIA/skills",
                "daemons": "SddIA/daemons"
            },
            "daemons_instance": { "state": ".SddIA/daemons/state" },
            "radamanto": {
                "stats": ".SddIA/radamanto/stats.json",
                "revoked_entities": ".SddIA/cerbero/revoked_entities.json"
            },
            "observability": {
                "map_snapshot": ".SddIA/observability/map-snapshot.json",
                "ecosystem_health": ".SddIA/observability/ecosystem-health.json"
            }
        });
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            serde_json::to_string_pretty(&cumulo).unwrap(),
        )
        .unwrap();

        fs::write(
            repo.join("SddIA/daemons/index.md"),
            "| Archivo fuente | uuid | name | version |\n|---|---|---|---|\n| `event-watcher.md` | f995cc89-22a7-488d-9b25-ddb1e5e3a4a4 | event-watcher | 1.1.0 |\n",
        )
        .unwrap();
        fs::write(
            repo.join("SddIA/tools/index.md"),
            "| Archivo fuente | uuid | name | version |\n|---|---|---|---|\n| `git-tool.md` | 11111111-1111-4111-8111-111111111111 | demo-tool | 1.0.0 |\n",
        )
        .unwrap();
        fs::write(repo.join("SddIA/skills/index.md"), "| Archivo fuente | uuid | name | version |\n|---|---|---|---|\n")
            .unwrap();
    }

    #[test]
    fn ecosystem_health_daemon_red_on_missed_cycles() {
        let dir = tempdir().unwrap();
        let repo = dir.path();
        write_repo_layout(repo);
        compile_map_snapshot(repo).unwrap();
        fs::write(
            repo.join(".SddIA/daemons/state/heartbeat-audit.json"),
            r#"{"daemons":{"event-watcher":{"missed_cycles":3,"last_heartbeat_at":"2020-01-01T00:00:00Z"}}}"#,
        )
        .unwrap();
        let out = fuse_ecosystem_health(
            repo,
            FuseOptions {
                compile_map: false,
                persist: false,
            },
        )
        .unwrap();
        let row = out["rows"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["id"] == "event-watcher")
            .expect("daemon row");
        assert_eq!(row["color"], "red");
        assert_eq!(row["reason"], "missed_cycles");
    }

    #[test]
    fn daemon_yellow_on_heartbeat_degraded() {
        let dir = tempdir().unwrap();
        let repo = dir.path();
        write_repo_layout(repo);
        compile_map_snapshot(repo).unwrap();
        fs::write(
            repo.join(".SddIA/daemons/state/heartbeat-audit.json"),
            r#"{"daemons":{"event-watcher":{"missed_cycles":0,"last_heartbeat_at":"2020-01-01T00:00:00Z","status":"degraded"}}}"#,
        )
        .unwrap();
        let out = fuse_ecosystem_health(
            repo,
            FuseOptions {
                compile_map: false,
                persist: false,
            },
        )
        .unwrap();
        let row = out["rows"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["id"] == "event-watcher")
            .expect("daemon row");
        assert_eq!(row["color"], "yellow");
        assert_eq!(row["reason"], "heartbeat_degraded");
    }

    #[test]
    fn daemon_red_missed_beats_degraded() {
        let dir = tempdir().unwrap();
        let repo = dir.path();
        write_repo_layout(repo);
        compile_map_snapshot(repo).unwrap();
        fs::write(
            repo.join(".SddIA/daemons/state/heartbeat-audit.json"),
            r#"{"daemons":{"event-watcher":{"missed_cycles":3,"last_heartbeat_at":"2020-01-01T00:00:00Z","status":"degraded"}}}"#,
        )
        .unwrap();
        let out = fuse_ecosystem_health(
            repo,
            FuseOptions {
                compile_map: false,
                persist: false,
            },
        )
        .unwrap();
        let row = out["rows"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["id"] == "event-watcher")
            .expect("daemon row");
        assert_eq!(row["color"], "red");
        assert_eq!(row["reason"], "missed_cycles");
    }

    #[test]
    fn daemon_green_alive_compat() {
        let dir = tempdir().unwrap();
        let repo = dir.path();
        write_repo_layout(repo);
        compile_map_snapshot(repo).unwrap();
        fs::write(
            repo.join(".SddIA/daemons/state/heartbeat-audit.json"),
            r#"{"daemons":{"event-watcher":{"missed_cycles":0,"last_heartbeat_at":"2020-01-01T00:00:00Z"}}}"#,
        )
        .unwrap();
        let out = fuse_ecosystem_health(
            repo,
            FuseOptions {
                compile_map: false,
                persist: false,
            },
        )
        .unwrap();
        let row = out["rows"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["id"] == "event-watcher")
            .expect("daemon row");
        assert_eq!(row["color"], "green");
        assert_eq!(row["reason"], "heartbeat_ok");
    }

    #[test]
    fn ecosystem_health_skill_gray_without_samples() {
        let dir = tempdir().unwrap();
        let repo = dir.path();
        write_repo_layout(repo);
        compile_map_snapshot(repo).unwrap();
        fs::write(
            repo.join(".SddIA/radamanto/stats.json"),
            r#"{"cognitive":{},"demo-tool":{"samples":[],"status":"healthy"}}"#,
        )
        .unwrap();
        let out = fuse_ecosystem_health(
            repo,
            FuseOptions {
                compile_map: false,
                persist: false,
            },
        )
        .unwrap();
        let row = out["rows"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["id"] == "demo-tool")
            .expect("tool row");
        assert_eq!(row["color"], "gray");
    }

    #[test]
    fn ecosystem_health_map_absent_still_ok() {
        let dir = tempdir().unwrap();
        let repo = dir.path();
        write_repo_layout(repo);
        let out = fuse_ecosystem_health(
            repo,
            FuseOptions {
                compile_map: false,
                persist: false,
            },
        )
        .unwrap();
        assert_eq!(out["map_status"], "absent");
        assert_eq!(out["success"], true);
    }

    #[test]
    fn ecosystem_health_no_adapter_family() {
        let dir = tempdir().unwrap();
        let repo = dir.path();
        write_repo_layout(repo);
        let snap = compile_map_snapshot(repo).unwrap();
        let families = snap["snapshot"]["families"].as_object().unwrap();
        assert!(!families.contains_key("adapter"));
    }
}
