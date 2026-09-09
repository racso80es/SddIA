//! Handler nativo `append-mayeuta-hypothesis` — Tiempo 2 Kintsugi (fail-open).

use crate::core::fracture_pbi::resolve_todos_pending_rel;
use crate::core::paths::load_paths_config;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

use super::capsules::invoke_capsule_json;

pub const ACTION_NAME: &str = "append-mayeuta-hypothesis";
pub const HYPOTHESIS_MARKER: &str = "## Hipótesis Semántica (Mayeuta) — Inferencia Asíncrona";
pub const TRACE_MARKER: &str = "## Traza de error";
pub const CRITERIO_MARKER: &str = "## Criterio de cierre";
const SYNTHESIS_MAX_LINES: usize = 15;

fn optional_str(inputs: &Value, key: &str) -> String {
    inputs
        .get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("")
        .to_string()
}

fn required_rel(inputs: &Value, key: &str) -> Result<String, String> {
    let s = optional_str(inputs, key);
    if s.is_empty() {
        Err(format!("{key} es obligatorio (string)"))
    } else {
        Ok(s)
    }
}

fn pending_rel(repo: &Path) -> String {
    resolve_todos_pending_rel(repo).unwrap_or_else(|_| "docs/todos/pending".into())
}

fn normalize_rel(rel: &str) -> String {
    rel.replace('\\', "/")
        .trim()
        .trim_start_matches("./")
        .to_string()
}

fn path_is_under_pending(rel: &str, pending: &str) -> bool {
    let rel = normalize_rel(rel);
    let pending = normalize_rel(pending).trim_end_matches('/').to_string();
    if rel.contains("..") || Path::new(&rel).is_absolute() {
        return false;
    }
    rel == pending || rel.starts_with(&format!("{pending}/"))
}

pub fn extract_error_trace(content: &str) -> Option<String> {
    let rest = content.split_once(TRACE_MARKER)?.1;
    let after_fence = rest.split_once("```")?.1;
    let body = after_fence.split_once("```")?.0;
    let t = body.trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

pub fn compose_hypothesis_prompt(
    process_name: &str,
    attempted_action: &str,
    agent_emitter: &str,
    error_trace: &str,
) -> String {
    format!(
        "[EXECUTE AS RAW KERNEL. PROHIBIT VERBOSITY. PENALIZE SPECULATION. MAX 15 LINES]\n\
Analiza la siguiente fractura inédita de SddIA y emite una hipótesis diagnóstica concisa.\n\
No repitas la traza. No inventes rutas ni normas ausentes en CONTEXTO.\n\
Emite EXCLUSIVAMENTE:\n\
\n\
- **Causa más probable:** (máximo 3 líneas)\n\
- **Componentes sospechosos:** (solo ficheros/módulos citados o implicados por la traza)\n\
- **Línea de investigación recomendada:** (una acción para el operador humano)\n\
\n\
CONTEXTO:\n\
Proceso: {process_name}\n\
Acción fallida: {attempted_action}\n\
Emisor: {agent_emitter}\n\
Traza de error:\n\
{error_trace}\n"
    )
}

pub fn truncate_hypothesis_text(text: &str) -> String {
    text.lines()
        .take(SYNTHESIS_MAX_LINES)
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

pub fn wrap_hypothesis_section(body: &str) -> String {
    format!(
        "{HYPOTHESIS_MARKER}\n\n\
> ⚠️ **Aviso Consultivo:** Inferencia `mayeuta-llm` asíncrona. No altera el sello determinista. Requiere laudo del Vértice Biológico.\n\n\
{body}\n"
    )
}

pub fn upsert_hypothesis_section(content: &str, section: &str) -> String {
    if let Some((before, after)) = content.split_once(HYPOTHESIS_MARKER) {
        let remainder = after.find("\n## ").map(|i| &after[i..]).unwrap_or("");
        let before = before.trim_end();
        if remainder.is_empty() {
            format!("{before}\n\n{section}\n")
        } else {
            format!("{before}\n\n{section}{remainder}")
        }
    } else if let Some((before, after)) = content.split_once(CRITERIO_MARKER) {
        format!(
            "{}\n\n{section}\n\n{CRITERIO_MARKER}{after}",
            before.trim_end()
        )
    } else {
        format!("{}\n\n{section}\n", content.trim_end())
    }
}

fn fail_open(reason: &str, extra: Value) -> Value {
    let mut out = json!({
        "success": true,
        "synthesized": false,
        "reason": reason,
        "pbi_path": Value::Null,
        "hypothesis_chars": Value::Null,
    });
    if let Some(obj) = extra.as_object() {
        if let Some(dst) = out.as_object_mut() {
            for (k, v) in obj {
                dst.insert(k.clone(), v.clone());
            }
        }
    }
    out
}

fn try_synthesize(repo: &Path, prompt: &str) -> Result<String, String> {
    let payload = json!({
        "operation": "SYNTHESIZE",
        "prompt": prompt,
    });
    let result = invoke_capsule_json(repo, "mayeuta-llm", &payload, false)
        .map_err(|e| format!("llm_unavailable:{e}"))?;
    if result.exit_code != 0 || result.body.get("success") != Some(&json!(true)) {
        let err = result
            .body
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or("mayeuta-llm failed");
        return Err(format!("llm_unavailable:{err}"));
    }
    let text = result
        .body
        .get("data")
        .and_then(|d| d.get("text"))
        .and_then(|v| v.as_str())
        .or_else(|| {
            result
                .body
                .get("result")
                .and_then(|d| d.get("text"))
                .and_then(|v| v.as_str())
        })
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "llm_invalid_stdout:SYNTHESIZE sin text".to_string())?;
    Ok(truncate_hypothesis_text(text))
}

fn classify_llm_err(err: &str) -> &'static str {
    let e = err.to_lowercase();
    if e.contains("timeout") {
        "llm_timeout"
    } else if e.contains("invalid_stdout") || e.contains("sin text") {
        "llm_invalid_stdout"
    } else {
        "llm_unavailable"
    }
}

