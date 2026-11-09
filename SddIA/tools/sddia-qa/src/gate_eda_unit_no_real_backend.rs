//! Gate MAN-37 / **L-GATE**: veto de backend real **dentro** de unitarios (`#[cfg(test)]` / `#[test]`).
//! Complementario e inverso a [`crate::gate_event_manager_port`] (MAN-35, solo productivo).

use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

const FORBIDDEN: &[&str] = &[
    "EventStore::open",
    "sddia_event_store::",
    "use sddia_event_store",
    "StoreMode",
    "override_store_mode",
    "events.redb",
    ".events/",
    ".events\\",
];

#[derive(Debug, Serialize)]
struct Violation {
    path: String,
    line: u32,
    pattern: String,
    snippet: String,
}

#[derive(Debug, Serialize)]
struct Report {
    ok: bool,
    scanned_files: u64,
    violation_count: u64,
    violations: Vec<Violation>,
}

fn rel(repo: &Path, p: &Path) -> String {
    p.strip_prefix(repo)
        .unwrap_or(p)
        .to_string_lossy()
        .replace('\\', "/")
}

fn path_allowlisted(rel: &str) -> bool {
    if rel.contains("/tests/") || rel.ends_with("/tests.rs") {
        return true;
    }
    if rel.starts_with("SddIA/sddia-event-store/")
        || rel.starts_with("SddIA/infrastructure/")
        || rel.starts_with("SddIA/sddia-event-manager-host/")
    {
        return true;
    }
    if rel.ends_with("eda_port_legacy.rs")
        || rel.ends_with("eda_port.rs")
        || rel.ends_with("gate_eda_unit_no_real_backend.rs")
        || rel.ends_with("gate_event_manager_port.rs")
        || rel.ends_with("contract_suite.rs")
    {
        return true;
    }
    if rel.contains("eda_store_io_matrix")
        || rel.contains("eda_subscriber_")
        || rel.contains("event_store_writers")
        || rel.contains("memory_evolution_ingest")
        || rel.contains("dlt_telemetry")
    {
        return true;
    }
    // Integration-in-src (T3) — ver allowlists/eda-unit-backend.toml
    const INTEGRATION_SRC: &[&str] = &[
        "SddIA/engine/execute-process/src/engine/route_domain_core.rs",
        "SddIA/engine/execute-process/src/engine/delivery_close.rs",
        "SddIA/engine/execute-process/src/engine/actions.rs",
        "SddIA/engine/execute-process/src/engine/accept_pr.rs",
        "SddIA/engine/execute-process/src/engine/aiua_intent.rs",
        "SddIA/engine/execute-process/src/engine/progress_trace.rs",
        "SddIA/engine/execute-process/src/engine/suite_execution_requested.rs",
        "SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs",
        "SddIA/engine/execute-process/src/engine/workspace_init.rs",
        "SddIA/engine/execute-process/src/engine/thermodynamic.rs",
        "SddIA/engine/execute-process/src/engine/cli_detach.rs",
        "SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs",
        "SddIA/engine/execute-process/src/engine/persist_pec_correlation_proof.rs",
        "SddIA/engine/execute-process/src/engine/domain_authority.rs",
        "SddIA/engine/execute-process/src/engine/capability_di_reactor.rs",
        "SddIA/engine/execute-process/src/engine/eda_bus_topology.rs",
        "SddIA/engine/execute-process/src/engine/linear_direct_cycle_e2e.rs",
        "SddIA/engine/execute-process/src/engine/linear_done_gate_both_e2e.rs",
        "SddIA/engine/execute-process/src/engine/handlers/system_vitality.rs",
        "SddIA/engine/execute-process/src/engine/handlers/daemon_heartbeat.rs",
        "SddIA/engine/execute-process/src/engine/handlers/tracker_markdown_apply.rs",
        "SddIA/engine/execute-process/src/engine/handlers/tracker_sync_replay.rs",
        "SddIA/engine/execute-process/src/engine/handlers/aiua_stimulus.rs",
        "SddIA/engine/execute-process/src/engine/handlers/user_preference.rs",
        "SddIA/engine/execute-process/src/engine/capability_di_gate.rs",
        "SddIA/engine/execute-process/src/engine/capability_di_output_validator.rs",
        "SddIA/engine/execute-process/src/engine/cerbero_di_envelope.rs",
        "SddIA/engine/execute-process/src/engine/cerbero_di_rbac.rs",
        "SddIA/engine/execute-process/src/engine/handlers/instance_creator.rs",
        "SddIA/engine/execute-process/src/engine/materialize_fracture_pbi.rs",
        "SddIA/engine/execute-process/src/engine/materialize_kaizen_alert_doc.rs",
        "SddIA/engine/execute-process/src/engine/tracker_pbi_meta.rs",
    ];
    if INTEGRATION_SRC.iter().any(|p| rel == *p) {
        return true;
    }
    if rel.contains("/handlers/kalma2") || rel.contains("/handlers/email_") {
        return true;
    }
    if rel.starts_with("SddIA/interfaces/kalma2-bridge/")
        || rel.starts_with("SddIA/tools/event-bus-audit/")
        || rel.starts_with("SddIA/tools/repo-retention-purger/")
        || rel.starts_with("SddIA/tools/thought-graph-access/")
    {
        return true;
    }
    false
}

