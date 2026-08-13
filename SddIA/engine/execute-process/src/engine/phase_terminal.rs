//! Agregación terminal unificada de fases (EV-AUD-005).
//!
//! Consumido por `executor` y `residual_runner` para que `success` / `status_code` /
//! `exit_code` reflejen fallos obligatorios de fase (no solo Argos block).

use serde_json::{json, Value};

const CODE_KEYS: &[&str] = &[
    "cerbero_di_code",
    "cerbero_envelope_di_code",
    "di_gate_code",
    "di_resolve_code",
    "error_code",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailedPhaseRef {
    pub phase_name: String,
    pub code: Option<String>,
    pub error: Option<String>,
    pub handler: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalVerdict {
    pub success: bool,
    pub status_code: i32,
    pub failed_phase: Option<FailedPhaseRef>,
    pub error: Option<String>,
}

fn report_fail_soft(report: &Value) -> bool {
    report
        .get("fail_soft")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

fn report_status(report: &Value) -> &str {
    report.get("status").and_then(|v| v.as_str()).unwrap_or("")
}

fn extract_code(report: &Value) -> Option<String> {
    for key in CODE_KEYS {
        if let Some(s) = report.get(*key).and_then(|v| v.as_str()) {
            let t = s.trim();
            if !t.is_empty() {
                return Some(t.to_string());
            }
        }
    }
    None
}

fn phase_name_of(report: &Value) -> String {
    report
        .get("phase_name")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".into())
}

fn first_causal_failure(phase_reports: &[Value]) -> Option<FailedPhaseRef> {
    for report in phase_reports {
        if report_fail_soft(report) {
            continue;
        }
        let status = report_status(report);
        if status == "failed" || status == "blocked" {
            return Some(FailedPhaseRef {
                phase_name: phase_name_of(report),
                code: extract_code(report),
                error: report
                    .get("error")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
                handler: report
                    .get("handler")
                    .and_then(|v| v.as_str())
                    .map(str::to_string),
            });
        }
    }
    None
}

fn format_error(failed: &FailedPhaseRef) -> String {
    match &failed.code {
        Some(code) => format!(
            "fase \"{}\" failed ({})",
            failed.phase_name, code
        ),
        None => format!("fase \"{}\" failed", failed.phase_name),
    }
}

/// Agrega el estado terminal de una ejecución multi-fase.
///
/// - `failed` / `blocked` (sin `fail_soft: true`) → éxito global falso.
/// - `argos_verdict == "block"` en state → éxito global falso.
/// - `skipped` / `simulated` / `awaiting` / `awaiting_agents` / `executed` → neutrales.
pub fn aggregate_execution_terminal(
    phase_reports: &[Value],
    state: &Value,
) -> TerminalVerdict {
    let argos_block = state.get("argos_verdict") == Some(&json!("block"));
    let failed = first_causal_failure(phase_reports);

    if let Some(ref fp) = failed {
        return TerminalVerdict {
            success: false,
            status_code: 1,
            failed_phase: Some(fp.clone()),
            error: Some(format_error(fp)),
        };
    }

    if argos_block {
        return TerminalVerdict {
            success: false,
            status_code: 1,
            failed_phase: None,
            error: Some("Argos: Ruido de Sistema".into()),
        };
    }

    TerminalVerdict {
        success: true,
        status_code: 0,
        failed_phase: None,
        error: None,
    }
}

/// Inyecta `failed_phase*` en el objeto `data` del envelope cuando hay causal.
pub fn apply_failed_phase_fields(data: &mut Value, verdict: &TerminalVerdict) {
    if let Some(ref fp) = verdict.failed_phase {
        data["failed_phase"] = json!(fp.phase_name);
        if let Some(ref code) = fp.code {
            data["failed_phase_code"] = json!(code);
        }
        if let Some(ref err) = fp.error {
            data["failed_phase_error"] = json!(err);
        }
        if let Some(ref handler) = fp.handler {
            data["failed_phase_handler"] = json!(handler);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn report(phase: &str, status: &str) -> Value {
        json!({ "phase_name": phase, "status": status })
    }

    #[test]
    fn t1_mix_executed_and_failed_is_global_failure() {
        let reports = vec![
            report("A", "executed"),
            json!({
                "phase_name": "Persistencia oficial",
                "status": "failed",
                "handler": "cerbero-di-rbac",
                "error": "config ausente",
                "cerbero_di_code": "CERBERO_CONFIG_ERROR",
            }),
            report("C", "simulated"),
        ];
        let v = aggregate_execution_terminal(&reports, &json!({}));
        assert!(!v.success);
        assert_eq!(v.status_code, 1);
        let fp = v.failed_phase.as_ref().expect("failed_phase");
        assert_eq!(fp.phase_name, "Persistencia oficial");
        assert_eq!(fp.code.as_deref(), Some("CERBERO_CONFIG_ERROR"));
    }

    #[test]
    fn t2_skipped_simulated_awaiting_are_not_failure() {
        let reports = vec![
            report("A", "skipped"),
            report("B", "simulated"),
            report("C", "awaiting_agents"),
            report("D", "awaiting"),
            report("E", "executed"),
        ];
        let v = aggregate_execution_terminal(&reports, &json!({}));
        assert!(v.success);
        assert_eq!(v.status_code, 0);
        assert!(v.failed_phase.is_none());
    }

    #[test]
    fn t2b_argos_block_without_phase_failure() {
        let reports = vec![report("A", "executed")];
        let state = json!({ "argos_verdict": "block" });
        let v = aggregate_execution_terminal(&reports, &state);
        assert!(!v.success);
        assert_eq!(v.status_code, 1);
        assert!(v.failed_phase.is_none());
        assert_eq!(v.error.as_deref(), Some("Argos: Ruido de Sistema"));
    }

    #[test]
    fn t3_cerbero_config_error_code_propagated() {
        let reports = vec![json!({
            "phase_name": "Persistencia oficial",
            "status": "failed",
            "cerbero_di_code": "CERBERO_CONFIG_ERROR",
            "error": "Cerbero DI config error",
        })];
        let v = aggregate_execution_terminal(&reports, &json!({}));
        assert_eq!(
            v.failed_phase.as_ref().and_then(|f| f.code.clone()).as_deref(),
            Some("CERBERO_CONFIG_ERROR")
        );
        assert!(v
            .error
            .as_deref()
            .unwrap_or("")
            .contains("CERBERO_CONFIG_ERROR"));
    }

    #[test]
    fn t4_di_gate_failed_fixture() {
        let reports = vec![json!({
            "phase_name": "Ignición",
            "status": "failed",
            "handler": "capability-di-gate",
            "di_gate_code": "CAPABILITY_NOT_INDEXED",
            "error": "capability missing",
        })];
        let v = aggregate_execution_terminal(&reports, &json!({}));
        assert!(!v.success);
        assert_eq!(
            v.failed_phase.as_ref().and_then(|f| f.code.clone()).as_deref(),
            Some("CAPABILITY_NOT_INDEXED")
        );
        assert_eq!(
            v.failed_phase.as_ref().and_then(|f| f.handler.clone()).as_deref(),
            Some("capability-di-gate")
        );
    }

    #[test]
    fn t5_cerbero_rbac_failed_fixture() {
        let reports = vec![json!({
            "phase_name": "Persistencia oficial",
            "status": "failed",
            "handler": "cerbero-di-rbac",
            "cerbero_di_code": "CERBERO_CONFIG_ERROR",
            "error": "rbac/config",
        })];
        let v = aggregate_execution_terminal(&reports, &json!({}));
        assert!(!v.success);
        assert_eq!(
            v.failed_phase.as_ref().map(|f| f.phase_name.as_str()),
            Some("Persistencia oficial")
        );
    }

    #[test]
    fn t6_capsule_invoke_failed_fixture() {
        let reports = vec![json!({
            "phase_name": "Cápsula",
            "status": "failed",
            "handler": "capsule-invoke",
            "error_code": "CAPSULE_INVOKE_FAILED",
            "error": "exit 1",
        })];
        let v = aggregate_execution_terminal(&reports, &json!({}));
        assert!(!v.success);
        assert_eq!(
            v.failed_phase.as_ref().and_then(|f| f.code.clone()).as_deref(),
            Some("CAPSULE_INVOKE_FAILED")
        );
    }

    #[test]
    fn t7_agent_runtime_failed_fixture() {
        let reports = vec![json!({
            "phase_name": "Diseño",
            "status": "failed",
            "handler": "agent-runtime",
            "error": "agent crashed",
        })];
        let v = aggregate_execution_terminal(&reports, &json!({}));
        assert!(!v.success);
        assert_eq!(
            v.failed_phase.as_ref().map(|f| f.phase_name.as_str()),
            Some("Diseño")
        );
    }

    #[test]
    fn t8_persistencia_capability_failed() {
        let reports = vec![
            report("Análisis", "executed"),
            json!({
                "phase_name": "Persistencia oficial",
                "status": "failed",
                "handler": "cerbero-di-rbac",
                "cerbero_di_code": "CERBERO_CONFIG_ERROR",
                "error": "CERBERO_CONFIG_ERROR",
            }),
        ];
        let v = aggregate_execution_terminal(&reports, &json!({}));
        assert!(!v.success);
        assert_eq!(v.status_code, 1);
    }

    #[test]
    fn t9_regression_62b201cf_persistencia_oficial() {
        // Reconstrucción del incidente: fases previas OK/sim; Persistencia oficial failed.
        let reports = vec![
            report("Inicialización", "executed"),
            report("Análisis", "simulated"),
            report("Diseño", "awaiting_agents"),
            json!({
                "phase_name": "Persistencia oficial",
                "status": "failed",
                "handler": "cerbero-di-rbac",
                "error": "Cerbero DI configuration error",
                "cerbero_di_code": "CERBERO_CONFIG_ERROR",
            }),
        ];
        let v = aggregate_execution_terminal(&reports, &json!({}));
        assert!(!v.success, "EV-AUD-005: no certificar éxito con fase failed");
        assert_ne!(v.status_code, 0);
        let fp = v.failed_phase.as_ref().expect("failed_phase");
        assert_eq!(fp.phase_name, "Persistencia oficial");
        assert_eq!(fp.code.as_deref(), Some("CERBERO_CONFIG_ERROR"));

        let mut data = json!({ "process_name": "evolution-audit" });
        apply_failed_phase_fields(&mut data, &v);
        assert_eq!(data["failed_phase"], json!("Persistencia oficial"));
        assert_eq!(data["failed_phase_code"], json!("CERBERO_CONFIG_ERROR"));
        assert!(data.get("failed_phase_error").is_some());
        assert_eq!(data["failed_phase_handler"], json!("cerbero-di-rbac"));
    }

    #[test]
    fn fail_soft_declared_does_not_fail_global() {
        let reports = vec![json!({
            "phase_name": "Toll IO",
            "status": "failed",
            "fail_soft": true,
            "error": "io soft",
        })];
        let v = aggregate_execution_terminal(&reports, &json!({}));
        assert!(v.success);
        assert!(v.failed_phase.is_none());
    }

    #[test]
    fn blocked_status_is_global_failure() {
        let reports = vec![json!({
            "phase_name": "Gate",
            "status": "blocked",
            "error": "policy",
        })];
        let v = aggregate_execution_terminal(&reports, &json!({}));
        assert!(!v.success);
        assert_eq!(
            v.failed_phase.as_ref().map(|f| f.phase_name.as_str()),
            Some("Gate")
        );
    }

    #[test]
    fn code_priority_cerbero_before_di_gate() {
        let reports = vec![json!({
            "phase_name": "X",
            "status": "failed",
            "cerbero_di_code": "CERBERO_CONFIG_ERROR",
            "di_gate_code": "CAPABILITY_NOT_INDEXED",
        })];
        let v = aggregate_execution_terminal(&reports, &json!({}));
        assert_eq!(
            v.failed_phase.as_ref().and_then(|f| f.code.clone()).as_deref(),
            Some("CERBERO_CONFIG_ERROR")
        );
    }
}
