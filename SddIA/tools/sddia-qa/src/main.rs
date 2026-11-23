mod gate_allowlist_ratchet;
mod gate_evolution;
mod gate_event_manager_port;
mod gate_eda_unit_no_real_backend;
mod gate_prompt_invariants;
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
mod workspace_prune;
mod reseal_diff_validacion;
mod verify_diff_digest;
mod verify_domain_subscription_parity;
mod merge_thermo_timings;
mod linear_direct_cycle_e2e_lab;
mod linear_done_gate_both_e2e_lab;
mod verify_capability_contracts;

use execute_process::core::repo::find_repo_root;
use execute_process::engine::done_gate;
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
  verify-capability-contracts [--json]\n\
  verify-process-integrity\n\
  verify-compiled-capsules [--json]\n\
  audit-eda-coverage --scan [--json]\n\
  recalc-process-hash-signatures [--write] [--files STEM ...]\n\
  run-iota-ci-smoke [--simulate] [--require-physical] [--json]\n\
  run-eda-e2e-lab [--entity-class CLASS] [--entity-name NAME] [--json]\n\
  run-linear-direct-cycle-e2e-lab [--json]\n\
  run-linear-done-gate-both-e2e-lab [--json]\n\
  run-wasi-ci-smoke [--skip-e2e] [--json]\n\
  validate-evolution-contract [--json] [--universe audit-cut|official] [--audit-ref PATH] [--manifest PATH]\n\
  migrate-evolution-history manifest|apply|verify|reindex [--json] [--write PATH] [--manifest PATH] [--lote L1|L2|L3|L4] [--dry-run]\n\
  gate-evolution [--json] [--range|--all] [--if-touched] [--sync-base] [--require-synced-base]\n\
  gate-event-manager-port [--json]\n\
  gate-eda-unit-no-real-backend [--json]\n\
  gate-allowlist-ratchet [--json] [--base REF]\n\
  verify-hooks [--json] [--fix]\n\
  workspace-prune --empty [--json]\n\
  evolution-rehash --id <uuid> [--json] [--dry-run]\n\
  evolution-register [--json] [--dry-run] [--request-file <path>] (lee JSON de --request-file, SDDIA_CAPSULE_REQUEST o stdin no-TTY)\n\
  verify-domain-subscription-parity [--json]\n\
  merge-thermo-timings-report [--json]\n\
  verify-tracker-done-pre-push --persist-ref PATH\n\
  verify-diff-digest [--json] [--head SHA] [--target-branch NAME]\n\
  reseal-diff-validacion [--json] --persist-ref PATH [--branch NAME] [--head SHA] [--target-branch NAME]\n"
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

fn recalc_process_hash_signatures(
    repo: &Path,
    write: bool,
    stems: Option<&[String]>,
) -> Result<usize, String> {
    use execute_process::core::parser::load_frontmatter_yaml;
    use execute_process::core::resolver::process_search_roots;
    use execute_process::forges::common::{refresh_process_hash, sha256_phases_integrity};
    use serde_json::Value;
    use serde_yaml::Value as YamlValue;

    let roots = process_search_roots(repo)?;
    let skip = ["process-contract", "index"];
    let mut changed = 0usize;
    let mut entries: Vec<std::path::PathBuf> = Vec::new();
    for dir in roots {
        if !dir.is_dir() {
            continue;
        }
        if let Ok(rd) = fs::read_dir(&dir) {
            for e in rd.flatten() {
                let p = e.path();
                if p.extension().and_then(|x| x.to_str()) == Some("md") {
                    entries.push(p);
                }
            }
        }
    }
    entries.sort();
    for md in entries {
        let stem = md.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        if skip.contains(&stem) {
            continue;
        }
        if let Some(list) = stems {
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
            refresh_process_hash(&md)?;
        }
        let rel = md
            .strip_prefix(repo)
            .unwrap_or(&md)
            .to_string_lossy()
            .replace('\\', "/");
        eprintln!(
            "{rel}: {} -> {}",
            &old[..old.len().min(16)],
            &new[..new.len().min(16)]
        );
        changed += 1;
    }
    Ok(changed)
}

