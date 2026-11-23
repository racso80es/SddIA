//! Gate estático AUT-118: valida que las invocaciones Rust `invoke_capsule_json` / `invoke_tool_capsule_json`
//! coincidan con el catálogo SSOT (`SddIA/skills/index.md` vs `SddIA/tools/index.md`).

use regex::Regex;
use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize)]
pub struct Violation {
    pub path: String,
    pub line: u32,
    pub capsule: String,
    pub invoked_as: String,
    pub catalog_kind: String,
}

#[derive(Debug, Serialize)]
pub struct Report {
    pub ok: bool,
    pub scanned_files: u64,
    pub violation_count: u64,
    pub violations: Vec<Violation>,
}

fn parse_catalog_names(index_path: &Path) -> HashSet<String> {
    let mut out = HashSet::new();
    let Ok(text) = fs::read_to_string(index_path) else {
        return out;
    };
    let re_bt = Regex::new(r"`([a-z0-9][a-z0-9\-]+)(?:\.md)?`").expect("regex");
    for line in text.lines() {
        let t = line.trim();
        if !t.starts_with('|') {
            continue;
        }
        for cap in re_bt.captures_iter(t) {
            if let Some(m) = cap.get(1) {
                out.insert(m.as_str().to_string());
            }
        }
    }
    out
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

pub fn scan_repo(repo: &Path) -> Report {
    let skills = parse_catalog_names(&repo.join("SddIA/skills/index.md"));
    let tools = parse_catalog_names(&repo.join("SddIA/tools/index.md"));

    let mut files = Vec::new();
    for sub in ["engine", "daemons", "tools", "interfaces"] {
        collect_rs_files(&repo.join("SddIA").join(sub), &mut files);
    }
    files.sort();
    files.dedup();

    let re_call = Regex::new(
        r#"(invoke_capsule_json|invoke_tool_capsule_json)\s*\(\s*[^,]+,\s*"([a-z0-9\-]+)""#,
    )
    .expect("regex");

    let mut violations = Vec::new();
    for f in &files {
        let Ok(text) = fs::read_to_string(f) else {
            continue;
        };
        let rel = f
            .strip_prefix(repo)
            .unwrap_or(f)
            .to_string_lossy()
            .replace('\\', "/");

        let mut in_cfg_test = false;
        let mut depth: i32 = 0;
        let mut saw_brace = false;
        let mut line_start = 0usize;
        let lines: Vec<&str> = text.lines().collect();
        let mut prod_ranges: Vec<(u32, String)> = Vec::new();
        for (idx, line) in lines.iter().enumerate() {
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
            prod_ranges.push((idx as u32 + 1, (*line).to_string()));
        }

        let prod_text = prod_ranges
            .iter()
            .map(|(_, l)| l.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        for cap in re_call.captures_iter(&prod_text) {
            let fn_name = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let capsule = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let start = cap.get(0).map(|m| m.start()).unwrap_or(0);
            let line_idx = prod_text[..start].matches('\n').count();
            let line_no = prod_ranges.get(line_idx).map(|(n, _)| *n).unwrap_or(1);

            if fn_name == "invoke_capsule_json" && tools.contains(capsule) && !skills.contains(capsule) {
                violations.push(Violation {
                    path: rel.clone(),
                    line: line_no,
                    capsule: capsule.to_string(),
                    invoked_as: "skill".to_string(),
                    catalog_kind: "tool".to_string(),
                });
            } else if fn_name == "invoke_tool_capsule_json" && skills.contains(capsule) && !tools.contains(capsule) {
                violations.push(Violation {
                    path: rel.clone(),
                    line: line_no,
                    capsule: capsule.to_string(),
                    invoked_as: "tool".to_string(),
                    catalog_kind: "skill".to_string(),
                });
            }
        }
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
            "gate-capsule-kind: OK ({} ficheros)",
            report.scanned_files
        );
    } else {
        eprintln!(
            "gate-capsule-kind: KO — {} violaciones en {} ficheros",
            report.violation_count, report.scanned_files
        );
        for v in &report.violations {
            eprintln!(
                "  {}:{} '{}' catalogado como '{}' pero invocado como '{}'",
                v.path, v.line, v.capsule, v.catalog_kind, v.invoked_as
            );
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
    fn detects_tool_invoked_as_skill_and_skill_invoked_as_tool() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/skills")).unwrap();
        fs::create_dir_all(repo.join("SddIA/tools")).unwrap();
        fs::create_dir_all(repo.join("SddIA/engine/demo/src")).unwrap();

        fs::write(
            repo.join("SddIA/skills/index.md"),
            "| `mayeuta-llm` | uuid | 1.0.0 | cat | desc |\n",
        )
        .unwrap();
        fs::write(
            repo.join("SddIA/tools/index.md"),
            "| `send-telegram-notification` | uuid | 1.0.0 | cat | desc |\n",
        )
        .unwrap();
        fs::write(
            repo.join("SddIA/engine/demo/src/lib.rs"),
            r#"
fn bad1(repo: &std::path::Path) {
    let _ = invoke_capsule_json(repo, "send-telegram-notification", &v, false);
}
fn bad2(repo: &std::path::Path) {
    let _ = invoke_tool_capsule_json(
        repo,
        "mayeuta-llm",
        &v,
        false,
    );
}
"#,
        )
        .unwrap();

        let r = scan_repo(repo);
        assert!(!r.ok);
        assert_eq!(r.violation_count, 2, "{:?}", r.violations);
    }

    #[test]
    fn passes_when_capsule_kinds_match() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/skills")).unwrap();
        fs::create_dir_all(repo.join("SddIA/tools")).unwrap();
        fs::create_dir_all(repo.join("SddIA/engine/demo/src")).unwrap();

        fs::write(
            repo.join("SddIA/skills/index.md"),
            "| `mayeuta-llm` | uuid | 1.0.0 | cat | desc |\n",
        )
        .unwrap();
        fs::write(
            repo.join("SddIA/tools/index.md"),
            "| `send-telegram-notification` | uuid | 1.0.0 | cat | desc |\n",
        )
        .unwrap();
        fs::write(
            repo.join("SddIA/engine/demo/src/lib.rs"),
            r#"
fn ok1(repo: &std::path::Path) {
    let _ = invoke_capsule_json(repo, "mayeuta-llm", &v, false);
    let _ = invoke_tool_capsule_json(repo, "send-telegram-notification", &v, false);
}
"#,
        )
        .unwrap();

        let r = scan_repo(repo);
        assert!(r.ok, "{:?}", r.violations);
    }
}
