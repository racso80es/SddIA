//! Gate KZ-EST-01 / KZ-PRM-02: ratchet monótono decreciente de allowlists.
//! Falla si `VIRTUAL_PATH_ALLOW`, `SCAN_EXCLUDE`, `INTEGRATION_SRC` o `allowlists/*.toml`
//! crecen en cardinalidad respecto a `origin/main` sin laudo `RATCHET_LAUDO` en `clarify.md`.

use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;

const HYGIENE_REL: &str = "SddIA/engine/execute-process/src/engine/eda_event_token_hygiene.rs";
const UNIT_GATE_REL: &str = "SddIA/tools/sddia-qa/src/gate_eda_unit_no_real_backend.rs";
const ALLOWLISTS_DIR_REL: &str = "SddIA/tools/sddia-qa/allowlists";

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct AllowlistMetric {
    pub name: String,
    pub path: String,
    pub baseline: usize,
    pub current: usize,
    pub grew: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct RatchetReport {
    pub ok: bool,
    pub base_ref: String,
    pub laudo_found: bool,
    pub laudo_sources: Vec<String>,
    pub metrics: Vec<AllowlistMetric>,
    pub violations: Vec<String>,
}

/// Cuenta entradas en `const <name>: &[...] = &[ ... ];` (tuplas `(...)` o strings `"..."`).
pub fn count_rust_const_entries(src: &str, const_name: &str) -> Result<usize, String> {
    let needle = format!("const {const_name}");
    let start_decl = src
        .find(&needle)
        .ok_or_else(|| format!("no se encontró `const {const_name}`"))?;
    let after_decl = &src[start_decl..];
    let eq_idx = after_decl
        .find("&[")
        .ok_or_else(|| format!("no se encontró `&[` en `const {const_name}`"))?;
    // Saltar `&[` dentro del tipo (ej. `const FOO: &[&str] = &[ ... ]`): buscar `= &[`
    let slice_start = if let Some(assign_idx) = after_decl.find("= &[") {
        assign_idx + 4
    } else {
        eq_idx + 2
    };
    let body = &after_decl[slice_start..];

    let mut depth_bracket = 1i32;
    let mut depth_paren = 0i32;
    let mut in_string = false;
    let mut escape = false;
    let mut in_line_comment = false;
    let mut tuple_count = 0usize;
    let mut string_top_count = 0usize;
    let mut prev_char = '\0';

    for ch in body.chars() {
        if in_line_comment {
            if ch == '\n' {
                in_line_comment = false;
            }
            prev_char = ch;
            continue;
        }
        if in_string {
            if escape {
                escape = false;
            } else if ch == '\\' {
                escape = true;
            } else if ch == '"' {
                in_string = false;
            }
            prev_char = ch;
            continue;
        }
        if prev_char == '/' && ch == '/' {
            in_line_comment = true;
            prev_char = ch;
            continue;
        }
        match ch {
            '"' => {
                in_string = true;
                if depth_bracket == 1 && depth_paren == 0 {
                    string_top_count += 1;
                }
            }
            '(' => {
                if depth_bracket == 1 && depth_paren == 0 {
                    tuple_count += 1;
                }
                depth_paren += 1;
            }
            ')' => {
                depth_paren -= 1;
            }
            '[' => {
                depth_bracket += 1;
            }
            ']' => {
                depth_bracket -= 1;
                if depth_bracket == 0 {
                    break;
                }
            }
            _ => {}
        }
        prev_char = ch;
    }

    Ok(if tuple_count > 0 {
        tuple_count
    } else {
        string_top_count
    })
}

/// Cuenta literales string dentro de arrays `[...]` en un fichero TOML de allowlist.
pub fn count_toml_array_entries(src: &str) -> usize {
    let mut in_array = false;
    let mut in_string = false;
    let mut escape = false;
    let mut in_comment = false;
    let mut count = 0usize;

    for ch in src.chars() {
        if in_comment {
            if ch == '\n' {
                in_comment = false;
            }
            continue;
        }
        if in_string {
            if escape {
                escape = false;
            } else if ch == '\\' {
                escape = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }
        match ch {
            '#' => in_comment = true,
            '[' => in_array = true,
            ']' => in_array = false,
            '"' if in_array => {
                in_string = true;
                count += 1;
            }
            _ => {}
        }
    }
    count
}

fn git_stdout(repo: &Path, args: &[&str]) -> Option<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        None
    }
}

