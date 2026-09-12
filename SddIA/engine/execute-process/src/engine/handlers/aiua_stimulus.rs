//! Handler nativo `aiua-stimulus-processing` — latido CLI de la Aiúa.

use super::super::capsules::{invoke_capsule_json, invoke_tool_capsule_json};
use super::super::workspace::load_paths_config;
use crate::envelope::OrchestratorEnvelope;
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::path::Path;

fn str_opt(v: &Value, key: &str) -> Option<String> {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn env_nonempty(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn normalize_effort(raw: &str) -> Result<String, String> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "low" | "medium" | "high" => Ok(raw.trim().to_ascii_lowercase()),
        other => Err(format!("effort inválido: {other}")),
    }
}

fn resolve_effort(inputs: &Value) -> Result<String, String> {
    let raw = str_opt(inputs, "effort")
        .or_else(|| env_nonempty("SDDIA_AGY_EFFORT"))
        .or_else(|| env_nonempty("SDDIA_GEMINI_THINKING_LEVEL"))
        .unwrap_or_else(|| "high".to_string());
    normalize_effort(&raw)
}

fn resolve_print_timeout() -> String {
    let secs = env_nonempty("SDDIA_AGY_TIMEOUT_SECS")
        .and_then(|s| s.parse::<u64>().ok())
        .filter(|n| *n > 0)
        .or_else(|| {
            env_nonempty("SDDIA_CLIENT_TIMEOUT_SECONDS")
                .and_then(|s| s.parse::<u64>().ok())
                .filter(|n| *n > 0)
        })
        .unwrap_or(300);
    format!("{secs}s")
}

fn capsule_error(body: &Value, fallback: &str) -> String {
    body.get("error")
        .and_then(|v| v.as_str())
        .or_else(|| body.get("feedback").and_then(|v| v.as_str()))
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(fallback)
        .to_string()
}

fn usage_tokens(result: &Value) -> Option<Value> {
    result.get("usage").and_then(|u| {
        u.as_object()
            .filter(|o| !o.is_empty())
            .map(|_| u.clone())
    })
}

fn unwrap_tool_result(body: &Value) -> Value {
    body.get("result")
        .cloned()
        .or_else(|| body.get("data").cloned())
        .unwrap_or_else(|| body.clone())
}

fn nonempty_infer_str(v: Option<&Value>) -> Option<String> {
    let v = v?;
    if let Some(s) = v.as_str() {
        return (!s.trim().is_empty()).then(|| s.to_string());
    }
    if let Some(obj) = v.as_object() {
        return nonempty_infer_str(obj.get("text"))
            .or_else(|| nonempty_infer_str(obj.get("content")))
            .or_else(|| nonempty_infer_str(obj.get("response")));
    }
    if let Some(arr) = v.as_array() {
        let parts: Vec<String> = arr
            .iter()
            .filter_map(|item| nonempty_infer_str(Some(item)))
            .collect();
        if parts.is_empty() {
            return None;
        }
        return Some(parts.join("\n"));
    }
    None
}

/// Texto de combustión: `result.text` ≻ `response` ≻ `raw_response.{response,text,result}`.
fn extract_infer_text(result: &Value) -> String {
    nonempty_infer_str(result.get("text"))
        .or_else(|| nonempty_infer_str(result.get("response")))
        .or_else(|| nonempty_infer_str(result.pointer("/raw_response/response")))
        .or_else(|| nonempty_infer_str(result.pointer("/raw_response/text")))
        .or_else(|| nonempty_infer_str(result.pointer("/raw_response/result")))
        .unwrap_or_default()
}

fn invoke_thought_graph(repo: &Path, request: Value) -> Result<Value, String> {
    let payload = json!({ "request": request });
    let cap = invoke_tool_capsule_json(repo, "thought-graph-access", &payload, false)?;
    if cap.exit_code != 0 || cap.body.get("success") == Some(&json!(false)) {
        return Err(cap
            .body
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or("thought-graph-access failed")
            .to_string());
    }
    Ok(unwrap_tool_result(&cap.body))
}

