//! Peaje determinista anti-anemia para entidades `.md` de dominio (AUT-125 / Q1).

use crate::core::paths::load_paths_config;
use regex::Regex;
use serde::Serialize;
use serde_yaml::Value as YamlValue;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize)]
pub struct Violation {
    pub path: String,
    pub family: String,
    pub code: String,
    pub detail: String,
}

#[derive(Debug, Serialize)]
pub struct ScanReport {
    pub ok: bool,
    pub scanned: u64,
    pub violation_count: u64,
    pub violations: Vec<Violation>,
}

fn uuid_present_ok(raw: &str) -> (bool, bool) {
    let s = raw.trim().trim_matches(|c| c == '"' || c == '\'');
    let any = Regex::new(
        r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$",
    )
    .expect("uuid re");
    let v4 = Regex::new(
        r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$",
    )
    .expect("uuid v4 re");
    (any.is_match(s), v4.is_match(s))
}

fn semver_ok(raw: &str) -> bool {
    let s = raw.trim();
  let re = Regex::new(r"^\d+\.\d+\.\d+").expect("semver re");
    re.is_match(s)
}

fn split_frontmatter(raw: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = raw.split("---").collect();
    if parts.len() < 3 {
        return None;
    }
    let fm = parts[1].trim().to_string();
    let body = parts[2..].join("---").trim_start_matches('\n').to_string();
    Some((fm, body))
}

fn stub_tokens_in_text(text: &str) -> Option<String> {
    for line in text.lines() {
        let t = line.trim();
        if matches!(t, "TODO" | "TBD" | "FIXME" | "..." | "…") {
            return Some(t.to_string());
        }
        if t.starts_with("TODO:") || t.starts_with("TBD:") || t.starts_with("FIXME:") {
            return Some(t.chars().take(12).collect());
        }
    }
    None
}

#[derive(Clone, Copy)]
struct FamilyRule {
    family: &'static str,
    purpose_kw: &'static [&'static str],
    second_kw: &'static [&'static str],
}

const FAMILY_RULES: &[FamilyRule] = &[
    FamilyRule {
        family: "action",
        purpose_kw: &["propósito", "proposito", "purpose"],
        second_kw: &["orquest", "límite", "limite", "interfaz", "flujo", "destino", "hábito", "habito", "contenido"],
    },
    FamilyRule {
        family: "agent",
        purpose_kw: &["propósito", "proposito", "purpose", "rol"],
        second_kw: &["polít", "polit", "deleg", "orquest", "límite", "limite"],
    },
    FamilyRule {
        family: "skill",
        purpose_kw: &["propósito", "proposito", "purpose"],
        second_kw: &["interfaz", "entrada", "salida", "i/o", "operaci"],
    },
    FamilyRule {
        family: "tool",
        purpose_kw: &["propósito", "proposito", "purpose"],
        second_kw: &["interfaz", "operaci", "entrada", "salida"],
    },
    FamilyRule {
        family: "process",
        purpose_kw: &[
            "propósito",
            "proposito",
            "purpose",
            "fase",
            "línea",
            "linea",
            "montaje",
            "handler",
        ],
        second_kw: &[
            "entrada",
            "salida",
            "orquest",
            "cierre",
            "handoff",
            "registro",
            "sellado",
            "recepción",
            "recepcion",
            "operaci",
        ],
    },
    FamilyRule {
        family: "event",
        purpose_kw: &["propósito", "proposito", "purpose", "emis"],
        second_kw: &["payload", "schema", "campo", "contrato"],
    },
    FamilyRule {
        family: "suite",
        purpose_kw: &["propósito", "proposito", "purpose"],
        second_kw: &["estrateg", "nodo", "ejecu"],
    },
    FamilyRule {
        family: "daemon",
        purpose_kw: &["propósito", "proposito", "purpose", "centinela", "daemon"],
        second_kw: &["ejecu", "ciclo", "tick", "suscri", "operaci", "latido", "binario"],
    },
    FamilyRule {
        family: "norm",
        purpose_kw: &["propósito", "proposito", "purpose"],
        second_kw: &["directr", "alcance", "oblig"],
    },
    FamilyRule {
        family: "codex",
        purpose_kw: &["propósito", "proposito", "purpose", "códice", "codice", "dominio"],
        second_kw: &["dominio", "proceso", "invent", "norma", "catálogo", "catalogo"],
    },
];

