use sddia_core_memory::models::evolution_node::{EvolutionEvent, SpatialPolarity};
use sddia_core_memory::models::thought_node::ThoughtNode;
use sddia_core_memory::ports::{EvolutionStore, ThoughtGraphRepository};
use sddia_core_memory::services::inference_binding::{LocalHashingEmbedder, SemanticInference};
use sddia_core_memory::EmbeddingGenerator;
use sddia_infrastructure_lancedb_evolution::{LanceDbEvolutionAdapter, TABLE_EVOLUTION};
use sddia_infrastructure_lancedb_preferences::{LanceDbPreferenceAdapter, TABLE_PREFERENCES};
use sddia_infrastructure_lancedb_thought::LanceDbThoughtRepo;
use sddia_io::read_stdin_json;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::thread;
use std::time::Duration;
use user_preference_core::{QuerySpec, UserPreference, UserPreferenceStore};

const ENTITY_ID: &str = "lancedb-memory-adapter";
const VECTOR_STORE_DEFAULT: &str = ".SddIA/vector_store";
const LANCEDB_SUBDIR: &str = "lancedb";
const WRITE_RETRY_ATTEMPTS: usize = 5;

fn emit_v2(
    success: bool,
    exit_code: i32,
    message: &str,
    result: Option<Value>,
    feedback: Option<&str>,
) -> ! {
    let mut body = json!({
        "meta": {
            "schemaVersion": "2.0",
            "entityKind": "tool",
            "entityId": ENTITY_ID,
        },
        "success": success,
        "exitCode": exit_code,
        "message": message,
    });
    if let Some(r) = result {
        body["result"] = r;
    }
    if let Some(fb) = feedback {
        body["feedback"] = json!(fb);
        body["error"] = json!(fb);
    }
    println!("{body}");
    process::exit(exit_code);
}

fn request_inner(doc: &Value) -> &Value {
    doc.get("request").unwrap_or(doc)
}

fn required_str(req: &Value, key: &str) -> Result<String, String> {
    req.get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .ok_or_else(|| format!("request.{key} obligatorio"))
}

fn load_cumulo(repo: &Path) -> Result<Value, String> {
    let text = fs::read_to_string(repo.join("SddIA/core/cumulo.paths.json"))
        .map_err(|e| format!("cumulo.paths.json: {e}"))?;
    serde_json::from_str(&text).map_err(|e| format!("cumulo JSON: {e}"))
}

fn resolve_lancedb_path(repo: &Path) -> Result<PathBuf, String> {
    let cfg = load_cumulo(repo)?;
    if let Some(lance_rel) = cfg
        .pointer("/paths/lancedbStore")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        return Ok(repo.join(lance_rel.trim_start_matches("./")));
    }
    let vs = cfg
        .pointer("/paths/vectorStore")
        .and_then(|v| v.as_str())
        .unwrap_or(VECTOR_STORE_DEFAULT);
    let vs_rel = vs.trim().trim_start_matches("./");
    Ok(repo.join(vs_rel).join(LANCEDB_SUBDIR))
}

fn is_transient_lance_err(msg: &str) -> bool {
    let lower = msg.to_ascii_lowercase();
    lower.contains("commit conflict")
        || lower.contains("already exists")
        || lower.contains("retry")
        || lower.contains("lock")
        || lower.contains("temporarily unavailable")
}

fn with_lancedb_retry<T, E: ToString>(mut op: impl FnMut() -> Result<T, E>) -> Result<T, String> {
    let mut attempt = 0usize;
    loop {
        match op() {
            Ok(v) => return Ok(v),
            Err(e) => {
                let msg = e.to_string();
                attempt += 1;
                if attempt >= WRITE_RETRY_ATTEMPTS || !is_transient_lance_err(&msg) {
                    return Err(msg);
                }
                thread::sleep(Duration::from_millis((25 * attempt) as u64));
            }
        }
    }
}

fn parse_polarity(raw: &str) -> Result<SpatialPolarity, String> {
    match raw {
        "EfficientSymmetry" => Ok(SpatialPolarity::EfficientSymmetry),
        "StructuralFracture" => Ok(SpatialPolarity::StructuralFracture),
        other => Err(format!("polarity desconocida: {other}")),
    }
}

fn evolution_get_by_id(repo: &Path, req: &Value) -> Result<Value, String> {
    let record_id = required_str(req, "record_id")?;
    let uri = resolve_lancedb_path(repo)?;
    let adapter = LanceDbEvolutionAdapter::open(&uri).map_err(|e| e.to_string())?;
    let found = adapter
        .get_event_by_id(&record_id)
        .map_err(|e| e.to_string())?;
    Ok(json!({
        "found": found.is_some(),
        "record_id": record_id,
        "table": TABLE_EVOLUTION,
    }))
}

