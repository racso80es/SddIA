use std::collections::HashSet;

const SERVER_CONTEXTS: &[&str] = &[
    "filesystem-ops",
    "source-control",
    "system-operations",
];

/// Base fija de `shell-executor` (paridad AC-5; sin `SDDIA_SHELL_EXECUTOR_ALLOWLIST`).
pub fn shell_executor_base_allowlist() -> HashSet<String> {
    let mut base: HashSet<String> = [
        "gh",
        "npm",
        "node",
        "python",
        "python3",
        "pwsh",
        "dotnet",
        "docker",
        "sddia-qa",
        "sddia-run.sh",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    if let Ok(extra) = std::env::var("SDDIA_SHELL_EXECUTOR_ALLOWLIST") {
        for item in extra.split(',') {
            let it = item.trim();
            if !it.is_empty() {
                base.insert(it.to_lowercase());
            }
        }
    }
    base
}

pub fn tool_context(tool_name: &str) -> Option<&'static str> {
    if tool_name.starts_with("fs_") {
        return Some("filesystem-ops");
    }
    if tool_name.starts_with("git_") {
        return Some("source-control");
    }
    if tool_name == "run_check" {
        return Some("system-operations");
    }
    None
}

pub fn cerbero_allow_tool(tool_name: &str) -> bool {
    match tool_context(tool_name) {
        Some(ctx) => SERVER_CONTEXTS.contains(&ctx),
        None => false,
    }
}

pub fn executable_basename(executable: &str) -> String {
    executable
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .rsplit('/')
        .next()
        .unwrap_or(executable)
        .rsplit('\\')
        .next()
        .unwrap_or(executable)
        .to_lowercase()
}

pub fn whitelist_executable(allowed: &HashSet<String>, executable: &str) -> bool {
    let base = executable_basename(executable);
    allowed.contains(&base)
        || allowed.contains(executable)
        || allowed.contains(&executable.to_lowercase())
}

/// Intersección manifiesto × shell-executor (basename, case-insensitive en shell set).
pub fn manifest_shell_intersection(manifest: &HashSet<String>) -> HashSet<String> {
    let shell = shell_executor_base_allowlist();
    let mut out = HashSet::new();
    for ex in manifest {
        let base = executable_basename(ex);
        if shell.contains(&base) {
            out.insert(base);
        }
    }
    out
}

/// Capacidades gobernadas expuestas por `run_check` (AC-1 / AC-2 / AC-3).
fn governed_executable_kind(executable: &str) -> Option<&'static str> {
    match executable_basename(executable).as_str() {
        "sddia-qa" => Some("sddia-qa"),
        "sddia-run.sh" => Some("entity-manager"),
        "git" | "git.exe" => None,
        _ => None,
    }
}

fn validate_entity_manager_argv(arguments: &[String]) -> Result<(), String> {
    let mut saw_entity_manager = false;
    let mut i = 0;
    while i < arguments.len() {
        let arg = arguments[i].as_str();
        if arg == "--process" {
            let proc = arguments.get(i + 1).map(|s| s.as_str()).unwrap_or("");
            if proc != "entity-manager" {
                return Err("CERBERO_DENY: --process not authorized".into());
            }
            saw_entity_manager = true;
            i += 2;
            continue;
        }
        if arg == "--tool" || arg.starts_with("--tool=") {
            return Err("CERBERO_DENY: --tool not authorized via run_check".into());
        }
        if arg == "--process" && arguments.get(i + 1).is_none() {
            return Err("CERBERO_DENY: --process value missing".into());
        }
        if arg.starts_with("--process=") {
            let proc = arg.trim_start_matches("--process=");
            if proc != "entity-manager" {
                return Err("CERBERO_DENY: --process not authorized".into());
            }
            saw_entity_manager = true;
        }
        i += 1;
    }
    if !saw_entity_manager {
        return Err("CERBERO_DENY: entity-manager invocation required (--process entity-manager)".into());
    }
    Ok(())
}

/// Valida intención acotada tras whitelist del manifiesto y antes de shell-executor.
pub fn validate_governed_run_check(executable: &str, arguments: &[String]) -> Result<(), String> {
    if executable_basename(executable) == "git" {
        return Err("CERBERO_DENY: git must use git-manager".into());
    }
    match governed_executable_kind(executable) {
        Some("sddia-qa") => Ok(()),
        Some("entity-manager") => validate_entity_manager_argv(arguments),
        _ => Err("CERBERO_DENY: executable not in governed MCP allowlist".into()),
    }
}

pub const ENV_RUNTIME_DEPTH: &str = "SDDIA_AGENT_RUNTIME_DEPTH";

pub fn child_runtime_depth(parent_depth: u32) -> u32 {
    parent_depth.saturating_add(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection_non_empty_for_governed_manifest() {
        let manifest = HashSet::from([
            String::from("sddia-qa"),
            String::from("./sddia-run.sh"),
            String::from("git"),
            String::from("cargo"),
        ]);
        let inter = manifest_shell_intersection(&manifest);
        assert!(inter.contains("sddia-qa"));
        assert!(inter.contains("sddia-run.sh"));
        assert!(!inter.contains("git"));
        assert!(!inter.contains("cargo"));
        assert!(!inter.is_empty());
    }

    #[test]
    fn entity_manager_argv_accepts_minimal() {
        let args = vec![
            "--process".into(),
            "entity-manager".into(),
            "--inputs".into(),
            "{}".into(),
        ];
        assert!(validate_governed_run_check("./sddia-run.sh", &args).is_ok());
    }

    #[test]
    fn entity_manager_denies_other_process() {
        let args = vec!["--process".into(), "feature".into()];
        assert!(validate_governed_run_check("sddia-run.sh", &args).is_err());
    }

    #[test]
    fn sddia_qa_allowed_empty_args() {
        assert!(validate_governed_run_check("sddia-qa", &[]).is_ok());
    }

    #[test]
    fn cargo_denied_even_if_whitelisted_in_manifest() {
        let args = vec!["test".into()];
        assert!(validate_governed_run_check("cargo", &args).is_err());
    }

    #[test]
    fn depth_increments() {
        assert_eq!(child_runtime_depth(0), 1);
        assert_eq!(child_runtime_depth(2), 3);
    }
}