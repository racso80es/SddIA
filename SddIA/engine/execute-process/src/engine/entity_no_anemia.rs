//! Peaje determinista anti-anemia para entidades `.md` de dominio (AUT-125 / Q1).

use crate::core::paths::load_paths_config;
use regex::Regex;
use serde::Serialize;
use serde_yaml::Value as YamlValue;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

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

impl ScanReport {
    fn from_violations(scanned: u64, violations: Vec<Violation>) -> Self {
        Self {
            ok: violations.is_empty(),
            scanned,
            violation_count: violations.len() as u64,
            violations,
        }
    }
}

fn uuid_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$")
            .expect("uuid re")
    })
}

fn uuid_v4_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$")
            .expect("uuid v4 re")
    })
}

fn semver_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^\d+\.\d+\.\d+").expect("semver re"))
}

fn uuid_present_ok(raw: &str) -> (bool, bool) {
    let s = raw.trim().trim_matches(|c| c == '"' || c == '\'');
    (uuid_re().is_match(s), uuid_v4_re().is_match(s))
}

/// Separa frontmatter YAML y cuerpo. Exige `---` como primera línea (BOM tolerado) y una línea
/// de cierre `---`; cualquier otro formato no tiene frontmatter.
pub fn split_frontmatter(raw: &str) -> Option<(String, String)> {
    let raw = raw.strip_prefix('\u{feff}').unwrap_or(raw);
    let rest = raw
        .strip_prefix("---\n")
        .or_else(|| raw.strip_prefix("---\r\n"))?;
    let mut offset = 0usize;
    for line in rest.split_inclusive('\n') {
        if line.trim_end_matches(['\n', '\r']) == "---" {
            let fm = rest[..offset].to_string();
            let body = rest[offset + line.len()..].to_string();
            return Some((fm, body));
        }
        offset += line.len();
    }
    None
}

/// Líneas fuera de bloques de código cercados (```).
fn prose_lines(text: &str) -> impl Iterator<Item = &str> {
    let mut in_fence = false;
    text.lines().filter(move |line| {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            return false;
        }
        !in_fence
    })
}