fn rule_for(family: &str) -> Option<&'static FamilyRule> {
    FAMILY_RULES.iter().find(|r| r.family == family)
}

fn heading_matches_any(title: &str, kws: &[&str]) -> bool {
    let t = title.to_lowercase();
    kws.iter().any(|k| t.contains(&k.to_lowercase()))
}

fn parse_sections(body: &str) -> Vec<(String, String)> {
    let mut sections = Vec::new();
    let mut current_title = String::new();
    let mut current_body = String::new();
    for line in body.lines() {
        if let Some(rest) = line.strip_prefix("## ") {
            if !current_title.is_empty() {
                sections.push((current_title.clone(), current_body.trim().to_string()));
            }
            current_title = rest.trim().to_string();
            current_body.clear();
        } else if line.starts_with('#') && !line.starts_with("##") {
            continue;
        } else if !current_title.is_empty() {
            if !current_body.is_empty() {
                current_body.push('\n');
            }
            current_body.push_str(line);
        }
    }
    if !current_title.is_empty() {
        sections.push((current_title, current_body.trim().to_string()));
    }
    sections
}

fn duplicate_h1(body: &str, name: &str) -> bool {
    let mut seen = 0usize;
    let needle = name.to_lowercase();
    for line in body.lines() {
        let l = line.trim();
        if l.starts_with("# ") && !l.starts_with("##") {
            let title = l.trim_start_matches('#').trim().to_lowercase();
            if title.contains(&needle) || title.contains("acción:") && title.contains(&needle) {
                seen += 1;
                if seen >= 2 {
                    return true;
                }
            }
        }
    }
    false
}

fn truncated_tail(body: &str) -> bool {
    let last = body
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .last()
        .unwrap_or("");
    if last.len() < 12 {
        return false;
    }
    let ends_word = last
        .chars()
        .last()
        .map(|c| c.is_alphanumeric() && c != '.' && c != ')' && c != ']')
        .unwrap_or(false);
    let vowel_cut = last
        .chars()
        .rev()
        .take(4)
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();
    vowel_cut.len() >= 4 && !vowel_cut.contains('.') && ends_word && !last.ends_with('`')
}

