use sddia_core_memory::ports::ThoughtGraphRepository;
use sddia_core_memory::models::thought_node::ThoughtNode;
use sddia_core_memory::services::inference_binding::LocalHashingEmbedder;
use sddia_core_memory::EmbeddingGenerator;
use sddia_infrastructure_lancedb_thought::LanceDbThoughtRepo;
use sddia_io::read_stdin_json;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

const ENTITY_ID: &str = "thought-graph-access";

fn emit_v2(success: bool, exit_code: i32, message: &str, result: Option<Value>, feedback: Option<&str>) -> ! {
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

fn resolve_lancedb_and_pending(repo: &Path) -> Result<(PathBuf, PathBuf), String> {
    let cfg = load_cumulo(repo)?;
    let vs = cfg
        .pointer("/paths/vectorStore")
        .and_then(|v| v.as_str())
        .unwrap_or(".SddIA/vector_store/");
    let pending = cfg
        .pointer("/eda_bus/pending")
        .and_then(|v| v.as_str())
        .unwrap_or("./.events/pending");
    let vs_rel = vs.trim().trim_start_matches("./");
    let pending_rel = pending.trim().trim_start_matches("./");
    Ok((repo.join(vs_rel).join("lancedb"), repo.join(pending_rel)))
}

fn open_repo(repo: &Path) -> Result<LanceDbThoughtRepo, String> {
    let (lance, pending) = resolve_lancedb_and_pending(repo)?;
    LanceDbThoughtRepo::open_with_bus(lance, Some(pending)).map_err(|e| e.to_string())
}

fn search(repo: &Path, req: &Value) -> Result<Value, String> {
    let query = required_str(req, "query_text")?;
    let limit = req.get("limit").and_then(|v| v.as_u64()).unwrap_or(5) as usize;
    let embedding = LocalHashingEmbedder
        .generate_embedding(&query)
        .map_err(|e| e.to_string())?;
    let store = open_repo(repo)?;
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

fn store(repo: &Path, req: &Value) -> Result<Value, String> {
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
    let store = open_repo(repo)?;
    store.store_thought(thought).map_err(|e| e.to_string())?;
    Ok(json!({ "node_id": node_id, "persisted": true }))
}

fn run(doc: &Value) -> Result<Value, String> {
    let req = request_inner(doc);
    let op = required_str(req, "operation")?;
    let repo_s = required_str(req, "repository_path")?;
    let repo = PathBuf::from(repo_s);
    match op.as_str() {
        "search" => search(&repo, req),
        "store" => store(&repo, req),
        other => Err(format!("operation desconocida: {other}")),
    }
}

fn main() {
    let doc = read_stdin_json();
    match run(&doc) {
        Ok(result) => emit_v2(true, 0, "ok", Some(result), None),
        Err(msg) => emit_v2(false, 1, "thought-graph-failed", None, Some(&msg)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn fixture_repo() -> tempfile::TempDir {
        let dir = tempdir().unwrap();
        let repo = dir.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{
  "paths": {"vectorStore": ".SddIA/vector_store/"},
  "eda_bus": {"pending": "./.events/pending"}
}"#,
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
    fn search_empty_store_returns_empty_memories() {
        let dir = fixture_repo();
        let out = search(
            dir.path(),
            &json!({"query_text": "hola", "limit": 5}),
        )
        .unwrap();
        assert_eq!(out["memories"], json!([]));
    }

    #[test]
    fn store_returns_node_id_and_emits_thought_persisted() {
        let dir = fixture_repo();
        let out = store(
            dir.path(),
            &json!({"content": "pensamiento de prueba", "metadata": {"status": "ACTIVE"}}),
        )
        .unwrap();
        let node_id = out["node_id"].as_str().unwrap_or("");
        assert!(!node_id.is_empty());
        assert_eq!(node_id.len(), 64);
        let pending = dir.path().join(".events/pending");
        let files: Vec<_> = fs::read_dir(&pending)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("json"))
            .collect();
        assert_eq!(files.len(), 1);
        let body: Value =
            serde_json::from_str(&fs::read_to_string(files[0].path()).unwrap()).unwrap();
        assert_eq!(body["event_type"], "Thought_Persisted");
        assert_eq!(body["emitter_agent"], "lancedb-thought-repo");
        assert_eq!(body["payload"]["node_id"], node_id);
        assert!(body["payload"].get("biological_vertex_output").is_none());
    }
}