pub fn retrieve_active_context(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let query = str_opt(inputs, "query_text").ok_or("query_text obligatorio")?;
    let limit = inputs.get("limit").and_then(|v| v.as_u64()).unwrap_or(5);
    let result = invoke_thought_graph(
        repo,
        json!({
            "operation": "search",
            "query_text": query,
            "limit": limit,
            "repository_path": repo.display().to_string(),
        }),
    )?;
    let memories = result.get("memories").cloned().unwrap_or_else(|| json!([]));
    Ok(json!({ "success": true, "memories": memories }))
}

fn yaml_frontmatter_str(genome: &str, key: &str) -> Option<String> {
    let rest = genome.strip_prefix("---")?;
    let yaml = rest.split_once("\n---").map(|(y, _)| y).unwrap_or(rest);
    let prefix = format!("{key}:");
    for line in yaml.lines() {
        let line = line.trim();
        let Some(raw) = line.strip_prefix(&prefix) else {
            continue;
        };
        let v = raw.trim().trim_matches('"').trim_matches('\'').trim();
        if !v.is_empty() {
            return Some(v.to_string());
        }
    }
    None
}

fn identity_preface(genome: &str) -> String {
    let name = yaml_frontmatter_str(genome, "name");
    let entity_type = yaml_frontmatter_str(genome, "entity_type");
    match (name.as_deref(), entity_type.as_deref()) {
        (Some(n), Some(t)) => format!(
            "Eres {n}, la {t} del ecosistema SddIA. Hablas en primera persona. El genoma que sigue es tu identidad; no eres un asistente genérico.\n\n"
        ),
        (Some(n), None) => format!(
            "Eres {n}, la Aiúa del ecosistema SddIA. Hablas en primera persona. El genoma que sigue es tu identidad; no eres un asistente genérico.\n\n"
        ),
        _ => "Eres la Aiúa del ecosistema SddIA. Hablas en primera persona. El genoma que sigue es tu identidad; no eres un asistente genérico.\n\n"
            .to_string(),
    }
}

pub fn invoke_aiua_core(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let prompt = str_opt(inputs, "prompt").ok_or("prompt obligatorio")?;
    let cfg = load_paths_config(repo)?;
    let conscience = cfg
        .pointer("/directories/conscience")
        .and_then(|v| v.as_str())
        .unwrap_or("SddIA/conscience");
    let genome_path = repo.join(conscience).join("aiua_core.md");
    let genome = fs::read_to_string(&genome_path)
        .map_err(|e| format!("genoma ausente {}: {e}", genome_path.display()))?;
    let mut assembled = identity_preface(&genome);
    assembled.push_str("---\n\n");
    assembled.push_str(&genome);
    assembled.push_str("\n\n---\n\n");
    if let Some(ctx) = inputs.get("active_context") {
        assembled.push_str("## Contexto activo\n\n");
        assembled.push_str(&ctx.to_string());
        assembled.push_str("\n\n");
    }
    assembled.push_str("## Estímulo\n\n");
    assembled.push_str(&prompt);
    let model = str_opt(inputs, "model")
        .or_else(|| {
            env::var("SDDIA_GEMINI_MODEL")
                .ok()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
        })
        .unwrap_or_default();
    Ok(json!({
        "success": true,
        "assembled_prompt": assembled,
        "model": model,
    }))
}

pub fn persist_thought_record(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let prompt = str_opt(inputs, "prompt").ok_or("prompt obligatorio")?;
    let response = str_opt(inputs, "response_text").ok_or("response_text obligatorio")?;
    let content = format!("stimulus:\n{prompt}\n\nresponse:\n{response}");
    let metadata = inputs
        .get("metadata")
        .cloned()
        .unwrap_or_else(|| json!({"status": "ACTIVE"}));
    let result = invoke_thought_graph(
        repo,
        json!({
            "operation": "store",
            "content": content,
            "metadata": metadata,
            "repository_path": repo.display().to_string(),
        }),
    )?;
    let thought_id = result
        .get("node_id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    Ok(json!({
        "success": true,
        "thought_id": thought_id,
        "persisted": true,
    }))
}