fn evolution_store(repo: &Path, req: &Value) -> Result<Value, String> {
    let ev = req.get("event").unwrap_or(req);
    let id = required_str(ev, "id")?;
    let polarity = parse_polarity(&required_str(ev, "polarity")?)?;
    let payload = required_str(ev, "payload")?;
    let operational_metadata = ev
        .get("operational_metadata")
        .cloned()
        .unwrap_or_else(|| json!({}));
    let mut event = EvolutionEvent {
        id: id.clone(),
        polarity,
        payload,
        operational_metadata,
        embedding: None,
    };
    LocalHashingEmbedder
        .embed_event(&mut event)
        .map_err(|e| e.to_string())?;
    let uri = resolve_lancedb_path(repo)?;
    let adapter = LanceDbEvolutionAdapter::open(&uri).map_err(|e| e.to_string())?;
    with_lancedb_retry(|| adapter.store_event(event.clone()))?;
    Ok(json!({
        "stored": true,
        "record_id": id,
        "store_backend": "lancedb",
        "table": TABLE_EVOLUTION,
    }))
}

fn evolution_ingest_telemetry(repo: &Path, req: &Value) -> Result<Value, String> {
    let record_id = required_str(req, "record_id")?;
    let polarity_raw = required_str(req, "polarity")?;
    let polarity = parse_polarity(&polarity_raw)?;
    let payload = required_str(req, "payload")?;
    let operational_metadata = req
        .get("operational_metadata")
        .cloned()
        .unwrap_or_else(|| json!({}));

    let uri = resolve_lancedb_path(repo)?;
    let adapter = LanceDbEvolutionAdapter::open(&uri).map_err(|e| e.to_string())?;
    if adapter
        .get_event_by_id(&record_id)
        .map_err(|e| e.to_string())?
        .is_some()
    {
        return Ok(json!({
            "skipped": "already_indexed",
            "record_id": record_id,
            "store_backend": "lancedb",
            "table": TABLE_EVOLUTION,
        }));
    }

    let mut event = EvolutionEvent {
        id: record_id.clone(),
        polarity,
        payload,
        operational_metadata,
        embedding: None,
    };
    LocalHashingEmbedder
        .embed_event(&mut event)
        .map_err(|e| e.to_string())?;
    with_lancedb_retry(|| adapter.store_event(event.clone()))?;
    Ok(json!({
        "record_id": record_id,
        "store_backend": "lancedb",
        "table": TABLE_EVOLUTION,
    }))
}

fn preferences_put_revision(repo: &Path, req: &Value) -> Result<Value, String> {
    let rev_val = req
        .get("revision")
        .ok_or_else(|| "request.revision obligatorio".to_string())?;
    let pref: UserPreference =
        serde_json::from_value(rev_val.clone()).map_err(|e| format!("revision inválida: {e}"))?;
    let uri = resolve_lancedb_path(repo)?;
    let adapter = LanceDbPreferenceAdapter::open(&uri).map_err(|e| e.to_string())?;
    let stored = with_lancedb_retry(|| UserPreferenceStore::put_revision(&adapter, pref.clone()))?;
    Ok(json!({
        "stored": true,
        "preference_id": stored.preference_id,
        "revision_id": stored.revision_id,
        "table": TABLE_PREFERENCES,
    }))
}

fn preferences_purge(repo: &Path, req: &Value) -> Result<Value, String> {
    let preference_id = required_str(req, "preference_id")?;
    let uri = resolve_lancedb_path(repo)?;
    let adapter = LanceDbPreferenceAdapter::open(&uri).map_err(|e| e.to_string())?;
    with_lancedb_retry(|| UserPreferenceStore::purge_preference(&adapter, &preference_id))?;
    Ok(json!({
        "purged": true,
        "preference_id": preference_id,
        "table": TABLE_PREFERENCES,
    }))
}

fn preferences_query_context(repo: &Path, req: &Value) -> Result<Value, String> {
    let spec_val = req.get("spec").cloned().unwrap_or_else(|| json!({}));
    let spec: QuerySpec =
        serde_json::from_value(spec_val).map_err(|e| format!("spec inválido: {e}"))?;
    let uri = resolve_lancedb_path(repo)?;
    let adapter = LanceDbPreferenceAdapter::open(&uri).map_err(|e| e.to_string())?;
    let block = UserPreferenceStore::query_context_block(&adapter, &spec).map_err(|e| e.to_string())?;
    Ok(json!({
        "context_block": block,
        "table": TABLE_PREFERENCES,
    }))
}

fn thought_search(repo: &Path, req: &Value) -> Result<Value, String> {
    let query = required_str(req, "query_text")?;
    let limit = req.get("limit").and_then(|v| v.as_u64()).unwrap_or(5) as usize;
    let embedding = LocalHashingEmbedder
        .generate_embedding(&query)
        .map_err(|e| e.to_string())?;
    let uri = resolve_lancedb_path(repo)?;
    let store = LanceDbThoughtRepo::open(&uri).map_err(|e| e.to_string())?;
    let hits = store
        .search_similar_thoughts(&embedding, limit)
        .map_err(|e| e.to_string())?;
    let memories: Vec<Value> = hits
        .into_iter()
        .map(|t| {
            json!({
                "node_id": t.node_id,
                "content": t.content,
                "metadata": t.metadata,
            })
        })
        .collect();
    Ok(json!({ "memories": memories }))
}