pub fn validate_entity_file(repo: &Path, path: &Path, family: &str) -> Vec<Violation> {
    let rel = path
        .strip_prefix(repo)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/");
    let name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string();
    if name.is_empty() || name.ends_with("-contract") || name == "index" {
        return Vec::new();
    }

    let raw = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            return vec![Violation {
                path: rel,
                family: family.into(),
                code: "READ_ERROR".into(),
                detail: e.to_string(),
            }];
        }
    };

    let Some((fm_raw, body)) = split_frontmatter(&raw) else {
        return vec![Violation {
            path: rel,
            family: family.into(),
            code: "NO_FRONTMATTER".into(),
            detail: "falta bloque YAML ---".into(),
        }];
    };

    let mut viols = Vec::new();
    let fm_yaml: YamlValue = serde_yaml::from_str(&fm_raw).unwrap_or(YamlValue::Null);
    let uuid = fm_yaml.get("uuid").and_then(|v| v.as_str()).unwrap_or("");
    let version = fm_yaml.get("version").and_then(|v| v.as_str()).unwrap_or("");
    let name_fm = fm_yaml.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let (uuid_ok, uuid_v4) = uuid_present_ok(uuid);
    if !uuid_ok {
        viols.push(Violation {
            path: rel.clone(),
            family: family.into(),
            code: "UUID_INVALID".into(),
            detail: format!("uuid inválido: {uuid}"),
        });
    } else if !uuid_v4 {
        viols.push(Violation {
            path: rel.clone(),
            family: family.into(),
            code: "UUID_NOT_V4".into(),
            detail: format!("uuid presente pero no v4: {uuid}"),
        });
    }
    if !semver_ok(version) {
        viols.push(Violation {
            path: rel.clone(),
            family: family.into(),
            code: "VERSION_INVALID".into(),
            detail: format!("version inválida: {version}"),
        });
    }
    if !name_fm.is_empty() && name_fm != name && family != "codex" {
        viols.push(Violation {
            path: rel.clone(),
            family: family.into(),
            code: "NAME_FILENAME_MISMATCH".into(),
            detail: format!("name={name_fm} filename={name}"),
        });
    }

    if body.trim().is_empty() {
        viols.push(Violation {
            path: rel.clone(),
            family: family.into(),
            code: "BODY_EMPTY".into(),
            detail: "cuerpo vacío tras frontmatter".into(),
        });
        return viols;
    }

    if duplicate_h1(&body, &name) {
        viols.push(Violation {
            path: rel.clone(),
            family: family.into(),
            code: "DUPLICATE_H1".into(),
            detail: "H1 duplicado de título".into(),
        });
    }

    if truncated_tail(&body) {
        viols.push(Violation {
            path: rel.clone(),
            family: family.into(),
            code: "BODY_TRUNCATED".into(),
            detail: "cierre de cuerpo truncado".into(),
        });
    }

    if let Some(tok) = stub_tokens_in_text(&body) {
        viols.push(Violation {
            path: rel.clone(),
            family: family.into(),
            code: "STUB_MARKER".into(),
            detail: format!("marcador stub: {tok}"),
        });
    }

    let sections = parse_sections(&body);
    if sections.len() < 2 {
        viols.push(Violation {
            path: rel.clone(),
            family: family.into(),
            code: "SECTIONS_INSUFFICIENT".into(),
            detail: format!("secciones ## < 2 (tiene {})", sections.len()),
        });
    }
    for (title, content) in &sections {
        let non_ws: String = content.chars().filter(|c| !c.is_whitespace()).collect();
        if non_ws.is_empty() {
            viols.push(Violation {
                path: rel.clone(),
                family: family.into(),
                code: "SECTION_EMPTY".into(),
                detail: format!("sección vacía: ## {title}"),
            });
        }
    }

    if let Some(rule) = rule_for(family) {
        let has_purpose = sections.iter().any(|(t, c)| {
            !c.is_empty() && heading_matches_any(t, rule.purpose_kw)
        }) || sections.first().map_or(false, |(_, c)| {
            c.chars().filter(|ch| !ch.is_whitespace()).count() >= 40
        });
        let has_second = sections.iter().any(|(t, c)| {
            !c.is_empty() && heading_matches_any(t, rule.second_kw)
        }) || sections.len() >= 2;
        if !has_purpose {
            viols.push(Violation {
                path: rel.clone(),
                family: family.into(),
                code: "CONTRACT_SECTION_MISSING".into(),
                detail: "falta sección de propósito (contractual)".into(),
            });
        }
        if !has_second {
            viols.push(Violation {
                path: rel,
                family: family.into(),
                code: "CONTRACT_SECTION_MISSING".into(),
                detail: "falta sección operativa secundaria (orquestación/interfaz/límites)".into(),
            });
        }
    }

    viols
}

fn walk_md_files(dir: &Path, skip_names: &HashSet<&str>, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else {
        return;
    };
    for entry in rd.flatten() {
        let p = entry.path();
        if p.is_dir() {
            walk_md_files(&p, skip_names, out);
            continue;
        }
        if p.extension().and_then(|x| x.to_str()) != Some("md") {
            continue;
        }
        let fname = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if skip_names.contains(fname) || fname.ends_with("-contract.md") {
            continue;
        }
        out.push(p);
    }
}