fn resolve_base_ref(repo: &Path, preferred: &str) -> Option<String> {
    for candidate in [preferred, "main", "HEAD"] {
        if git_stdout(repo, &["rev-parse", "--verify", candidate]).is_some() {
            return Some(candidate.to_string());
        }
    }
    None
}

fn list_toml_allowlists_in_fs(repo: &Path) -> BTreeMap<String, usize> {
    let mut map = BTreeMap::new();
    let dir = repo.join(ALLOWLISTS_DIR_REL);
    if let Ok(rd) = fs::read_dir(&dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.extension().and_then(|x| x.to_str()) == Some("toml") {
                if let (Some(name), Ok(content)) = (
                    p.file_name().and_then(|n| n.to_str()),
                    fs::read_to_string(&p),
                ) {
                    let rel = format!("{ALLOWLISTS_DIR_REL}/{name}");
                    map.insert(rel, count_toml_array_entries(&content));
                }
            }
        }
    }
    map
}

fn list_toml_allowlists_in_git(repo: &Path, base_ref: &str) -> BTreeMap<String, usize> {
    let mut map = BTreeMap::new();
    let Some(tree_out) = git_stdout(
        repo,
        &["ls-tree", "-r", "--name-only", base_ref, ALLOWLISTS_DIR_REL],
    ) else {
        return map;
    };
    for line in tree_out.lines().map(str::trim).filter(|l| l.ends_with(".toml")) {
        if let Some(content) = git_stdout(repo, &["show", &format!("{base_ref}:{line}")]) {
            map.insert(line.to_string(), count_toml_array_entries(&content));
        }
    }
    map
}

fn find_clarify_laudos(repo: &Path, base_ref: &str) -> Vec<String> {
    let mut candidates: Vec<String> = Vec::new();
    if let Some(diff_out) = git_stdout(
        repo,
        &["diff", "--name-only", &format!("{base_ref}...HEAD")],
    ) {
        candidates.extend(diff_out.lines().map(str::trim).map(str::to_string));
    }
    if let Some(wt_out) = git_stdout(repo, &["status", "--porcelain", "-uall"]) {
        for line in wt_out.lines() {
            if line.len() > 3 {
                candidates.push(line[3..].trim().to_string());
            }
        }
    }
    candidates.sort();
    candidates.dedup();

    let mut found = Vec::new();
    for rel in candidates {
        if (rel.starts_with("docs/features/") || rel.starts_with("docs/fixes/"))
            && rel.ends_with("/clarify.md")
        {
            if let Ok(text) = fs::read_to_string(repo.join(&rel)) {
                if text.contains("RATCHET_LAUDO") {
                    found.push(rel);
                }
            }
        }
    }
    found
}

