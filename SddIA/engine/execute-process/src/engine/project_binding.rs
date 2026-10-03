//! Binding de proyecto cliente (ABSTRACT-04): contrato, git propio, delivery_mode, rutas.

use super::domain_authority::has_software_authority;
use super::domain_profile::{resolve_execution_profile, ExecutionProfile};
use crate::core::paths::load_paths_config;
use serde_json::{json, Value};
use serde_yaml::Value as YamlValue;
use std::fs;
use std::path::{Path, PathBuf};

pub const CONTRACT_VERSION: &str = "1.3.0";
const CONTRACT_VERSION_LEGACY: &str = "1.0.0";
const CONTRACT_VERSION_MID: &str = "1.1.0";
const CONTRACT_VERSION_TRACKER: &str = "1.2.0";
pub const CONTRACT_REL: &str =
    "SddIA/library/codexes/codex-software-engineering/contracts/project-config-contract.md";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryMode {
    BranchPr,
    TrunkDirect,
}

impl DeliveryMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BranchPr => "branch_pr",
            Self::TrunkDirect => "trunk_direct",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocsLayout {
    pub features: String,
    pub fixes: String,
    pub todos_pending: String,
    pub todos_done: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrackerConfig {
    pub team_key: String,
    pub project_id: Option<String>,
    pub state_map: std::collections::BTreeMap<String, String>,
    pub label_hu: String,
    pub label_pbi: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundProject {
    pub slug: String,
    pub uuid: String,
    pub project_root: PathBuf,
    pub codex_slug: String,
    pub delivery_mode: DeliveryMode,
    pub delivery_mode_source: &'static str,
    pub default_branch: String,
    pub docs: DocsLayout,
}

pub fn parse_delivery_mode(raw: &str) -> Result<DeliveryMode, String> {
    match raw.trim() {
        "branch_pr" => Ok(DeliveryMode::BranchPr),
        "trunk_direct" => Ok(DeliveryMode::TrunkDirect),
        other => Err(format!("PROJECT_CONFIG_INVALID: delivery_mode '{other}'")),
    }
}

/// Precedencia: inputs > manifiesto > default `branch_pr`.
pub fn resolve_delivery_mode(
    inputs: &Value,
    manifest_mode: Option<&str>,
) -> Result<(DeliveryMode, &'static str), String> {
    if let Some(raw) = inputs
        .get("delivery_mode")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        return Ok((parse_delivery_mode(raw)?, "inputs"));
    }
    if let Some(raw) = manifest_mode.map(str::trim).filter(|s| !s.is_empty()) {
        return Ok((parse_delivery_mode(raw)?, "project"));
    }
    Ok((DeliveryMode::BranchPr, "default"))
}

pub fn omits_pr_cycle(mode: DeliveryMode) -> bool {
    mode == DeliveryMode::TrunkDirect
}

/// Push a `main` solo si el repo no es Core y el modo es `trunk_direct`.
pub fn main_push_allowed(repo_is_core: bool, mode: DeliveryMode) -> bool {
    !repo_is_core && mode == DeliveryMode::TrunkDirect
}

pub fn repo_is_core(repo: &Path) -> bool {
    repo.join("SddIA/core/cumulo.paths.json").is_file()
}

fn projects_rel(repo: &Path) -> String {
    load_paths_config(repo)
        .ok()
        .and_then(|cfg| {
            cfg.get("instance")
                .and_then(|i| i.get("projects"))
                .and_then(|v| v.as_str())
                .map(|s| s.trim().trim_end_matches('/').to_string())
        })
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| ".SddIA/projects".to_string())
}

fn split_frontmatter(raw: &str) -> Result<(String, String), String> {
    let mut lines = raw.lines();
    if lines.next() != Some("---") {
        return Err("PROJECT_CONFIG_INVALID: sin frontmatter".into());
    }
    let mut fm = String::new();
    for line in lines.by_ref() {
        if line == "---" {
            let body: String = lines.collect::<Vec<_>>().join("\n");
            return Ok((fm, body));
        }
        fm.push_str(line);
        fm.push('\n');
    }
    Err("PROJECT_CONFIG_INVALID: frontmatter sin cierre".into())
}

