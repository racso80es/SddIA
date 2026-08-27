//! Memoria soberana de preferencias del usuario (`UserPreference`). Store JSON durable local.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

pub const STORE_REL_DEFAULT: &str = ".SddIA/vector_store/user_preferences";
pub const CONTEXT_SCHEMA_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PreferenceStatus {
    Proposed,
    Active,
    Revoked,
    Superseded,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PreferenceAuthority {
    ExplicitUser,
    Inferred,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScopeType {
    Global,
    Domain,
    Project,
    Channel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreference {
    pub preference_id: String,
    pub revision_id: String,
    pub subject_kind: String,
    pub subject_key: String,
    pub predicate: String,
    pub value: Value,
    pub scope_type: ScopeType,
    pub scope_id: Option<String>,
    pub status: PreferenceStatus,
    pub authority: PreferenceAuthority,
    pub sensitivity: String,
    pub valid_from: Option<String>,
    pub valid_until: Option<String>,
    pub supersedes: Option<String>,
    pub provenance: Value,
    pub recorded_at: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QuerySpec {
    pub subject_key: Option<String>,
    pub predicate: Option<String>,
    pub scope_type: Option<ScopeType>,
    pub scope_id: Option<String>,
    pub max_results: Option<usize>,
    pub include_proposed: Option<bool>,
}

pub fn store_root(repo: &Path) -> PathBuf {
    let cfg_path = repo.join("SddIA/core/cumulo.paths.json");
    let default = repo.join(STORE_REL_DEFAULT);
    let Ok(text) = fs::read_to_string(&cfg_path) else {
        return default;
    };
    let Ok(cfg) = serde_json::from_str::<Value>(&text) else {
        return default;
    };
    cfg.get("paths")
        .and_then(|p| p.get("userPreferencesStore"))
        .and_then(|v| v.as_str())
        .map(|s| {
            let rel = s.trim().trim_start_matches("./");
            repo.join(rel)
        })
        .unwrap_or(default)
}

fn revisions_dir(root: &Path) -> PathBuf {
    root.join("revisions")
}

fn head_index_path(root: &Path) -> PathBuf {
    root.join("head_index.json")
}

fn hex_sha256(parts: &[&[u8]]) -> String {
    let mut hasher = Sha256::new();
    for p in parts {
        hasher.update(p);
    }
    hex::encode(hasher.finalize())
}

pub fn compute_preference_id(
    scope_type: &ScopeType,
    scope_id: Option<&str>,
    subject_kind: &str,
    subject_key: &str,
    predicate: &str,
) -> String {
    let scope = format!("{scope_type:?}");
    let sid = scope_id.unwrap_or("");
    let sep = [0x1f_u8];
    hex_sha256(&[
        scope.as_bytes(),
        &sep,
        sid.as_bytes(),
        &sep,
        subject_kind.as_bytes(),
        &sep,
        subject_key.as_bytes(),
        &sep,
        predicate.as_bytes(),
    ])
}

pub fn compute_revision_id(preference_id: &str, value: &Value, recorded_at: &str) -> String {
    let canon = serde_json::to_string(value).unwrap_or_else(|_| "{}".into());
    let sep = [0x1f_u8];
    hex_sha256(&[
        preference_id.as_bytes(),
        &sep,
        canon.as_bytes(),
        &sep,
        recorded_at.as_bytes(),
    ])
}

fn read_head_index(root: &Path) -> Result<Map<String, Value>, String> {
    let path = head_index_path(root);
    if !path.is_file() {
        return Ok(Map::new());
    }
    let raw = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let v: Value = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    Ok(v.as_object().cloned().unwrap_or_default())
}

fn write_head_index(root: &Path, index: &Map<String, Value>) -> Result<(), String> {
    fs::create_dir_all(root).map_err(|e| e.to_string())?;
    let path = head_index_path(root);
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, serde_json::to_string_pretty(index).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    fs::rename(&tmp, &path).map_err(|e| e.to_string())
}

fn write_revision(root: &Path, pref: &UserPreference) -> Result<(), String> {
    let dir = revisions_dir(root);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join(format!("{}.json", pref.revision_id));
    if path.is_file() {
        return Ok(());
    }
    let tmp = path.with_extension("json.tmp");
    fs::write(
        &tmp,
        serde_json::to_string_pretty(pref).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    fs::rename(&tmp, &path).map_err(|e| e.to_string())
}

fn read_revision(root: &Path, revision_id: &str) -> Result<Option<UserPreference>, String> {
    let path = revisions_dir(root).join(format!("{revision_id}.json"));
    if !path.is_file() {
        return Ok(None);
    }
    let raw = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&raw).map_err(|e| e.to_string()).map(Some)
}

pub fn put_revision(repo: &Path, mut pref: UserPreference) -> Result<UserPreference, String> {
    let root = store_root(repo);
    if pref.preference_id.is_empty() {
        pref.preference_id = compute_preference_id(
            &pref.scope_type,
            pref.scope_id.as_deref(),
            &pref.subject_kind,
            &pref.subject_key,
            &pref.predicate,
        );
    }
    if pref.recorded_at.is_empty() {
        pref.recorded_at = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    }
    if pref.revision_id.is_empty() {
        pref.revision_id = compute_revision_id(&pref.preference_id, &pref.value, &pref.recorded_at);
    }
    write_revision(&root, &pref)?;
    let mut index = read_head_index(&root)?;
    if matches!(pref.status, PreferenceStatus::Active | PreferenceStatus::Proposed) {
        index.insert(
            pref.preference_id.clone(),
            json!({
                "revision_id": pref.revision_id,
                "status": format!("{:?}", pref.status).to_lowercase(),
            }),
        );
    } else if matches!(pref.status, PreferenceStatus::Revoked | PreferenceStatus::Superseded) {
        index.insert(
            pref.preference_id.clone(),
            json!({
                "revision_id": pref.revision_id,
                "status": format!("{:?}", pref.status).to_lowercase(),
            }),
        );
    }
    write_head_index(&root, &index)?;
    Ok(pref)
}

pub fn purge_preference(repo: &Path, preference_id: &str) -> Result<(), String> {
    let root = store_root(repo);
    let mut index = read_head_index(&root)?;
    let Some(head) = index.remove(preference_id) else {
        return Ok(());
    };
    if let Some(rid) = head.get("revision_id").and_then(|v| v.as_str()) {
        let path = revisions_dir(&root).join(format!("{rid}.json"));
        let _ = fs::remove_file(path);
    }
    write_head_index(&root, &index)?;
    Ok(())
}

fn scope_rank(scope: &ScopeType) -> u8 {
    match scope {
        ScopeType::Global => 0,
        ScopeType::Domain => 1,
        ScopeType::Project => 2,
        ScopeType::Channel => 3,
    }
}

fn authority_rank(auth: &PreferenceAuthority) -> u8 {
    match auth {
        PreferenceAuthority::Inferred => 0,
        PreferenceAuthority::ExplicitUser => 1,
    }
}

pub fn query(repo: &Path, spec: &QuerySpec) -> Result<Vec<UserPreference>, String> {
    let root = store_root(repo);
    let index = read_head_index(&root)?;
    let max = spec.max_results.unwrap_or(8).min(32);
    let include_proposed = spec.include_proposed.unwrap_or(false);
    let mut hits: Vec<UserPreference> = Vec::new();

    for (_pid, head) in index.iter() {
        let status = head
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if status == "revoked" || status == "superseded" {
            continue;
        }
        if status == "proposed" && !include_proposed {
            continue;
        }
        let Some(rid) = head.get("revision_id").and_then(|v| v.as_str()) else {
            continue;
        };
        let Some(pref) = read_revision(&root, rid)? else {
            continue;
        };
        if let Some(sk) = spec.subject_key.as_deref() {
            if pref.subject_key != sk {
                continue;
            }
        }
        if let Some(pred) = spec.predicate.as_deref() {
            if pref.predicate != pred {
                continue;
            }
        }
        if let Some(st) = &spec.scope_type {
            if &pref.scope_type != st {
                continue;
            }
        }
        if let Some(sid) = spec.scope_id.as_deref() {
            if pref.scope_id.as_deref() != Some(sid) {
                continue;
            }
        }
        hits.push(pref);
    }

    hits.sort_by(|a, b| {
        scope_rank(&b.scope_type)
            .cmp(&scope_rank(&a.scope_type))
            .then(authority_rank(&b.authority).cmp(&authority_rank(&a.authority)))
            .then(b.recorded_at.cmp(&a.recorded_at))
    });
    hits.truncate(max);
    Ok(hits)
}

pub fn query_context_block(repo: &Path, spec: &QuerySpec) -> Value {
    match query(repo, spec) {
        Ok(prefs) => {
            let items: Vec<Value> = prefs
                .iter()
                .map(|p| {
                    json!({
                        "preference_id": p.preference_id,
                        "revision_id": p.revision_id,
                        "subject_kind": p.subject_kind,
                        "subject_key": p.subject_key,
                        "predicate": p.predicate,
                        "value": p.value,
                        "scope_type": format!("{:?}", p.scope_type).to_lowercase(),
                        "scope_id": p.scope_id,
                        "status": format!("{:?}", p.status).to_lowercase(),
                        "authority": format!("{:?}", p.authority).to_lowercase(),
                    })
                })
                .collect();
            json!({
                "schema_version": CONTEXT_SCHEMA_VERSION,
                "preferences": items,
            })
        }
        Err(_) => json!({
            "schema_version": CONTEXT_SCHEMA_VERSION,
            "preferences": [],
        }),
    }
}

pub fn preference_from_event_payload(payload: &Value, operation: &str) -> Result<Option<UserPreference>, String> {
    let op = operation.to_ascii_lowercase();
    if op == "ignore" {
        return Ok(None);
    }

    let subject_kind = payload
        .get("subject_kind")
        .or_else(|| payload.get("subject_hint"))
        .and_then(|v| v.as_str())
        .unwrap_or("person")
        .to_string();
    let subject_key = payload
        .get("subject_key")
        .or_else(|| payload.get("subject_hint"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or("subject_key o subject_hint requerido")?
        .to_string();
    let predicate = payload
        .get("predicate")
        .or_else(|| payload.get("predicate_hint"))
        .and_then(|v| v.as_str())
        .unwrap_or("priority")
        .to_string();

    let scope_type = match payload
        .get("scope_type")
        .and_then(|v| v.as_str())
        .unwrap_or("channel")
    {
        "global" => ScopeType::Global,
        "domain" => ScopeType::Domain,
        "project" => ScopeType::Project,
        _ => ScopeType::Channel,
    };
    let scope_id = payload
        .get("scope_id")
        .and_then(|v| v.as_str())
        .map(str::to_string);

    let value = payload.get("value").cloned().unwrap_or_else(|| {
        json!({"level": payload.get("priority_level").and_then(|v| v.as_str()).unwrap_or("high")})
    });

    let (status, authority) = match op.as_str() {
        "propose" => (PreferenceStatus::Proposed, PreferenceAuthority::ExplicitUser),
        "activate" => (PreferenceStatus::Active, PreferenceAuthority::ExplicitUser),
        "revoke" => (PreferenceStatus::Revoked, PreferenceAuthority::ExplicitUser),
        "purge" => return Ok(None),
        _ => (PreferenceStatus::Active, PreferenceAuthority::ExplicitUser),
    };

    if matches!(authority, PreferenceAuthority::Inferred) && status == PreferenceStatus::Active {
        return Err("inferencia no puede activarse sin confirmación".into());
    }

    let recorded_at = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let preference_id = compute_preference_id(
        &scope_type,
        scope_id.as_deref(),
        &subject_kind,
        &subject_key,
        &predicate,
    );
    let revision_id = compute_revision_id(&preference_id, &value, &recorded_at);

    let provenance = json!({
        "channel": payload.get("channel").cloned().unwrap_or(Value::Null),
        "emitter": payload.get("emitter").cloned().unwrap_or(Value::Null),
        "causal_event_id": payload.get("source_event_id").cloned().unwrap_or(Value::Null),
        "at": recorded_at,
    });

    Ok(Some(UserPreference {
        preference_id,
        revision_id,
        subject_kind,
        subject_key,
        predicate,
        value,
        scope_type,
        scope_id,
        status,
        authority,
        sensitivity: payload
            .get("sensitivity")
            .and_then(|v| v.as_str())
            .unwrap_or("personal")
            .to_string(),
        valid_from: None,
        valid_until: None,
        supersedes: payload
            .get("supersedes")
            .and_then(|v| v.as_str())
            .map(str::to_string),
        provenance,
        recorded_at,
    }))
}

pub fn run_capsule(repo: &Path, request: &Value) -> Value {
    let op = request
        .get("op")
        .or_else(|| request.get("operation"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_ascii_uppercase();

    match op.as_str() {
        "PUT" | "ACTIVATE" | "PROPOSE" | "REVOKE" | "SUPERSEDE" => {
            let revision = request.get("revision").cloned().unwrap_or_else(|| request.clone());
            let operation = op.to_ascii_lowercase();
            match serde_json::from_value::<UserPreference>(revision) {
                Ok(mut pref) => match put_revision(repo, pref.clone()) {
                    Ok(stored) => {
                        pref = stored;
                        json!({
                            "success": true,
                            "exitCode": 0,
                            "message": "preference stored",
                            "result": {"preference_id": pref.preference_id, "revision_id": pref.revision_id, "operation": operation}
                        })
                    }
                    Err(e) => json!({"success": false, "exitCode": 1, "message": e}),
                },
                Err(_e) => {
                    let payload = request.get("revision").unwrap_or(request);
                    match preference_from_event_payload(payload, &operation.to_ascii_lowercase()) {
                        Ok(Some(pref)) => match put_revision(repo, pref) {
                            Ok(p) => json!({
                                "success": true,
                                "exitCode": 0,
                                "message": "preference stored",
                                "result": {"preference_id": p.preference_id, "revision_id": p.revision_id}
                            }),
                            Err(e) => json!({"success": false, "exitCode": 1, "message": e}),
                        },
                        Ok(None) => json!({"success": true, "exitCode": 0, "message": "noop"}),
                        Err(e) => json!({"success": false, "exitCode": 1, "message": format!("revision inválida: {e}")}),
                    }
                }
            }
        }
        "QUERY" => {
            let spec_val = request.get("spec").cloned().unwrap_or(json!({}));
            let spec: QuerySpec = serde_json::from_value(spec_val).unwrap_or_default();
            match query(repo, &spec) {
                Ok(prefs) => {
                    let items: Vec<Value> = prefs
                        .iter()
                        .map(|p| serde_json::to_value(p).unwrap_or(json!({})))
                        .collect();
                    json!({
                        "success": true,
                        "exitCode": 0,
                        "result": {"preferences": items}
                    })
                }
                Err(e) => json!({"success": false, "exitCode": 1, "message": e}),
            }
        }
        "PURGE" => {
            let pid = request
                .get("preference_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if pid.is_empty() {
                return json!({"success": false, "exitCode": 1, "message": "preference_id requerido"});
            }
            match purge_preference(repo, pid) {
                Ok(()) => json!({"success": true, "exitCode": 0, "message": "purged"}),
                Err(e) => json!({"success": false, "exitCode": 1, "message": e}),
            }
        }
        "EXPORT" => {
            let spec = QuerySpec {
                include_proposed: Some(true),
                max_results: Some(32),
                ..Default::default()
            };
            match query(repo, &spec) {
                Ok(prefs) => json!({
                    "success": true,
                    "exitCode": 0,
                    "result": {"export": prefs}
                }),
                Err(e) => json!({"success": false, "exitCode": 1, "message": e}),
            }
        }
        "QUERY_CONTEXT" => {
            let spec_val = request.get("spec").cloned().unwrap_or(json!({}));
            let spec: QuerySpec = serde_json::from_value(spec_val).unwrap_or_default();
            let block = query_context_block(repo, &spec);
            json!({
                "success": true,
                "exitCode": 0,
                "result": block
            })
        }
        _ => json!({"success": false, "exitCode": 1, "message": "op desconocida"}),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn setup_repo(tmp: &Path) {
        fs::create_dir_all(tmp.join("SddIA/core")).unwrap();
        fs::create_dir_all(tmp.join(".events/domain")).unwrap();
        fs::write(
            tmp.join("SddIA/core/cumulo.paths.json"),
            r#"{"paths":{"userPreferencesStore":".SddIA/vector_store/user_preferences"},"eda_fractal":{"domain":".events/domain"}}"#,
        )
        .unwrap();
    }

    #[test]
    fn put_and_reopen_active_preference() {
        let tmp = tempfile::tempdir().unwrap();
        setup_repo(tmp.path());
        let pref = UserPreference {
            preference_id: String::new(),
            revision_id: String::new(),
            subject_kind: "person".into(),
            subject_key: "hash-juan".into(),
            predicate: "priority".into(),
            value: json!({"level": "max"}),
            scope_type: ScopeType::Channel,
            scope_id: Some("email".into()),
            status: PreferenceStatus::Active,
            authority: PreferenceAuthority::ExplicitUser,
            sensitivity: "personal".into(),
            valid_from: None,
            valid_until: None,
            supersedes: None,
            provenance: json!({"channel": "kalma2"}),
            recorded_at: String::new(),
        };
        let stored = put_revision(tmp.path(), pref).unwrap();
        drop(stored);
        let hits = query(
            tmp.path(),
            &QuerySpec {
                subject_key: Some("hash-juan".into()),
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].value["level"], "max");
    }

    #[test]
    fn revoke_stops_query() {
        let tmp = tempfile::tempdir().unwrap();
        setup_repo(tmp.path());
        let mut pref = UserPreference {
            preference_id: String::new(),
            revision_id: String::new(),
            subject_kind: "person".into(),
            subject_key: "k1".into(),
            predicate: "priority".into(),
            value: json!({"level": "high"}),
            scope_type: ScopeType::Global,
            scope_id: None,
            status: PreferenceStatus::Active,
            authority: PreferenceAuthority::ExplicitUser,
            sensitivity: "internal".into(),
            valid_from: None,
            valid_until: None,
            supersedes: None,
            provenance: json!({}),
            recorded_at: String::new(),
        };
        pref = put_revision(tmp.path(), pref).unwrap();
        let mut revoked = pref.clone();
        revoked.status = PreferenceStatus::Revoked;
        revoked.recorded_at = "2026-08-27T12:00:01Z".into();
        revoked.revision_id = compute_revision_id(&revoked.preference_id, &revoked.value, &revoked.recorded_at);
        put_revision(tmp.path(), revoked).unwrap();
        let hits = query(tmp.path(), &QuerySpec::default()).unwrap();
        assert!(hits.is_empty());
    }

    #[test]
    fn proposed_not_returned_by_default() {
        let tmp = tempfile::tempdir().unwrap();
        setup_repo(tmp.path());
        let pref = UserPreference {
            preference_id: String::new(),
            revision_id: String::new(),
            subject_kind: "topic".into(),
            subject_key: "gesfer".into(),
            predicate: "mute".into(),
            value: json!({"muted": true}),
            scope_type: ScopeType::Domain,
            scope_id: Some("email".into()),
            status: PreferenceStatus::Proposed,
            authority: PreferenceAuthority::Inferred,
            sensitivity: "internal".into(),
            valid_from: None,
            valid_until: None,
            supersedes: None,
            provenance: json!({}),
            recorded_at: String::new(),
        };
        put_revision(tmp.path(), pref).unwrap();
        assert!(query(tmp.path(), &QuerySpec::default()).unwrap().is_empty());
    }

    #[test]
    fn specific_scope_beats_global_same_predicate() {
        let tmp = tempfile::tempdir().unwrap();
        setup_repo(tmp.path());
        let subject = "racso";
        let predicate = "priority";

        let global = UserPreference {
            preference_id: String::new(),
            revision_id: String::new(),
            subject_kind: "person".into(),
            subject_key: subject.into(),
            predicate: predicate.into(),
            value: json!({"level": "low"}),
            scope_type: ScopeType::Global,
            scope_id: None,
            status: PreferenceStatus::Active,
            authority: PreferenceAuthority::ExplicitUser,
            sensitivity: "internal".into(),
            valid_from: None,
            valid_until: None,
            supersedes: None,
            provenance: json!({}),
            recorded_at: "2026-08-27T12:00:00Z".into(),
        };
        put_revision(tmp.path(), global).unwrap();

        let channel = UserPreference {
            preference_id: String::new(),
            revision_id: String::new(),
            subject_kind: "person".into(),
            subject_key: subject.into(),
            predicate: predicate.into(),
            value: json!({"level": "max"}),
            scope_type: ScopeType::Channel,
            scope_id: Some("telegram".into()),
            status: PreferenceStatus::Active,
            authority: PreferenceAuthority::ExplicitUser,
            sensitivity: "internal".into(),
            valid_from: None,
            valid_until: None,
            supersedes: None,
            provenance: json!({}),
            recorded_at: "2026-08-27T12:00:01Z".into(),
        };
        put_revision(tmp.path(), channel).unwrap();

        let hits = query(
            tmp.path(),
            &QuerySpec {
                subject_key: Some(subject.into()),
                predicate: Some(predicate.into()),
                max_results: Some(2),
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].scope_type, ScopeType::Channel);
        assert_eq!(hits[0].value["level"], "max");
        assert_eq!(hits[1].scope_type, ScopeType::Global);
        assert_eq!(hits[1].value["level"], "low");
    }

    #[test]
    fn purge_removes_revision_from_store() {
        let tmp = tempfile::tempdir().unwrap();
        setup_repo(tmp.path());
        let pref = UserPreference {
            preference_id: String::new(),
            revision_id: String::new(),
            subject_kind: "person".into(),
            subject_key: "purge-me".into(),
            predicate: "priority".into(),
            value: json!({"level": "low"}),
            scope_type: ScopeType::Global,
            scope_id: None,
            status: PreferenceStatus::Active,
            authority: PreferenceAuthority::ExplicitUser,
            sensitivity: "internal".into(),
            valid_from: None,
            valid_until: None,
            supersedes: None,
            provenance: json!({}),
            recorded_at: String::new(),
        };
        let stored = put_revision(tmp.path(), pref).unwrap();
        let root = store_root(tmp.path());
        assert!(revisions_dir(&root)
            .join(format!("{}.json", stored.revision_id))
            .is_file());
        purge_preference(tmp.path(), &stored.preference_id).unwrap();
        assert!(!revisions_dir(&root)
            .join(format!("{}.json", stored.revision_id))
            .exists());
        assert!(read_head_index(&root).unwrap().get(&stored.preference_id).is_none());
    }
}