pub fn evaluate_ratchet(repo: &Path, preferred_base: &str) -> Result<RatchetReport, String> {
    let base_ref = resolve_base_ref(repo, preferred_base)
        .ok_or_else(|| format!("no se pudo resolver referencia git base ({preferred_base})"))?;

    let mut metrics = Vec::new();

    // 1. VIRTUAL_PATH_ALLOW y SCAN_EXCLUDE
    let hygiene_cur_src =
        fs::read_to_string(repo.join(HYGIENE_REL)).map_err(|e| e.to_string())?;
    let hygiene_base_src = git_stdout(repo, &["show", &format!("{base_ref}:{HYGIENE_REL}")])
        .unwrap_or_else(|| hygiene_cur_src.clone());

    for name in ["VIRTUAL_PATH_ALLOW", "SCAN_EXCLUDE"] {
        let cur = count_rust_const_entries(&hygiene_cur_src, name)?;
        let base = count_rust_const_entries(&hygiene_base_src, name)?;
        metrics.push(AllowlistMetric {
            name: name.to_string(),
            path: HYGIENE_REL.to_string(),
            baseline: base,
            current: cur,
            grew: cur > base,
        });
    }

    // 2. INTEGRATION_SRC
    let unit_cur_src = fs::read_to_string(repo.join(UNIT_GATE_REL)).map_err(|e| e.to_string())?;
    let unit_base_src = git_stdout(repo, &["show", &format!("{base_ref}:{UNIT_GATE_REL}")])
        .unwrap_or_else(|| unit_cur_src.clone());
    let cur_int = count_rust_const_entries(&unit_cur_src, "INTEGRATION_SRC")?;
    let base_int = count_rust_const_entries(&unit_base_src, "INTEGRATION_SRC")?;
    metrics.push(AllowlistMetric {
        name: "INTEGRATION_SRC".to_string(),
        path: UNIT_GATE_REL.to_string(),
        baseline: base_int,
        current: cur_int,
        grew: cur_int > base_int,
    });

    // 3. allowlists/*.toml
    let fs_tomls = list_toml_allowlists_in_fs(repo);
    let git_tomls = list_toml_allowlists_in_git(repo, &base_ref);
    let mut all_tomls: Vec<String> = fs_tomls.keys().chain(git_tomls.keys()).cloned().collect();
    all_tomls.sort();
    all_tomls.dedup();
    for rel in all_tomls {
        let cur = *fs_tomls.get(&rel).unwrap_or(&0);
        let base = *git_tomls.get(&rel).unwrap_or(&0);
        metrics.push(AllowlistMetric {
            name: format!("toml:{rel}"),
            path: rel,
            baseline: base,
            current: cur,
            grew: cur > base,
        });
    }

    let laudo_sources = find_clarify_laudos(repo, &base_ref);
    let laudo_found = !laudo_sources.is_empty();
    let mut violations = Vec::new();
    for m in &metrics {
        if m.grew && !laudo_found {
            violations.push(format!(
                "ratchet violated: {} ({}) creció de {} a {} respecto a {} sin RATCHET_LAUDO en clarify.md",
                m.name, m.path, m.baseline, m.current, base_ref
            ));
        }
    }

    Ok(RatchetReport {
        ok: violations.is_empty(),
        base_ref,
        laudo_found,
        laudo_sources,
        metrics,
        violations,
    })
}