fn stub_tokens_in_text(text: &str) -> Option<String> {
    for line in prose_lines(text) {
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
    let mut in_fence = false;
    for line in body.lines() {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
        }
        if !in_fence {
            if let Some(rest) = line.strip_prefix("## ") {
                if !current_title.is_empty() {
                    sections.push((current_title.clone(), current_body.trim().to_string()));
                }
                current_title = rest.trim().to_string();
                current_body.clear();
                continue;
            }
            if line.starts_with("# ") {
                continue;
            }
        }
        if !current_title.is_empty() {
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
    let needle = name.to_lowercase();
    prose_lines(body)
        .map(str::trim)
        .filter(|l| l.starts_with("# "))
        .filter(|l| l[2..].trim().to_lowercase().contains(&needle))
        .count()
        >= 2
}

fn is_prose_line(line: &str) -> bool {
    let t = line.trim_start();
    let list_item = t.starts_with("- ")
        || t.starts_with("* ")
        || t.starts_with("+ ")
        || t.split_once(". ")
            .map_or(false, |(n, _)| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit()));
    !(t.is_empty() || t.starts_with('#') || t.starts_with('|') || t.starts_with('>') || list_item)
}

/// Firmas mecánicas de truncado (no estilo): bloque de código sin cerrar, o fichero sin salto
/// final cuya última línea es prosa sin puntuación de cierre. Las forjas escriben siempre `\n`
/// final, así que un corte de escritura deja esa firma; prosa o listas sin punto final con
/// salto de línea no se consideran truncadas.
fn truncated_tail(body: &str) -> bool {
    let fences = body
        .lines()
        .filter(|l| l.trim_start().starts_with("```"))
        .count();
    if fences % 2 == 1 {
        return true;
    }
    if body.ends_with('\n') {
        return false;
    }
    let Some(last) = body.lines().map(str::trim).filter(|l| !l.is_empty()).last() else {
        return false;
    };
    let terminal = last
        .chars()
        .last()
        .map_or(true, |c| ".!?:;)]}`\"'»*_".contains(c));
    is_prose_line(last) && !terminal
}

fn violation(rel: &str, family: &str, code: &str, detail: impl Into<String>) -> Violation {
    Violation {
        path: rel.to_string(),
        family: family.to_string(),
        code: code.to_string(),
        detail: detail.into(),
    }
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
        Err(e) => return vec![violation(&rel, family, "READ_ERROR", e.to_string())],
    };

    let Some((fm_raw, body)) = split_frontmatter(&raw) else {
        return vec![violation(
            &rel,
            family,
            "NO_FRONTMATTER",
            "falta bloque YAML delimitado por --- en la primera línea",
        )];
    };

    let mut viols = Vec::new();
    let fm_yaml = match serde_yaml::from_str::<YamlValue>(&fm_raw) {
        Ok(v @ YamlValue::Mapping(_)) => v,
        Ok(_) => {
            viols.push(violation(&rel, family, "FRONTMATTER_YAML_INVALID", "frontmatter no es un mapa YAML"));
            YamlValue::Null
        }
        Err(e) => {
            viols.push(violation(&rel, family, "FRONTMATTER_YAML_INVALID", e.to_string()));
            YamlValue::Null
        }
    };
    let uuid = fm_yaml.get("uuid").and_then(|v| v.as_str()).unwrap_or("");
    let version = fm_yaml.get("version").and_then(|v| v.as_str()).unwrap_or("");
    let name_fm = fm_yaml.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let (uuid_ok, uuid_v4) = uuid_present_ok(uuid);
    if !uuid_ok {
        viols.push(violation(&rel, family, "UUID_INVALID", format!("uuid inválido: {uuid}")));
    } else if !uuid_v4 {
        viols.push(violation(&rel, family, "UUID_NOT_V4", format!("uuid presente pero no v4: {uuid}")));
    }
    if !semver_re().is_match(version.trim()) {
        viols.push(violation(&rel, family, "VERSION_INVALID", format!("version inválida: {version}")));
    }
    if !name_fm.is_empty() && name_fm != name && family != "codex" {
        viols.push(violation(
            &rel,
            family,
            "NAME_FILENAME_MISMATCH",
            format!("name={name_fm} filename={name}"),
        ));
    }

    if body.trim().is_empty() {
        viols.push(violation(&rel, family, "BODY_EMPTY", "cuerpo vacío tras frontmatter"));
        return viols;
    }

    if duplicate_h1(&body, &name) {
        viols.push(violation(&rel, family, "DUPLICATE_H1", "H1 duplicado de título"));
    }
    if truncated_tail(&body) {
        viols.push(violation(&rel, family, "BODY_TRUNCATED", "cierre de cuerpo truncado"));
    }
    if let Some(tok) = stub_tokens_in_text(&body) {
        viols.push(violation(&rel, family, "STUB_MARKER", format!("marcador stub: {tok}")));
    }

    let sections = parse_sections(&body);
    if sections.len() < 2 {
        viols.push(violation(
            &rel,
            family,
            "SECTIONS_INSUFFICIENT",
            format!("secciones ## < 2 (tiene {})", sections.len()),
        ));
    }
    for (title, content) in &sections {
        if content.chars().all(char::is_whitespace) {
            viols.push(violation(&rel, family, "SECTION_EMPTY", format!("sección vacía: ## {title}")));
        }
    }

    if let Some(rule) = rule_for(family) {
        let matching = |kws: &[&str]| -> Vec<usize> {
            sections
                .iter()
                .enumerate()
                .filter(|(_, (t, c))| !c.trim().is_empty() && heading_matches_any(t, kws))
                .map(|(i, _)| i)
                .collect()
        };
        let purpose = matching(rule.purpose_kw);
        let second = matching(rule.second_kw);
        if purpose.is_empty() {
            viols.push(violation(
                &rel,
                family,
                "CONTRACT_SECTION_MISSING",
                format!("falta sección ## de propósito (heading con: {})", rule.purpose_kw.join(" | ")),
            ));
        }
        let distinct_pair = second
            .iter()
            .any(|s| purpose.iter().any(|p| p != s) || purpose.is_empty());
        if !distinct_pair {
            viols.push(violation(
                &rel,
                family,
                "CONTRACT_SECTION_MISSING",
                format!(
                    "falta sección ## operativa distinta del propósito (heading con: {})",
                    rule.second_kw.join(" | ")
                ),
            ));
        }
    }

    viols
}