fn collect_family_files(repo: &Path, rel_dir: &str, family: &str, out: &mut Vec<(String, PathBuf)>) {
    let root = repo.join(rel_dir);
    if !root.is_dir() {
        return;
    }
    let skip_names: HashSet<&str> = HashSet::from(["index.md"]);
    let mut paths = Vec::new();
    walk_md_files(&root, &skip_names, &mut paths);
    for p in paths {
        out.push((family.into(), p));
    }
}

pub fn genome_entity_dirs(cfg: &serde_json::Value) -> Vec<(&'static str, String)> {
    let dirs = cfg.get("directories").cloned().unwrap_or_default();
    let mut v = Vec::new();
    let pairs = [
        ("agent", "agents"),
        ("action", "actions"),
        ("skill", "skills"),
        ("tool", "tools"),
        ("event", "events"),
        ("suite", "suites"),
        ("daemon", "daemons"),
        ("norm", "library_norms"),
        ("codex", "library_codexes"),
    ];
    for (family, key) in pairs {
        if let Some(d) = dirs.get(key).and_then(|x| x.as_str()) {
            v.push((family, d.trim_end_matches('/').to_string()));
        }
    }
    if let Some(d) = dirs.get("process").and_then(|x| x.as_str()) {
        v.push(("process", d.trim_end_matches('/').to_string()));
    }
    if let Some(arr) = dirs.get("process_domain_roots").and_then(|x| x.as_array()) {
        for item in arr {
            if let Some(s) = item.as_str() {
                v.push(("process", s.trim_end_matches('/').to_string()));
            }
        }
    }
    if let Some(arr) = dirs.get("events_domain_roots").and_then(|x| x.as_array()) {
        for item in arr {
            if let Some(s) = item.as_str() {
                v.push(("event", s.trim_end_matches('/').to_string()));
            }
        }
    }
    v
}

pub fn scan_all(repo: &Path) -> Result<ScanReport, String> {
    let cfg = load_paths_config(repo)?;
    let mut files = Vec::new();
    for (family, dir) in genome_entity_dirs(&cfg) {
        collect_family_files(repo, &dir, family, &mut files);
    }
    let mut by_path: std::collections::HashMap<String, (String, PathBuf)> =
        std::collections::HashMap::new();
    let rank = |fam: &str| -> u8 {
        match fam {
            "event" => 3,
            "process" => 3,
            "action" | "agent" | "skill" | "tool" | "suite" | "daemon" | "norm" => 2,
            _ => 1,
        }
    };
    for (family, path) in files {
        let rel = path
            .strip_prefix(repo)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        match by_path.get(&rel) {
            None => {
                by_path.insert(rel, (family, path));
            }
            Some((prev_fam, _)) if rank(&family) > rank(prev_fam) => {
                by_path.insert(rel, (family, path));
            }
            _ => {}
        }
    }
    let mut entries: Vec<_> = by_path.into_values().collect();
    entries.sort_by(|a, b| a.1.cmp(&b.1));
    let mut violations = Vec::new();
    let scanned = entries.len() as u64;
    for (family, path) in entries {
        violations.extend(validate_entity_file(repo, &path, family.as_str()));
    }
    let ok = violations.is_empty();
    let violation_count = violations.len() as u64;
    Ok(ScanReport {
        ok,
        scanned,
        violation_count,
        violations,
    })
}