pub fn run(repo: &Path, json_mode: bool, base_ref: Option<&str>) -> i32 {
    let preferred = base_ref.unwrap_or("origin/main");
    match evaluate_ratchet(repo, preferred) {
        Ok(report) => {
            if json_mode {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).unwrap_or_else(|_| "{}".into())
                );
            } else {
                for m in &report.metrics {
                    println!(
                        "{}: {} -> {} (grew={})",
                        m.name, m.baseline, m.current, m.grew
                    );
                }
                for v in &report.violations {
                    eprintln!("ERROR: {v}");
                }
            }
            if report.ok { 0 } else { 2 }
        }
        Err(e) => {
            if json_mode {
                println!(
                    "{}",
                    serde_json::json!({ "ok": false, "error": e })
                );
            } else {
                eprintln!("gate-allowlist-ratchet error: {e}");
            }
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use execute_process::core::repo::find_repo_root;

    fn git(repo: &Path, args: &[&str]) {
        let st = Command::new("git")
            .arg("-C")
            .arg(repo)
            .args(args)
            .env("GIT_AUTHOR_NAME", "test")
            .env("GIT_AUTHOR_EMAIL", "test@example.com")
            .env("GIT_COMMITTER_NAME", "test")
            .env("GIT_COMMITTER_EMAIL", "test@example.com")
            .status()
            .expect("git");
        assert!(st.success(), "git {args:?} falló");
    }

    fn setup_ratchet_repo(
        base_allow: &str,
        cur_allow: &str,
        clarify_content: Option<&str>,
    ) -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let r = tmp.path();
        git(r, &["init", "-b", "main"]);
        fs::create_dir_all(r.join("SddIA/engine/execute-process/src/engine")).unwrap();
        fs::create_dir_all(r.join("SddIA/tools/sddia-qa/src")).unwrap();
        fs::create_dir_all(r.join("SddIA/tools/sddia-qa/allowlists")).unwrap();

        fs::write(
            r.join(HYGIENE_REL),
            format!(
                "const SCAN_EXCLUDE: &[(&str, &str)] = &[(\"a.rs\", \"r\")];\nconst VIRTUAL_PATH_ALLOW: &[(&str, &str)] = &[{base_allow}];\n"
            ),
        )
        .unwrap();
        fs::write(
            r.join(UNIT_GATE_REL),
            "const INTEGRATION_SRC: &[&str] = &[];\n",
        )
        .unwrap();
        fs::write(
            r.join("SddIA/tools/sddia-qa/allowlists/eda-unit-backend.toml"),
            "path_contains = [\"/tests/\"]\n",
        )
        .unwrap();
        git(r, &["add", "."]);
        git(r, &["commit", "-m", "base"]);

        git(r, &["checkout", "-b", "feat/test"]);
        fs::write(
            r.join(HYGIENE_REL),
            format!(
                "const SCAN_EXCLUDE: &[(&str, &str)] = &[(\"a.rs\", \"r\")];\nconst VIRTUAL_PATH_ALLOW: &[(&str, &str)] = &[{cur_allow}];\n"
            ),
        )
        .unwrap();
        if let Some(c) = clarify_content {
            fs::create_dir_all(r.join("docs/features/test")).unwrap();
            fs::write(r.join("docs/features/test/clarify.md"), c).unwrap();
        }
        tmp
    }

    #[test]
    fn ratchet_allows_same_or_smaller_allowlist() {
        let tmp = setup_ratchet_repo(r#"("x.rs", "r1"), ("y.rs", "r2")"#, r#"("x.rs", "r1")"#, None);
        let rep = evaluate_ratchet(tmp.path(), "main").expect("ratchet");
        assert!(rep.ok, "debe aprobar reducción: {:?}", rep.violations);
    }

    #[test]
    fn ratchet_rejects_growth_without_clarify_laudo() {
        let tmp = setup_ratchet_repo(
            r#"("x.rs", "r1")"#,
            r#"("x.rs", "r1"), ("y.rs", "r2")"#,
            Some("# Clarify sin token"),
        );
        let rep = evaluate_ratchet(tmp.path(), "main").expect("ratchet");
        assert!(!rep.ok, "debe rechazar crecimiento sin RATCHET_LAUDO");
        assert!(rep.violations[0].contains("VIRTUAL_PATH_ALLOW"));
    }

    #[test]
    fn ratchet_allows_growth_with_clarify_laudo() {
        let tmp = setup_ratchet_repo(
            r#"("x.rs", "r1")"#,
            r#"("x.rs", "r1"), ("y.rs", "r2")"#,
            Some("# Clarify\n\nRATCHET_LAUDO: aprobado por operador."),
        );
        let rep = evaluate_ratchet(tmp.path(), "main").expect("ratchet");
        assert!(rep.ok, "debe aprobar con RATCHET_LAUDO: {:?}", rep.violations);
        assert!(rep.laudo_found);
    }

    #[test]
    fn repo_allowlist_ratchet_passes() {
        let repo = find_repo_root().expect("repo");
        let rep = evaluate_ratchet(&repo, "origin/main").expect("ratchet repo");
        assert!(rep.ok, "ratchet en repo real falló: {:?}", rep.violations);
    }
}
