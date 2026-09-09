//! Handler nativo `enrich-fracture-pbi-kaizen` — síntesis Mayeuta sobre PBI Cúmulo (D-P6T.1).

use crate::core::fracture_pbi::{
    fracture_trace_hash, resolve_enrich_target, scan_fracture_ledger, slugify_process_name,
};
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

/// Traza canónica Argos `emit_orphan_lock_fracture`. Match **solo** `error_trace`
/// (F-MAYEUTA-ORPHAN-TOKEN-TRAP: no usar `orphan`/`huérfan` sobre el blob concatenado).
fn is_orphan_lock_trace(error_trace: &str) -> bool {
    error_trace.contains("Centinela ")
        && error_trace.contains("lock huérfano")
        && error_trace.contains("PID ")
        && error_trace.contains("muerto")
        && error_trace.contains("last_heartbeat=")
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

fn is_workflow_scope_trace(error_trace: &str) -> bool {
    let t = error_trace.to_lowercase();
    t.contains("without") && t.contains("workflow") && t.contains("scope")
}

fn is_remote_branch_absent_trace(error_trace: &str) -> bool {
    let t = error_trace.to_lowercase();
    t.contains("head sha can't be blank") || t.contains("head ref must be a branch")
}

fn is_snapshot_gitignore_trace(error_trace: &str) -> bool {
    let t = error_trace.to_lowercase();
    (t.contains("snapshot_dirty_skipped") || t.contains("git add failed"))
        && (t.contains("gitignore") || t.contains("ignorad") || t.contains("ignored by"))
}

fn is_symbolic_head_branch_trace(error_trace: &str) -> bool {
    let t = error_trace.to_lowercase();
    t.contains("branch_symbolic_head")
        || (t.contains("head ref must be a branch")
            && (t.contains("branch_name=head") || t.contains("--head head") || t.contains("simbólico")))
}

fn is_shell_executor_wasm_fallback_trace(error_trace: &str) -> bool {
    let t = error_trace.to_lowercase();
    t.contains("shell-executor wasm fallback marker")
        || (t.contains("cápsula skill")
            && t.contains("shell-executor")
            && t.contains("no encontrada bajo sddia/target"))
}

fn is_shell_metachar_fracture_trace(error_trace: &str) -> bool {
    error_trace.contains("PR_TITLE_METACHAR")
        || error_trace.contains("PR_BODY_METACHAR")
        || error_trace.contains("SHELL_METACHAR")
        || (error_trace.contains("forbidden shell metacharacters")
            && error_trace.contains("arguments["))
}

/// Publish IOTA con relay vivo (`F-DLT-PUBLISH-ERROR`). Match **solo** `error_trace`
/// (el blob concatenado incluye `failed` de `merkle-batch-preseal failed` y dispara el catch-all).
fn is_dlt_publish_error_trace(error_trace: &str) -> bool {
    let t = error_trace.to_lowercase();
    t.contains("iota-relay-publish-error") || t.contains("f-dlt-publish-error")
}

fn is_dlt_gas_version_trace(error_trace: &str) -> bool {
    let t = error_trace.to_lowercase();
    t.contains("is not available for consumption") && t.contains("current version:")
}

fn is_dlt_transport_trace(error_trace: &str) -> bool {
    let t = error_trace.to_lowercase();
    t.contains("enetunreach")
        || t.contains("etimedout")
        || t.contains("enotfound")
        || t.contains("network is unreachable")
        || t.contains("connection timed out")
}

/// Paridad `execute-action.py::_analyze_fracture_kaizen` → (veredicto, root_md, section).
pub fn analyze_fracture_kaizen(
    process_name: &str,
    error_trace: &str,
    attempted_action: &str,
    agent_emitter: &str,
) -> (String, String, String, bool) {
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

    if is_orphan_lock_trace(error_trace) {
        root_causes.push(
            "Lock de centinela con PID muerto (ciclo de vida de daemon / sesión de host); \
             no es entidad genómica huérfana de bus EDA. `process_name` es `daemon_id`."
                .into(),
        );
        proposals.push((
            "refactor_tool".into(),
            "Discriminar lock anterior a `btime` del host frente a colapso en caliente; \
             no ejecutar backfill `audit-entity-eda-coverage`."
                .into(),
        ));
    }

    let credential_workflow = is_workflow_scope_trace(error_trace);
    let snapshot_gitignore = is_snapshot_gitignore_trace(error_trace);
    let symbolic_head = is_symbolic_head_branch_trace(error_trace);
    let remote_branch_absent = is_remote_branch_absent_trace(error_trace) && !symbolic_head;
    let shell_wasm_fallback = is_shell_executor_wasm_fallback_trace(error_trace);
    let dlt_publish = is_dlt_publish_error_trace(error_trace);

    if snapshot_gitignore {
        root_causes.push(
            "Snapshot final invocó `git add -A` sobre un path cubierto por `.gitignore` \
             (`**/.dev/*` / starter-kit `.SddIA/.dev`). El commit aborta; no es rama ausente ni recursión hook (`F-DCC-SNAPSHOT-GITIGNORE`)."
                .into(),
        );
        proposals.push((
            "process_fix".into(),
            "Omitir bóvedas `.dev` (salvo `.env.example`) del inventario de snapshot; \
             `git-manager` commit no debe tumbar el lote si `git add` reporta path ignorado."
                .into(),
        ));
    }
    if symbolic_head {
        root_causes.push(
            "`branch_name=HEAD` (ref simbólica) llegó a Apertura en forja (`gh pr create --head HEAD` → `Head ref must be a branch`). No es PAT ni rama no empujada."
                .into(),
        );
        proposals.push((
            "process_fix".into(),
            "Resolver HEAD a la rama actual (`branch_list` `*`) antes del fan-out DCC; abortar si detached. El hook pre-push no debe reenviar `HEAD` literal."
                .into(),
        ));
    }

    if credential_workflow {
        root_causes.push(
            "PAT de `git-manager` (HTTPS) sin scope `workflow` al tocar `.github/workflows/`; \
             distinto del token `gh auth`. Colapso de credencial, no recursión hook (`F-DCC-WORKFLOW-SCOPE`)."
                .into(),
        );
        proposals.push((
            "process_fix".into(),
            "Unificar credential helper git→`gh` (`gh auth setup-git`). \
             `gh auth refresh -s workflow` solo basta si git ya delega en gh. \
             Envelope DCC `blocked` `F-DCC-WORKFLOW-SCOPE`; no reimplementar `SDDIA_HOOK_DELIVERY_CLOSE`."
                .into(),
        ));
    }
    if remote_branch_absent {
        root_causes.push(
            "Rama ausente en origin tras push rechazado (`Head sha can't be blank` / `Head ref must be a branch`). \
             DCC no abortó tras Publicación remota failed; no es recursión hook."
                .into(),
        );
        proposals.push((
            "process_fix".into(),
            "Halt de Apertura/Sello/Higiene si Publicación remota es `failed`/`blocked` (genoma DCC ya lo exige)."
                .into(),
        ));
    }

    if shell_wasm_fallback {
        root_causes.push(
            "`shell-executor` WASM falló (sandbox WASI / working_directory / exec) y el fallback nativo faltó o el centinela interno se fugó. No es git push ni rama ausente."
                .into(),
        );
        proposals.push((
            "refactor_tool".into(),
            "Saneamiento de `invoke_shell_executor`: no re-ejecutar WASM; error canónico si el ELF nativo falta; no emitir el centinela hacia DCC. Aduana `dcc_lab_binary_missing_trace` suprime Kintsugi."
                .into(),
        ));
    }

    if is_shell_metachar_fracture_trace(error_trace) {
        root_causes.push(
            "`delivery-close-cycle` Apertura en forja: token argv rechazado por `shell-executor` \
             (`pr_title` / `branch_name` / `--body-file`). No es recursión hook. \
             No reabrir K2 `--body-file` (`F-DCC-PR-TITLE-METACHAR`)."
                .into(),
        );
        proposals.push((
            "process_fix".into(),
            "Preflight argv y sanear `pr_title`; códigos `PR_TITLE_METACHAR` ≠ `PR_BODY_METACHAR`. \
             No relajar `assert_safe_token`. No reimplementar `SDDIA_HOOK_DELIVERY_CLOSE`."
                .into(),
        ));
    }

    if dlt_publish {
        if is_dlt_gas_version_trace(error_trace) {
            root_causes.push(
                "HTTP 500 de publish IOTA con relay vivo (`iota-relay-publish-error` / `F-DLT-PUBLISH-ERROR`). \
                 Colisión de versión de gas/inputs (`is not available for consumption` / `current version:`), \
                 no operador ni transporte hacia fullnode."
                    .into(),
            );
            proposals.push((
                "process_fix".into(),
                "Serializar `publishImmutableData` en el relay; si la firma de consumo/versión está presente, \
                 no emitir `System_Fracture_Detected`; `dlt_reanchor` absorbe."
                    .into(),
            ));
        } else if is_dlt_transport_trace(error_trace) {
            root_causes.push(
                "HTTP 500 de publish IOTA con relay vivo (`iota-relay-publish-error` / `F-DLT-PUBLISH-ERROR`). \
                 Causa de transporte (`err.cause`) hacia fullnode Testnet, no operador ni relay caído."
                    .into(),
            );
            proposals.push((
                "process_fix".into(),
                "Si `err.cause` es red transitoria (`ENETUNREACH`/`ETIMEDOUT`/`ENOTFOUND`), no emitir \
                 `System_Fracture_Detected`; `dlt_reanchor` absorbe. Opaco (config-missing / Move) sí fractura."
                    .into(),
            ));
        } else {
            root_causes.push(
                "HTTP 500 de publish IOTA con relay vivo (`iota-relay-publish-error` / `F-DLT-PUBLISH-ERROR`). \
                 Causa opaca (ni red transitoria ni colisión de versión de gas). No es operador."
                    .into(),
            );
            proposals.push((
                "process_fix".into(),
                "Opaco (`config-missing` / Move / inputs permanentes) sí fractura; no afirmar transporte."
                    .into(),
            ));
        }
    }

    if !credential_workflow
        && !remote_branch_absent
        && !shell_wasm_fallback
        && !snapshot_gitignore
        && !symbolic_head
        && has_any_in(
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
    let genomic_ctx = has_any(&[
        "eda genómica",
        "domain_entity_created",
        "audit-entity-eda-coverage",
        "entity-manager",
        "ruido de sistema",
        "orphan_count",
    ]);
    if !is_orphan_lock_trace(error_trace)
        && genomic_ctx
        && has_any(&["orphan", "ruido de sistema", "eda genómica", "huérfan", "orphan_count"])
    {
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
    if !credential_workflow
        && !remote_branch_absent
        && !shell_wasm_fallback
        && !snapshot_gitignore
        && !symbolic_head
        && !dlt_publish
        && has_any(&["timeout", "block", "abort", "colaps"])
    {
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

    let unclassified = root_causes.is_empty();
    if unclassified {
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

    (verdict, root_md, section, unclassified)
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
    let (verdict, _, section, unclassified) = analyze_fracture_kaizen(
        &process_name,
        &error_trace,
        &attempted_action,
        &agent_emitter,
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
        let (verdict, _, _, _) = analyze_fracture_kaizen(
            "feature",
            "operator used gh pr create",
            "delivery-close-cycle",
            "tekton",
        );
        assert_eq!(verdict, "new_norm");
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
        // Fixture: "colapsó" → catch-all Kintsugi (`prompt_adjustment`), no cubo hook/orphan/EDA.
        assert_eq!(
            out.get("evolution_verdict"),
            Some(&json!("prompt_adjustment"))
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
    fn enrich_classified_colaps_does_not_emit_clarification() {
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
        assert_eq!(out.get("unclassified"), Some(&json!(false)));
        assert!(list_orch_events(repo).is_empty());
    }
}
