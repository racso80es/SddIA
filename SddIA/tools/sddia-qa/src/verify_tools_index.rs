use regex::Regex;
use std::fs;
use std::path::Path;

const EXCLUDE_STEMS: &[&str] = &["index", "tools-contract", "README"];
const EXCLUDE_SUFFIXES: &[&str] = &["-contract.md"];
const LEGACY_LOCAL_DIRNAME: &str = "Tools";

fn is_tool_definition(path: &Path) -> bool {
    if path.extension().and_then(|e| e.to_str()) != Some("md") {
        return false;
    }
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    if EXCLUDE_STEMS.contains(&stem) {
        return false;
    }
    let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
    if EXCLUDE_SUFFIXES.iter().any(|s| name.ends_with(s)) {
        return false;
    }
    true
}

fn scan_definitions(tools_dir: &Path) -> std::collections::HashSet<String> {
    let mut out = std::collections::HashSet::new();
    let Ok(rd) = fs::read_dir(tools_dir) else {
        return out;
    };
    for entry in rd.flatten() {
        let path = entry.path();
        if path.is_file() && is_tool_definition(&path) {
            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                out.insert(name.to_string());
            }
        }
    }
    out
}

fn parse_index_filenames(index_path: &Path) -> std::collections::HashSet<String> {
    let Ok(text) = fs::read_to_string(index_path) else {
        return std::collections::HashSet::new();
    };
    let re_backtick = Regex::new(r"`([^`]+)`").expect("regex");
    let mut in_catalog = false;
    let mut indexed = std::collections::HashSet::new();
    for line in text.lines() {
        if line.contains("## Catálogo") || line.contains("## Catalogo") {
            in_catalog = true;
            continue;
        }
        if in_catalog && line.starts_with("## ") && !line.contains("Catálogo") && !line.contains("Catalogo") {
            break;
        }
        if !in_catalog || !line.trim().starts_with('|') {
            continue;
        }
        if Regex::new(r"^\|\s*[-:]+").unwrap().is_match(line) {
            continue;
        }
        if line.contains("Archivo fuente") || line.trim() == "| name |" {
            continue;
        }
        let inner = line.trim().trim_start_matches('|').trim_end_matches('|');
        let cells: Vec<_> = inner.split('|').map(str::trim).filter(|s| !s.is_empty()).collect();
        if cells.is_empty() {
            continue;
        }
        let first = cells[0];
        let token = if let Some(caps) = re_backtick.captures(first) {
            caps[1].to_string()
        } else {
            first.to_string()
        };
        if token.is_empty() || token.eq_ignore_ascii_case("archivo fuente") || token == "name" {
            continue;
        }
        let name = if token.ends_with(".md") {
            token
        } else {
            format!("{token}.md")
        };
        indexed.insert(name);
    }
    indexed
}

fn audit_legacy_tools_dirs(repo: &Path) -> Vec<String> {
    let mut errors = Vec::new();
    fn walk(dir: &Path, repo: &Path, errors: &mut Vec<String>) {
        let Ok(rd) = fs::read_dir(dir) else { return };
        for entry in rd.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().and_then(|s| s.to_str()) == Some(".SddIA") {
                    if let Ok(names) = fs::read_dir(&path) {
                        for child in names.flatten() {
                            if child.file_name().to_string_lossy() == LEGACY_LOCAL_DIRNAME {
                                let legacy = child.path();
                                errors.push(format!(
                                    "Violación de Simetría Fractal: {} — exigido `{}` en minúscula estricta",
                                    legacy.display(),
                                    path.join("tools").display()
                                ));
                            }
                        }
                    }
                }
                walk(&path, repo, errors);
            }
        }
    }
    walk(repo, repo, &mut errors);
    errors
}

fn audit_scope(label: &str, tools_dir: &Path, index_path: &Path) -> Vec<String> {
    let mut errors = Vec::new();
    if !tools_dir.is_dir() && !index_path.is_file() {
        return errors;
    }
    if index_path.is_file() && !tools_dir.is_dir() {
        errors.push(format!("{label}: index exists but directory missing: {}", tools_dir.display()));
        return errors;
    }
    if tools_dir.is_dir() && !index_path.is_file() {
        errors.push(format!("{label}: definitions present but index missing: {}", index_path.display()));
        return errors;
    }
    let on_disk = scan_definitions(tools_dir);
    let in_index = parse_index_filenames(index_path);
    for name in on_disk.difference(&in_index) {
        errors.push(format!("{label}: orphan file (not in index): {}/{}", tools_dir.display(), name));
    }
    for name in in_index.difference(&on_disk) {
        errors.push(format!("{label}: orphan index row (no file): {name}"));
    }
    errors
}

pub fn run(repo: &Path) -> i32 {
    let mut errors = Vec::new();
    errors.extend(audit_legacy_tools_dirs(repo));
    let core_dir = repo.join("SddIA/tools");
    errors.extend(audit_scope("core", &core_dir, &core_dir.join("index.md")));
    let local_dir = repo.join(".SddIA/tools");
    let local_index = repo.join(".SddIA/tools/index.md");
    if local_index.is_file() || local_dir.is_dir() {
        errors.extend(audit_scope("local", &local_dir, &local_index));
    }
    if errors.is_empty() {
        println!("verify-tools-index: OK");
        0
    } else {
        eprintln!("verify-tools-index: FAILED");
        for e in errors {
            eprintln!("{e}");
        }
        1
    }
}