fn infer_antigravity_cli(
    repo: &Path,
    prompt: &str,
    model: &str,
    effort: &str,
    print_timeout: &str,
) -> Result<Value, String> {
    let mut params = json!({
        "effort": effort,
        "print_timeout": print_timeout,
    });
    if !model.is_empty() {
        params["model"] = json!(model);
    }
    let cap = invoke_capsule_json(
        repo,
        "antigravity-cli-executor",
        &json!({
            "request": {
                "prompt": prompt,
                "parameters": params,
            }
        }),
        false,
    )?;
    if cap.exit_code != 0 || cap.body.get("success") == Some(&json!(false)) {
        return Err(capsule_error(&cap.body, "agy-failed"));
    }
    Ok(cap.body)
}

pub fn run(repo: &Path, inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let prompt = str_opt(inputs, "prompt").ok_or("prompt obligatorio")?;
    let query = str_opt(inputs, "context_query").unwrap_or_else(|| prompt.clone());
    let model_in = str_opt(inputs, "model").unwrap_or_default();
    let effort = resolve_effort(inputs)?;
    let print_timeout = resolve_print_timeout();

    let ctx = retrieve_active_context(
        repo,
        &json!({ "query_text": query, "limit": 5 }),
    )?;
    let memories = ctx.get("memories").cloned().unwrap_or_else(|| json!([]));

    let assembled = invoke_aiua_core(
        repo,
        &json!({
            "prompt": prompt,
            "active_context": memories,
            "model": model_in,
        }),
    )?;
    let assembled_prompt = assembled
        .get("assembled_prompt")
        .and_then(|v| v.as_str())
        .ok_or("invoke-aiua-core sin assembled_prompt")?;
    let model = assembled
        .get("model")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let infer_body = infer_antigravity_cli(
        repo,
        assembled_prompt,
        &model,
        &effort,
        &print_timeout,
    )?;
    let result = unwrap_tool_result(&infer_body);
    let mut response = extract_infer_text(&result);
    let intent = super::super::aiua_intent::resolve_intent(&response);
    let mut dispatched: Option<Value> = None;
    if let Some(fc) = intent {
        let name = fc.get("name").and_then(|v| v.as_str()).unwrap_or("");
        if super::super::aiua_intent::is_motor_tendon(name) {
            match super::super::aiua_intent::run(
                repo,
                &json!({ "function_call": fc }),
            ) {
                Ok(data) => dispatched = Some(data),
                Err(e) => {
                    response = format!("{response}\n\n[despacho motor rechazado: {e}]");
                }
            }
        }
    }
    if response.trim().is_empty() {
        if let Some(data) = &dispatched {
            let name = data
                .get("intent_dispatched")
                .and_then(|v| v.as_str())
                .unwrap_or("intent");
            let eid = data.get("event_id").and_then(|v| v.as_str()).unwrap_or("");
            response = format!("intent={name}; event_id={eid}");
        } else {
            return Err("agy respuesta vacía".into());
        }
    }
    let duration_ms = result
        .get("durationMs")
        .or_else(|| infer_body.get("result").and_then(|r| r.get("durationMs")))
        .cloned()
        .unwrap_or(json!(null));
    let infer_model = if model.is_empty() {
        result
            .get("model")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    } else {
        model
    };
    let tokens = usage_tokens(&result);

    let persisted = persist_thought_record(
        repo,
        &json!({
            "prompt": prompt,
            "response_text": response,
        }),
    )?;
    let thought_id = persisted
        .get("thought_id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let mut telemetry = json!({
        "duration_ms": duration_ms,
        "model": infer_model,
    });
    if let Some(t) = tokens {
        telemetry["tokens"] = t;
    }

    let mut data = json!({
        "thought_id": thought_id,
        "response": response,
        "telemetry": telemetry,
    });
    let dispatch_phase = if let Some(d) = &dispatched {
        if let Some(eid) = d.get("event_id") {
            data["event_id"] = eid.clone();
            data["correlation_id"] = d
                .get("correlation_id")
                .cloned()
                .unwrap_or_else(|| eid.clone());
        }
        if let Some(name) = d.get("intent_dispatched") {
            data["intent_dispatched"] = name.clone();
        }
        json!({
            "phase_name": "Despacho-Motor",
            "status": "executed",
            "handler": "dispatch-aiua-intent"
        })
    } else {
        json!({
            "phase_name": "Despacho-Motor",
            "status": "skipped",
            "handler": "dispatch-aiua-intent"
        })
    };

    Ok(OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(data),
        error: None,
        execution_report: Some(json!({
            "process_name": "aiua-stimulus-processing",
            "phases": [
                {"phase_name": "Triaje-Contexto", "status": "executed", "handler": "retrieve-active-context"},
                {"phase_name": "Inyeccion-Genomica", "status": "executed", "handler": "invoke-aiua-core"},
                {"phase_name": "Combustion-Inferencia", "status": "executed", "handler": "antigravity-cli-executor"},
                dispatch_phase,
                {"phase_name": "Consolidacion-Memoria", "status": "executed", "handler": "persist-thought-record"},
            ]
        })),
        exit_code: 0,
    })
}

