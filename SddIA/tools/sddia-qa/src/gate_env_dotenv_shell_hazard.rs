//! AUT-120 / KA-ENV-2..3: valores dotenv sin comillas con metacaracteres de shell y `source`/`.`
//! directo sobre ficheros `.env` en scripts del ecosistema.

use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

const SHELL_META: &[char] = &['|', '&', ';', '<', '>', '$', '`', '\n', '\r'];

/// Scripts que documentan el anti-patrón de forma controlada (solo en comentarios / cadenas de test del gate).
const SHELL_SOURCE_ALLOW: &[&str] = &["SddIA/tools/sddia-qa/src/gate_env_dotenv_shell_hazard.rs"];

#[derive(Debug, Serialize)]
pub struct Violation {
    pub kind: &'static str,
    pub path: String,
    pub line: u32,
    pub key: Option<String>,
    pub hint: &'static str,
}

#[derive(Debug, Serialize)]
pub struct Report {
    pub ok: bool,
    pub dotenv_files_scanned: u64,
    pub shell_files_scanned: u64,
    pub violation_count: u64,
    pub violations: Vec<Violation>,
}

fn strip_quotes(value: &str) -> (bool, String) {
    let v = value.trim();
    if v.len() >= 2 {
        let b = v.as_bytes();
        if (b[0] == b'"' && b[v.len() - 1] == b'"') || (b[0] == b'\'' && b[v.len() - 1] == b'\'') {
            return (true, v[1..v.len() - 1].to_string());
        }
    }
    (false, v.to_string())
}

fn value_has_shell_hazard(raw_value: &str) -> bool {
    let (quoted, inner) = strip_quotes(raw_value);
    if quoted {
        return false;
    }
    if inner.contains("$(") {
        return true;
    }
    SHELL_META.iter().any(|c| inner.contains(*c))
}

fn parse_dotenv_line(line: &str) -> Option<(String, String)> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    let rest = line.strip_prefix("export ").unwrap_or(line);
    let (key, value) = rest.split_once('=')?;
    let key = key.trim();
    if key.is_empty() || !key.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_') {
        return None;
    }
    Some((key.to_string(), value.to_string()))
}

fn collect_env_examples(repo: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let candidates = [
        repo.join(".dev/.env.example"),
        repo.join("SddIA/scripts/starter-kit/.SddIA/.dev/.env.example"),
    ];
    for p in candidates {
        if p.is_file() {
            paths.push(p);
        }
    }
    paths
}

fn scan_dotenv_file(repo: &Path, path: &Path) -> Vec<Violation> {
    let rel = path
        .strip_prefix(repo)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");
    let Ok(text) = fs::read_to_string(path) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for (idx, line) in text.lines().enumerate() {
        let Some((key, value)) = parse_dotenv_line(line) else {
            continue;
        };
        if value_has_shell_hazard(&value) {
            out.push(Violation {
                kind: "dotenv-unquoted-metachar",
                path: rel.clone(),
                line: idx as u32 + 1,
                key: Some(key),
                hint: "Entrecomillar el valor (\"...\") o escapar metacaracteres; cargar con _sddia_load_vault, no source",
            });
        }
    }
    out
}

fn collect_shell_files(dir: &Path, acc: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else {
        return;
    };
    for entry in rd.flatten() {
        let p = entry.path();
        if p.is_dir() {
            let name = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if name == "target" || name == "node_modules" || name.starts_with('.') {
                continue;
            }
            collect_shell_files(&p, acc);
        } else if p.extension().and_then(|e| e.to_str()) == Some("sh") {
            acc.push(p);
        }
    }
}

fn line_has_source_dotenv(line: &str) -> bool {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return false;
    }
    let lower = trimmed.to_lowercase();
    if lower.contains("source ") && lower.contains(".env") {
        return true;
    }
    // `. /path/.env` — evitar confundir `source "$ROOT/.../sddia_shell_lib.sh"`
    if trimmed.starts_with('.') {
        let rest = trimmed.trim_start_matches('.').trim_start();
        if rest.contains(".env") {
            return true;
        }
    }
    false
}

