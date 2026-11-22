use execute_process::core::parser::load_frontmatter_yaml;
use execute_process::core::paths::load_paths_config;
use execute_process::forges::capability_contract::{contract_hash, is_semver, X_SDDIA};
use jsonschema::Validator;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

const AUXILIARY_CONTRACTS: &[&str] = &["di.binding", "llm-registry"];

#[derive(Debug, Clone, PartialEq, Eq)]
struct IndexRow {
    file: String,
    uuid: String,
    name: String,
    version: String,
    hash_signature: String,
}

fn contract_file_stem(contract: &str) -> String {
    if contract == "eda.event_manager" {
        "eda.event_manager.v1".to_string()
    } else {
        contract.to_string()
    }
}

fn parse_index_rows(index_path: &Path) -> Result<BTreeMap<String, IndexRow>, String> {
    let raw = fs::read_to_string(index_path)
        .map_err(|e| format!("no se pudo leer {}: {e}", index_path.display()))?;
    let mut rows = BTreeMap::new();
    let mut in_table = false;
    for line in raw.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('|') {
            if in_table {
                break;
            }
            continue;
        }
        if trimmed.contains("---") {
            in_table = true;
            continue;
        }
        if !in_table {
            continue;
        }
        let cols: Vec<&str> = trimmed
            .split('|')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();
        if cols.len() < 5 {
            return Err(format!("fila malformada en {}: {trimmed}", index_path.display()));
        }
        let file = cols[0].trim_matches('`').to_string();
        let uuid = cols[1].trim_matches('`').to_string();
        let name = cols[2].trim_matches('`').to_string();
        let version = cols[3].trim_matches('`').to_string();
        let hash_signature = cols[4].trim_matches('`').to_string();
        if rows.insert(
            name.clone(),
            IndexRow {
                file,
                uuid,
                name,
                version,
                hash_signature,
            },
        )
        .is_some()
        {
            return Err(format!("fila duplicada en {}", index_path.display()));
        }
    }
    if !in_table {
        return Err(format!("tabla de catálogo ausente en {}", index_path.display()));
    }
    Ok(rows)
}

fn load_taxonomy_catalog(repo: &Path) -> Result<Vec<(String, String)>, String> {
    let path = repo.join("SddIA/library/norms/capability-taxonomy.md");
    let fm = load_frontmatter_yaml(&path)?;
    let catalog = fm
        .get("catalog")
        .and_then(|v| v.as_sequence())
        .ok_or("catalog ausente en capability-taxonomy.md")?;
    let mut out = Vec::with_capacity(catalog.len());
    for item in catalog {
        let Some(c) = item.get("contract").and_then(|v| v.as_str()) else {
            continue;
        };
        let Some(v) = item.get("version").and_then(|v| v.as_str()) else {
            continue;
        };
        out.push((c.to_string(), v.to_string()));
    }
    Ok(out)
}

