//! Handler nativo `enrich-fracture-pbi-kaizen` — síntesis Mayeuta sobre PBI Cúmulo (D-P6T.1).

use crate::core::fracture_pbi::{
    fracture_trace_hash, resolve_enrich_target, scan_fracture_ledger, slugify_process_name,
};
use crate::core::fracture_signatures::{
    build_match_surfaces, extract_evidence_lines, load_fracture_catalog,
    resolve_refined_diagnosis, shared_default_catalog, signature_matches_mayeuta,
    DccMatchContext, Diagnosis, FractureCatalog,
};
use std::collections::HashSet;
use chrono::Utc;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use uuid::Uuid;

use super::fractal::{load_fractal_dirs, write_fractal_event};

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

fn verdict_label(v: &str) -> &str {
    match v {
        "new_norm" => "Nueva norma o endurecimiento normativo",
        "refactor_tool" => "Refactor de herramienta / cápsula / handler lab",
        "prompt_adjustment" => "Ajuste de prompt o regla operador IA",
        "process_fix" => "Corrección de proceso oficial",
        other => other,
    }
}

fn build_kaizen_section(
    classification: &str,
    primary_id: Option<&str>,
    evidence: &[String],
    secondary: &[String],
    diagnosis: &Diagnosis,
    verdict: &str,
    extra_proposals: &[(String, String)],
) -> String {
    let root_md = format!("- {}", diagnosis.root_cause);
    let mut meta = format!(
        "- **Clasificación:** `{classification}`\n",
        classification = classification
    );
    if let Some(id) = primary_id {
        meta.push_str(&format!("- **Firma:** `{id}`\n"));
    }
    if !evidence.is_empty() {
        let ev = evidence.join(" / ");
        meta.push_str(&format!("- **Evidencia:** `{ev}`\n"));
    }
    if !secondary.is_empty() {
        meta.push_str(&format!(
            "- **Señales secundarias:** `{}`\n",
            secondary.join("`, `")
        ));
    }
    let proposal_md = std::iter::once(format!(
        "- **{}:** {}",
        verdict_label(verdict),
        diagnosis.proposal
    ))
    .chain(
        extra_proposals
            .iter()
            .map(|(v, p)| format!("- **{}:** {p}", verdict_label(v))),
    )
    .collect::<Vec<_>>()
    .join("\n");

    format!(
        r#"## Conclusión Analítica y Propuesta Evolutiva

*(Síntesis Mayeuta — Kintsugi async)*

{meta}
### Diagnóstico de causa raíz

{root_md}

### Veredicto evolutivo

**{verdict_label}** (`{verdict}`)

### Propuestas

{proposal_md}

> Mayeuta transforma la fractura en deuda accionable; el Vértice Biológico valida antes de ejecutar."#,
        verdict_label = verdict_label(verdict),
    )
}

/// Paridad `execute-action.py::_analyze_fracture_kaizen` → (veredicto, root_md, section, unclassified).
pub fn analyze_fracture_kaizen(
    process_name: &str,
    error_trace: &str,
    attempted_action: &str,
    agent_emitter: &str,
) -> (String, String, String, bool) {
    analyze_fracture_kaizen_with_options(
        process_name,
        error_trace,
        attempted_action,
        agent_emitter,
        None,
        None,
    )
}