fn brace_block_end(lines: &[&str], start: usize) -> usize {
    let mut depth = 0i32;
    let mut i = start;
    while i < lines.len() {
        for ch in lines[i].chars() {
            if ch == '{' {
                depth += 1;
            } else if ch == '}' {
                depth -= 1;
            }
        }
        i += 1;
        if depth <= 0 && i > start {
            return i;
        }
    }
    lines.len()
}

fn skip_cfg_test_item(lines: &[&str], cfg_line: usize) -> usize {
    let mut i = cfg_line + 1;
    while i < lines.len() && lines[i].trim().is_empty() {
        i += 1;
    }
    if i >= lines.len() {
        return lines.len();
    }
    if lines[i].trim() == "{" || lines[i].contains('{') {
        return brace_block_end(lines, i);
    }
    i
}

/// Líneas pertenecientes a regiones unitarias (`#[cfg(test)]` / `#[test]`).
fn unit_test_lines(src: &str) -> Vec<(u32, String)> {
    let lines: Vec<&str> = src.lines().collect();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed == "#[cfg(test)]" || trimmed.starts_with("#[cfg(test)]") {
            let end = skip_cfg_test_item(&lines, i);
            for j in i..end {
                out.push((j as u32 + 1, lines[j].to_string()));
            }
            i = end;
            continue;
        }
        if trimmed.starts_with("#[test]") {
            let mut j = i;
            while j < lines.len() && !lines[j].contains('{') {
                j += 1;
            }
            let end = if j < lines.len() {
                brace_block_end(&lines, j)
            } else {
                i + 1
            };
            for k in i..end {
                out.push((k as u32 + 1, lines[k].to_string()));
            }
            i = end;
            continue;
        }
        i += 1;
    }
    out
}

fn collect_rs_files(repo: &Path, root: &Path, acc: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(root) else {
        return;
    };
    for entry in rd.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if name == "target" || name.starts_with('.') && name != ".SddIA" {
                continue;
            }
            collect_rs_files(repo, &path, acc);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            let r = rel(repo, &path);
            if path_allowlisted(&r) {
                continue;
            }
            if r.starts_with("SddIA/") {
                acc.push(path);
            }
        }
    }
}

fn scan_file(repo: &Path, path: &Path) -> Vec<Violation> {
    let Ok(text) = fs::read_to_string(path) else {
        return Vec::new();
    };
    let r = rel(repo, path);
    let mut out = Vec::new();
    for (line_no, line) in unit_test_lines(&text) {
        let trimmed = line.trim();
        if trimmed.starts_with("//") {
            continue;
        }
        for pat in FORBIDDEN {
            if line.contains(pat) {
                out.push(Violation {
                    path: r.clone(),
                    line: line_no,
                    pattern: (*pat).into(),
                    snippet: trimmed.chars().take(120).collect(),
                });
            }
        }
    }
    out
}

pub fn run(repo: &Path, json: bool) -> i32 {
    let roots = [
        repo.join("SddIA/engine"),
        repo.join("SddIA/interfaces"),
        repo.join("SddIA/daemons"),
        repo.join("SddIA/tools"),
        repo.join("SddIA/sddia-core"),
    ];
    let mut files = Vec::new();
    for root in roots {
        if root.is_dir() {
            collect_rs_files(repo, &root, &mut files);
        }
    }
    files.sort();
    files.dedup();

    let mut violations = Vec::new();
    for f in &files {
        violations.extend(scan_file(repo, f));
    }

    let report = Report {
        ok: violations.is_empty(),
        scanned_files: files.len() as u64,
        violation_count: violations.len() as u64,
        violations,
    };

    if json {
        println!("{}", serde_json::to_string(&report).unwrap_or_default());
    } else if report.ok {
        println!(
            "gate-eda-unit-no-real-backend: OK ({} ficheros)",
            report.scanned_files
        );
    } else {
        eprintln!(
            "gate-eda-unit-no-real-backend: KO — {} violaciones en {} ficheros",
            report.violation_count, report.scanned_files
        );
        let mut by_file: BTreeMap<&str, Vec<&Violation>> = BTreeMap::new();
        for v in &report.violations {
            by_file.entry(v.path.as_str()).or_default().push(v);
        }
        for (path, rows) in by_file {
            for v in rows {
                eprintln!("  {}:{} [{}] {}", path, v.line, v.pattern, v.snippet);
            }
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
    use std::io::Write;

    #[test]
    fn detects_event_store_open_in_unit_test() {
        let dir = tempfile::tempdir().unwrap();
        let repo = dir.path();
        let engine = repo.join("SddIA/engine/demo/src");
        fs::create_dir_all(&engine).unwrap();
        let f = engine.join("lib.rs");
        let mut file = fs::File::create(&f).unwrap();
        writeln!(
            file,
            "fn prod() {{}}\n#[cfg(test)]\nmod tests {{\n    #[test]\n    fn t() {{ let _ = EventStore::open; }}\n}}\n"
        )
        .unwrap();
        let v = scan_file(repo, &f);
        assert!(
            v.iter().any(|x| x.pattern == "EventStore::open"),
            "{v:?}"
        );
    }

    #[test]
    fn allowlisted_integration_path_skipped_by_collector() {
        let dir = tempfile::tempdir().unwrap();
        let repo = dir.path();
        let p = repo.join("SddIA/engine/execute-process/tests/eda_store_io_matrix_x.rs");
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(&p, "EventStore::open();\n").unwrap();
        assert!(path_allowlisted(&rel(repo, &p)));
    }
}