fn walk_md_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let rd = fs::read_dir(dir).map_err(|e| format!("read_dir {}: {e}", dir.display()))?;
    for entry in rd {
        let p = entry
            .map_err(|e| format!("read_dir {}: {e}", dir.display()))?
            .path();
        if p.is_dir() {
            walk_md_files(&p, out)?;
            continue;
        }
        if p.extension().and_then(|x| x.to_str()) != Some("md") {
            continue;
        }
        let fname = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if fname == "index.md" || fname.ends_with("-contract.md") {
            continue;
        }
        out.push(p);
    }
    Ok(())
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
        ("process", "process"),
    ];
    for (family, key) in pairs {
        if let Some(d) = dirs.get(key).and_then(|x| x.as_str()) {
            v.push((family, d.trim_end_matches('/').to_string()));
        }
    }
    for (family, key) in [("process", "process_domain_roots"), ("event", "events_domain_roots")] {
        if let Some(arr) = dirs.get(key).and_then(|x| x.as_array()) {
            for s in arr.iter().filter_map(|i| i.as_str()) {
                v.push((family, s.trim_end_matches('/').to_string()));
            }
        }
    }
    v
}

fn family_rank(fam: &str) -> u8 {
    match fam {
        "event" | "process" => 3,
        "action" | "agent" | "skill" | "tool" | "suite" | "daemon" | "norm" => 2,
        _ => 1,
    }
}

