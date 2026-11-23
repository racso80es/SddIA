//! Gate estático AUT-116: prohíbe nuevas lecturas de `SDDIA_LAB_SKIP_*` en código productivo
//! fuera de `#[cfg(test)]` y fuera de la lista decreciente de módulos pendientes de migración DI.

use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

/// Módulos legacy pendientes de migración progresiva (lista decreciente; prohibido añadir nuevos).
const LEGACY_SKIP_PENDING_DI: &[&str] = &[
    "SddIA/engine/execute-process/src/engine/accept_pr.rs",
    "SddIA/engine/execute-process/src/engine/agent_runtime.rs",
    "SddIA/engine/execute-process/src/engine/apto_coherence.rs",
    "SddIA/engine/execute-process/src/engine/capability_di_gate.rs",
    "SddIA/engine/execute-process/src/engine/capability_di_output_validator.rs",
    "SddIA/engine/execute-process/src/engine/capability_di_resolver.rs",
    "SddIA/engine/execute-process/src/engine/cerbero_di_envelope.rs",
    "SddIA/engine/execute-process/src/engine/cerbero_di_rbac.rs",
    "SddIA/engine/execute-process/src/engine/delivery_close.rs",
    "SddIA/engine/execute-process/src/engine/diff_seal.rs",
    "SddIA/engine/execute-process/src/engine/executor.rs",
    "SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs",
    "SddIA/engine/execute-process/src/engine/handlers/tracker_stamp.rs",
    "SddIA/engine/execute-process/src/engine/parental_close_gate.rs",
    "SddIA/engine/execute-process/src/engine/phase_capsules.rs",
    "SddIA/engine/execute-process/src/engine/pull_request_review.rs",
    "SddIA/engine/execute-process/src/engine/qa_attestation.rs",
    "SddIA/engine/execute-process/src/engine/runtime_recycle.rs",
    "SddIA/engine/execute-process/src/engine/thermodynamic.rs",
    "SddIA/engine/execute-process/src/engine/tracker_outbound.rs",
    "SddIA/engine/execute-process/src/engine/workspace_init.rs",
    "SddIA/tools/github-raw-fetcher/src/main.rs",
    "SddIA/tools/sddia-qa/src/iota_ci_smoke.rs",
    "SddIA/tools/send-telegram-notification/src/main.rs",
];

#[derive(Debug, Serialize)]
pub struct Violation {
    pub path: String,
    pub line: u32,
    pub snippet: String,
}

#[derive(Debug, Serialize)]
pub struct Report {
    pub ok: bool,
    pub scanned_files: u64,
    pub violation_count: u64,
    pub violations: Vec<Violation>,
}

fn collect_rs_files(dir: &Path, acc: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else {
        return;
    };
    for entry in rd.flatten() {
        let p = entry.path();
        if p.is_dir() {
            let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if name == "target" || name.starts_with('.') {
                continue;
            }
            collect_rs_files(&p, acc);
        } else if p.extension().and_then(|e| e.to_str()) == Some("rs") {
            acc.push(p);
        }
    }
}

pub fn scan_file(repo: &Path, path: &Path) -> Vec<Violation> {
    let rel = path
        .strip_prefix(repo)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");
    if rel.ends_with("gate_no_env_bypass.rs") || LEGACY_SKIP_PENDING_DI.contains(&rel.as_str()) {
        return Vec::new();
    }
    let Ok(text) = fs::read_to_string(path) else {
        return Vec::new();
    };
    let mut violations = Vec::new();
    let mut in_cfg_test = false;
    let mut depth: i32 = 0;
    let mut saw_brace = false;
    let mut line_start = 0usize;

    for (idx, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if !in_cfg_test && trimmed.starts_with("#[cfg(test)]") {
            in_cfg_test = true;
            depth = 0;
            saw_brace = false;
            line_start = idx;
        }
        if in_cfg_test {
            for ch in line.chars() {
                if ch == '{' {
                    depth += 1;
                    saw_brace = true;
                } else if ch == '}' {
                    depth -= 1;
                }
            }
            if saw_brace && depth <= 0 && idx > line_start {
                in_cfg_test = false;
            }
            continue;
        }
        if line.contains("SDDIA_LAB_SKIP_") {
            violations.push(Violation {
                path: rel.clone(),
                line: idx as u32 + 1,
                snippet: trimmed.chars().take(120).collect(),
            });
        }
    }
    violations
}

pub fn scan_repo(repo: &Path) -> Report {
    let mut files = Vec::new();
    for sub in ["engine", "daemons", "tools", "interfaces", "sddia-io"] {
        collect_rs_files(&repo.join("SddIA").join(sub), &mut files);
    }
    files.sort();
    files.dedup();
    let mut violations = Vec::new();
    for f in &files {
        violations.extend(scan_file(repo, f));
    }
    Report {
        ok: violations.is_empty(),
        scanned_files: files.len() as u64,
        violation_count: violations.len() as u64,
        violations,
    }
}

pub fn run(repo: &Path, json: bool) -> i32 {
    let report = scan_repo(repo);
    if json {
        println!("{}", serde_json::to_string(&report).unwrap_or_default());
    } else if report.ok {
        println!(
            "gate-no-env-bypass: OK ({} ficheros)",
            report.scanned_files
        );
    } else {
        eprintln!(
            "gate-no-env-bypass: KO — {} violaciones en {} ficheros",
            report.violation_count, report.scanned_files
        );
        for v in &report.violations {
            eprintln!("  {}:{} {}", v.path, v.line, v.snippet);
        }
    }
    if report.ok {
        0
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_sddia_lab_skip_in_migrated_prod_code() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        let dir = repo.join("SddIA/engine/execute-process/src/engine");
        fs::create_dir_all(&dir).unwrap();
        let f = dir.join("notify_humanized_pr_merged.rs");
        fs::write(
            &f,
            "fn prod() { if std::env::var(\"SDDIA_LAB_SKIP_OUTBOUND_TELEGRAM\").is_ok() {} }\n",
        )
        .unwrap();
        let v = scan_file(repo, &f);
        assert_eq!(v.len(), 1, "{v:?}");
    }

    #[test]
    fn ignores_sddia_lab_skip_inside_cfg_test() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        let dir = repo.join("SddIA/engine/execute-process/src/engine");
        fs::create_dir_all(&dir).unwrap();
        let f = dir.join("notify_humanized_pr_merged.rs");
        fs::write(
            &f,
            "fn prod() {}\n#[cfg(test)]\nmod tests {\n    fn t() { std::env::set_var(\"SDDIA_LAB_SKIP_OUTBOUND_TELEGRAM\", \"1\"); }\n}\n",
        )
        .unwrap();
        let v = scan_file(repo, &f);
        assert!(v.is_empty(), "{v:?}");
    }
}