fn resolve_pbi_abs(repo: &Path, rel: &str) -> PathBuf {
    repo.join(normalize_rel(rel))
}

pub fn run(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let fracture_pbi_path = match required_rel(inputs, "fracture_pbi_path") {
        Ok(s) => s,
        Err(_) => return Ok(fail_open("target_absent_or_closed", json!({}))),
    };
    let process_name = optional_str(inputs, "process_name");
    let attempted_action = optional_str(inputs, "attempted_action");
    let agent_emitter = optional_str(inputs, "agent_emitter");
    let _ = load_paths_config(repo);
    let pending = pending_rel(repo);
    if !path_is_under_pending(&fracture_pbi_path, &pending) {
        return Ok(fail_open("target_absent_or_closed", json!({})));
    }
    let abs = resolve_pbi_abs(repo, &fracture_pbi_path);
    if !abs.is_file() {
        return Ok(fail_open("target_absent_or_closed", json!({})));
    }
    let content = match fs::read_to_string(&abs) {
        Ok(c) => c,
        Err(_) => return Ok(fail_open("target_absent_or_closed", json!({}))),
    };
    let Some(trace) = extract_error_trace(&content) else {
        return Ok(fail_open("target_absent_or_closed", json!({"pbi_path": fracture_pbi_path})));
    };
    let prompt = compose_hypothesis_prompt(
        &process_name,
        &attempted_action,
        &agent_emitter,
        &trace,
    );
    let text = match try_synthesize(repo, &prompt) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("[append-mayeuta-hypothesis] fail-open: {e}");
            return Ok(fail_open(classify_llm_err(&e), json!({"pbi_path": fracture_pbi_path})));
        }
    };
    if text.is_empty() {
        return Ok(fail_open("llm_invalid_stdout", json!({"pbi_path": fracture_pbi_path})));
    }
    let replaced = content.contains(HYPOTHESIS_MARKER);
    let section = wrap_hypothesis_section(&text);
    let updated = upsert_hypothesis_section(&content, &section);
    if fs::write(&abs, &updated).is_err() {
        return Ok(fail_open("target_absent_or_closed", json!({"pbi_path": fracture_pbi_path})));
    }
    Ok(json!({
        "success": true,
        "synthesized": true,
        "reason": if replaced { "replaced" } else { "injected" },
        "pbi_path": fracture_pbi_path,
        "hypothesis_chars": text.chars().count() as i64,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;

    fn setup_repo(repo: &Path) {
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"paths":{"todos":{"pending":"docs/todos/pending","done":"docs/todos/done"}}}"#,
        )
        .unwrap();
        fs::create_dir_all(repo.join("docs/todos/pending")).unwrap();
        fs::create_dir_all(repo.join("docs/todos/done")).unwrap();
    }

    fn fixture_pbi() -> String {
        r#"---
document_id: PBI-FIX-FRACTURE-deadbeef
status: "abierto"
fracture_hash: deadbeefcafe
fracture_process: event-watcher
incident_ref: "System_Fracture_Detected — deadbeefcafe"
---

# [FIX] event-watcher — fractura sistémica

## Traza de error

```
test_trace_inedita_xyz unexplained-capsule-error
```

## Conclusión Analítica y Propuesta Evolutiva

*(Síntesis Mayeuta — Kintsugi async)*

### Diagnóstico de causa raíz

- Causa raíz no clasificada automáticamente para `event-watcher`; requiere laudo humano.

## Criterio de cierre

- [ ] Causa raíz resuelta
"#
        .to_string()
    }

    #[test]
    fn prompt_contains_kernel_and_forbids_invented_paths() {
        let p = compose_hypothesis_prompt("route-domain-event", "preseal", "execute-process", "boom.rs:1");
        assert!(p.contains("MAX 15 LINES"));
        assert!(p.contains("PENALIZE SPECULATION"));
        assert!(p.contains("No inventes rutas"));
        assert!(p.contains("Causa más probable"));
        assert!(p.contains("Componentes sospechosos"));
        assert!(p.contains("Línea de investigación recomendada"));
        assert!(p.contains("boom.rs:1"));
        assert!(!p.contains("temperature"));
    }

    #[test]
    fn truncate_caps_at_15_lines() {
        let long = (0..20).map(|i| format!("L{i}")).collect::<Vec<_>>().join("\n");
        let out = truncate_hypothesis_text(&long);
        assert_eq!(out.lines().count(), 15);
    }

    #[test]
    fn upsert_inserts_before_criterio_and_is_idempotent() {
        let content = fixture_pbi();
        let section = wrap_hypothesis_section("- **Causa más probable:** x");
        let once = upsert_hypothesis_section(&content, &section);
        assert_eq!(once.matches(HYPOTHESIS_MARKER).count(), 1);
        assert!(once.contains("## Criterio de cierre"));
        let conc_pos = once.find("## Conclusión Analítica").unwrap();
        let hyp_pos = once.find(HYPOTHESIS_MARKER).unwrap();
        let crit_pos = once.find("## Criterio de cierre").unwrap();
        assert!(conc_pos < hyp_pos && hyp_pos < crit_pos);
        let twice = upsert_hypothesis_section(&once, &wrap_hypothesis_section("- **Causa más probable:** y"));
        assert_eq!(twice.matches(HYPOTHESIS_MARKER).count(), 1);
        assert!(twice.contains("**Causa más probable:** y"));
        assert!(!twice.contains("**Causa más probable:** x"));
        assert!(twice.contains("requiere laudo humano"));
        assert!(twice.contains("fracture_hash: deadbeefcafe"));
    }

    #[test]
    fn extract_trace_from_cumulo_template() {
        let t = extract_error_trace(&fixture_pbi()).unwrap();
        assert_eq!(t, "test_trace_inedita_xyz unexplained-capsule-error");
    }

    #[test]
    fn run_absent_and_done_fail_open() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        setup_repo(repo);
        let out = run(
            repo,
            &json!({
                "fracture_pbi_path": "docs/todos/pending/missing.md",
                "process_name": "x",
            }),
        )
        .unwrap();
        assert_eq!(out.get("success"), Some(&json!(true)));
        assert_eq!(out.get("synthesized"), Some(&json!(false)));
        assert_eq!(out.get("reason"), Some(&json!("target_absent_or_closed")));

        fs::write(repo.join("docs/todos/done/closed.md"), fixture_pbi()).unwrap();
        let out = run(
            repo,
            &json!({
                "fracture_pbi_path": "docs/todos/done/closed.md",
                "process_name": "x",
            }),
        )
        .unwrap();
        assert_eq!(out.get("reason"), Some(&json!("target_absent_or_closed")));
        assert!(!fs::read_to_string(repo.join("docs/todos/done/closed.md"))
            .unwrap()
            .contains(HYPOTHESIS_MARKER));
    }

    #[test]
    fn run_without_llm_cli_fail_open_leaves_pbi() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        setup_repo(repo);
        let rel = "docs/todos/pending/pbi.md";
        fs::write(repo.join(rel), fixture_pbi()).unwrap();
        let before = fs::read_to_string(repo.join(rel)).unwrap();
        let out = run(
            repo,
            &json!({
                "fracture_pbi_path": rel,
                "process_name": "event-watcher",
                "error_trace_hash": "deadbeefcafe",
            }),
        )
        .unwrap();
        assert_eq!(out.get("success"), Some(&json!(true)));
        assert_eq!(out.get("synthesized"), Some(&json!(false)));
        let after = fs::read_to_string(repo.join(rel)).unwrap();
        assert_eq!(before, after);
        assert!(!after.contains(HYPOTHESIS_MARKER));
        assert!(matches!(
            out.get("reason").and_then(|v| v.as_str()),
            Some("llm_unavailable" | "llm_timeout" | "llm_invalid_stdout")
        ));
    }
}