pub fn analyze_fracture_kaizen_with_options(
    process_name: &str,
    error_trace: &str,
    attempted_action: &str,
    agent_emitter: &str,
    friction_id: Option<&str>,
    catalog: Option<&FractureCatalog>,
) -> (String, String, String, bool) {
    let catalog = match catalog {
        Some(c) => c,
        None => shared_default_catalog(),
    };
    let surfaces = build_match_surfaces(process_name, error_trace, attempted_action, true);
    let ctx = DccMatchContext {
        process_name,
        attempted_action,
        status: "",
        error_trace,
        report_friction_id: None,
        report_error_code: None,
    };

    if let Some(fid) = friction_id.filter(|id| catalog.catalog_contains_id(id)) {
        let sig = catalog.get(fid).expect("catalog id");
        let diagnosis = sig.diagnosis.clone().unwrap_or_else(|| Diagnosis {
            root_cause: sig.id.clone(),
            verdict: "process_fix".into(),
            proposal: "Revisar catálogo.".into(),
        });
        let evidence = extract_evidence_lines(&surfaces.error_trace_norm, 2);
        let section = build_kaizen_section(
            "structured",
            Some(fid),
            &evidence,
            &[],
            &diagnosis,
            &diagnosis.verdict,
            &[],
        );
        let root_md = format!("- {}", diagnosis.root_cause);
        return (diagnosis.verdict.clone(), root_md, section, false);
    }

    let mut matched: Vec<String> = Vec::new();
    let mut matched_set = HashSet::new();
    for sig in catalog.signatures() {
        if sig.refine_only {
            continue;
        }
        if signature_matches_mayeuta(catalog, sig, &surfaces, &ctx, &matched_set) {
            matched.push(sig.id.clone());
            matched_set.insert(sig.id.clone());
        }
    }

    let unclassified = matched.is_empty();
    if unclassified {
        let root = format!(
            "Causa raíz no clasificada automáticamente para `{process_name}`; requiere laudo humano."
        );
        let diagnosis = Diagnosis {
            root_cause: root,
            verdict: "process_fix".into(),
            proposal: format!(
                "Auditar proceso `{process_name}`, acción `{attempted_action}` y emisor `{agent_emitter}`."
            ),
        };
        let section = build_kaizen_section(
            "unclassified",
            None,
            &[],
            &[],
            &diagnosis,
            "process_fix",
            &[],
        );
        let root_md = format!("- {}", diagnosis.root_cause);
        return ("process_fix".into(), root_md, section, true);
    }

    let primary_id = matched[0].clone();
    let secondary = matched.iter().skip(1).cloned().collect::<Vec<_>>();
    let primary_sig = catalog.get(&primary_id).expect("primary sig");
    let (effective_id, diagnosis) = if !primary_sig.refine.is_empty() {
        resolve_refined_diagnosis(catalog, primary_sig, &surfaces, &ctx)
    } else {
        (
            primary_sig.id.clone(),
            primary_sig.diagnosis.clone().unwrap_or_else(|| Diagnosis {
                root_cause: primary_sig.id.clone(),
                verdict: "process_fix".into(),
                proposal: "Revisar catálogo.".into(),
            }),
        )
    };
    let verdict = diagnosis.verdict.clone();
    let evidence = extract_evidence_lines(&surfaces.error_trace_norm, 2);
    let section = build_kaizen_section(
        "signature",
        Some(&effective_id),
        &evidence,
        &secondary,
        &diagnosis,
        &verdict,
        &[],
    );
    let root_md = format!("- {}", diagnosis.root_cause);
    (verdict, root_md, section, false)
}


fn iso_now() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

