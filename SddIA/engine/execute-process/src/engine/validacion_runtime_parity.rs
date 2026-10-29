//! Gate KZ-CA5: validación exige paridad de binario vivo en ítems WUI / kalma2-bridge.

use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

pub fn persist_requires_runtime_parity_smoke(repo: &Path, persist_ref: &str) -> bool {
    let dir = if Path::new(persist_ref).is_absolute() {
        PathBuf::from(persist_ref)
    } else {
        repo.join(persist_ref)
    };
    for name in ["objectives.md", "spec.md", "plan.md", "implementation.md"] {
        let path = dir.join(name);
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let t = text.to_ascii_lowercase();
        if t.contains("kalma2-bridge")
            || t.contains("workspace_init")
            || t.contains("surface: kalma2-wui")
            || t.contains("kalma2-wui")
            || t.contains("/api/system-health")
        {
            return true;
        }
    }
    false
}

/// Misma lógica que `/api/system-health` → `runtime_binary_parity` (sin HTTP).
pub fn check_runtime_binary_parity_gate(repo: &Path) -> Result<Value, String> {
    if sddia_runtime_parity::allow_stale_runtime() {
        return Ok(json!({
            "ok": true,
            "skipped": true,
            "reason": "SDDIA_ALLOW_STALE_RUNTIME",
        }));
    }
    let orchestrator = sddia_daemon_runtime::resolve_execute_process_elf(repo);
    sddia_runtime_parity::assess_runtime_binary_parity(repo, None, orchestrator.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn detects_kalma2_bridge_in_persist_docs() {
        let td = tempfile::tempdir().unwrap();
        let persist = td.path().join("docs/features/x");
        fs::create_dir_all(&persist).unwrap();
        fs::write(
            persist.join("spec.md"),
            "# spec\n\nToca `kalma2-bridge`.\n",
        )
        .unwrap();
        let rel = persist
            .strip_prefix(td.path())
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        assert!(persist_requires_runtime_parity_smoke(td.path(), &rel));
    }
}