fn thought_store(repo: &Path, req: &Value) -> Result<Value, String> {
    let content = required_str(req, "content")?;
    let parent_id = req
        .get("parent_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string);
    let metadata = req
        .get("metadata")
        .cloned()
        .unwrap_or_else(|| json!({"status": "ACTIVE"}));
    let thought = ThoughtNode::new(parent_id, content, metadata, None);
    let node_id = thought.node_id.clone();
    let uri = resolve_lancedb_path(repo)?;
    let store = LanceDbThoughtRepo::open(&uri).map_err(|e| e.to_string())?;
    with_lancedb_retry(|| store.store_thought(thought.clone()))?;
    Ok(json!({ "node_id": node_id, "persisted": true }))
}

fn run(doc: &Value) -> Result<Value, String> {
    let req = request_inner(doc);
    let op = required_str(req, "operation")?;
    let repo_s = required_str(req, "repository_path")?;
    let repo = PathBuf::from(repo_s);
    match op.as_str() {
        "evolution.get_by_id" => evolution_get_by_id(&repo, req),
        "evolution.store" => evolution_store(&repo, req),
        "evolution.ingest_telemetry" => evolution_ingest_telemetry(&repo, req),
        "preferences.put_revision" => preferences_put_revision(&repo, req),
        "preferences.purge" => preferences_purge(&repo, req),
        "preferences.query_context" => preferences_query_context(&repo, req),
        "thought.search" => thought_search(&repo, req),
        "thought.store" => thought_store(&repo, req),
        other => Err(format!("operation desconocida: {other}")),
    }
}

fn main() {
    let doc = read_stdin_json();
    match run(&doc) {
        Ok(result) => emit_v2(true, 0, "ok", Some(result), None),
        Err(msg) => emit_v2(false, 1, "lancedb-memory-adapter-failed", None, Some(&msg)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use user_preference_core::{PreferenceAuthority, PreferenceStatus, ScopeType};

    fn fixture_repo() -> tempfile::TempDir {
        let dir = tempdir().unwrap();
        let repo = dir.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"paths":{"vectorStore":".SddIA/vector_store/","lancedbStore":".SddIA/vector_store/lancedb/"}}"#,
        )
        .unwrap();
        dir
    }

    #[test]
    fn required_operation_rejects_empty() {
        let req = json!({"operation": "  "});
        assert!(required_str(&req, "operation").is_err());
    }

    #[test]
    fn evolution_ingest_telemetry_is_idempotent() {
        let dir = fixture_repo();
        let req = json!({
            "operation": "evolution.ingest_telemetry",
            "repository_path": dir.path().display().to_string(),
            "record_id": "stim-1234",
            "polarity": "EfficientSymmetry",
            "payload": "{\"entity_id\":\"feature\"}",
            "operational_metadata": {"success": true}
        });
        let first = run(&req).unwrap();
        assert_eq!(first["record_id"], "stim-1234");
        assert!(first.get("skipped").is_none());

        let second = run(&req).unwrap();
        assert_eq!(second["skipped"], "already_indexed");
    }

    #[test]
    fn preferences_roundtrip_put_query_and_purge() {
        let dir = fixture_repo();
        let repo_s = dir.path().display().to_string();
        let pref = UserPreference {
            preference_id: String::new(),
            revision_id: String::new(),
            subject_kind: "person".into(),
            subject_key: "hash-test".into(),
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
            provenance: json!({}),
            recorded_at: "2026-11-23T12:00:00Z".into(),
            embedding: None,
        };
        let put = run(&json!({
            "operation": "preferences.put_revision",
            "repository_path": repo_s,
            "revision": pref
        }))
        .unwrap();
        assert_eq!(put["stored"], true);
        let pid = put["preference_id"].as_str().unwrap().to_string();

        let queried = run(&json!({
            "operation": "preferences.query_context",
            "repository_path": repo_s,
            "spec": {"subject_key": "hash-test", "max_results": 4}
        }))
        .unwrap();
        assert_eq!(
            queried["context_block"]["preferences"].as_array().unwrap().len(),
            1
        );

        let purged = run(&json!({
            "operation": "preferences.purge",
            "repository_path": repo_s,
            "preference_id": pid
        }))
        .unwrap();
        assert_eq!(purged["purged"], true);
    }

    #[test]
    fn thought_store_and_search_roundtrip() {
        let dir = fixture_repo();
        let repo_s = dir.path().display().to_string();
        let stored = run(&json!({
            "operation": "thought.store",
            "repository_path": repo_s,
            "content": "arquitectura hexagonal de memoria vectorial",
            "metadata": {"status": "ACTIVE"}
        }))
        .unwrap();
        assert_eq!(stored["persisted"], true);
        assert_eq!(stored["node_id"].as_str().unwrap().len(), 64);

        let found = run(&json!({
            "operation": "thought.search",
            "repository_path": repo_s,
            "query_text": "memoria vectorial",
            "limit": 5
        }))
        .unwrap();
        assert_eq!(found["memories"].as_array().unwrap().len(), 1);
    }
}