fn yaml_str(fm: &YamlValue, key: &str) -> Result<String, String> {
    fm.get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| format!("PROJECT_CONFIG_INVALID: falta '{key}'"))
}

fn parse_yaml_fm(text: &str) -> Result<YamlValue, String> {
    let (fm, _) = split_frontmatter(text)?;
    serde_yaml::from_str(&fm).map_err(|e| format!("PROJECT_CONFIG_INVALID: yaml {e}"))
}

fn docs_from_manifest(fm: &YamlValue) -> Result<DocsLayout, String> {
    let layout = fm
        .get("docs_layout")
        .ok_or("PROJECT_CONFIG_INVALID: falta 'docs_layout'")?;
    let req = |k: &str| -> Result<String, String> {
        layout
            .get(k)
            .and_then(|v| v.as_str())
            .map(|s| s.trim().trim_start_matches("./").to_string())
            .filter(|s| !s.is_empty() && !s.contains(".."))
            .ok_or_else(|| format!("PROJECT_CONFIG_INVALID: docs_layout.{k}"))
    };
    Ok(DocsLayout {
        features: req("features")?,
        fixes: req("fixes")?,
        todos_pending: req("todos_pending")?,
        todos_done: req("todos_done")?,
    })
}

pub fn resolve_doc_path(project_root: &Path, rel: &str) -> Result<PathBuf, String> {
    let rel = rel.trim().trim_start_matches("./");
    if rel.is_empty() || rel.contains("..") || Path::new(rel).is_absolute() {
        return Err(format!("PROJECT_SCOPE_ESCAPE: layout '{rel}'"));
    }
    let full = project_root.join(rel);
    if !full.starts_with(project_root) {
        return Err(format!("PROJECT_SCOPE_ESCAPE: {rel}"));
    }
    Ok(full)
}

const TRACKER_STATE_KEYS: &[&str] = &[
    "backlog",
    "todo",
    "in_progress",
    "in_review",
    "done",
    "cancelled",
];

const TRACKER_LABEL_OPTIONAL_KEYS: &[&str] = &["fix", "kaizen", "deuda", "spike", "editable"];

const DONE_GATE_VALUES: &[&str] = &["git", "linear", "both"];

/// HU-A: `linear`/`both` declarados en contrato 1.3.0; validación activa en HU-B.
const DONE_GATE_HU_A_ONLY_GIT: bool = true;

fn validate_tracker_config(fm: &YamlValue) -> Result<(), String> {
    let Some(tracker) = fm.get("tracker") else {
        return Ok(());
    };
    if !tracker.is_mapping() {
        return Err("PROJECT_CONFIG_INVALID: tracker debe ser objeto".into());
    }
    let provider = tracker
        .get("provider")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty());
    if provider.is_none() {
        return Ok(());
    }
    let p = provider.unwrap();
    if p != "linear" {
        return Err(format!(
            "PROJECT_CONFIG_INVALID: tracker.provider '{p}' no admitido (linear)"
        ));
    }
    let team = tracker
        .get("team_key")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty());
    if team.is_none() {
        return Err(
            "PROJECT_CONFIG_INVALID: tracker.team_key obligatorio con provider linear".into(),
        );
    }
    if let Some(map) = tracker.get("state_map").and_then(|m| m.as_mapping()) {
        for (k, v) in map {
            let key = k.as_str().unwrap_or("");
            if !TRACKER_STATE_KEYS.contains(&key) {
                return Err(format!(
                    "PROJECT_CONFIG_INVALID: tracker.state_map clave desconocida '{key}'"
                ));
            }
            if !v.as_str().is_some_and(|s| !s.trim().is_empty()) {
                return Err(format!(
                    "PROJECT_CONFIG_INVALID: tracker.state_map.{key} vacío"
                ));
            }
        }
    }
    if let Some(labels) = tracker.get("labels") {
        if !labels.is_mapping() {
            return Err("PROJECT_CONFIG_INVALID: tracker.labels debe ser objeto".into());
        }
        for key in ["hu", "pbi"] {
            if let Some(v) = labels.get(key) {
                if !v.as_str().is_some_and(|s| !s.trim().is_empty()) {
                    return Err(format!(
                        "PROJECT_CONFIG_INVALID: tracker.labels.{key} vacío"
                    ));
                }
            }
        }
        for key in TRACKER_LABEL_OPTIONAL_KEYS {
            if let Some(v) = labels.get(*key) {
                if !v.as_str().is_some_and(|s| !s.trim().is_empty()) {
                    return Err(format!(
                        "PROJECT_CONFIG_INVALID: tracker.labels.{key} vacío"
                    ));
                }
            }
        }
    }
    if let Some(raw) = tracker
        .get("done_gate")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        if !DONE_GATE_VALUES.contains(&raw) {
            return Err(format!(
                "PROJECT_CONFIG_INVALID: tracker.done_gate '{raw}' (git | linear | both)"
            ));
        }
        if DONE_GATE_HU_A_ONLY_GIT && raw != "git" {
            return Err(format!(
                "PROJECT_CONFIG_INVALID: tracker.done_gate '{raw}' no habilitado hasta HU-B (solo git)"
            ));
        }
    }
    Ok(())
}

