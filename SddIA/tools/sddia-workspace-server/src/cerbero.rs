use std::collections::HashSet;

const SERVER_CONTEXTS: &[&str] = &[
    "filesystem-ops",
    "source-control",
    "system-operations",
];

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

pub fn whitelist_executable(allowed: &HashSet<String>, executable: &str) -> bool {
    let base = executable
        .rsplit('/')
        .next()
        .unwrap_or(executable)
        .rsplit('\\')
        .next()
        .unwrap_or(executable);
    allowed.contains(base) || allowed.contains(executable)
}
