mod gate_evolution;
mod verify_hooks;
mod eda_e2e_lab;
mod iota_ci_smoke;
mod lab_teardown;
mod migrate_evolution_history;
mod resolve;
mod validate_evolution_contract;
mod verify_compiled_capsules;
mod verify_tools_index;
mod wasi_ci_smoke;

use execute_process::core::repo::find_repo_root;
use execute_process::engine::eda_coverage;
use execute_process::engine::verify_process_integrity;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn usage() -> &'static str {
    "sddia-qa — aduana QA SddIA (sin Python)\n\
Comandos:\n\
  verify-tools-index\n\
  verify-process-integrity\n\
  verify-compiled-capsules [--json]\n\
  audit-eda-coverage --scan [--json]\n\
  recalc-process-hash-signatures [--write] [--files STEM ...]\n\
  run-iota-ci-smoke [--simulate] [--require-physical] [--json]\n\
  run-eda-e2e-lab [--entity-class CLASS] [--entity-name NAME] [--json]\n\
  run-wasi-ci-smoke [--skip-e2e] [--json]\n\
  validate-evolution-contract [--json] [--universe audit-cut|official] [--audit-ref PATH] [--manifest PATH]\n\
  migrate-evolution-history manifest|apply|verify|reindex [--json] [--write PATH] [--manifest PATH] [--lote L1|L2|L3|L4] [--dry-run]\n\
  gate-evolution [--json] [--range|--all] [--if-touched] [--sync-base] [--require-synced-base]\n\
  verify-hooks [--json] [--fix]\n\
  evolution-rehash --id <uuid> [--json] [--dry-run]\n\
  evolution-register [--json] [--dry-run]\n"
}

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|a| a == flag)
}

fn flag_value<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
}

fn run_recalc(repo: &Path, args: &[String]) -> i32 {
    use execute_process::core::parser::load_frontmatter_yaml;
    use execute_process::forges::common::{refresh_process_hash, sha256_phases_integrity};
    use serde_json::Value;
    use serde_yaml::Value as YamlValue;

    let write = has_flag(args, "--write");
    let mut stems: Option<Vec<String>> = None;
    if let Some(pos) = args.iter().position(|a| a == "--files") {
        stems = Some(args[(pos + 1)..].to_vec());
    }
    let process_dir = repo.join("SddIA/process");
    let skip = ["process-contract", "index"];
    let mut changed = 0usize;
    let mut entries: Vec<_> = fs::read_dir(&process_dir)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("md"))
                .collect()
        })
        .unwrap_or_default();
    entries.sort();
    for md in entries {
        let stem = md.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        if skip.contains(&stem) {
            continue;
        }
        if let Some(ref list) = stems {
            if !list.iter().any(|s| s == stem) {
                continue;
            }
        }
        let fm = match load_frontmatter_yaml(&md) {
            Ok(f) if !f.is_empty() => f,
            _ => continue,
        };
        let hs = fm
            .get("hash_signature")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if !hs.starts_with("sha256:") {
            continue;
        }
        let old = hs.split(':').nth(1).unwrap_or("");
        let phases: Value = match fm.get("phases") {
            Some(YamlValue::Sequence(seq)) => Value::Array(
                seq.iter()
                    .filter_map(|v| serde_json::to_value(v).ok())
                    .collect(),
            ),
            _ => continue,
        };
        let computed_full = sha256_phases_integrity(&phases);
        let new = computed_full.strip_prefix("sha256:").unwrap_or(&computed_full);
        if old == new {
            continue;
        }
        if write {
            if let Err(e) = refresh_process_hash(&md) {
                eprintln!("{}: {e}", md.display());
                return 1;
            }
        }
        eprintln!(
            "{}: {} -> {}",
            md.file_name().unwrap_or_default().to_string_lossy(),
            &old[..old.len().min(16)],
            &new[..new.len().min(16)]
        );
        changed += 1;
    }
    if changed == 0 {
        println!("recalc-process-hash-signatures: no changes");
    }
    0
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        let _ = writeln!(std::io::stderr(), "{}", usage());
        std::process::exit(if args.is_empty() { 1 } else { 0 });
    }

    let cmd = args[0].as_str();
    let rest = &args[1..];

    let repo = match find_repo_root() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    let code = match cmd {
        "verify-tools-index" => verify_tools_index::run(&repo),
        "verify-process-integrity" => verify_process_integrity::run_cli(&repo),
        "verify-compiled-capsules" => {
            verify_compiled_capsules::run(&repo, has_flag(rest, "--json"))
        }
        "audit-eda-coverage" => {
            if !has_flag(rest, "--scan") {
                eprintln!("audit-eda-coverage requiere --scan");
                1
            } else {
                match eda_coverage::scan_orphans(&repo) {
                    Ok(report) => {
                        if has_flag(rest, "--json") {
                            println!("{}", serde_json::to_string(&report).unwrap_or_default());
                        } else {
                            println!(
                                "orphan_count={}",
                                report.get("orphan_count").and_then(|v| v.as_u64()).unwrap_or(0)
                            );
                        }
                        0
                    }
                    Err(e) => {
                        eprintln!("{e}");
                        1
                    }
                }
            }
        }
        "recalc-process-hash-signatures" => run_recalc(&repo, rest),
        "run-iota-ci-smoke" => iota_ci_smoke::run(&repo, rest),
        "run-eda-e2e-lab" => eda_e2e_lab::run(&repo, rest),
        "run-wasi-ci-smoke" => wasi_ci_smoke::run(&repo, rest),
        "validate-evolution-contract" => validate_evolution_contract::run(&repo, rest),
        "migrate-evolution-history" => migrate_evolution_history::run(&repo, rest),
        "gate-evolution" => gate_evolution::run_gate(&repo, rest),
        "verify-hooks" => verify_hooks::run(
            &repo,
            has_flag(rest, "--json"),
            has_flag(rest, "--fix"),
        ),
        "evolution-rehash" => gate_evolution::run_rehash(&repo, rest),
        "evolution-register" => gate_evolution::run_mutate(&repo, rest),
        other => {
            eprintln!("comando desconocido: {other}\n{}", usage());
            1
        }
    };
    std::process::exit(code);
}