pub fn try_action(repo: &Path, action_name: &str, inputs: &Value) -> Result<Option<Value>, String> {
    let data = match action_name {
        "retrieve-active-context" => retrieve_active_context(repo, inputs)?,
        "invoke-aiua-core" => invoke_aiua_core(repo, inputs)?,
        "persist-thought-record" => persist_thought_record(repo, inputs)?,
        _ => return Ok(None),
    };
    Ok(Some(data))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use tempfile::tempdir;

    static LAB_ENV: Mutex<()> = Mutex::new(());

    #[test]
    fn invoke_aiua_core_concatenates_genome() {
        let dir = tempdir().unwrap();
        let repo = dir.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::create_dir_all(repo.join("SddIA/conscience")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"directories":{"conscience":"SddIA/conscience"}}"#,
        )
        .unwrap();
        fs::write(repo.join("SddIA/conscience/aiua_core.md"), "# GENOMA\nley").unwrap();
        let out = invoke_aiua_core(
            repo,
            &json!({"prompt": "hola", "active_context": []}),
        )
        .unwrap();
        let assembled = out["assembled_prompt"].as_str().unwrap();
        assert!(assembled.contains("# GENOMA"));
        assert!(assembled.contains("hola"));
        assert!(assembled.contains("Eres la Aiúa"));
        assert!(assembled.contains("Hablas en primera persona"));
        assert!(assembled.starts_with("Eres la Aiúa"));
        assert!(!assembled.contains("generativelanguage.googleapis.com"));
        assert!(!assembled.contains("CONSTITUTION_CORE"));
    }

    #[test]
    fn invoke_aiua_core_preface_uses_frontmatter_name() {
        let dir = tempdir().unwrap();
        let repo = dir.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::create_dir_all(repo.join("SddIA/conscience")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"directories":{"conscience":"SddIA/conscience"}}"#,
        )
        .unwrap();
        fs::write(
            repo.join("SddIA/conscience/aiua_core.md"),
            "---\nname: \"FixtureName\"\nentity_type: \"Aiúa\"\n---\n# GENOMA\nley",
        )
        .unwrap();
        let out = invoke_aiua_core(
            repo,
            &json!({"prompt": "hola", "active_context": []}),
        )
        .unwrap();
        let assembled = out["assembled_prompt"].as_str().unwrap();
        assert!(assembled.starts_with("Eres FixtureName, la Aiúa"));
        assert!(assembled.contains("# GENOMA"));
        assert!(!assembled.contains("CONSTITUTION_CORE"));
    }

    fn workspace_debug_bin(name: &str) -> Option<std::path::PathBuf> {
        let p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/debug")
            .join(name);
        p.is_file().then_some(p)
    }

    fn copy_bin(src: &std::path::Path, dest_dir: &Path, name: &str) {
        fs::create_dir_all(dest_dir).unwrap();
        fs::copy(src, dest_dir.join(name)).unwrap();
    }

    #[test]
    fn normalize_effort_accepts_case_insensitive_whitelist() {
        assert_eq!(normalize_effort("HIGH").unwrap(), "high");
        assert_eq!(normalize_effort("Medium").unwrap(), "medium");
        assert_eq!(normalize_effort("low").unwrap(), "low");
        assert!(normalize_effort("ultra").unwrap_err().contains("effort inválido"));
    }

    #[test]
    fn usage_tokens_omits_empty_object() {
        assert!(usage_tokens(&json!({"usage": {}})).is_none());
        let t = usage_tokens(&json!({"usage": {"input_tokens": 1}})).unwrap();
        assert_eq!(t["input_tokens"], 1);
        assert!(usage_tokens(&json!({})).is_none());
    }

    #[test]
    fn extract_infer_text_prefers_text_then_raw_response_object() {
        assert_eq!(
            extract_infer_text(&json!({"text": "ok", "raw_response": {"response": "no"}})),
            "ok"
        );
        assert_eq!(
            extract_infer_text(&json!({
                "text": "",
                "raw_response": {"status": "SUCCESS", "response": {"text": "latido"}}
            })),
            "latido"
        );
        assert!(extract_infer_text(&json!({"text": "  ", "raw_response": {"response": ""}})).is_empty());
    }

    #[test]
    fn lab_mock_empty_memories_yields_duration_and_thought_id() {
        let Some(graph) = workspace_debug_bin("thought-graph-access") else {
            return;
        };
        let Some(agy) = workspace_debug_bin("antigravity-cli-executor") else {
            return;
        };
        let dir = tempdir().unwrap();
        let repo = dir.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::create_dir_all(repo.join("SddIA/conscience")).unwrap();
        fs::create_dir_all(repo.join("SddIA/target/debug")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{
  "directories": {"conscience": "SddIA/conscience"},
  "paths": {"vectorStore": ".SddIA/vector_store/"},
  "eda_bus": {"pending": "./.events/pending"}
}"#,
        )
        .unwrap();
        fs::write(repo.join("SddIA/conscience/aiua_core.md"), "# GENOMA\nley").unwrap();
        copy_bin(&graph, &repo.join("SddIA/target/debug"), "thought-graph-access");
        copy_bin(&agy, &repo.join("SddIA/target/debug"), "antigravity-cli-executor");
        let _lab = LAB_ENV.lock().unwrap();
        let prev_out = std::env::var("SDDIA_LAB_MOCK_OUTBOUND").ok();
        let prev_intent = std::env::var("SDDIA_LAB_MOCK_AIUA_INTENT").ok();
        std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "1");
        std::env::remove_var("SDDIA_LAB_MOCK_AIUA_INTENT");
        let out = run(
            repo,
            &json!({
                "prompt": "latido de prueba",
                "model": "lab-flash"
            }),
        );
        match prev_out {
            Some(v) => std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", v),
            None => std::env::remove_var("SDDIA_LAB_MOCK_OUTBOUND"),
        }
        match prev_intent {
            Some(v) => std::env::set_var("SDDIA_LAB_MOCK_AIUA_INTENT", v),
            None => std::env::remove_var("SDDIA_LAB_MOCK_AIUA_INTENT"),
        }
        let env = out.expect("latido lab-mock");
        assert!(env.success);
        let data = env.data.expect("data");
        let thought_id = data["thought_id"].as_str().unwrap_or("");
        assert!(!thought_id.is_empty());
        assert_eq!(thought_id.len(), 64);
        assert!(data["response"].as_str().unwrap_or("").starts_with("lab-mock-agy:"));
        assert!(data["telemetry"]["duration_ms"].as_u64().is_some());
        assert_eq!(data["telemetry"]["model"].as_str().unwrap_or(""), "lab-flash");
        assert!(data["telemetry"].get("tokens").is_none());
        let pending = repo.join(".events/pending");
        let events: Vec<_> = fs::read_dir(&pending)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("json"))
            .collect();
        assert_eq!(events.len(), 1);
        let body: Value =
            serde_json::from_str(&fs::read_to_string(events[0].path()).unwrap()).unwrap();
        assert_eq!(body["event_type"], "Thought_Persisted");
        assert_eq!(body["emitter_agent"], "lancedb-thought-repo");
        assert!(data.get("intent_dispatched").is_none());
        let domain = repo.join(".events/domain");
        if domain.is_dir() {
            let domain_events: Vec<_> = fs::read_dir(&domain)
                .unwrap()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("json"))
                .collect();
            assert!(domain_events.is_empty());
        }
    }

    #[test]
    fn lab_mock_overlay_intent_dispatches_domain_without_tqm() {
        let Some(graph) = workspace_debug_bin("thought-graph-access") else {
            return;
        };
        let Some(agy) = workspace_debug_bin("antigravity-cli-executor") else {
            return;
        };
        let dir = tempdir().unwrap();
        let repo = dir.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::create_dir_all(repo.join("SddIA/conscience")).unwrap();
        fs::create_dir_all(repo.join("SddIA/events/domain")).unwrap();
        fs::create_dir_all(repo.join("SddIA/target/debug")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{
  "directories": {"conscience": "SddIA/conscience", "events": "SddIA/events", "suites": "SddIA/suites"},
  "paths": {"vectorStore": ".SddIA/vector_store/"},
  "eda_bus": {"pending": "./.events/pending"},
  "eda_fractal": {"domain": "./.events/domain"}
}"#,
        )
        .unwrap();
        let class = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../../SddIA/events/domain/aiua-process-requested.md");
        fs::copy(
            class,
            repo.join("SddIA/events/domain/aiua-process-requested.md"),
        )
        .unwrap();
        fs::write(repo.join("SddIA/conscience/aiua_core.md"), "# GENOMA\nley").unwrap();
        copy_bin(&graph, &repo.join("SddIA/target/debug"), "thought-graph-access");
        copy_bin(&agy, &repo.join("SddIA/target/debug"), "antigravity-cli-executor");
        let _lab = LAB_ENV.lock().unwrap();
        let prev_out = std::env::var("SDDIA_LAB_MOCK_OUTBOUND").ok();
        let prev_intent = std::env::var("SDDIA_LAB_MOCK_AIUA_INTENT").ok();
        std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "1");
        std::env::set_var(
            "SDDIA_LAB_MOCK_AIUA_INTENT",
            r#"{"name":"iniciar_feature","args":{"goal":"motor","target_component":"aiua"}}"#,
        );
        let out = run(repo, &json!({"prompt": "ordena un feature", "model": "lab-flash"}));
        match prev_out {
            Some(v) => std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", v),
            None => std::env::remove_var("SDDIA_LAB_MOCK_OUTBOUND"),
        }
        match prev_intent {
            Some(v) => std::env::set_var("SDDIA_LAB_MOCK_AIUA_INTENT", v),
            None => std::env::remove_var("SDDIA_LAB_MOCK_AIUA_INTENT"),
        }
        let env = out.expect("latido overlay");
        assert!(env.success);
        let data = env.data.expect("data");
        assert_eq!(data["intent_dispatched"], "iniciar_feature");
        let eid = data["event_id"].as_str().unwrap();
        assert!(!eid.is_empty());
        let domain_path = repo.join(".events/domain").join(format!("{eid}.json"));
        assert!(domain_path.is_file());
        let body: Value = serde_json::from_str(&fs::read_to_string(&domain_path).unwrap()).unwrap();
        assert_eq!(body["event_type"], "Aiua_Process_Requested");
        assert_eq!(body["payload"]["process"], "feature");
        let pending_aiua = repo.join(".events/pending").join(format!("{eid}.json"));
        assert!(!pending_aiua.is_file());
        let report = env.execution_report.unwrap();
        let phases = report["phases"].as_array().unwrap();
        assert!(phases.iter().any(|p| p["phase_name"] == "Despacho-Motor" && p["status"] == "executed"));
    }

    #[test]
    fn lab_mock_overlay_delegar_habito_does_not_join_ingest() {
        let Some(graph) = workspace_debug_bin("thought-graph-access") else {
            return;
        };
        let Some(agy) = workspace_debug_bin("antigravity-cli-executor") else {
            return;
        };
        let dir = tempdir().unwrap();
        let repo = dir.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::create_dir_all(repo.join("SddIA/conscience")).unwrap();
        fs::create_dir_all(repo.join("SddIA/events/domain")).unwrap();
        fs::create_dir_all(repo.join("SddIA/target/debug")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{
  "directories": {"conscience": "SddIA/conscience", "events": "SddIA/events", "suites": "SddIA/suites"},
  "paths": {"vectorStore": ".SddIA/vector_store/"},
  "eda_bus": {"pending": "./.events/pending"},
  "eda_fractal": {"domain": "./.events/domain"}
}"#,
        )
        .unwrap();
        let class_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../SddIA/events/domain");
        fs::copy(
            class_dir.join("user-preference-change-requested.md"),
            repo.join("SddIA/events/domain/user-preference-change-requested.md"),
        )
        .unwrap();
        fs::write(repo.join("SddIA/conscience/aiua_core.md"), "# GENOMA\nley").unwrap();
        copy_bin(&graph, &repo.join("SddIA/target/debug"), "thought-graph-access");
        copy_bin(&agy, &repo.join("SddIA/target/debug"), "antigravity-cli-executor");
        let _lab = LAB_ENV.lock().unwrap();
        let prev_out = std::env::var("SDDIA_LAB_MOCK_OUTBOUND").ok();
        let prev_intent = std::env::var("SDDIA_LAB_MOCK_AIUA_INTENT").ok();
        std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", "1");
        std::env::set_var(
            "SDDIA_LAB_MOCK_AIUA_INTENT",
            r#"{"name":"delegar_habito","args":{"subject_hint":"computrabajo","raw_utterance":"borra los correos de computrabajo"}}"#,
        );
        let out = run(
            repo,
            &json!({"prompt": "borra los correos de computrabajo", "model": "lab-flash"}),
        );
        match prev_out {
            Some(v) => std::env::set_var("SDDIA_LAB_MOCK_OUTBOUND", v),
            None => std::env::remove_var("SDDIA_LAB_MOCK_OUTBOUND"),
        }
        match prev_intent {
            Some(v) => std::env::set_var("SDDIA_LAB_MOCK_AIUA_INTENT", v),
            None => std::env::remove_var("SDDIA_LAB_MOCK_AIUA_INTENT"),
        }
        let env = out.expect("latido overlay habito");
        assert!(env.success);
        let data = env.data.expect("data");
        assert_eq!(data["intent_dispatched"], "delegar_habito");
        let eid = data["event_id"].as_str().unwrap();
        assert!(!eid.is_empty());
        let domain_path = repo.join(".events/domain").join(format!("{eid}.json"));
        assert!(domain_path.is_file());
        let body: Value = serde_json::from_str(&fs::read_to_string(&domain_path).unwrap()).unwrap();
        assert_eq!(body["event_type"], "User_Preference_Change_Requested");
        assert_eq!(body["emitter_agent"], "emit-user-preference-change-requested");
        assert_eq!(body["payload"]["predicate"], "mute");
        assert_eq!(body["payload"]["value"]["muted"], true);
        assert!(body["payload"].get("raw_utterance").is_none());
        let report = env.execution_report.unwrap();
        let phases = report["phases"].as_array().unwrap();
        assert!(phases.iter().any(|p| p["phase_name"] == "Despacho-Motor" && p["status"] == "executed"));
        assert!(!phases.iter().any(|p| p["phase_name"].as_str().unwrap_or("").contains("user-preference-ingest")
            || p["process_name"].as_str() == Some("user-preference-ingest")));
        assert_eq!(report["process_name"], "aiua-stimulus-processing");
    }

    #[test]
    fn process_genome_combustion_is_antigravity_cli() {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let text = fs::read_to_string(root.join("SddIA/process/aiua-stimulus-processing.md")).unwrap();
        assert!(text.contains("skill:antigravity-cli-executor"));
        assert!(text.contains("action:dispatch-aiua-intent"));
        assert!(!text.contains("tool:gemini-http-infer"));
        assert!(!text.contains("gemini-3.8-flash"));
    }

    #[test]
    fn process_genome_has_no_kalma2_ui_coupling() {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
        for rel in [
            "SddIA/process/aiua-stimulus-processing.md",
            "SddIA/actions/retrieve-active-context.md",
            "SddIA/actions/invoke-aiua-core.md",
            "SddIA/actions/persist-thought-record.md",
            "SddIA/tools/thought-graph-access.md",
            "SddIA/tools/thought-graph-access/src/main.rs",
        ] {
            let text = fs::read_to_string(root.join(rel)).unwrap_or_default().to_lowercase();
            assert!(
                !text.contains("kalma2-interact") && !text.contains("caja de texto"),
                "fuga Kalma2 UI en {rel}"
            );
        }
    }
}
