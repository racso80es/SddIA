//! Persistencia best-effort de `execution_report.json` en el workspace de proceso.

use serde_json::{json, Value};
use std::fs;
use std::path::Path;

pub fn inject_elapsed_ms(entry: Value, elapsed_ms: i64) -> Value {
    let mut out = entry;
    if let Some(obj) = out.as_object_mut() {
        obj.insert("elapsed_ms".into(), json!(elapsed_ms));
    }
    out
}

pub fn try_persist_execution_report(
    process_name: &str,
    exempt: bool,
    workspace_path: Option<&str>,
    execution_id: Option<&str>,
    correlation_id: Option<&str>,
    status_code: i32,
    duration_ms: i64,
    phase_reports: &[Value],
) {
    if exempt {
        return;
    }
    let ws = workspace_path
        .map(str::trim)
        .filter(|s| !s.is_empty());
    let ws = match ws {
        Some(w) => w,
        None => return,
    };
    let report = json!({
        "process_name": process_name,
        "execution_id": execution_id,
        "correlation_id": correlation_id,
        "status_code": status_code,
        "duration_ms": duration_ms,
        "phases": phase_reports,
    });
    let dir = Path::new(ws);
    if fs::create_dir_all(dir).is_err() {
        eprintln!("execution_report.json: no se pudo crear workspace {ws}");
        return;
    }
    let path = dir.join("execution_report.json");
    match serde_json::to_vec_pretty(&report) {
        Ok(body) => {
            if let Err(e) = fs::write(&path, body) {
                eprintln!("execution_report.json: {e}");
            }
        }
        Err(e) => eprintln!("execution_report.json: serialización: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use uuid::Uuid;

    #[test]
    fn inject_elapsed_ms_sets_field() {
        let entry = inject_elapsed_ms(json!({"status": "executed"}), 42);
        assert_eq!(entry["elapsed_ms"], 42);
    }

    #[test]
    fn persist_writes_execution_report_with_phases() {
        let dir = std::env::temp_dir().join(format!("sddia-er-{}", Uuid::new_v4()));
        let ws = dir.to_str().unwrap();
        let phases = vec![json!({"phase_name": "Triaje", "status": "executed", "elapsed_ms": 7})];
        try_persist_execution_report(
            "pull-request-review",
            false,
            Some(ws),
            Some("exec-uuid"),
            Some("corr-uuid"),
            0,
            99,
            &phases,
        );
        let raw = fs::read_to_string(dir.join("execution_report.json")).expect("read");
        assert!(raw.contains("pull-request-review"));
        assert!(raw.contains("elapsed_ms"));
        assert!(raw.contains("exec-uuid"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn exempt_skips_persist() {
        let dir = std::env::temp_dir().join(format!("sddia-er-ex-{}", Uuid::new_v4()));
        let ws = dir.to_str().unwrap();
        fs::create_dir_all(&dir).unwrap();
        try_persist_execution_report("route-domain-event", true, Some(ws), None, None, 0, 0, &[]);
        assert!(!dir.join("execution_report.json").exists());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn phase_timing_roundtrip() {
        let t0 = Instant::now();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let ms = t0.elapsed().as_millis() as i64;
        let entry = inject_elapsed_ms(json!({"phase_name": "x"}), ms);
        assert!(entry["elapsed_ms"].as_i64().unwrap_or(0) >= 1);
    }
}
