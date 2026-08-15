//! Emisión best-effort de Progress Trace Capsule (PTC) bajo `eda_fractal.progress`.

use chrono::Utc;
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub fn map_phase_name(raw: &str) -> (&'static str, bool) {
    let lower = raw.to_lowercase();
    if lower.contains("inicialización") || lower.contains("inicializacion") || lower.contains("init") {
        return ("spec", false);
    }
    if raw.starts_with("Clarify") || lower.contains("estabilización de requisitos") || lower.contains("estabilizacion de requisitos") {
        return ("clarify", false);
    }
    if lower.contains("diseño de blueprint") || lower.contains("diseno de blueprint") || lower.contains("diseño del fix") || lower.contains("diseno del fix") {
        return ("plan", false);
    }
    if raw == "Ejecución" || raw == "Ejecucion" {
        return ("implementation", false);
    }
    if raw == "Verificación" || raw == "Verificacion" {
        return ("validation", false);
    }
    if lower.contains("cierre documental")
        || lower.contains("cierre de entrega")
        || lower.contains("finalize")
    {
        return ("closure", false);
    }
    ("implementation", true)
}

pub fn source_agent_from_delegates(delegates: &[Value]) -> String {
    for d in delegates {
        if let Some(s) = d.as_str() {
            if let Some(agent) = s.strip_prefix("agent:") {
                let trimmed = agent.trim();
                if !trimmed.is_empty() {
                    return trimmed.to_string();
                }
            }
        }
    }
    "orchestrator".to_string()
}

fn severity_for_moment(moment: &str, phase_status: Option<&str>) -> &'static str {
    if moment == "end" {
        if let Some(st) = phase_status {
            if matches!(st, "failed" | "blocked") {
                return "error";
            }
            if matches!(st, "awaiting_agents" | "awaiting") {
                return "warn";
            }
        }
    }
    "info"
}

fn build_message(phase_name_raw: &str, moment: &str, phase_status: Option<&str>) -> String {
    match moment {
        "start" => format!("Inicio fase: {phase_name_raw}"),
        "end" => {
            let st = phase_status.unwrap_or("unknown");
            format!("Fin fase: {phase_name_raw} (status={st})")
        }
        _ => format!("Hito fase: {phase_name_raw}"),
    }
}

fn write_ptc_best_effort(repo: &Path, correlation_id: &str, envelope: &Value) {
    let progress_rel = super::fractal::load_progress_dir(repo);
    let trace_id = envelope
        .get("trace_id")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if trace_id.is_empty() {
        return;
    }
    let target = repo
        .join(progress_rel)
        .join(correlation_id)
        .join(format!("{trace_id}.json"));
    if let Some(parent) = target.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(text) = serde_json::to_string(envelope) {
        let _ = fs::write(&target, format!("{text}\n"));
    }
}

/// Emite PTC fire-and-forget; cualquier error IO se traga (nunca falla el orquestador).
pub fn emit_progress_trace(
    repo: &Path,
    correlation_id: &str,
    phase_name_raw: &str,
    delegates: &[Value],
    moment: &str,
    phase_status: Option<&str>,
    process_name: Option<&str>,
) {
    let cid = correlation_id.trim();
    if cid.is_empty() {
        return;
    }

    let (phase_ui, unknown) = map_phase_name(phase_name_raw);
    let message = build_message(phase_name_raw, moment, phase_status);
    if message.trim().is_empty() {
        return;
    }

    let mut metadata = serde_json::Map::new();
    if unknown {
        metadata.insert("phase_name_raw".into(), json!(phase_name_raw));
    }
    if let Some(pn) = process_name.filter(|s| !s.is_empty()) {
        metadata.insert("process_name".into(), json!(pn));
    }
    if let Some(st) = phase_status.filter(|s| !s.is_empty()) {
        metadata.insert("phase_status".into(), json!(st));
    }
    metadata.insert("moment".into(), json!(moment));

    let envelope = json!({
        "trace_id": Uuid::new_v4().to_string(),
        "correlation_id": cid,
        "timestamp": Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        "phase": phase_ui,
        "severity": severity_for_moment(moment, phase_status),
        "source_agent": source_agent_from_delegates(delegates),
        "message": message,
        "metadata": Value::Object(metadata),
    });

    write_ptc_best_effort(repo, cid, &envelope);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use uuid::Uuid;

    #[test]
    fn map_phase_name_covers_spec_and_closure() {
        assert_eq!(map_phase_name("Inicialización Git").0, "spec");
        assert_eq!(map_phase_name("Cierre documental en rama").0, "closure");
        assert_eq!(map_phase_name("Verificación").0, "validation");
    }

    #[test]
    fn unknown_phase_defaults_implementation_with_raw_metadata() {
        let (phase, raw) = map_phase_name("Fase custom");
        assert_eq!(phase, "implementation");
        assert!(raw);
    }

    #[test]
    fn emit_skipped_without_correlation_id() {
        let base = std::env::temp_dir().join(format!("ptc-skip-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&base).unwrap();
        emit_progress_trace(
            &base,
            "",
            "Ejecución",
            &[json!("agent:tekton")],
            "start",
            None,
            Some("feature"),
        );
        assert!(!base.join(".events/progress").exists());
        let _ = std::fs::remove_dir_all(base);
    }

    #[test]
    fn emit_writes_under_progress_leaf_not_telemetry() {
        let base = std::env::temp_dir().join(format!("ptc-write-{}", Uuid::new_v4()));
        let repo = base.join("repo");
        std::fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        std::fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"eda_fractal":{"telemetry":"./.events/telemetry","progress":"./.events/progress"}}"#,
        )
        .unwrap();
        let cid = "22222222-2222-4222-8222-222222222222";
        emit_progress_trace(
            &repo,
            cid,
            "Ejecución",
            &[json!("agent:tekton")],
            "start",
            None,
            Some("feature"),
        );
        let progress_dir = repo.join(".events/progress").join(cid);
        assert!(progress_dir.is_dir());
        assert!(!repo.join(".events/telemetry").join(cid).exists());
        let count = std::fs::read_dir(&progress_dir).unwrap().count();
        assert_eq!(count, 1);
        let _ = std::fs::remove_dir_all(base);
    }

    #[test]
    fn io_failure_does_not_panic() {
        let base = std::env::temp_dir().join(format!("ptc-io-{}", Uuid::new_v4()));
        let repo = base.join("repo");
        std::fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        let cfg = repo.join("SddIA/core/cumulo.paths.json");
        std::fs::write(
            &cfg,
            r#"{"eda_fractal":{"progress":"./.events/progress"}}"#,
        )
        .unwrap();
        // Bloquear escritura: progress es un archivo, no directorio.
        let blocked = repo.join(".events/progress");
        std::fs::create_dir_all(blocked.parent().unwrap()).unwrap();
        let mut f = std::fs::File::create(&blocked).unwrap();
        f.write_all(b"not-a-dir").unwrap();
        emit_progress_trace(
            &repo,
            "33333333-3333-4333-8333-333333333333",
            "Ejecución",
            &[],
            "end",
            Some("failed"),
            None,
        );
        let _ = std::fs::remove_dir_all(base);
    }
}