pub fn verify(repo: &Path) -> Value {
    let mut errors: Vec<String> = Vec::new();
    let mut checked_contracts = 0usize;

    let cfg = match load_paths_config(repo) {
        Ok(c) => c,
        Err(e) => {
            return json!({
                "ok": false,
                "checked_contracts": 0,
                "errors": [format!("cumulo.paths.json: {e}")]
            });
        }
    };

    let rel_dir = cfg
        .pointer("/directories/capability_contracts")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().trim_start_matches("./").trim_end_matches('/'))
        .filter(|s| !s.is_empty());
    let Some(rel_dir) = rel_dir else {
        return json!({
            "ok": false,
            "checked_contracts": 0,
            "errors": ["directories.capability_contracts ausente en cumulo.paths.json"]
        });
    };

    match cfg
        .pointer("/contracts/capability_contracts")
        .and_then(|v| v.as_str())
    {
        Some(p) if !p.trim().is_empty() => {
            if !repo.join(p.trim()).is_file() {
                errors.push(format!("contracts.capability_contracts no existe en disco: {p}"));
            }
        }
        _ => errors.push("contracts.capability_contracts ausente en cumulo.paths.json".into()),
    }

    let dir = repo.join(rel_dir);
    let index_path = dir.join("index.md");
    let index_rows = match parse_index_rows(&index_path) {
        Ok(r) => r,
        Err(e) => {
            errors.push(e);
            BTreeMap::new()
        }
    };

    let mut disk_files = Vec::new();
    match fs::read_dir(&dir) {
        Ok(rd) => {
            for ent in rd.flatten() {
                let p = ent.path();
                if p.extension().and_then(|x| x.to_str()) == Some("json")
                    && p.file_name()
                        .and_then(|n| n.to_str())
                        .is_some_and(|n| n.ends_with(".schema.json"))
                {
                    disk_files.push(p);
                }
            }
        }
        Err(e) => errors.push(format!("no se pudo listar {}: {e}", dir.display())),
    }
    disk_files.sort();

    let mut seen_uuids = BTreeSet::new();
    let mut disk_versions: BTreeMap<String, String> = BTreeMap::new();
    let mut disk_stems = BTreeSet::new();

    for path in &disk_files {
        checked_contracts += 1;
        let fname = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        let stem = fname.trim_end_matches(".schema.json").to_string();
        disk_stems.insert(stem.clone());

        let raw = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                errors.push(format!("{fname}: error de lectura: {e}"));
                continue;
            }
        };
        let schema: Value = match serde_json::from_str(&raw) {
            Ok(v) => v,
            Err(e) => {
                errors.push(format!("{fname}: JSON inválido: {e}"));
                continue;
            }
        };
        if let Err(e) = Validator::new(&schema) {
            errors.push(format!("{fname}: JSON Schema inválido: {e}"));
            continue;
        }
        let Some(xs) = schema.get(X_SDDIA).and_then(|v| v.as_object()) else {
            errors.push(format!("{fname}: falta bloque {X_SDDIA}"));
            continue;
        };
        let uuid = xs.get("uuid").and_then(|v| v.as_str()).unwrap_or("");
        let version = xs.get("version").and_then(|v| v.as_str()).unwrap_or("");
        let hash = xs
            .get("hash_signature")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        match uuid::Uuid::parse_str(uuid) {
            Ok(u) if u.get_version_num() == 4 => {}
            _ => errors.push(format!("{fname}: uuid no es v4: {uuid}")),
        }
        if !seen_uuids.insert(uuid.to_string()) {
            errors.push(format!("{fname}: uuid duplicado: {uuid}"));
        }
        if !is_semver(version) {
            errors.push(format!("{fname}: version no SemVer: {version}"));
        }
        match contract_hash(repo, &schema) {
            Ok(expected) if expected == hash => {}
            Ok(expected) => errors.push(format!(
                "{fname}: hash_signature desalineada ({hash} != {expected})"
            )),
            Err(e) => errors.push(format!("{fname}: {e}")),
        }

        if let Some(row) = index_rows.get(&stem) {
            if row.file != fname
                || row.uuid != uuid
                || row.version != version
                || row.hash_signature != hash
            {
                errors.push(format!(
                    "{fname}: fila en index.md desalineada con x-sddia"
                ));
            }
        } else if !index_rows.is_empty() {
            errors.push(format!("{fname}: ausente en index.md"));
        }

        disk_versions.insert(stem, version.to_string());
    }

    for stem in index_rows.keys() {
        if !disk_stems.contains(stem) {
            errors.push(format!("index.md lista contrato inexistente en disco: {stem}"));
        }
    }

    match load_taxonomy_catalog(repo) {
        Ok(catalog) => {
            let mut in_catalog = BTreeSet::new();
            for (contract, ver) in &catalog {
                let stem = contract_file_stem(contract);
                in_catalog.insert(stem.clone());
                match disk_versions.get(&stem) {
                    Some(v) if v == ver => {}
                    Some(v) => errors.push(format!(
                        "{stem}: x-sddia.version ({v}) != capability-taxonomy ({ver})"
                    )),
                    None => errors.push(format!(
                        "contrato declarado en capability-taxonomy ausente en disco: {stem}.schema.json"
                    )),
                }
            }
            for stem in disk_versions.keys() {
                if !in_catalog.contains(stem) && !AUXILIARY_CONTRACTS.contains(&stem.as_str()) {
                    errors.push(format!(
                        "{stem}.schema.json fuera de capability-taxonomy y no declarado como auxiliar"
                    ));
                }
            }
        }
        Err(e) => errors.push(e),
    }

    json!({
        "ok": errors.is_empty(),
        "checked_contracts": checked_contracts,
        "indexed_rows": index_rows.len(),
        "errors": errors,
    })
}

pub fn run(repo: &Path, json_out: bool) -> i32 {
    let report = verify(repo);
    let ok = report.get("ok").and_then(|v| v.as_bool()).unwrap_or(false);
    if json_out {
        println!("{}", serde_json::to_string_pretty(&report).unwrap_or_default());
    } else if ok {
        let n = report
            .get("checked_contracts")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        println!("verify-capability-contracts: OK ({n} contratos)");
    } else {
        eprintln!("verify-capability-contracts: FAIL");
        if let Some(errs) = report.get("errors").and_then(|v| v.as_array()) {
            for e in errs {
                if let Some(s) = e.as_str() {
                    eprintln!("  - {s}");
                }
            }
        }
    }
    if ok { 0 } else { 1 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use execute_process::core::repo::find_repo_root;

    #[test]
    fn repo_capability_contracts_pass_qa_gate() {
        let repo = find_repo_root().expect("repo root");
        let report = verify(&repo);
        assert_eq!(
            report.get("ok").and_then(|v| v.as_bool()),
            Some(true),
            "report={report}"
        );
        assert_eq!(
            report.get("checked_contracts").and_then(|v| v.as_u64()),
            Some(17)
        );
        assert_eq!(
            report.get("indexed_rows").and_then(|v| v.as_u64()),
            Some(17)
        );
    }
}
