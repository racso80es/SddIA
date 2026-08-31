//! Handler nativo `enrich-fracture-pbi-kaizen` — síntesis Mayeuta sobre PBI Cúmulo (D-P6T.1).

use crate::core::fracture_pbi::{
    fracture_trace_hash, resolve_enrich_target, scan_fracture_ledger, slugify_process_name,
};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

fn required_str(inputs: &Value, key: &str) -> Result<String, String> {
    match inputs.get(key).and_then(|v| v.as_str()) {
        Some(s) if !s.trim().is_empty() => Ok(s.trim().to_string()),
        _ => Err(format!("{key} es obligatorio (string)")),
    }
}

fn optional_str(inputs: &Value, key: &str) -> Option<String> {
    inputs
        .get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

/// Traza canónica Argos `emit_system_fracture`. Match **solo** `error_trace`
/// (F-MAYEUTA-HB-TOKEN-TRAP: no usar `heartbeat`/`daemon`/`audit` sobre el blob concatenado).
fn is_heartbeat_starvation_trace(error_trace: &str) -> bool {
    error_trace.contains("Centinela ")
        && error_trace.contains("omitió")
        && error_trace.contains("ciclos consecutivos de Daemon_Heartbeat")
        && error_trace.contains("umbral=")
        && error_trace.contains("last_heartbeat=")
}

/// Paridad `execute-action.py::_analyze_fracture_kaizen` → (veredicto, root_md, section).
pub fn analyze_fracture_kaizen(
    process_name: &str,
    error_trace: &str,
    attempted_action: &str,
    agent_emitter: &str,
) -> (String, String, String) {
    let blob = format!(
        "{error_trace}\n{attempted_action}\n{process_name}",
    )
    .to_lowercase();
    let hook_blob = format!("{error_trace}\n{attempted_action}").to_lowercase();

    let mut root_causes: Vec<String> = Vec::new();
    let mut proposals: Vec<(String, String)> = Vec::new();

    let has_any = |tokens: &[&str]| tokens.iter().any(|t| blob.contains(t));
    let has_any_in = |hay: &str, tokens: &[&str]| tokens.iter().any(|t| hay.contains(t));

    if is_heartbeat_starvation_trace(error_trace) {
        root_causes.push(
            "Inanición de `Daemon_Heartbeat` con proceso vivo (el auditor no emite si el PID está muerto); \
             no es muerte del centinela. `process_name` es `daemon_id`, no un proceso de `directories.process`."
                .into(),
        );
        proposals.push((
            "refactor_tool".into(),
            "Emitir latido en worker / no bloquear el hilo de heartbeat (paridad keepalive de centinelas hermanos)."
                .into(),
        ));
    }

    if has_any_in(
        &hook_blob,
        &[
            "delivery-close-cycle failed for",
            "recurs",
            "re-entrada",
        ],
    ) {
        root_causes.push(
            "Recursión o re-entrada en la cadena hook Git ↔ proceso de cierre (`delivery-close-cycle`)."
                .into(),
        );
        proposals.push((
            "refactor_tool".into(),
            "Auditar por qué la guarda `SDDIA_HOOK_DELIVERY_CLOSE` no cortó la re-entrada; \
             no reimplementarla. Verificar `in_delivery_close_cycle` en el subproceso `git-manager`."
                .into(),
        ));
    }
    if has_any(&[
        "gh ",
        "gh pr",
        "git push",
        "git merge",
        "bypass",
        "skip_hooks",
        "curl ",
    ]) {
        root_causes.push(
            "Violación de jurisdicción delegada: terminal raw usada para evadir cápsula o proceso oficial."
                .into(),
        );
        proposals.push((
            "new_norm".into(),
            "Reforzar `SddIA/norms/obediencia-procesos.md` § Ley de Jurisdicción Delegada; \
             prohibir bypass silencioso ante fallo."
                .into(),
        ));
    }
    if has_any(&["orphan", "ruido de sistema", "eda genómica", "huérfan"]) {
        root_causes.push(
            "Entidad genómica indexada sin correlato `Domain_Entity_Created` en bus EDA.".into(),
        );
        proposals.push((
            "refactor_tool".into(),
            "Ejecutar backfill Fase C (`audit-entity-eda-coverage --emit`) o integrar sello en \
             `entity-manager` create."
                .into(),
        ));
    }
    if has_any(&["timeout", "block", "abort", "failed", "colaps"]) {
        root_causes.push(
            "Bloqueo operativo sin escalado Kintsugi previo al intento de recuperación manual."
                .into(),
        );
        proposals.push((
            "prompt_adjustment".into(),
            "Ajustar instrucción operador IA: detener, emitir `System_Fracture_Detected`, \
             notificar al Vértice Biológico — no continuar entrega."
                .into(),
        ));
    }

    if root_causes.is_empty() {
        root_causes.push(format!(
            "Causa raíz no clasificada automáticamente para `{process_name}`; requiere laudo humano."
        ));
        proposals.push((
            "process_fix".into(),
            format!(
                "Auditar proceso `{process_name}`, acción `{attempted_action}` y emisor `{agent_emitter}`."
            ),
        ));
    }

    let verdict_priority = [
        "new_norm",
        "refactor_tool",
        "prompt_adjustment",
        "process_fix",
    ];
    let mut verdict = proposals[0].0.clone();
    for vp in verdict_priority {
        if proposals.iter().any(|p| p.0 == vp) {
            verdict = vp.to_string();
            break;
        }
    }

    fn verdict_label(v: &str) -> &str {
        match v {
            "new_norm" => "Nueva norma o endurecimiento normativo",
            "refactor_tool" => "Refactor de herramienta / cápsula / handler lab",
            "prompt_adjustment" => "Ajuste de prompt o regla operador IA",
            "process_fix" => "Corrección de proceso oficial",
            other => other,
        }
    }

    let proposal_md = proposals
        .iter()
        .map(|(v, p)| format!("- **{}:** {p}", verdict_label(v)))
        .collect::<Vec<_>>()
        .join("\n");
    let root_md = root_causes
        .iter()
        .map(|c| format!("- {c}"))
        .collect::<Vec<_>>()
        .join("\n");

    let section = format!(
        r#"## Conclusión Analítica y Propuesta Evolutiva

*(Síntesis Mayeuta — Kintsugi async)*

### Diagnóstico de causa raíz

{root_md}

### Veredicto evolutivo

**{verdict_label}** (`{verdict}`)

### Propuestas

{proposal_md}

> Mayeuta transforma la fractura en deuda accionable; el Vértice Biológico valida antes de ejecutar."#,
        verdict_label = verdict_label(&verdict),
    );

    (verdict, root_md, section)
}

/// Paridad `execute-action.py::_upsert_fracture_kaizen_section`.
pub fn upsert_fracture_kaizen_section(content: &str, section: &str) -> String {
    const MARKER: &str = "## Conclusión Analítica y Propuesta Evolutiva";

    if let Some((before, after)) = content.split_once(MARKER) {
        let remainder = after.find("\n## ").map(|i| &after[i..]).unwrap_or("");
        let before = before.trim_end();
        if remainder.is_empty() {
            format!("{before}\n\n{section}\n")
        } else {
            format!("{before}\n\n{section}{remainder}")
        }
    } else {
        format!("{}\n\n{section}\n", content.trim_end())
    }
}

/// Ejecuta `enrich-fracture-pbi-kaizen` (paridad `execute-action.py::_run_enrich_fracture_pbi_kaizen`).
pub fn run(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let process_name = required_str(inputs, "process_name")?;
    let error_trace = required_str(inputs, "error_trace")?;
    let agent_emitter = required_str(inputs, "agent_emitter")?;
    let attempted_action = required_str(inputs, "attempted_action")?;

    let trace_hash = fracture_trace_hash(&error_trace);
    let fracture_process = slugify_process_name(&process_name);
    let scan = scan_fracture_ledger(repo)?;

    let target_rel = match resolve_enrich_target(
        repo,
        &scan,
        optional_str(inputs, "cumulo_pbi_path").as_deref(),
        &trace_hash,
        &fracture_process,
    ) {
        Some(rel) => rel,
        None => {
            return Ok(json!({
                "success": true,
                "reason": "no_target",
                "message": "no_target",
            }));
        }
    };

    let target = repo.join(&target_rel);
    let (verdict, _, section) = analyze_fracture_kaizen(
        &process_name,
        &error_trace,
        &attempted_action,
        &agent_emitter,
    );
    let content = fs::read_to_string(&target).map_err(|e| e.to_string())?;
    fs::write(&target, upsert_fracture_kaizen_section(&content, &section))
        .map_err(|e| e.to_string())?;

    Ok(json!({
        "success": true,
        "target_path": target_rel,
        "message": "PBI enriquecido con síntesis Kaizen",
        "evolution_verdict": verdict,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::materialize_fracture_pbi;
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

    #[test]
    fn analyze_fracture_kaizen_recursion_verdict() {
        let (verdict, _, section) = analyze_fracture_kaizen(
            "delivery-close-cycle",
            "SddIA pre-push: BLOCKED — delivery-close-cycle failed for feat/x",
            "Publicación remota",
            "execute-process",
        );
        assert_eq!(verdict, "refactor_tool");
        assert!(section.contains("Recursión o re-entrada"));
        assert!(section.contains("no reimplementarla"));
    }

    #[test]
    fn analyze_fracture_kaizen_prepush_evol_gate_not_hook_recursion() {
        let (verdict, _, section) = analyze_fracture_kaizen(
            "delivery-close-cycle",
            "SddIA pre-push: BLOCKED — evolution gate (--range --if-touched) failed\nerror: falló el empuje de algunas referencias a 'https://github.com/racso80es/SddIA.git'",
            "Publicación remota",
            "execute-process",
        );
        assert!(!section.contains("Recursión o re-entrada"));
        assert!(!section.contains("Implementar guarda `SDDIA_HOOK_DELIVERY_CLOSE`"));
        assert_ne!(verdict, "refactor_tool");
    }

    #[test]
    fn analyze_fracture_kaizen_dns_not_hook_recursion() {
        let (verdict, _, section) = analyze_fracture_kaizen(
            "delivery-close-cycle",
            "fatal: Could not resolve host: github.com",
            "Publicación remota",
            "execute-process",
        );
        assert!(!section.contains("Recursión o re-entrada"));
        assert_ne!(verdict, "refactor_tool");
    }

    #[test]
    fn analyze_fracture_kaizen_bypass_new_norm() {
        let (verdict, _, _) = analyze_fracture_kaizen(
            "feature",
            "operator used gh pr create",
            "delivery-close-cycle",
            "tekton",
        );
        assert_eq!(verdict, "new_norm");
    }

    #[test]
    fn analyze_fracture_kaizen_heartbeat_starvation() {
        let (verdict, _, section) = analyze_fracture_kaizen(
            "email-watcher",
            "Centinela email-watcher omitió 3 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-08-30T07:51:47Z",
            "daemon-heartbeat-audit",
            "argos",
        );
        assert_eq!(verdict, "refactor_tool");
        assert!(section.contains("Inanición de `Daemon_Heartbeat`"));
        assert!(!section.contains("no clasificada"));
        assert!(!section.contains("Auditar proceso"));
        assert!(!section.contains("Recursión o re-entrada"));
    }

    #[test]
    fn analyze_fracture_kaizen_heartbeat_not_from_action_name() {
        let (verdict, _, section) = analyze_fracture_kaizen(
            "email-watcher",
            "timeout in worker",
            "daemon-heartbeat-audit",
            "argos",
        );
        assert!(!section.contains("Inanición de `Daemon_Heartbeat`"));
        assert_ne!(verdict, "refactor_tool");
    }

    #[test]
    fn upsert_replaces_placeholder() {
        let content = "## Mandato\n\nfoo\n\n## Conclusión Analítica y Propuesta Evolutiva\n\n_Pendiente de síntesis Mayeuta (Kintsugi async)._\n\n## Criterio\n\nbar\n";
        let section = "## Conclusión Analítica y Propuesta Evolutiva\n\n*(Síntesis Mayeuta — Kintsugi async)*\n";
        let out = upsert_fracture_kaizen_section(content, section);
        assert!(out.contains("*(Síntesis Mayeuta"));
        assert!(!out.contains("Pendiente de síntesis"));
        assert!(out.contains("## Mandato"));
        assert!(out.contains("## Criterio"));
    }

    #[test]
    fn enrich_fracture_pbi_kaizen_e2e() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        setup_repo(repo);
        fs::create_dir_all(repo.join(".events/telemetry")).unwrap();

        let inputs = json!({
            "process_name": "event-watcher",
            "error_trace": "colapsó el daemon pre-push hook",
            "agent_emitter": "event-watcher",
            "attempted_action": "delivery-close-cycle",
        });

        materialize_fracture_pbi::run(repo, &inputs).expect("materialize");

        let out = run(repo, &inputs).expect("enrich");
        assert_eq!(out.get("success"), Some(&json!(true)));
        assert_eq!(
            out.get("evolution_verdict"),
            Some(&json!("refactor_tool"))
        );

        let path = out
            .get("target_path")
            .and_then(|v| v.as_str())
            .expect("target_path");
        let content = fs::read_to_string(repo.join(path)).unwrap();
        assert!(content.contains("Síntesis Mayeuta"));
        assert!(!content.contains("Pendiente de síntesis Mayeuta"));
    }

    #[test]
    fn enrich_returns_no_target_without_pbi() {
        let tmp = tempfile::tempdir().expect("tempdir");
        setup_repo(tmp.path());
        let out = run(
            tmp.path(),
            &json!({
                "process_name": "x",
                "error_trace": "e",
                "agent_emitter": "a",
                "attempted_action": "b",
            }),
        )
        .expect("no_target");
        assert_eq!(out.get("reason"), Some(&json!("no_target")));
    }

    #[test]
    fn enrich_finds_deduped_process_pbi_not_hash_path() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        setup_repo(repo);
        fs::create_dir_all(repo.join(".events/telemetry")).unwrap();

        let first = json!({
            "process_name": "event-watcher",
            "error_trace": "Centinela event-watcher omitió 13 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-07-16T15:55:03Z",
            "agent_emitter": "argos",
            "attempted_action": "daemon-heartbeat-audit",
        });
        let second = json!({
            "process_name": "event-watcher",
            "error_trace": "Centinela event-watcher omitió 37 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-07-16T16:08:11Z",
            "agent_emitter": "argos",
            "attempted_action": "daemon-heartbeat-audit",
        });

        materialize_fracture_pbi::run(repo, &first).expect("materialize");
        materialize_fracture_pbi::run(repo, &second).expect("dedup");

        let out = run(repo, &second).expect("enrich deduped");
        assert_eq!(out.get("success"), Some(&json!(true)));
        let path = out.get("target_path").and_then(|v| v.as_str()).unwrap();
        let content = fs::read_to_string(repo.join(path)).unwrap();
        assert!(content.contains("Síntesis Mayeuta"));
    }
}