pub fn anchor_persist(project_root: &Path, persist_ref: &str) -> Result<PathBuf, String> {
    let raw = Path::new(persist_ref.trim());
    if persist_ref.contains("..") {
        return Err(format!("PROJECT_SCOPE_ESCAPE: {persist_ref}"));
    }
    let full = if raw.is_absolute() {
        raw.to_path_buf()
    } else {
        project_root.join(raw)
    };
    if !full.starts_with(project_root) {
        return Err(format!("PROJECT_SCOPE_ESCAPE: {persist_ref}"));
    }
    Ok(full)
}

fn git_root_marker(root: &Path) -> bool {
    let git = root.join(".git");
    git.is_dir() || git.is_file()
}

fn registered_roots(projects_dir: &Path, except_slug: &str) -> Result<Vec<PathBuf>, String> {
    let mut out = Vec::new();
    if !projects_dir.is_dir() {
        return Ok(out);
    }
    for entry in fs::read_dir(projects_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        if stem == except_slug || stem == "index" {
            continue;
        }
        let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let Ok(fm) = parse_yaml_fm(&text) else {
            continue;
        };
        if let Ok(root) = yaml_str(&fm, "project_root") {
            out.push(PathBuf::from(root));
        }
    }
    Ok(out)
}

fn assert_git_isolation(project_root: &Path, others: &[PathBuf]) -> Result<(), String> {
    if !git_root_marker(project_root) {
        return Err(format!(
            "PROJECT_GIT_INVALID: {} sin .git propio",
            project_root.display()
        ));
    }
    for other in others {
        if project_root.starts_with(other) && project_root != other {
            return Err(format!(
                "PROJECT_GIT_INVALID: {} anidado en {}",
                project_root.display(),
                other.display()
            ));
        }
        if other.starts_with(project_root) && other != project_root {
            return Err(format!(
                "PROJECT_GIT_INVALID: {} contiene otro proyecto {}",
                project_root.display(),
                other.display()
            ));
        }
    }
    Ok(())
}

