//! Gate MAN-35 / **L-GATE**: veto de `sddia_event_store` y modos legacy fuera de allowlist.

use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

const FORBIDDEN: &[&str] = &[
    "EventStore::open",
    "sddia_event_store::",
    "use sddia_event_store",
    "StoreMode::Filesystem",
    "StoreMode::Dual",
    "SDDIA_EVENT_STORE",
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
    if rel.starts_with("SddIA/sddia-event-store/") {
        return true;
    }
    if rel.ends_with("eda_port_legacy.rs")
        || rel.ends_with("gate_event_manager_port.rs")
        || rel.ends_with("gate_eda_unit_no_real_backend.rs")
    {
        return true;
    }
    if rel.contains("eda_store_io_matrix")
        || rel.contains("eda_subscriber_")
        || rel.contains("event_store_writers")
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
    if lines[i].trim() == "{" {
        return brace_block_end(lines, i);
    }
    if lines[i].contains('{') {
        return brace_block_end(lines, i);
    }
    i
}

fn skip_test_fn(lines: &[&str], test_line: usize) -> usize {
    if lines[test_line].contains('{') {
        return brace_block_end(lines, test_line);
    }
    test_line + 1
}

fn productivo_lines(src: &str) -> Vec<(u32, String)> {
    let lines: Vec<&str> = src.lines().collect();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed == "#[cfg(test)]" || trimmed.starts_with("#[cfg(test)]") {
            i = skip_cfg_test_item(&lines, i);
            continue;
        }
        if trimmed.starts_with("#[test]") {
            i = skip_test_fn(&lines, i);
            continue;
        }
        out.push((i as u32 + 1, lines[i].to_string()));
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
            let rel = rel(repo, &path);
            if path_allowlisted(&rel) {
                continue;
            }
            if rel.starts_with("SddIA/")
                || rel.starts_with("SddIA\\")
                || rel.contains("/interfaces/")
                || rel.contains("/daemons/")
                || rel.contains("/tools/")
            {
                acc.push(path);
            }
        }
    }
}

fn scan_file(repo: &Path, path: &Path) -> Vec<Violation> {
    let Ok(text) = fs::read_to_string(path) else {
        return Vec::new();
    };
    let rel = rel(repo, path);
    let mut out = Vec::new();
    for (line_no, line) in productivo_lines(&text) {
        for pat in FORBIDDEN {
            if line.contains(pat) {
                out.push(Violation {
                    path: rel.clone(),
                    line: line_no,
                    pattern: (*pat).into(),
                    snippet: line.trim().chars().take(120).collect(),
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
        repo.join("SddIA/infrastructure"),
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
            "gate-event-manager-port: OK ({} ficheros)",
            report.scanned_files
        );
    } else {
        eprintln!(
            "gate-event-manager-port: KO — {} violaciones en {} ficheros",
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
    fn ignores_cfg_test_block() {
        let dir = tempfile::tempdir().unwrap();
        let repo = dir.path();
        let engine = repo.join("SddIA/engine/demo/src");
        fs::create_dir_all(&engine).unwrap();
        let f = engine.join("lib.rs");
        let mut file = fs::File::create(&f).unwrap();
        writeln!(
            file,
            "fn prod() {{}}\n#[cfg(test)]\nmod tests {{\n    fn t() {{ sddia_event_store::EventStore::open(); }}\n}}\n"
        )
        .unwrap();
        let v = scan_file(repo, &f);
        assert!(v.is_empty(), "{v:?}");
    }
}
