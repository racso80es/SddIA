//! Sellado operativo de anclas de cápsula (genoma `source_sha256` + testigo ELF).

use super::capsule_paths::{
    compute_capsule_source_digest, load_compiled_capsule_roots, locate_capsule_crate_dir,
    resolve_capsule_genome, write_capsule_witness,
};
use crate::envelope::OrchestratorEnvelope;
use crate::forges::common::patch_genome_source_sha256;
use serde_json::{json, Value};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct CapsuleInventoryEntry {
    pub name: String,
    pub genome_path: Option<PathBuf>,
    pub crate_dir: Option<PathBuf>,
}

pub fn inventory_indexed_capsules(repo: &Path) -> Vec<CapsuleInventoryEntry> {
    let mut names: Vec<String> = Vec::new();
    for class in ["tools", "skills", "daemons"] {
        let dir = repo.join("SddIA").join(class);
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for ent in entries.flatten() {
            let p = ent.path();
            if !p.is_file() {
                continue;
            }
            if p.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }
            let Some(stem) = p.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            if stem == "index" {
                continue;
            }
            names.push(stem.to_string());
        }
    }
    for name in ["execute-process", "kalma2-bridge"] {
        names.push(name.to_string());
    }
    names.sort();
    names.dedup();
    names
        .into_iter()
        .map(|name| CapsuleInventoryEntry {
            genome_path: resolve_capsule_genome(repo, &name),
            crate_dir: locate_capsule_crate_dir(repo, &name),
            name,
        })
        .filter(|e| e.crate_dir.is_some())
        .collect()
}

fn resolve_elf(repo: &Path, name: &str, profile: &str) -> Option<PathBuf> {
    let roots = load_compiled_capsule_roots(repo).ok()?;
    let elf = roots.native_root.join(profile).join(name);
    if elf.is_file() {
        Some(elf)
    } else {
        None
    }
}

fn bool_input(v: &Value, key: &str, default: bool) -> bool {
    v.get(key)
        .and_then(|x| x.as_bool())
        .unwrap_or(default)
}

fn str_input(v: &Value, key: &str, default: &str) -> String {
    v.get(key)
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(default)
        .to_string()
}

pub fn seal_one(
    repo: &Path,
    name: &str,
    profile: &str,
    dry_run: bool,
    write_genome: bool,
    write_witness: bool,
) -> Result<Value, String> {
    let digest = compute_capsule_source_digest(repo, name)?;
    let genome = resolve_capsule_genome(repo, name)
        .ok_or_else(|| format!("genome ausente para '{name}'"))?;
    let elf = resolve_elf(repo, name, profile)
        .ok_or_else(|| format!("ELF ausente: {name} (profile={profile})"))?;

    let mut out = json!({
        "name": name,
        "profile": profile,
        "source_sha256": digest,
        "elf_path": elf.strip_prefix(repo).unwrap_or(&elf).to_string_lossy().replace('\\', "/"),
        "genome_path": genome.strip_prefix(repo).unwrap_or(&genome).to_string_lossy().replace('\\', "/"),
        "dry_run": dry_run,
    });

    if dry_run {
        out["status"] = json!("dry-run");
        return Ok(out);
    }

    if write_genome {
        patch_genome_source_sha256(&genome, &digest)?;
        out["genome_patched"] = json!(true);
    }
    if write_witness {
        write_capsule_witness(&elf, &digest)?;
        out["witness_written"] = json!(true);
    }
    out["status"] = json!("sealed");
    Ok(out)
}

pub fn run_batch(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let profile = str_input(inputs, "profile", "release");
    let dry_run = bool_input(inputs, "dry_run", false);
    let write_genome = bool_input(inputs, "write_genome", true);
    let write_witness = bool_input(inputs, "write_witness", true);
    let only: Option<Vec<String>> = inputs.get("names").and_then(|v| {
        v.as_array().map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect()
        })
    });

    let inventory = inventory_indexed_capsules(repo);
    let mut sealed = Vec::new();
    let mut errors = Vec::new();

    for entry in &inventory {
        if let Some(ref names) = only {
            if !names.iter().any(|n| n == &entry.name) {
                continue;
            }
        }
        match seal_one(
            repo,
            &entry.name,
            &profile,
            dry_run,
            write_genome,
            write_witness,
        ) {
            Ok(row) => sealed.push(row),
            Err(e) => errors.push(json!({"name": entry.name, "error": e})),
        }
    }

    Ok(json!({
        "profile": profile,
        "dry_run": dry_run,
        "sealed": sealed,
        "errors": errors,
        "count_ok": sealed.len(),
        "count_err": errors.len(),
    }))
}

pub fn run_entity_anchor(repo: &Path, inputs: &Value) -> Result<OrchestratorEnvelope, String> {
    let class = inputs
        .get("entity_class")
        .and_then(|v| v.as_str())
        .ok_or("entity_class requerido para seal-anchor")?;
    if !matches!(class, "tool" | "skill" | "daemon") {
        return Err(format!("seal-anchor no aplica a entity_class={class}"));
    }
    let name = inputs
        .get("entity_name")
        .and_then(|v| v.as_str())
        .ok_or("entity_name requerido")?;
    let seed = inputs.get("semantic_seed").cloned().unwrap_or(json!({}));
    let profile = seed
        .get("profile")
        .and_then(|v| v.as_str())
        .unwrap_or("release");
    let dry_run = seed.get("dry_run").and_then(|v| v.as_bool()).unwrap_or(false);
    let write_genome = seed
        .get("write_genome")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let write_witness = seed
        .get("write_witness")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let row = seal_one(
        repo,
        name,
        profile,
        dry_run,
        write_genome,
        write_witness,
    )?;

    Ok(OrchestratorEnvelope {
        success: true,
        status_code: 0,
        data: Some(json!({
            "process_name": "entity-manager",
            "lifecycle_operation": "seal-anchor",
            "entity_class": class,
            "entity_name": name,
            "seal": row,
        })),
        error: None,
        execution_report: None,
        exit_code: 0,
    })
}

pub fn run_cli(repo: &Path, inputs: &Value) -> i32 {
    match run_batch(repo, inputs) {
        Ok(data) => {
            println!(
                "{}",
                serde_json::to_string(&data).unwrap_or_else(|_| "{}".into())
            );
            let errs = data.get("count_err").and_then(|v| v.as_u64()).unwrap_or(0);
            if errs > 0 { 1 } else { 0 }
        }
        Err(e) => {
            let _ = writeln!(std::io::stderr(), "{e}");
            1
        }
    }
}