pub fn tracker_config_from_manifest(fm: &YamlValue) -> Option<TrackerConfig> {
    let tracker = fm.get("tracker")?;
    if tracker.get("provider")?.as_str()? != "linear" {
        return None;
    }
    let team_key = tracker
        .get("team_key")?
        .as_str()?
        .trim()
        .to_string();
    if team_key.is_empty() {
        return None;
    }
    let mut state_map = std::collections::BTreeMap::new();
    if let Some(map) = tracker.get("state_map").and_then(|m| m.as_mapping()) {
        for (k, v) in map {
            if let (Some(key), Some(val)) = (k.as_str(), v.as_str()) {
                if !key.is_empty() && !val.trim().is_empty() {
                    state_map.insert(key.to_string(), val.trim().to_string());
                }
            }
        }
    }
    let labels = tracker.get("labels");
    let label_hu = labels
        .and_then(|l| l.get("hu"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("hu")
        .to_string();
    let label_pbi = labels
        .and_then(|l| l.get("pbi"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("pbi")
        .to_string();
    let project_id = tracker
        .get("project_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string);
    Some(TrackerConfig {
        team_key,
        project_id,
        state_map,
        label_hu,
        label_pbi,
    })
}

fn manifest_frontmatter_for_slug(repo: &Path, slug: &str) -> Result<YamlValue, String> {
    let index_path = repo.join(projects_rel(repo)).join(format!("{slug}.md"));
    if !index_path.is_file() {
        return Err(format!("PROJECT_NOT_REGISTERED: {slug}"));
    }
    let index_fm = parse_yaml_fm(&fs::read_to_string(&index_path).map_err(|e| e.to_string())?)?;
    let project_root = PathBuf::from(yaml_str(&index_fm, "project_root")?);
    let manifest_ref = yaml_str(&index_fm, "manifest_ref")?;
    let manifest_path = if Path::new(&manifest_ref).is_absolute() {
        PathBuf::from(manifest_ref)
    } else {
        project_root.join(manifest_ref)
    };
    if !manifest_path.is_file() {
        return Err(format!(
            "PROJECT_CONFIG_INVALID: manifiesto ausente {}",
            manifest_path.display()
        ));
    }
    parse_yaml_fm(&fs::read_to_string(&manifest_path).map_err(|e| e.to_string())?)
}

/// Tracker del manifiesto del proyecto (`project_slug` en inputs). Sin slug → `None` (Core-self).
pub fn tracker_config_for_inputs(repo: &Path, inputs: &Value) -> Result<Option<TrackerConfig>, String> {
    let Some(slug) = inputs
        .get("project_slug")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    else {
        return Ok(None);
    };
    if slug.contains('/') || slug.contains("..") {
        return Err(format!("PROJECT_CONFIG_INVALID: project_slug '{slug}'"));
    }
    let fm = manifest_frontmatter_for_slug(repo, slug)?;
    validate_tracker_config(&fm)?;
    Ok(tracker_config_from_manifest(&fm))
}

fn validate_env_ref(env_ref: &str) -> Result<(), String> {
    let rel = env_ref.trim().trim_start_matches("./");
    if rel.is_empty() || rel.contains("..") || Path::new(rel).is_absolute() {
        return Err(format!("PROJECT_CONFIG_INVALID: env_ref '{env_ref}'"));
    }
    Ok(())
}

/// Sin `project_slug` → `Ok(None)` (repo auto-hospedado, sin cambio de flujo).
pub fn bind(repo: &Path, inputs: &Value) -> Result<Option<BoundProject>, String> {
    let Some(slug) = inputs
        .get("project_slug")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    else {
        return Ok(None);
    };
    if slug.contains('/') || slug.contains("..") {
        return Err(format!("PROJECT_CONFIG_INVALID: project_slug '{slug}'"));
    }

    let index_path = repo.join(projects_rel(repo)).join(format!("{slug}.md"));
    if !index_path.is_file() {
        return Err(format!("PROJECT_NOT_REGISTERED: {slug}"));
    }
    let index_fm = parse_yaml_fm(&fs::read_to_string(&index_path).map_err(|e| e.to_string())?)?;
    let index_uuid = yaml_str(&index_fm, "uuid")?;
    let project_root = PathBuf::from(yaml_str(&index_fm, "project_root")?);
    if !project_root.is_absolute() {
        return Err("PROJECT_CONFIG_INVALID: project_root no absoluto".into());
    }
    let manifest_ref = yaml_str(&index_fm, "manifest_ref")?;
    let _ = yaml_str(&index_fm, "codex_slug")?;
    let _ = yaml_str(&index_fm, "status")?;
    let _ = yaml_str(&index_fm, "id")?;

    let manifest_path = if Path::new(&manifest_ref).is_absolute() {
        PathBuf::from(&manifest_ref)
    } else {
        project_root.join(&manifest_ref)
    };
    if !manifest_path.is_file() {
        return Err(format!(
            "PROJECT_CONFIG_INVALID: manifiesto ausente {}",
            manifest_path.display()
        ));
    }
    let manifest_fm = parse_yaml_fm(&fs::read_to_string(&manifest_path).map_err(|e| e.to_string())?)?;
    let manifest_uuid = yaml_str(&manifest_fm, "uuid")?;
    if manifest_uuid != index_uuid {
        return Err(format!(
            "PROJECT_CONFIG_INVALID: uuid divergente índice={index_uuid} manifiesto={manifest_uuid}"
        ));
    }
    let contract_version = yaml_str(&manifest_fm, "contract_version")?;
    if contract_version != CONTRACT_VERSION
        && contract_version != CONTRACT_VERSION_TRACKER
        && contract_version != CONTRACT_VERSION_MID
        && contract_version != CONTRACT_VERSION_LEGACY
    {
        return Err(format!(
            "PROJECT_CONFIG_INVALID: contract_version '{contract_version}' no admitida (1.0.0 | 1.1.0 | 1.2.0 | 1.3.0)"
        ));
    }
    if let Some(env_ref) = manifest_fm
        .get("env_ref")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        validate_env_ref(env_ref)?;
    }
    let _ = yaml_str(&manifest_fm, "id")?;
    let _ = yaml_str(&manifest_fm, "git_remote")?;
    let default_branch = yaml_str(&manifest_fm, "default_branch")?;
    let manifest_codex_slug = yaml_str(&manifest_fm, "codex_slug")?;
    let docs = docs_from_manifest(&manifest_fm)?;
    validate_tracker_config(&manifest_fm)?;
    let manifest_mode = yaml_str(&manifest_fm, "delivery_mode")?;
    let (delivery_mode, delivery_mode_source) =
        resolve_delivery_mode(inputs, Some(&manifest_mode))?;

    if delivery_mode == DeliveryMode::TrunkDirect && env_flag("SDDIA_SKIP_HOOKS") {
        return Err("TRUNK_HOOKS_REQUIRED: trunk_direct prohíbe SDDIA_SKIP_HOOKS".into());
    }

    let others = registered_roots(index_path.parent().unwrap_or(repo), slug)?;
    assert_git_isolation(&project_root, &others)?;

    Ok(Some(BoundProject {
        slug: slug.to_string(),
        uuid: index_uuid,
        project_root,
        codex_slug: manifest_codex_slug,
        delivery_mode,
        delivery_mode_source,
        default_branch,
        docs,
    }))
}

fn env_flag(key: &str) -> bool {
    matches!(
        std::env::var(key).ok().as_deref().map(str::trim),
        Some("1") | Some("true") | Some("yes") | Some("on")
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutePlan {
    pub subscribers: Vec<Value>,
    pub dead_letter_no_subscriber: bool,
}

/// Core ∪ suscripciones del códice si hay autoridad software.
/// Clave exclusiva del códice sin autoridad → dead-letter `no_subscriber`.
pub fn plan_route(
    core: &Value,
    codex: Option<&Value>,
    authority: bool,
    event_type: &str,
) -> RoutePlan {
    let empty = Value::Null;
    let codex = codex.unwrap_or(&empty);
    let in_codex = codex.get(event_type).is_some();
    let in_core = core.get(event_type).is_some();
    if in_codex && !in_core && !authority {
        return RoutePlan {
            subscribers: Vec::new(),
            dead_letter_no_subscriber: true,
        };
    }
    let mut subscribers = Vec::new();
    if let Some(arr) = core.get(event_type).and_then(|v| v.as_array()) {
        subscribers.extend(arr.iter().cloned());
    }
    if authority {
        if let Some(arr) = codex.get(event_type).and_then(|v| v.as_array()) {
            subscribers.extend(arr.iter().cloned());
        }
    }
    RoutePlan {
        subscribers,
        dead_letter_no_subscriber: false,
    }
}

pub fn codex_subscriptions_rel(repo: &Path) -> Option<String> {
    load_paths_config(repo).ok().and_then(|cfg| {
        cfg.get("codex_subscriptions")
            .and_then(|c| c.get("codex-software-engineering"))
            .and_then(|v| v.as_str())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    })
}

pub fn load_codex_subscriptions(repo: &Path) -> Option<Value> {
    let rel = codex_subscriptions_rel(repo).unwrap_or_else(|| {
        "SddIA/library/codexes/codex-software-engineering/subscriptions.json".into()
    });
    let text = fs::read_to_string(repo.join(rel)).ok()?;
    serde_json::from_str(text.trim_start_matches('\u{feff}')).ok()
}

pub fn authority_from_repo(repo: &Path) -> bool {
    let profile: ExecutionProfile = resolve_execution_profile(repo, &json!({}));
    has_software_authority(&profile)
}

pub fn plan_route_for_repo(repo: &Path, core: &Value, event_type: &str) -> RoutePlan {
    let authority = authority_from_repo(repo);
    let codex = load_codex_subscriptions(repo);
    plan_route(core, codex.as_ref(), authority, event_type)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    fn write_git_root(root: &Path) {
        fs::create_dir_all(root.join(".git")).unwrap();
        let _ = fs::set_permissions(root.join(".git"), fs::Permissions::from_mode(0o755));
    }

    fn index_md(uuid: &str, root: &Path) -> String {
        format!(
            "---\nid: demo\nuuid: \"{uuid}\"\nproject_root: \"{}\"\nmanifest_ref: .SddIA/project.md\ncodex_slug: codex-software-engineering\nstatus: active\n---\n\n# demo\n",
            root.display()
        )
    }

    fn manifest_md(uuid: &str, mode: &str, version: &str) -> String {
        format!(
            "---\nid: demo\nuuid: \"{uuid}\"\ngit_remote: https://example.invalid/demo.git\ndefault_branch: main\ndelivery_mode: {mode}\ncontract_version: \"{version}\"\ncodex_slug: codex-software-engineering\ndocs_layout:\n  features: docs/features\n  fixes: docs/fixes\n  todos_pending: docs/todos/pending\n  todos_done: docs/todos/done\n---\n\n# demo\n"
        )
    }

    fn layout(repo: &Path, uuid: &str, mode: &str) -> PathBuf {
        let client = repo.join("client-demo");
        write_git_root(&client);
        fs::create_dir_all(client.join(".SddIA")).unwrap();
        fs::write(client.join(".SddIA/project.md"), manifest_md(uuid, mode, "1.0.0")).unwrap();
        fs::create_dir_all(repo.join(".SddIA/projects")).unwrap();
        fs::write(
            repo.join(".SddIA/projects/demo.md"),
            index_md(uuid, &client),
        )
        .unwrap();
        client
    }

    #[test]
    fn precedence_inputs_over_project_over_default() {
        let inputs = json!({"delivery_mode": "trunk_direct"});
        let (m, s) = resolve_delivery_mode(&inputs, Some("branch_pr")).unwrap();
        assert_eq!(m, DeliveryMode::TrunkDirect);
        assert_eq!(s, "inputs");
        let (m, s) = resolve_delivery_mode(&json!({}), Some("trunk_direct")).unwrap();
        assert_eq!(m, DeliveryMode::TrunkDirect);
        assert_eq!(s, "project");
        let (m, s) = resolve_delivery_mode(&json!({}), None).unwrap();
        assert_eq!(m, DeliveryMode::BranchPr);
        assert_eq!(s, "default");
    }

    #[test]
    fn main_push_double() {
        assert!(!main_push_allowed(true, DeliveryMode::TrunkDirect));
        assert!(main_push_allowed(false, DeliveryMode::TrunkDirect));
        assert!(!main_push_allowed(false, DeliveryMode::BranchPr));
        assert!(omits_pr_cycle(DeliveryMode::TrunkDirect));
        assert!(!omits_pr_cycle(DeliveryMode::BranchPr));
    }

    #[test]
    fn contract_accepts_1_1_0_and_rejects_bad_env_ref() {
        let repo = tempfile::tempdir().unwrap();
        let uuid = "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee";
        let client = layout(repo.path(), uuid, "branch_pr");
        let mut body = manifest_md(uuid, "branch_pr", "1.1.0");
        body = body.replace("---\n", "---\nenv_ref: .SddIA/.dev/.env\n");
        fs::write(client.join(".SddIA/project.md"), body).unwrap();
        assert!(bind(repo.path(), &json!({"project_slug": "demo"})).is_ok());

        let bad = manifest_md(uuid, "branch_pr", "1.1.0").replace(
            "delivery_mode: branch_pr",
            "delivery_mode: branch_pr\nenv_ref: ../.env",
        );
        fs::write(client.join(".SddIA/project.md"), bad).unwrap();
        let err = bind(repo.path(), &json!({"project_slug": "demo"})).unwrap_err();
        assert!(err.contains("env_ref"), "{err}");
    }

    #[test]
    fn contract_accepts_1_2_0_tracker_and_rejects_bad_provider() {
        let repo = tempfile::tempdir().unwrap();
        let uuid = "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee";
        let client = layout(repo.path(), uuid, "branch_pr");
        let mut body = manifest_md(uuid, "branch_pr", "1.2.0");
        body = body.replace(
            "docs_layout:",
            "tracker:\n  provider: linear\n  team_key: BX\n  state_map:\n    backlog: Backlog\n    done: Done\ndocs_layout:",
        );
        fs::write(client.join(".SddIA/project.md"), &body).unwrap();
        assert!(bind(repo.path(), &json!({"project_slug": "demo"})).is_ok());

        let mut bad = body.clone();
        bad = bad.replace("team_key: BX", "team_key: ");
        fs::write(client.join(".SddIA/project.md"), bad).unwrap();
        let err = bind(repo.path(), &json!({"project_slug": "demo"})).unwrap_err();
        assert!(err.contains("team_key"), "{err}");

        let bad2 = body.replace("provider: linear", "provider: jira");
        fs::write(client.join(".SddIA/project.md"), bad2).unwrap();
        let err = bind(repo.path(), &json!({"project_slug": "demo"})).unwrap_err();
        assert!(err.contains("provider"), "{err}");
    }

    #[test]
    fn contract_accepts_1_3_0_tracker_todo_and_done_gate_git() {
        let repo = tempfile::tempdir().unwrap();
        let uuid = "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee";
        let client = layout(repo.path(), uuid, "branch_pr");
        let mut body = manifest_md(uuid, "branch_pr", "1.3.0");
        body = body.replace(
            "docs_layout:",
            "tracker:\n  provider: linear\n  team_key: OSC\n  done_gate: git\n  state_map:\n    backlog: Backlog\n    todo: Todo\n    done: Done\n  labels:\n    hu: hu\n    pbi: pbi\n    editable: sddia-editable\ndocs_layout:",
        );
        fs::write(client.join(".SddIA/project.md"), &body).unwrap();
        assert!(bind(repo.path(), &json!({"project_slug": "demo"})).is_ok());
    }

    #[test]
    fn contract_1_3_0_rejects_done_gate_linear_until_hu_b() {
        let repo = tempfile::tempdir().unwrap();
        let uuid = "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee";
        let client = layout(repo.path(), uuid, "branch_pr");
        let mut body = manifest_md(uuid, "branch_pr", "1.3.0");
        body = body.replace(
            "docs_layout:",
            "tracker:\n  provider: linear\n  team_key: OSC\n  done_gate: linear\n  state_map:\n    backlog: Backlog\ndocs_layout:",
        );
        fs::write(client.join(".SddIA/project.md"), &body).unwrap();
        let err = bind(repo.path(), &json!({"project_slug": "demo"})).unwrap_err();
        assert!(err.contains("done_gate"), "{err}");
    }

    #[test]
    fn contract_rejects_bad_version_uuid_and_enum() {
        let repo = tempfile::tempdir().unwrap();
        let uuid = "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee";
        let client = layout(repo.path(), uuid, "branch_pr");
        fs::write(
            client.join(".SddIA/project.md"),
            manifest_md(uuid, "branch_pr", "9.9.9"),
        )
        .unwrap();
        let err = bind(repo.path(), &json!({"project_slug": "demo"})).unwrap_err();
        assert!(err.contains("contract_version"), "{err}");

        fs::write(
            client.join(".SddIA/project.md"),
            manifest_md("bbbbbbbb-bbbb-4ccc-8ddd-eeeeeeeeeeee", "branch_pr", "1.0.0"),
        )
        .unwrap();
        let err = bind(repo.path(), &json!({"project_slug": "demo"})).unwrap_err();
        assert!(err.contains("uuid divergente"), "{err}");

        fs::write(
            client.join(".SddIA/project.md"),
            manifest_md(uuid, "yolo", "1.0.0"),
        )
        .unwrap();
        let err = bind(repo.path(), &json!({"project_slug": "demo"})).unwrap_err();
        assert!(err.contains("delivery_mode"), "{err}");
    }

    #[test]
    fn unregistered_and_missing_git_and_nested() {
        let repo = tempfile::tempdir().unwrap();
        let err = bind(repo.path(), &json!({"project_slug": "nope"})).unwrap_err();
        assert!(err.starts_with("PROJECT_NOT_REGISTERED"), "{err}");

        let uuid = "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee";
        let client = repo.path().join("client-demo");
        fs::create_dir_all(client.join(".SddIA")).unwrap();
        fs::write(
            client.join(".SddIA/project.md"),
            manifest_md(uuid, "branch_pr", "1.0.0"),
        )
        .unwrap();
        fs::create_dir_all(repo.path().join(".SddIA/projects")).unwrap();
        fs::write(
            repo.path().join(".SddIA/projects/demo.md"),
            index_md(uuid, &client),
        )
        .unwrap();
        let err = bind(repo.path(), &json!({"project_slug": "demo"})).unwrap_err();
        assert!(err.starts_with("PROJECT_GIT_INVALID"), "{err}");

        write_git_root(&client);
        let inner = client.join("nested");
        write_git_root(&inner);
        let uuid2 = "cccccccc-dddd-4eee-8fff-000000000001";
        fs::create_dir_all(inner.join(".SddIA")).unwrap();
        fs::write(
            inner.join(".SddIA/project.md"),
            manifest_md(uuid2, "branch_pr", "1.0.0"),
        )
        .unwrap();
        fs::write(
            repo.path().join(".SddIA/projects/nested.md"),
            index_md(uuid2, &inner),
        )
        .unwrap();
        let err = bind(repo.path(), &json!({"project_slug": "nested"})).unwrap_err();
        assert!(err.contains("anidado"), "{err}");
    }

    #[test]
    fn doc_path_stays_inside_project_root() {
        let repo = tempfile::tempdir().unwrap();
        let uuid = "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee";
        let client = layout(repo.path(), uuid, "branch_pr");
        let bound = bind(repo.path(), &json!({"project_slug": "demo"}))
            .unwrap()
            .unwrap();
        let pending = resolve_doc_path(&bound.project_root, &bound.docs.todos_pending).unwrap();
        assert!(pending.starts_with(&client));
        assert!(!pending.starts_with(repo.path().join("SddIA")));
        let err = anchor_persist(&client, "../outside").unwrap_err();
        assert!(err.starts_with("PROJECT_SCOPE_ESCAPE"), "{err}");
        let ok = anchor_persist(&client, "docs/features/x").unwrap();
        assert!(ok.starts_with(&client));
    }

    #[test]
    fn inputs_override_recorded_on_bind() {
        let repo = tempfile::tempdir().unwrap();
        let uuid = "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee";
        layout(repo.path(), uuid, "branch_pr");
        let bound = bind(
            repo.path(),
            &json!({"project_slug": "demo", "delivery_mode": "trunk_direct"}),
        )
        .unwrap()
        .unwrap();
        assert_eq!(bound.delivery_mode, DeliveryMode::TrunkDirect);
        assert_eq!(bound.delivery_mode_source, "inputs");
        let bound = bind(repo.path(), &json!({"project_slug": "demo"}))
            .unwrap()
            .unwrap();
        assert_eq!(bound.delivery_mode, DeliveryMode::BranchPr);
        assert_eq!(bound.delivery_mode_source, "project");
    }

    #[test]
    fn codex_event_without_authority_is_no_subscriber() {
        let core = json!({"Local_QA_Requested": []});
        let codex = json!({"PBI_Forged": [{"agent": "tekton", "process": "feature"}]});
        let denied = plan_route(&core, Some(&codex), false, "PBI_Forged");
        assert!(denied.dead_letter_no_subscriber);
        assert!(denied.subscribers.is_empty());
        let allowed = plan_route(&core, Some(&codex), true, "PBI_Forged");
        assert!(!allowed.dead_letter_no_subscriber);
        assert_eq!(allowed.subscribers.len(), 1);
        let sweep = plan_route(&core, Some(&codex), false, "Local_QA_Requested");
        assert!(!sweep.dead_letter_no_subscriber);
        assert!(sweep.subscribers.is_empty());
    }
}