fn emit_clarification_requested(
    repo: &Path,
    target_rel: &str,
    process_name: &str,
    trace_hash: &str,
    attempted_action: &str,
    agent_emitter: &str,
) {
    let orch_dir = load_fractal_dirs(repo).1;
    let event = json!({
        "event_id": Uuid::new_v4().to_string(),
        "event_type": "Fracture_Clarification_Requested",
        "event_family": "orchestration",
        "timestamp": iso_now(),
        "emitter_agent": "enrich-fracture-pbi-kaizen",
        "payload": {
            "fracture_pbi_path": target_rel,
            "process_name": process_name,
            "error_trace_hash": trace_hash,
            "attempted_action": attempted_action,
            "agent_emitter": agent_emitter,
        },
        "delivery_state": {},
    });
    if let Err(e) = write_fractal_event(repo, &event, &orch_dir) {
        eprintln!("[enrich-fracture-pbi-kaizen] fail-soft Fracture_Clarification_Requested: {e}");
    }
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
    let friction_id = optional_str(inputs, "friction_id");
    let (verdict, _, section, unclassified) = analyze_fracture_kaizen_with_options(
        &process_name,
        &error_trace,
        &attempted_action,
        &agent_emitter,
        friction_id.as_deref(),
        None,
    );
    let content = fs::read_to_string(&target).map_err(|e| e.to_string())?;
    fs::write(&target, upsert_fracture_kaizen_section(&content, &section))
        .map_err(|e| e.to_string())?;

    if unclassified {
        emit_clarification_requested(
            repo,
            &target_rel,
            &process_name,
            &trace_hash,
            &attempted_action,
            &agent_emitter,
        );
    }

    Ok(json!({
        "success": true,
        "target_path": target_rel,
        "message": "PBI enriquecido con síntesis Kaizen",
        "evolution_verdict": verdict,
        "unclassified": unclassified,
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
        let (verdict, _, section, _) = analyze_fracture_kaizen(
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
        let (verdict, _, section, _) = analyze_fracture_kaizen(
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
        let (verdict, _, section, _) = analyze_fracture_kaizen(
            "delivery-close-cycle",
            "fatal: Could not resolve host: github.com",
            "Publicación remota",
            "execute-process",
        );
        assert!(!section.contains("Recursión o re-entrada"));
        assert_ne!(verdict, "refactor_tool");
    }

    #[test]
    fn analyze_fracture_kaizen_dlt_publish_error_not_prompt() {
        let (verdict, _, section, _) = analyze_fracture_kaizen(
            "route-domain-event",
            "merkle-batch-preseal failed: iota-relay-publish-error: status=500 fetch failed | cause: ENETUNREACH",
            "merkle-batch-preseal",
            "execute-process",
        );
        assert_eq!(verdict, "process_fix");
        assert!(section.contains("iota-relay-publish-error"));
        assert!(section.contains("Causa de transporte"));
        assert!(!section.contains("Ajustar instrucción operador"));
        assert!(!section.contains("prompt_adjustment"));
    }

    #[test]
    fn analyze_fracture_kaizen_dlt_gas_version_not_transport() {
        let (verdict, _, section, _) = analyze_fracture_kaizen(
            "route-domain-event",
            "merkle-batch-preseal failed: iota-relay-publish-error: status=500 Transaction execution failed due to issues with transaction inputs, please review the errors and try again:\n- Object ID 0x93e4c1ee3a81aa2815b2f23485885a37f6c97eb20d7e58458d8dcc4dce6ff880 Version 850727115 Digest 5fX2xzFeULF5XhHLu3iLNotiKiR5bu8CVwJC9j5X6GBa is not available for consumption, current version: 850727116",
            "merkle-batch-preseal",
            "execute-process",
        );
        assert_eq!(verdict, "process_fix");
        assert!(section.contains("gas/inputs") || section.contains("versión de gas"));
        assert!(!section.contains("err.cause"));
        assert!(!section.contains("Ajustar instrucción operador"));
        assert!(!section.contains("prompt_adjustment"));
        assert!(!section.contains("Causa de transporte"));
    }

    #[test]
    fn analyze_fracture_kaizen_dlt_object_lock_not_opaque() {
        let (verdict, _, section, unclassified) = analyze_fracture_kaizen(
            "route-domain-event",
            "merkle-batch-preseal failed: iota-relay-publish-error: status=500 Failed to sign transaction by a quorum of validators because one or more of its objects is reserved for another transaction. Other transactions locking these objects:\n- 7R6vWf14dsfmfGtQKJXAcjagG5G5phFMYwTKtvUCKfTX (stake 35.14)",
            "merkle-batch-preseal",
            "execute-process",
        );
        assert_eq!(verdict, "process_fix");
        assert!(!unclassified);
        assert!(section.contains("reservado"));
        assert!(section.contains("dlt_reanchor"));
        assert!(!section.contains("Ajustar instrucción operador"));
        assert!(!section.contains("prompt_adjustment"));
        assert!(!section.contains("Causa de transporte"));
        assert!(!section.contains("inputs permanentes"));
    }

    #[test]
    fn analyze_fracture_kaizen_prosthetic_exit3_not_unclassified() {
        let (verdict, _, section, unclassified) = analyze_fracture_kaizen(
            "kalma2-bridge",
            "mayeuta-llm/prótesis exit 3",
            "sse_chat_stream",
            "kalma2-bridge",
        );
        assert_eq!(verdict, "process_fix");
        assert!(!unclassified);
        assert!(section.contains("SDDIA_LLM_REQUIRE_INFER"));
        assert!(!section.contains("requiere laudo humano"));
        assert!(!section.contains("prompt_adjustment"));
        assert!(!section.contains("Ajustar instrucción operador"));
    }

    #[test]
    fn analyze_fracture_kaizen_generic_failed_not_prompt() {
        let (verdict, _, section, unclassified) = analyze_fracture_kaizen(
            "route-domain-event",
            "merkle-batch-preseal failed: unexplained-capsule-error",
            "merkle-batch-preseal",
            "execute-process",
        );
        assert_eq!(verdict, "process_fix");
        assert!(unclassified);
        assert!(section.contains("requiere laudo humano"));
        assert!(!section.contains("Ajustar instrucción operador"));
        assert!(!section.contains("prompt_adjustment"));
    }

    #[test]
    fn analyze_fracture_kaizen_workflow_scope_not_hook() {
        let (verdict, _, section, _) = analyze_fracture_kaizen(
            "delivery-close-cycle",
            "Publicación remota failed:\nrefusing to allow a Personal Access Token to create or update workflow\n`.github/workflows/sddia-index-qa.yml` without `workflow` scope",
            "Publicación remota",
            "execute-process",
        );
        assert!(!section.contains("Recursión o re-entrada"));
        assert!(!section.contains("Implementar guarda `SDDIA_HOOK_DELIVERY_CLOSE`"));
        assert!(section.contains("F-DCC-WORKFLOW-SCOPE"));
        assert!(section.contains("credential helper"));
        assert_eq!(verdict, "process_fix");
    }

    #[test]
    fn analyze_fracture_kaizen_head_sha_blank_not_hook() {
        let (verdict, _, section, _) = analyze_fracture_kaizen(
            "delivery-close-cycle",
            "no se pudo resolver pr_url desde gh; gh_stdout=; gh_stderr=pull request create failed: GraphQL: Head sha can't be blank, Base sha can't be blank, No commits between main and feat/lancedb-real-vector-memory, Head ref must be a branch (createPullRequest)",
            "Apertura en forja",
            "execute-process",
        );
        assert!(!section.contains("Recursión o re-entrada"));
        assert!(!section.contains("Implementar guarda `SDDIA_HOOK_DELIVERY_CLOSE`"));
        assert!(section.contains("Head sha can't be blank") || section.contains("Rama ausente"));
        assert_eq!(verdict, "process_fix");
    }

    #[test]
    fn analyze_fracture_kaizen_snapshot_gitignore_not_head_sha() {
        let (verdict, _, section, _) = analyze_fracture_kaizen(
            "delivery-close-cycle",
            "[SNAPSHOT_DIRTY_SKIPPED] git add failed: Las siguientes rutas son ignoradas por uno de tus archivos .gitignore:\nSddIA/scripts/starter-kit/.SddIA/.dev",
            "Snapshot final",
            "execute-process",
        );
        assert_eq!(verdict, "process_fix");
        assert!(section.contains("F-DCC-SNAPSHOT-GITIGNORE") || section.contains(".gitignore"));
        assert!(!section.contains("Rama ausente"));
        assert!(!section.contains("Recursión o re-entrada"));
        assert!(!section.contains("escalado Kintsugi previo"));
    }

    #[test]
    fn analyze_fracture_kaizen_shell_executor_wasm_fallback_not_head_sha() {
        let (verdict, _, section, _) = analyze_fracture_kaizen(
            "delivery-close-cycle",
            "shell-executor wasm fallback marker",
            "Apertura en forja",
            "execute-process",
        );
        assert_eq!(verdict, "refactor_tool");
        assert!(section.contains("WASI") || section.contains("fallback nativo"));
        assert!(!section.contains("Head sha can't be blank"));
        assert!(!section.contains("Halt de Apertura"));
        assert!(!section.contains("Rama ausente"));
        assert!(!section.contains("no clasificada"));
        assert!(!section.contains("Auditar proceso"));
    }

    #[test]
    fn analyze_fracture_kaizen_pr_title_metachar_not_hook() {
        let (verdict, _, section, _) = analyze_fracture_kaizen(
            "delivery-close-cycle",
            "[PR_BODY_METACHAR] arguments[3] contains forbidden shell metacharacters",
            "Apertura en forja",
            "execute-process",
        );
        assert_eq!(verdict, "process_fix");
        assert!(section.contains("argv") || section.contains("pr_title"));
        assert!(section.contains("F-DCC-PR-TITLE-METACHAR") || section.contains("PR_TITLE_METACHAR"));
        assert!(!section.contains("Recursión o re-entrada"));
        assert!(!section.contains("Auditar proceso"));
        assert!(!section.contains("no clasificada"));
        assert!(!section.contains("Implementar guarda `SDDIA_HOOK_DELIVERY_CLOSE`"));
    }

    #[test]
    fn analyze_fracture_kaizen_bypass_new_norm() {
        let (verdict, _, section, unclassified) = analyze_fracture_kaizen(
            "feature",
            "operator used gh pr create",
            "delivery-close-cycle",
            "tekton",
        );
        assert_eq!(verdict, "process_fix");
        assert!(unclassified);
        assert!(!section.contains("Violación de jurisdicción"));
        assert!(!section.contains("new_norm"));
    }

    #[test]
    fn analyze_fracture_kaizen_heartbeat_starvation() {
        let (verdict, _, section, _) = analyze_fracture_kaizen(
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
        let (verdict, _, section, _) = analyze_fracture_kaizen(
            "email-watcher",
            "timeout in worker",
            "daemon-heartbeat-audit",
            "argos",
        );
        assert!(!section.contains("Inanición de `Daemon_Heartbeat`"));
        assert!(!section.contains("Lock de centinela"));
        assert_ne!(verdict, "refactor_tool");
    }

    #[test]
    fn analyze_fracture_kaizen_orphan_lock_not_eda() {
        let (verdict, _, section, _) = analyze_fracture_kaizen(
            "email-watcher",
            "Centinela email-watcher lock huérfano: PID 13215 muerto. last_heartbeat=2026-09-06T06:12:04Z",
            "daemon-heartbeat-audit",
            "argos",
        );
        assert_eq!(verdict, "refactor_tool");
        assert!(section.contains("Lock de centinela") || section.contains("ciclo de vida"));
        assert!(!section.contains("Domain_Entity_Created"));
        assert!(!section.contains("audit-entity-eda-coverage --emit"));
        assert!(!section.contains("no clasificada"));
        assert!(!section.contains("Auditar proceso"));
    }

    #[test]
    fn analyze_fracture_kaizen_genomic_orphan_still_eda() {
        let (verdict, _, section, _) = analyze_fracture_kaizen(
            "entity-manager",
            "Ruido de Sistema: orphan_count=2 sin Domain_Entity_Created; ejecutar audit-entity-eda-coverage",
            "entity-manager",
            "cerbero",
        );
        assert_eq!(verdict, "refactor_tool");
        assert!(section.contains("Domain_Entity_Created"));
        assert!(!section.contains("Lock de centinela"));
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
        assert_eq!(out.get("evolution_verdict"), Some(&json!("process_fix")));
        assert_eq!(out.get("unclassified"), Some(&json!(true)));

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

    fn list_orch_events(repo: &Path) -> Vec<String> {
        let dir = repo.join(".events/orchestration");
        if !dir.is_dir() {
            return Vec::new();
        }
        fs::read_dir(dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("json"))
            .map(|p| fs::read_to_string(p).unwrap())
            .collect()
    }

    #[test]
    fn enrich_unclassified_emits_fracture_clarification_requested() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        setup_repo(repo);
        fs::create_dir_all(repo.join(".events/telemetry")).unwrap();

        let inputs = json!({
            "process_name": "route-domain-event",
            "error_trace": "test_trace_inedita_xyz unexplained-capsule-error",
            "agent_emitter": "execute-process",
            "attempted_action": "merkle-batch-preseal",
        });
        materialize_fracture_pbi::run(repo, &inputs).expect("materialize");
        let out = run(repo, &inputs).expect("enrich");
        assert_eq!(out.get("unclassified"), Some(&json!(true)));
        let files = list_orch_events(repo);
        assert_eq!(files.len(), 1);
        let ev: Value = serde_json::from_str(&files[0]).unwrap();
        assert_eq!(
            ev.get("event_type").and_then(|v| v.as_str()),
            Some("Fracture_Clarification_Requested")
        );
        let payload = ev.get("payload").and_then(|v| v.as_object()).unwrap();
        assert!(payload.contains_key("fracture_pbi_path"));
        assert!(payload.contains_key("process_name"));
        assert!(payload.contains_key("error_trace_hash"));
        assert_eq!(
            payload.get("error_trace_hash").and_then(|v| v.as_str()).unwrap().len(),
            12
        );
        let (ok, errors) = crate::engine::ecst_validation::validate_ecst_instance(
            &ev,
            Some(&crate::engine::ecst_validation::EventClassSchema {
                required: vec![
                    "fracture_pbi_path".into(),
                    "process_name".into(),
                    "error_trace_hash".into(),
                ],
                optional: vec![
                    "attempted_action".into(),
                    "agent_emitter".into(),
                    "correlation_id".into(),
                ],
                forbidden: vec![],
            }),
        );
        assert!(ok, "{errors:?}");
    }

    #[test]
    #[test]
    fn fracture_corpus_regression() {
        let catalog = shared_default_catalog();
        let cases: &[(&str, &str, &str, &str, &str, Option<&str>)] = &[
            (
                "4c01d65b972f",
                "delivery-close-cycle",
                "Publicación remota",
                "execute-process",
                "SddIA pre-push: SKIPPED (delivery-close-cycle guard)\n ! [rejected]        branch -> branch (non-fast-forward)\nerror: falló el empuje",
                Some("F-DCC-PUSH-NON-FAST-FORWARD"),
            ),
            (
                "d0cfd5b66ff1",
                "delivery-close-cycle",
                "Publicación remota",
                "execute-process",
                "fatal: Could not resolve host: github.com",
                Some("F-DCC-DNS-UNRESOLVED"),
            ),
            (
                "0c5268362b9a",
                "delivery-close-cycle",
                "Publicación remota",
                "execute-process",
                "SddIA pre-push: BLOCKED — evolution gate (--range --if-touched) failed",
                Some("F-DCC-HOOK-EVOL-OVERESCALATION"),
            ),
            (
                "01c9040df256",
                "delivery-close-cycle",
                "Apertura en forja",
                "execute-process",
                "gh_stderr=pull request create failed: GraphQL: Head sha can't be blank",
                Some("F-DCC-REMOTE-BRANCH-ABSENT"),
            ),
            (
                "6c0db1296181",
                "email-watcher",
                "daemon-heartbeat-audit",
                "argos",
                "Centinela email-watcher omitió 3 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-08-30T07:51:47Z",
                Some("F-ARGOS-HEARTBEAT-STARVATION"),
            ),
            (
                "63c439de23d0",
                "telegram-watcher",
                "daemon-heartbeat-audit",
                "argos",
                "Centinela telegram-watcher omitió 11 ciclos consecutivos de Daemon_Heartbeat (umbral=3). last_heartbeat=2026-08-11T07:45:50Z",
                Some("F-ARGOS-HEARTBEAT-STARVATION"),
            ),
            (
                "60db1db67e49",
                "route-domain-event",
                "merkle-batch-preseal",
                "execute-process",
                "merkle-batch-preseal failed: iota-relay-publish-error: is not available for consumption, current version: 1",
                Some("F-DLT-GAS-VERSION"),
            ),
            (
                "ca3d901fdc9a-ola1",
                "delivery-close-cycle",
                "Snapshot final",
                "execute-process",
                "cápsula skill 'git-manager' no encontrada bajo SddIA/target",
                Some("F-DCC-LAB-BINARY-MISSING"),
            ),
        ];
        for (label, process, action, emitter, trace, expected_sig) in cases {
            let (verdict, _, section, unclassified) = analyze_fracture_kaizen_with_options(
                process,
                trace,
                action,
                emitter,
                None,
                Some(catalog),
            );
            assert!(!unclassified, "{label}: unclassified");
            assert!(
                section.contains(&format!("**Firma:** `{}`", expected_sig.unwrap())),
                "{label}: section missing firma"
            );
            assert!(!section.contains("sin escalado Kintsugi"), "{label}");
            if *label == "4c01d65b972f" {
                assert_eq!(verdict, "process_fix");
                assert!(!section.contains("Violación de jurisdicción"));
            }
            if *label == "6c0db1296181" || *label == "63c439de23d0" {
                assert_eq!(verdict, "refactor_tool");
            }
        }
    }

    #[test]
    fn enrich_colaps_unclassified_emits_clarification() {
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
        assert_eq!(out.get("unclassified"), Some(&json!(true)));
        assert_eq!(list_orch_events(repo).len(), 1);
    }
}