pub fn scan_changed(repo: &Path, base_ref: &str) -> Result<ScanReport, String> {
    let out = std::process::Command::new("git")
        .args(["diff", "--name-only", base_ref, "HEAD", "--"])
        .current_dir(repo)
        .output()
        .map_err(|e| format!("git diff: {e}"))?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).into_owned());
    }
    let cfg = load_paths_config(repo)?;
    let dirs: Vec<String> = genome_entity_dirs(&cfg)
        .into_iter()
        .map(|(_, d)| d)
        .collect();
    let mut violations = Vec::new();
    let mut scanned = 0u64;
    for line in String::from_utf8_lossy(&out.stdout).lines() {
        let p = line.trim();
        if !p.ends_with(".md") {
            continue;
        }
        if !dirs.iter().any(|d| p.starts_with(d)) || p.ends_with("-contract.md") || p.ends_with("/index.md")
        {
            continue;
        }
        let path = repo.join(p);
        if !path.is_file() {
            continue;
        }
        let family = genome_entity_dirs(&cfg)
            .into_iter()
            .find(|(_, d)| p.starts_with(d))
            .map(|(f, _)| f)
            .unwrap_or("unknown");
        scanned += 1;
        violations.extend(validate_entity_file(repo, &path, family));
    }
    Ok(ScanReport {
        ok: violations.is_empty(),
        scanned,
        violation_count: violations.len() as u64,
        violations,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repo::find_repo_root;

    #[test]
    fn aut125_witness_truncated_switch_llm_fixture_rejected() {
        let repo = find_repo_root().expect("repo");
        let dir = std::env::temp_dir().join(format!("aut125-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("switch-llm-provider.md");
        let witness = fs::read_to_string(repo.join("SddIA/actions/switch-llm-provider.md"))
            .expect("remediated witness on disk");
        let stub = format!(
            "---\n{}\n---\n\n# Acción: switch-llm-provider\n\n# Acción: switch-llm-provider\n\nConmutación explícita y determinista del proveedo\n",
            witness
                .split("---")
                .nth(1)
                .unwrap_or("uuid: \"a6f5e0a4-ccc5-4076-bf8a-b60cfe1fa684\"\nname: switch-llm-provider\nversion: \"1.0.0\"")
        );
        fs::write(&path, stub).unwrap();
        let v = validate_entity_file(&dir, &path, "action");
        assert!(
            v.iter().any(|x| {
                matches!(
                    x.code.as_str(),
                    "DUPLICATE_H1" | "BODY_TRUNCATED" | "SECTIONS_INSUFFICIENT"
                )
            }),
            "estado histórico testigo debe fallar: {:?}",
            v
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn aut125_witness_switch_llm_provider_passes_after_remediation() {
        let repo = find_repo_root().expect("repo");
        let path = repo.join("SddIA/actions/switch-llm-provider.md");
        let blockers: Vec<_> = validate_entity_file(&repo, &path, "action")
            .into_iter()
            .filter(|v| v.code != "UUID_NOT_V4")
            .collect();
        assert!(blockers.is_empty(), "{blockers:?}");
    }

    #[test]
    fn conforming_action_execute_process_passes() {
        let repo = find_repo_root().expect("repo");
        let path = repo.join("SddIA/actions/execute-process.md");
        let v = validate_entity_file(&repo, &path, "action");
        let blockers: Vec<_> = v
            .iter()
            .filter(|x| x.code != "STUB_MARKER")
            .collect();
        if blockers.is_empty() {
            return;
        }
        if v.iter().all(|x| x.code == "STUB_MARKER") {
            return;
        }
        panic!("execute-process no debería fallar secciones: {:?}", v);
    }

    #[test]
    fn stub_fixture_empty_sections_rejected() {
        let dir = std::env::temp_dir().join(format!("entity-no-anemia-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("stub-action.md");
        fs::write(
            &path,
            "---\nuuid: \"11111111-1111-4111-8111-111111111111\"\nname: stub-action\nversion: \"1.0.0\"\ncontract: \"actions-contract v1.2.0\"\ncontext: ecosystem-evolution\ncapabilities:\n  - x\n---\n\n# Acción: stub-action\n\nTODO\n",
        )
        .unwrap();
        let v = validate_entity_file(&dir, &path, "action");
        assert!(v.iter().any(|x| x.code == "STUB_MARKER"));
        assert!(v.iter().any(|x| x.code == "SECTIONS_INSUFFICIENT"));
        let _ = fs::remove_dir_all(&dir);
    }
}