fn scan_shell_file(repo: &Path, path: &Path) -> Vec<Violation> {
    let rel = path
        .strip_prefix(repo)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");
    if SHELL_SOURCE_ALLOW.contains(&rel.as_str()) {
        return Vec::new();
    }
    let Ok(text) = fs::read_to_string(path) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for (idx, line) in text.lines().enumerate() {
        if line_has_source_dotenv(line) {
            out.push(Violation {
                kind: "shell-source-dotenv",
                path: rel.clone(),
                line: idx as u32 + 1,
                key: None,
                hint: "Usar _sddia_load_vault; prohibido source/. sobre ficheros .env",
            });
        }
    }
    out
}

pub fn scan_repo(repo: &Path, include_local_vault: bool) -> Report {
    let mut violations = Vec::new();
    let mut dotenv_count = 0u64;
    for p in collect_env_examples(repo) {
        dotenv_count += 1;
        violations.extend(scan_dotenv_file(repo, &p));
    }
    if include_local_vault {
        for p in [repo.join(".dev/.env"), repo.join(".SddIA/.dev/.env")] {
            if p.is_file() {
                dotenv_count += 1;
                violations.extend(scan_dotenv_file(repo, &p));
            }
        }
    }
    let mut shell_files = Vec::new();
    for sub in [
        "",
        "SddIA/scripts",
        "SddIA/scripts/qa",
        "SddIA/scripts/daemons",
        "SddIA/scripts/installer",
        "SddIA/scripts/tools",
    ] {
        let base = if sub.is_empty() {
            repo.to_path_buf()
        } else {
            repo.join(sub)
        };
        if base.is_dir() {
            collect_shell_files(&base, &mut shell_files);
        }
    }
    shell_files.sort();
    shell_files.dedup();
    for f in &shell_files {
        violations.extend(scan_shell_file(repo, f));
    }
    Report {
        ok: violations.is_empty(),
        dotenv_files_scanned: dotenv_count,
        shell_files_scanned: shell_files.len() as u64,
        violation_count: violations.len() as u64,
        violations,
    }
}

pub fn run(repo: &Path, json: bool, include_local_vault: bool) -> i32 {
    let report = scan_repo(repo, include_local_vault);
    if json {
        println!("{}", serde_json::to_string(&report).unwrap_or_default());
    } else if report.ok {
        println!(
            "gate-env-dotenv-shell-hazard: OK ({} dotenv, {} shell)",
            report.dotenv_files_scanned, report.shell_files_scanned
        );
    } else {
        eprintln!(
            "gate-env-dotenv-shell-hazard: KO — {} hallazgos",
            report.violation_count
        );
        for v in &report.violations {
            match &v.key {
                Some(k) => eprintln!(
                    "  {}:{}:{} ({}) — {}",
                    v.path, v.line, k, v.kind, v.hint
                ),
                None => eprintln!("  {}:{} ({}) — {}", v.path, v.line, v.kind, v.hint),
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
    fn detects_unquoted_pipe() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join(".dev")).unwrap();
        let f = root.join(".dev/.env.example");
        fs::write(&f, "TOKEN=abc|def\n").unwrap();
        let r = scan_repo(root, false);
        assert_eq!(r.violation_count, 1);
        assert_eq!(r.violations[0].key.as_deref(), Some("TOKEN"));
    }

    #[test]
    fn ignores_quoted_metachar() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join(".dev")).unwrap();
        fs::write(root.join(".dev/.env.example"), "TOKEN=\"abc|def\"\n").unwrap();
        let r = scan_repo(root, false);
        assert!(r.ok);
    }

    #[test]
    fn rejects_source_dotenv_in_shell() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        let sh = repo.join("SddIA/scripts/qa/evil.sh");
        fs::create_dir_all(sh.parent().unwrap()).unwrap();
        let mut f = fs::File::create(&sh).unwrap();
        writeln!(f, "#!/bin/bash").unwrap();
        writeln!(f, "source .dev/.env").unwrap();
        let v = scan_shell_file(repo, &sh);
        assert_eq!(v.len(), 1);
    }
}