/// Familia de una ruta relativa: entre los directorios que la contienen (límite `/`), gana el
/// de mayor rango y, a igual rango, el prefijo más largo. Procesos y eventos de dominio bajo
/// `library/codexes/` se clasifican como `process` / `event`, no como `codex`.
pub fn classify_entity_path(rel: &str, dirs: &[(&'static str, String)]) -> Option<&'static str> {
    dirs.iter()
        .filter(|(_, d)| rel.starts_with(&format!("{d}/")))
        .max_by_key(|(f, d)| (family_rank(f), d.len()))
        .map(|(f, _)| *f)
}

fn is_candidate(rel: &str) -> bool {
    rel.ends_with(".md") && !rel.ends_with("-contract.md") && !rel.ends_with("/index.md")
}

pub fn scan_all(repo: &Path) -> Result<ScanReport, String> {
    let cfg = load_paths_config(repo)?;
    let dirs = genome_entity_dirs(&cfg);
    let mut by_path: HashMap<String, (&'static str, PathBuf)> = HashMap::new();
    for (_, dir) in &dirs {
        let root = repo.join(dir);
        if !root.is_dir() {
            continue;
        }
        let mut paths = Vec::new();
        walk_md_files(&root, &mut paths)?;
        for p in paths {
            let rel = p
                .strip_prefix(repo)
                .unwrap_or(&p)
                .to_string_lossy()
                .replace('\\', "/");
            if let Some(family) = classify_entity_path(&rel, &dirs) {
                by_path.insert(rel, (family, p));
            }
        }
    }
    let mut entries: Vec<_> = by_path.into_values().collect();
    entries.sort_by(|a, b| a.1.cmp(&b.1));
    let scanned = entries.len() as u64;
    let violations = entries
        .into_iter()
        .flat_map(|(family, path)| validate_entity_file(repo, &path, family))
        .collect();
    Ok(ScanReport::from_violations(scanned, violations))
}

fn git(repo: &Path, args: &[&str]) -> Result<std::process::Output, String> {
    std::process::Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .map_err(|e| format!("git {}: {e}", args.join(" ")))
}

/// Entidades `.md` de genoma añadidas o modificadas en la rama respecto a `base_ref`
/// (`base_ref...HEAD`, desde el merge-base). Las borradas no se validan.
pub fn scan_changed(repo: &Path, base_ref: &str) -> Result<ScanReport, String> {
    let verify = git(repo, &["rev-parse", "--verify", "--quiet", &format!("{base_ref}^{{commit}}")])?;
    if !verify.status.success() {
        return Err(format!("base ref no resoluble: {base_ref}"));
    }
    let range = format!("{base_ref}...HEAD");
    let out = git(repo, &["diff", "--name-only", "--diff-filter=d", &range, "--"])?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    let cfg = load_paths_config(repo)?;
    let dirs = genome_entity_dirs(&cfg);
    let mut violations = Vec::new();
    let mut scanned = 0u64;
    for rel in String::from_utf8_lossy(&out.stdout).lines().map(str::trim) {
        if !is_candidate(rel) {
            continue;
        }
        let Some(family) = classify_entity_path(rel, &dirs) else {
            continue;
        };
        let path = repo.join(rel);
        if !path.is_file() {
            continue;
        }
        scanned += 1;
        violations.extend(validate_entity_file(repo, &path, family));
    }
    Ok(ScanReport::from_violations(scanned, violations))
}

/// Ref base por defecto: `origin/{default_branch}` según `.SddIA/project.md` de la instancia;
/// `origin/main` si el manifiesto no existe o no declara `default_branch`.
pub fn default_base_ref(repo: &Path) -> String {
    let branch = fs::read_to_string(repo.join(".SddIA/project.md"))
        .ok()
        .and_then(|raw| split_frontmatter(&raw))
        .and_then(|(fm, _)| serde_yaml::from_str::<YamlValue>(&fm).ok())
        .and_then(|v| v.get("default_branch").and_then(|b| b.as_str()).map(str::to_string))
        .filter(|b| !b.trim().is_empty())
        .unwrap_or_else(|| "main".to_string());
    format!("origin/{}", branch.trim())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::repo::find_repo_root;

    const FM: &str = "---\nuuid: \"11111111-1111-4111-8111-111111111111\"\nname: NAME\nversion: \"1.0.0\"\n---\n";

    fn fixture(dir: &Path, name: &str, body: &str) -> PathBuf {
        let path = dir.join(format!("{name}.md"));
        fs::write(&path, format!("{}{}", FM.replace("NAME", name), body)).unwrap();
        path
    }

    fn codes(v: &[Violation]) -> Vec<&str> {
        v.iter().map(|x| x.code.as_str()).collect()
    }

    #[test]
    fn aut125_witness_truncated_switch_llm_fixture_rejected() {
        let tmp = tempfile::tempdir().unwrap();
        let path = fixture(
            tmp.path(),
            "switch-llm-provider",
            "\n# Acción: switch-llm-provider\n\n# Acción: switch-llm-provider\n\nConmutación explícita y determinista del proveedo\n",
        );
        let v = validate_entity_file(tmp.path(), &path, "action");
        let c = codes(&v);
        assert!(c.contains(&"DUPLICATE_H1"), "{v:?}");
        assert!(c.contains(&"SECTIONS_INSUFFICIENT"), "{v:?}");
        assert!(c.contains(&"CONTRACT_SECTION_MISSING"), "{v:?}");
    }

    #[test]
    fn aut125_witness_switch_llm_provider_passes_after_remediation() {
        let repo = find_repo_root().expect("repo");
        let path = repo.join("SddIA/actions/switch-llm-provider.md");
        let v = validate_entity_file(&repo, &path, "action");
        assert!(v.is_empty(), "{v:?}");
    }

    #[test]
    fn conforming_action_fixture_passes() {
        let tmp = tempfile::tempdir().unwrap();
        let path = fixture(
            tmp.path(),
            "demo-action",
            "\n## Propósito\n\nHace algo concreto.\n\n## Orquestación\n\n- paso uno\n- paso dos\n",
        );
        let v = validate_entity_file(tmp.path(), &path, "action");
        assert!(v.is_empty(), "{v:?}");
    }

    #[test]
    fn stub_fixture_empty_sections_rejected() {
        let tmp = tempfile::tempdir().unwrap();
        let path = fixture(tmp.path(), "stub-action", "\n# Acción: stub-action\n\nTODO\n");
        let v = validate_entity_file(tmp.path(), &path, "action");
        let c = codes(&v);
        assert!(c.contains(&"STUB_MARKER"));
        assert!(c.contains(&"SECTIONS_INSUFFICIENT"));
    }

    #[test]
    fn purpose_kw_enforced_even_with_long_first_section() {
        let tmp = tempfile::tempdir().unwrap();
        let path = fixture(
            tmp.path(),
            "no-purpose",
            "\n## Contexto\n\nUn bloque largo de texto que supera de sobra los cuarenta caracteres sin ser propósito.\n\n## Orquestación\n\nPasos.\n",
        );
        let v = validate_entity_file(tmp.path(), &path, "action");
        assert!(
            v.iter().any(|x| x.code == "CONTRACT_SECTION_MISSING" && x.detail.contains("propósito")),
            "{v:?}"
        );
    }

    #[test]
    fn second_kw_enforced_with_two_sections() {
        let tmp = tempfile::tempdir().unwrap();
        let path = fixture(
            tmp.path(),
            "no-second",
            "\n## Propósito\n\nHace algo.\n\n## Notas\n\nComentarios.\n",
        );
        let v = validate_entity_file(tmp.path(), &path, "action");
        assert!(
            v.iter().any(|x| x.code == "CONTRACT_SECTION_MISSING" && x.detail.contains("operativa")),
            "{v:?}"
        );
    }

    #[test]
    fn second_kw_must_be_distinct_section_from_purpose() {
        let tmp = tempfile::tempdir().unwrap();
        let path = fixture(
            tmp.path(),
            "codex-x",
            "\n## Dominio\n\nTexto.\n\n## Notas\n\nMás.\n",
        );
        let v = validate_entity_file(tmp.path(), &path, "codex");
        assert!(
            v.iter().any(|x| x.code == "CONTRACT_SECTION_MISSING" && x.detail.contains("operativa")),
            "{v:?}"
        );
    }

    #[test]
    fn truncated_tail_ignores_lists_and_prose_without_period() {
        assert!(!truncated_tail("## A\n\n- uno\n- dos sin punto\n"));
        assert!(!truncated_tail("## A\n\nProsa válida sin punto final\n"));
        assert!(!truncated_tail("## A\n\n1. paso numerado sin punto"));
        assert!(!truncated_tail("## A\n\n| a | b |"));
    }

    #[test]
    fn truncated_tail_detects_mechanical_cuts() {
        assert!(truncated_tail("## A\n\nConmutación explícita y determinista del proveedo"));
        assert!(truncated_tail("## A\n\n```rust\nfn main() {}\n"));
        assert!(!truncated_tail("## A\n\nFrase completa."));
    }

    #[test]
    fn split_frontmatter_requires_leading_delimiter() {
        assert!(split_frontmatter("texto\n---\nuuid: x\n---\ncuerpo").is_none());
        assert!(split_frontmatter("---\nuuid: x\nsin cierre\n").is_none());
        let (fm, body) = split_frontmatter("---\nuuid: x\n---\n\n## A\n\nuno --- dos\n").unwrap();
        assert_eq!(fm, "uuid: x\n");
        assert!(body.contains("uno --- dos"));
        assert!(split_frontmatter("\u{feff}---\na: 1\n---\nb").is_some());
    }

    #[test]
    fn frontmatter_yaml_error_is_reported() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("bad-yaml.md");
        fs::write(&path, "---\nuuid: [sin cerrar\n---\n\n## Propósito\n\nX.\n\n## Orquestación\n\nY.\n").unwrap();
        let v = validate_entity_file(tmp.path(), &path, "action");
        assert!(codes(&v).contains(&"FRONTMATTER_YAML_INVALID"), "{v:?}");
    }

    #[test]
    fn missing_leading_delimiter_is_no_frontmatter() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("late-fm.md");
        fs::write(&path, "# Título\n---\nuuid: x\n---\n").unwrap();
        let v = validate_entity_file(tmp.path(), &path, "action");
        assert_eq!(codes(&v), vec!["NO_FRONTMATTER"]);
    }

    fn dirs_fixture() -> Vec<(&'static str, String)> {
        genome_entity_dirs(&serde_json::json!({
            "directories": {
                "actions": "SddIA/actions",
                "events": "SddIA/events",
                "library_codexes": "SddIA/library/codexes/",
                "process": "SddIA/process",
                "process_domain_roots": ["SddIA/library/codexes/codex-a/process"],
                "events_domain_roots": ["SddIA/library/codexes/codex-a/events"]
            }
        }))
    }

    #[test]
    fn classify_prefers_rank_over_config_order() {
        let dirs = dirs_fixture();
        assert_eq!(
            classify_entity_path("SddIA/library/codexes/codex-a/process/p.md", &dirs),
            Some("process")
        );
        assert_eq!(
            classify_entity_path("SddIA/library/codexes/codex-a/events/e.md", &dirs),
            Some("event")
        );
        assert_eq!(classify_entity_path("SddIA/library/codexes/codex-a.md", &dirs), Some("codex"));
    }

    #[test]
    fn classify_respects_directory_boundary() {
        let dirs = dirs_fixture();
        assert_eq!(classify_entity_path("SddIA/actionsX/foo.md", &dirs), None);
        assert_eq!(classify_entity_path("SddIA/actions/foo.md", &dirs), Some("action"));
    }

    fn run_git(dir: &Path, args: &[&str]) {
        let out = std::process::Command::new("git")
            .args(args)
            .current_dir(dir)
            .output()
            .expect("git");
        assert!(out.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
    }

    fn git_fixture_repo() -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join("SddIA/core")).unwrap();
        fs::create_dir_all(root.join("SddIA/actions")).unwrap();
        fs::create_dir_all(root.join("SddIA/actionsX")).unwrap();
        fs::create_dir_all(root.join("SddIA/library/codexes/codex-a/process")).unwrap();
        fs::write(
            root.join("SddIA/core/cumulo.paths.json"),
            serde_json::json!({
                "directories": {
                    "actions": "SddIA/actions",
                    "library_codexes": "SddIA/library/codexes/",
                    "process_domain_roots": ["SddIA/library/codexes/codex-a/process"]
                }
            })
            .to_string(),
        )
        .unwrap();
        run_git(root, &["init", "-q", "-b", "main"]);
        run_git(root, &["config", "user.email", "t@example.invalid"]);
        run_git(root, &["config", "user.name", "t"]);
        run_git(root, &["add", "."]);
        run_git(root, &["commit", "-q", "--allow-empty", "-m", "base"]);
        tmp
    }

    #[test]
    fn scan_changed_uses_merge_base_and_rank() {
        let tmp = git_fixture_repo();
        let root = tmp.path();
        run_git(root, &["checkout", "-q", "-b", "feature"]);
        fixture(&root.join("SddIA/library/codexes/codex-a/process"), "proc-a", "\n## Notas\n\nX.\n");
        fixture(&root.join("SddIA/actionsX"), "outside", "\nTODO\n");
        run_git(root, &["add", "."]);
        run_git(root, &["commit", "-q", "-m", "feature"]);
        run_git(root, &["checkout", "-q", "main"]);
        fixture(&root.join("SddIA/actions"), "main-only", "\nTODO\n");
        run_git(root, &["add", "."]);
        run_git(root, &["commit", "-q", "-m", "main advances"]);
        run_git(root, &["checkout", "-q", "feature"]);

        let rep = scan_changed(root, "main").unwrap();
        assert_eq!(rep.scanned, 1, "{rep:?}");
        assert!(rep.violations.iter().all(|v| v.family == "process"), "{rep:?}");
        assert!(rep.violations.iter().all(|v| !v.path.contains("main-only")));
    }

    #[test]
    fn scan_changed_skips_deleted_files() {
        let tmp = git_fixture_repo();
        let root = tmp.path();
        fixture(&root.join("SddIA/actions"), "doomed", "\nTODO\n");
        run_git(root, &["add", "."]);
        run_git(root, &["commit", "-q", "-m", "add doomed"]);
        run_git(root, &["checkout", "-q", "-b", "feature"]);
        run_git(root, &["rm", "-q", "SddIA/actions/doomed.md"]);
        run_git(root, &["commit", "-q", "-m", "rm"]);
        let rep = scan_changed(root, "main").unwrap();
        assert_eq!(rep.scanned, 0);
        assert!(rep.ok);
    }

    #[test]
    fn scan_changed_rejects_unknown_base() {
        let tmp = git_fixture_repo();
        let err = scan_changed(tmp.path(), "origin/no-existe").unwrap_err();
        assert!(err.contains("no resoluble"), "{err}");
    }

    #[test]
    fn default_base_ref_reads_instance_manifest() {
        let tmp = tempfile::tempdir().unwrap();
        assert_eq!(default_base_ref(tmp.path()), "origin/main");
        fs::create_dir_all(tmp.path().join(".SddIA")).unwrap();
        fs::write(tmp.path().join(".SddIA/project.md"), "---\ndefault_branch: trunk\n---\n").unwrap();
        assert_eq!(default_base_ref(tmp.path()), "origin/trunk");
    }
}