fn run_recalc(repo: &Path, args: &[String]) -> i32 {
    let write = has_flag(args, "--write");
    let mut stems: Option<Vec<String>> = None;
    if let Some(pos) = args.iter().position(|a| a == "--files") {
        stems = Some(args[(pos + 1)..].to_vec());
    }
    match recalc_process_hash_signatures(repo, write, stems.as_deref()) {
        Ok(0) => {
            println!("recalc-process-hash-signatures: no changes");
            0
        }
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{e}");
            1
        }
    }
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
        "verify-capability-contracts" => {
            verify_capability_contracts::run(&repo, has_flag(rest, "--json"))
        }
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
        "run-linear-direct-cycle-e2e-lab" => linear_direct_cycle_e2e_lab::run(&repo, rest),
        "run-linear-done-gate-both-e2e-lab" => linear_done_gate_both_e2e_lab::run(&repo, rest),
        "run-wasi-ci-smoke" => wasi_ci_smoke::run(&repo, rest),
        "validate-evolution-contract" => validate_evolution_contract::run(&repo, rest),
        "migrate-evolution-history" => migrate_evolution_history::run(&repo, rest),
        "gate-evolution" => gate_evolution::run_gate(&repo, rest),
        "gate-event-manager-port" => {
            gate_event_manager_port::run(&repo, has_flag(rest, "--json"))
        }
        "gate-eda-unit-no-real-backend" => {
            gate_eda_unit_no_real_backend::run(&repo, has_flag(rest, "--json"))
        }
        "gate-allowlist-ratchet" => gate_allowlist_ratchet::run(
            &repo,
            has_flag(rest, "--json"),
            flag_value(rest, "--base"),
        ),
        "verify-hooks" => verify_hooks::run(
            &repo,
            has_flag(rest, "--json"),
            has_flag(rest, "--fix"),
        ),
        "workspace-prune" => {
            if !has_flag(rest, "--empty") {
                eprintln!("workspace-prune requiere --empty");
                1
            } else {
                workspace_prune::run(&repo, has_flag(rest, "--json"))
            }
        }
        "evolution-rehash" => gate_evolution::run_rehash(&repo, rest),
        "evolution-register" => gate_evolution::run_mutate(&repo, rest),
        "verify-domain-subscription-parity" => {
            verify_domain_subscription_parity::run(&repo, has_flag(rest, "--json"))
        }
        "merge-thermo-timings-report" => {
            merge_thermo_timings::run(&repo, has_flag(rest, "--json"))
        }
        "verify-tracker-done-pre-push" => {
            let pref = flag_value(rest, "--persist-ref");
            match pref {
                Some(p) => match done_gate::pre_push_check(&repo, Some(p)) {
                    Ok(()) => 0,
                    Err(e) => {
                        eprintln!("{e}");
                        2
                    }
                },
                None => {
                    eprintln!("verify-tracker-done-pre-push requiere --persist-ref");
                    1
                }
            }
        }
        "verify-diff-digest" => verify_diff_digest::run(&repo, rest),
        "reseal-diff-validacion" => reseal_diff_validacion::run(&repo, rest),
        other => {
            eprintln!("comando desconocido: {other}\n{}", usage());
            1
        }
    };
    std::process::exit(code);
}

#[cfg(test)]
mod tests {
    use super::recalc_process_hash_signatures;
    use execute_process::core::parser::load_frontmatter_yaml;
    use std::fs;

    #[test]
    fn recalc_covers_core_and_codex_process_roots() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::create_dir_all(repo.join("SddIA/process")).unwrap();
        fs::create_dir_all(repo.join("SddIA/library/codexes/codex-se/process")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"directories":{"process":"SddIA/process","process_domain_roots":["SddIA/library/codexes/codex-se/process"]}}"#,
        )
        .unwrap();

        let core_md = repo.join("SddIA/process/core-proc.md");
        let codex_md = repo.join("SddIA/library/codexes/codex-se/process/codex-proc.md");
        let sample = "---\nuuid: \"u1\"\nname: p\nversion: \"1.0.0\"\nhash_signature: sha256:0000000000000000\nphases:\n- name: F1\n  intent: algo\n---\n\n# p\n";
        fs::write(&core_md, sample).unwrap();
        fs::write(&codex_md, sample).unwrap();

        let changed = recalc_process_hash_signatures(repo, true, None).expect("recalc");
        assert_eq!(changed, 2);

        let core_fm = load_frontmatter_yaml(&core_md).unwrap();
        let codex_fm = load_frontmatter_yaml(&codex_md).unwrap();
        assert_ne!(
            core_fm.get("hash_signature").and_then(|v| v.as_str()),
            Some("sha256:0000000000000000")
        );
        assert_ne!(
            codex_fm.get("hash_signature").and_then(|v| v.as_str()),
            Some("sha256:0000000000000000")
        );

        let second = recalc_process_hash_signatures(repo, true, None).expect("idempotent");
        assert_eq!(second, 0);
    }
}
