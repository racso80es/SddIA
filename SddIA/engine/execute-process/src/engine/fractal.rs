//! Emisión de eventos fractales en `./.events/*`.

use serde_json::{json, Value};
use std::fs;
use std::path::Path;

pub fn load_fractal_dirs(repo: &Path) -> (String, String, String) {
    let defaults = (
        "./.events/telemetry".to_string(),
        "./.events/orchestration".to_string(),
        "./.events/domain".to_string(),
    );
    let cfg_path = repo.join("SddIA/core/cumulo.paths.json");
    let Ok(text) = fs::read_to_string(&cfg_path) else {
        return defaults;
    };
    let Ok(cfg) = serde_json::from_str::<Value>(&text) else {
        return defaults;
    };
    let fractal = cfg.get("eda_fractal");
    let tele = fractal
        .and_then(|f| f.get("telemetry"))
        .and_then(|v| v.as_str())
        .unwrap_or(&defaults.0);
    let orch = fractal
        .and_then(|f| f.get("orchestration"))
        .and_then(|v| v.as_str())
        .unwrap_or(&defaults.1);
    let dom = fractal
        .and_then(|f| f.get("domain"))
        .and_then(|v| v.as_str())
        .unwrap_or(&defaults.2);
    (
        tele.trim().replace('\\', "/"),
        orch.trim().replace('\\', "/"),
        dom.trim().replace('\\', "/"),
    )
}

pub fn write_fractal_event(repo: &Path, event: &Value, family_dir: &str) -> Result<Value, String> {
    let event_id = event
        .get("event_id")
        .and_then(|v| v.as_str())
        .ok_or("event_id required")?;
    let target = repo.join(family_dir).join(format!("{event_id}.json"));
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let text = serde_json::to_string_pretty(event).map_err(|e| e.to_string())?;
    fs::write(&target, text).map_err(|e| e.to_string())?;
    Ok(json!({
        "event_id": event_id,
        "target_path": target
            .strip_prefix(repo)
            .unwrap_or(&target)
            .to_string_lossy()
            .replace('\\', "/"),
        "family": family_dir.split('/').last().unwrap_or("telemetry"),
    }))
}
