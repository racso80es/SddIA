//! Runtime de agentes V5 (slice B kalma2-full-cycle).
//!
//! Cuando `SDDIA_AGENT_RUNTIME_COMMAND` está definido, las fases solo-`agent:`
//! invocan ese CLI (JSON stdin → última línea JSON stdout) en lugar de
//! marcarse `simulated`.

use crate::core::parser::{parse_frontmatter, parse_frontmatter_from_str};
use super::project_binding;
use super::workspace::load_paths_config;
use serde_json::{json, Value};
use serde_yaml::Value as YamlValue;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const ENV_CMD: &str = "SDDIA_AGENT_RUNTIME_COMMAND";
const ENV_RELAY: &str = "SDDIA_AGENT_RELAY_IDE";
const ENV_DEPTH: &str = "SDDIA_AGENT_RUNTIME_DEPTH";
const ENV_TIMEOUT: &str = "SDDIA_AGENT_RUNTIME_TIMEOUT_SECS";
const ENV_TIMEOUT_EXEC: &str = "SDDIA_AGENT_RUNTIME_TIMEOUT_SECS_EJECUCION";
const ENV_TIMEOUT_REFINE: &str = "SDDIA_AGENT_RUNTIME_TIMEOUT_SECS_REFINAMIENTO";
const DEFAULT_TIMEOUT_SECS: u64 = 660;
const DEFAULT_TIMEOUT_REFINE_SECS: u64 = 900;
const MIN_EXECUTION_TIMEOUT_SECS: u64 = 2400;
const KILL_GRACE_MS: u64 = 500;
const DETERMINISTIC_AGENTS: &[&str] = &["cerbero", "cumulo", "radamanto"];
const DEFAULT_LOCAL_RUNTIME_REL: &str = ".SddIA/llm-runtime.local.json";
const CANONICAL_AGENT_RUNTIME_ROLES: &[&str] = &["cursor", "antigravity", "local-llm", "test"];

pub const ERR_PROVIDER_QUOTA_EXCEEDED: &str = "PROVIDER_QUOTA_EXCEEDED";
pub const ERR_PROVIDER_TIMEOUT: &str = "PROVIDER_TIMEOUT";
pub const ERR_PROVIDER_UNAVAILABLE: &str = "PROVIDER_UNAVAILABLE";
pub const ERR_PROVIDER_UNATTENDED_UNSUPPORTED: &str = "PROVIDER_UNATTENDED_UNSUPPORTED";
pub const ERR_LLM_MODEL_UNRESOLVED: &str = "LLM_MODEL_UNRESOLVED";
pub const ERR_CAPABILITY_BINDING_MISSING: &str = "CAPABILITY_BINDING_MISSING";

/// Taxonomía cerrada de perfiles cognitivos (`agents-contract.md` v1.3.0, AUT-122).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThinkingProfile {
    Deterministic = 0,
    Execution = 1,
    Audit = 2,
    Reasoning = 3,
}

impl ThinkingProfile {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "reasoning" | "high" => Some(Self::Reasoning),
            "audit" | "medium" => Some(Self::Audit),
            "execution" | "low" => Some(Self::Execution),
            "deterministic" | "none" => Some(Self::Deterministic),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Reasoning => "reasoning",
            Self::Audit => "audit",
            Self::Execution => "execution",
            Self::Deterministic => "deterministic",
        }
    }

    pub fn legacy_tier(self) -> &'static str {
        match self {
            Self::Reasoning => "high",
            Self::Audit => "medium",
            Self::Execution => "low",
            Self::Deterministic => "none",
        }
    }

    pub fn legacy_env_key(self) -> Option<&'static str> {
        match self {
            Self::Reasoning => Some("SDDIA_LLM_TIER_HIGH"),
            Self::Audit => Some("SDDIA_LLM_TIER_MEDIUM"),
            Self::Execution => Some("SDDIA_LLM_TIER_LOW"),
            Self::Deterministic => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderMode {
    InteractiveRelay,
    HeadlessCli,
    HeadlessApi,
    Deterministic,
}

impl ProviderMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InteractiveRelay => "interactive_relay",
            Self::HeadlessCli => "headless_cli",
            Self::HeadlessApi => "headless_api",
            Self::Deterministic => "deterministic",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnattendedPolicy {
    PauseAwaitingAgents,
    FailFast,
}

impl UnattendedPolicy {
    pub fn parse(raw: Option<&str>) -> Self {
        match raw.map(str::trim).unwrap_or("pause_awaiting_agents") {
            "fail_fast" => Self::FailFast,
            _ => Self::PauseAwaitingAgents,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::PauseAwaitingAgents => "pause_awaiting_agents",
            Self::FailFast => "fail_fast",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AgentInvocationRequest {
    pub repo_root: PathBuf,
    pub process_name: String,
    pub phase_name: String,
    pub agents: Vec<String>,
    pub thinking_profile: ThinkingProfile,
    pub resolved_model: Option<String>,
    pub unattended_policy: UnattendedPolicy,
    pub payload: Value,
    pub cwd: PathBuf,
    pub timeout_secs: u64,
    pub timeout_floor_applied: bool,
    pub depth: u32,
}

#[derive(Debug, Clone)]
pub struct AgentExecutionResult {
    pub status: String,
    pub provider_id: String,
    pub provider_mode: ProviderMode,
    pub thinking_profile: ThinkingProfile,
    pub resolved_model: Option<String>,
    pub message: Option<String>,
    pub note: Option<String>,
    pub error_code: Option<String>,
    pub error: Option<String>,
    pub runtime_response: Option<Value>,
    pub raw_stdout: Option<String>,
    pub timeout_secs: Option<u64>,
    pub timeout_floor_applied: bool,
}

pub trait AgentRuntimePort: Send + Sync {
    fn provider_id(&self) -> &str;
    fn mode(&self) -> ProviderMode;
    fn supports_unattended(&self) -> bool;
    fn invoke(&self, req: &AgentInvocationRequest) -> AgentExecutionResult;
}

thread_local! {
    static MOCK_AGENT_RUNTIME: RefCell<Option<Arc<dyn AgentRuntimePort>>> = const { RefCell::new(None) };
}

pub fn with_mock_agent_runtime<F, R>(mock: Arc<dyn AgentRuntimePort>, f: F) -> R
where
    F: FnOnce() -> R,
{
    MOCK_AGENT_RUNTIME.with(|slot| {
        let prev = slot.replace(Some(mock));
        let out = f();
        slot.replace(prev);
        out
    })
}

pub fn has_mock_agent_runtime() -> bool {
    MOCK_AGENT_RUNTIME.with(|slot| slot.borrow().is_some())
}

fn current_mock_agent_runtime() -> Option<Arc<dyn AgentRuntimePort>> {
    MOCK_AGENT_RUNTIME.with(|slot| slot.borrow().clone())
}

/// Adaptador mock en memoria (`mock:in-memory-agent-runtime`) para tests unitarios sin variables `SDDIA_LAB_*`.
pub struct InMemoryAgentRuntimeMock {
    pub provider: String,
    pub mode: ProviderMode,
    pub status: String,
    pub resolved_model_override: Option<String>,
    pub message: Option<String>,
    pub error_code: Option<String>,
    pub error: Option<String>,
}

impl Default for InMemoryAgentRuntimeMock {
    fn default() -> Self {
        Self {
            provider: "mock".to_string(),
            mode: ProviderMode::HeadlessApi,
            status: "executed".to_string(),
            resolved_model_override: Some("mock-model-v1".to_string()),
            message: Some("mock-executed".to_string()),
            error_code: None,
            error: None,
        }
    }
}

impl AgentRuntimePort for InMemoryAgentRuntimeMock {
    fn provider_id(&self) -> &str {
        &self.provider
    }

    fn mode(&self) -> ProviderMode {
        self.mode
    }

    fn supports_unattended(&self) -> bool {
        true
    }

    fn invoke(&self, req: &AgentInvocationRequest) -> AgentExecutionResult {
        let resolved_model = self
            .resolved_model_override
            .clone()
            .or_else(|| req.resolved_model.clone());
        AgentExecutionResult {
            status: self.status.clone(),
            provider_id: self.provider.clone(),
            provider_mode: self.mode,
            thinking_profile: req.thinking_profile,
            resolved_model,
            message: self.message.clone(),
            note: None,
            error_code: self.error_code.clone(),
            error: self.error.clone(),
            runtime_response: Some(json!({
                "success": self.status == "executed" || self.status == "awaiting_agents",
                "data": {
                    "status": self.status,
                    "provider": self.provider,
                    "thinking_profile": req.thinking_profile.as_str(),
                    "message": self.message,
                    "error_code": self.error_code,
                },
                "error": self.error,
            })),
            raw_stdout: None,
            timeout_secs: None,
            timeout_floor_applied: req.timeout_floor_applied,
        }
    }
}

fn env_truthy(key: &str) -> bool {
    std::env::var(key)
        .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

fn lab_relay_active() -> bool {
    env_truthy(ENV_RELAY)
}

fn agent_runtime_depth() -> u32 {
    std::env::var(ENV_DEPTH)
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or(0)
}

pub fn is_configured() -> bool {
    if has_mock_agent_runtime() {
        return true;
    }
    if lab_relay_active() {
        return false;
    }
    std::env::var(ENV_CMD)
        .map(|v| !v.trim().is_empty())
        .unwrap_or(false)
}

pub fn is_configured_for(repo: &Path, inputs: &Value) -> bool {
    if is_configured() {
        return true;
    }
    if lab_relay_active() {
        return false;
    }
    if inputs.get("llm_runtime").is_some() {
        return true;
    }
    local_runtime_config_path(repo).is_file()
}

pub fn local_runtime_config_path(repo: &Path) -> PathBuf {
    let rel = load_paths_config(repo)
        .ok()
        .and_then(|cfg| {
            cfg.pointer("/instance/llm_runtime_local")
                .and_then(|v| v.as_str())
                .map(|s| s.trim().trim_start_matches("./").to_string())
        })
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| DEFAULT_LOCAL_RUNTIME_REL.to_string());
    repo.join(rel)
}

pub fn allowed_agent_runtime_roles(repo: &Path) -> HashSet<String> {
    let bindings_path = repo.join("SddIA/core/capability-bindings.md");
    if let Ok(fm) = parse_frontmatter(&bindings_path) {
        if let Some(YamlValue::Sequence(seq)) = fm.get("bindings") {
            for item in seq {
                let cap_id = item
                    .get("capability_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if cap_id == "agent:runtime" {
                    if let Some(YamlValue::Mapping(roles)) = item.get("roles") {
                        let mut set = HashSet::new();
                        for (k, _) in roles {
                            if let Some(role) = k.as_str() {
                                set.insert(role.trim().to_string());
                            }
                        }
                        if !set.is_empty() {
                            return set;
                        }
                    }
                }
            }
        }
    }
    CANONICAL_AGENT_RUNTIME_ROLES
        .iter()
        .map(|s| (*s).to_string())
        .collect()
}

fn load_local_runtime_config(repo: &Path, inputs: &Value) -> Result<Option<Value>, String> {
    if let Some(inline) = inputs.get("llm_runtime") {
        if inline.is_object() {
            return Ok(Some(inline.clone()));
        }
    }
    let path = local_runtime_config_path(repo);
    if !path.is_file() {
        return Ok(None);
    }
    let raw = fs::read_to_string(&path)
        .map_err(|e| format!("llm-runtime.local.json ilegible: {e}"))?;
    let val: Value = serde_json::from_str(&raw)
        .map_err(|e| format!("llm-runtime.local.json JSON inválido: {e}"))?;
    Ok(Some(val))
}

fn env_model_override(provider: &str, profile: ThinkingProfile) -> Option<String> {
    let prov_upper = provider.trim().to_ascii_uppercase().replace('-', "_");
    let prof_upper = profile.as_str().to_ascii_uppercase();
    let specific_key = format!("SDDIA_LLM_MODEL_{prov_upper}_{prof_upper}");
    if let Ok(v) = std::env::var(&specific_key) {
        let t = v.trim();
        if !t.is_empty() {
            return Some(t.to_string());
        }
    }
    if let Some(legacy_key) = profile.legacy_env_key() {
        if let Ok(v) = std::env::var(legacy_key) {
            let t = v.trim();
            if !t.is_empty() {
                return Some(t.to_string());
            }
        }
    }
    if let Ok(v) = std::env::var("SDDIA_AGENT_RUNTIME_MODEL") {
        let t = v.trim();
        if !t.is_empty() {
            return Some(t.to_string());
        }
    }
    None
}

/// Acción determinista `switch-llm-provider`: valida contra `capability-bindings.md` y persiste `.SddIA/llm-runtime.local.json`.
pub fn switch_llm_provider(repo: &Path, inputs: &Value) -> Result<Value, String> {
    let provider = inputs
        .get("provider")
        .or_else(|| inputs.get("active_provider"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "provider requerido para switch-llm-provider".to_string())?;
    let roles = allowed_agent_runtime_roles(repo);
    if !roles.contains(provider) {
        return Err(format!(
            "{ERR_CAPABILITY_BINDING_MISSING}: proveedor '{provider}' no registrado en roles de agent:runtime"
        ));
    }
    let path = local_runtime_config_path(repo);
    let mut cfg = if path.is_file() {
        let raw = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str::<Value>(&raw).unwrap_or_else(|_| json!({}))
    } else {
        let example_path = repo.join(".SddIA/llm-runtime.example.json");
        if example_path.is_file() {
            let raw = fs::read_to_string(&example_path).unwrap_or_default();
            serde_json::from_str::<Value>(&raw).unwrap_or_else(|_| json!({}))
        } else {
            json!({
                "schema_version": "1.0.0",
                "providers": {}
            })
        }
    };
    if !cfg.is_object() {
        cfg = json!({});
    }
    cfg["schema_version"] = json!("1.0.0");
    cfg["active_provider"] = json!(provider);
    let policy = UnattendedPolicy::parse(
        inputs
            .get("unattended_policy")
            .and_then(|v| v.as_str())
            .or_else(|| cfg.get("unattended_policy").and_then(|v| v.as_str())),
    );
    cfg["unattended_policy"] = json!(policy.as_str());
    if cfg.get("providers").and_then(|v| v.as_object()).is_none() {
        cfg["providers"] = json!({});
    }
    if let Some(models) = inputs.get("models").and_then(|v| v.as_object()) {
        if cfg["providers"].get(provider).and_then(|v| v.as_object()).is_none() {
            cfg["providers"][provider] = json!({ "models": {} });
        }
        if cfg["providers"][provider].get("models").and_then(|v| v.as_object()).is_none() {
            cfg["providers"][provider]["models"] = json!({});
        }
        if let Some(target) = cfg["providers"][provider]["models"].as_object_mut() {
            for (k, v) in models {
                target.insert(k.clone(), v.clone());
            }
        }
    }
    if let Some(endpoint) = inputs.get("endpoint").and_then(|v| v.as_str()) {
        if cfg["providers"].get(provider).and_then(|v| v.as_object()).is_none() {
            cfg["providers"][provider] = json!({});
        }
        cfg["providers"][provider]["endpoint"] = json!(endpoint.trim());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let tmp = path.with_extension("json.tmp");
    let serialized = serde_json::to_vec_pretty(&cfg).map_err(|e| e.to_string())?;
    fs::write(&tmp, serialized).map_err(|e| e.to_string())?;
    fs::rename(&tmp, &path).map_err(|e| e.to_string())?;
    let rel = path
        .strip_prefix(repo)
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| path.display().to_string());
    Ok(json!({
        "success": true,
        "active_provider": provider,
        "unattended_policy": policy.as_str(),
        "config_path": rel,
    }))
}

fn parse_env_timeout_secs(key: &str) -> Option<u64> {
    let v = std::env::var(key).ok()?;
    let trimmed = v.trim();
    if trimmed.is_empty() {
        return None;
    }
    let n = trimmed.parse::<u64>().ok()?;
    if n == 0 {
        None
    } else {
        Some(n)
    }
}

fn resolve_timeout_secs(phase_name: &str) -> u64 {
    let phase_l = phase_name.to_lowercase();
    let base = if phase_l.contains("refinamiento mayeuta") {
        parse_env_timeout_secs(ENV_TIMEOUT_REFINE).unwrap_or(DEFAULT_TIMEOUT_REFINE_SECS)
    } else if phase_l.starts_with("ejecuc") {
        parse_env_timeout_secs(ENV_TIMEOUT_EXEC)
            .or_else(|| parse_env_timeout_secs(ENV_TIMEOUT))
            .unwrap_or(DEFAULT_TIMEOUT_SECS)
    } else {
        parse_env_timeout_secs(ENV_TIMEOUT).unwrap_or(DEFAULT_TIMEOUT_SECS)
    };
    if phase_l.starts_with("ejecuc") && base < MIN_EXECUTION_TIMEOUT_SECS {
        MIN_EXECUTION_TIMEOUT_SECS
    } else {
        base
    }
}

fn execution_timeout_floor_applied(phase_name: &str, resolved_secs: u64) -> bool {
    phase_name.to_lowercase().starts_with("ejecuc") && resolved_secs >= MIN_EXECUTION_TIMEOUT_SECS
}

fn split_command(raw: &str) -> Result<Vec<String>, String> {
    let mut parts: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut in_quote: Option<char> = None;
    for c in raw.trim().chars() {
        match (c, in_quote) {
            ('"', None) => in_quote = Some('"'),
            ('\'', None) => in_quote = Some('\''),
            ('"', Some('"')) | ('\'', Some('\'')) => in_quote = None,
            (' ', None) if cur.is_empty() => {}
            (' ', None) => {
                parts.push(std::mem::take(&mut cur));
            }
            (_, _) => cur.push(c),
        }
    }
    if !cur.is_empty() {
        parts.push(cur);
    }
    if parts.is_empty() {
        return Err("SDDIA_AGENT_RUNTIME_COMMAND vacío".into());
    }
    Ok(parts)
}

fn resolve_persist_dir_for_guard(repo: &Path, inputs: &Value, state: &Value) -> Option<String> {
    let ref_val = resolve_persist_ref_value(inputs, state);
    let s = ref_val.as_str().map(str::trim).filter(|s| !s.is_empty())?;
    if Path::new(s).is_absolute() {
        return Some(s.to_string());
    }
    if let Ok(Some(bound)) = project_binding::bind(repo, inputs) {
        if let Ok(p) = project_binding::anchor_persist(&bound.project_root, s) {
            return Some(p.to_string_lossy().replace('\\', "/"));
        }
    }
    Some(s.to_string())
}

fn resolve_persist_ref_value(inputs: &Value, state: &Value) -> Value {
    // Tras workspace-init 1×N, `state.workspace.persist_ref` está anclado al piloto.
    if let Some(ws) = state
        .get("workspace")
        .and_then(|w| w.get("persist_ref"))
        .filter(|v| v.as_str().map(|s| !s.trim().is_empty()).unwrap_or(false))
    {
        return ws.clone();
    }
    inputs
        .get("persist_ref")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| json!(s))
        .unwrap_or(Value::Null)
}

/// Antes del sello de Verificación: si la ref local no existe, checkout vía git-manager.
/// Sin `.git` (tests de payload) no hay rama que cambiar.
fn ensure_verification_branch(repo: &Path, branch: &str) -> Result<(), String> {
    if !repo.join(".git").exists() {
        return Ok(());
    }
    let list = super::capsules::invoke_git_manager_for(repo, repo, "branch_list", &json!({}))?;
    let local = list
        .get("branches")
        .and_then(|v| v.as_array())
        .map(|branches| {
            branches.iter().any(|line| {
                let raw = line.as_str().unwrap_or("").trim();
                let rest = raw.strip_prefix('*').unwrap_or(raw).trim();
                if rest.starts_with("remotes/") {
                    return false;
                }
                rest.split_whitespace().next() == Some(branch)
            })
        })
        .unwrap_or(false);
    if local {
        return Ok(());
    }
    super::capsules::invoke_git_manager_for(
        repo,
        repo,
        "checkout",
        &json!({"branch_name": branch, "create_if_not_exists": false}),
    )?;
    Ok(())
}

fn resolve_execution_id(inputs: &Value, state: &Value) -> Option<String> {
    inputs
        .get("execution_id")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .or_else(|| {
            state
                .get("execution_id")
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
        })
}

/// Guard L-CONFLICT: artefactos con `execution_id` distinto del ciclo vivo.
pub fn check_persist_execution_id_conflict(
    repo: &Path,
    persist_ref: &str,
    live_id: &str,
) -> Result<(), Vec<String>> {
    let dir = if Path::new(persist_ref).is_absolute() {
        PathBuf::from(persist_ref)
    } else {
        repo.join(persist_ref)
    };
    if !dir.is_dir() {
        return Ok(());
    }
    let mut conflicts = Vec::new();
    let entries = fs::read_dir(&dir).map_err(|e| vec![e.to_string()])?;
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if name == "_agent_handoff.md" || !name.ends_with(".md") {
            continue;
        }
        let text = fs::read_to_string(&path).unwrap_or_default();
        let fm = parse_frontmatter_from_str(&text).unwrap_or_default();
        if let Some(existing) = fm.get("execution_id").and_then(|v| v.as_str()) {
            let existing = existing.trim();
            if !existing.is_empty() && existing != live_id {
                conflicts.push(
                    path.strip_prefix(repo)
                        .map(|p| p.to_string_lossy().replace('\\', "/"))
                        .unwrap_or_else(|_| path.display().to_string()),
                );
            }
        }
    }
    if conflicts.is_empty() {
        Ok(())
    } else {
        Err(conflicts)
    }
}

fn upsert_execution_id_in_markdown(text: &str, live_id: &str) -> Option<String> {
    let trimmed = text.trim_start();
    if !trimmed.starts_with("---") {
        return None;
    }
    let rest = trimmed.strip_prefix("---")?;
    let end = rest.find("\n---")?;
    let fm = rest[..end].trim_end();
    let body = rest[end + 4..].trim_start();
    let mut lines: Vec<String> = fm
        .lines()
        .filter(|l| !l.trim_start().starts_with("execution_id:"))
        .map(|l| l.to_string())
        .collect();
    lines.push(format!("execution_id: \"{live_id}\""));
    Some(format!("---\n{}\n---\n{}", lines.join("\n"), body))
}

/// Reentrada bug-fix: alinea `execution_id` de ciclos previos al ciclo vivo (handoff excluido).
pub fn reconcile_persist_execution_id_for_reentry(
    repo: &Path,
    persist_ref: &str,
    live_id: &str,
) -> usize {
    let dir = if Path::new(persist_ref).is_absolute() {
        PathBuf::from(persist_ref)
    } else {
        repo.join(persist_ref)
    };
    if !dir.is_dir() {
        return 0;
    }
    let mut updated = 0usize;
    let entries = match fs::read_dir(&dir) {
        Ok(e) => e,
        Err(_) => return 0,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if name == "_agent_handoff.md" || !name.ends_with(".md") {
            continue;
        }
        let text = fs::read_to_string(&path).unwrap_or_default();
        let fm = parse_frontmatter_from_str(&text).unwrap_or_default();
        let existing = fm
            .get("execution_id")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty());
        if existing == Some(live_id) {
            continue;
        }
        if let Some(updated_text) = upsert_execution_id_in_markdown(&text, live_id) {
            if fs::write(&path, updated_text).is_ok() {
                updated += 1;
            }
        }
    }
    updated
}

fn agent_names(delegates: &[Value]) -> Vec<String> {
    delegates
        .iter()
        .filter_map(|d| d.as_str())
        .filter_map(|s| s.strip_prefix("agent:"))
        .map(str::to_string)
        .collect()
}

fn is_deterministic_agent(name: &str) -> bool {
    DETERMINISTIC_AGENTS.contains(&name)
}

fn is_none_tier(tier: &str) -> bool {
    tier.eq_ignore_ascii_case("none")
}

fn default_thinking_profile_for_agent(name: &str) -> ThinkingProfile {
    match name {
        "mayeuta" | "dedalo" => ThinkingProfile::Reasoning,
        "argos" => ThinkingProfile::Audit,
        "cerbero" | "cumulo" | "radamanto" => ThinkingProfile::Deterministic,
        _ => ThinkingProfile::Execution,
    }
}

/// Lee `llm_profile` de `SddIA/agents/{name}.md`. Fail-soft si el `.md` falta.
/// Devuelve `(llm_profiles_map, dominant_thinking_profile, all_none)`.
pub fn load_llm_profiles_with_thinking(
    repo: &Path,
    agents: &[String],
) -> (Value, ThinkingProfile, bool) {
    let mut map = serde_json::Map::new();
    if agents.is_empty() {
        return (Value::Object(map), ThinkingProfile::Execution, false);
    }
    let mut all_none = true;
    let mut dominant: Option<ThinkingProfile> = None;
    for name in agents {
        let path = repo.join("SddIA/agents").join(format!("{name}.md"));
        if !path.is_file() {
            let inferred = default_thinking_profile_for_agent(name);
            if is_deterministic_agent(name) {
                map.insert(
                    name.clone(),
                    json!({
                        "thinking_profile": "deterministic",
                        "tier": "none",
                    }),
                );
            } else {
                all_none = false;
            }
            dominant = Some(dominant.map_or(inferred, |cur| cur.max(inferred)));
            continue;
        }
        let fm = match parse_frontmatter(&path) {
            Ok(m) => m,
            Err(_) => {
                all_none = false;
                let inferred = default_thinking_profile_for_agent(name);
                dominant = Some(dominant.map_or(inferred, |cur| cur.max(inferred)));
                continue;
            }
        };
        match fm.get("llm_profile") {
            Some(profile) => {
                let mut json_profile = serde_json::to_value(profile).unwrap_or(json!({}));
                let tp_raw = json_profile
                    .get("thinking_profile")
                    .and_then(|v| v.as_str())
                    .or_else(|| json_profile.get("tier").and_then(|v| v.as_str()))
                    .unwrap_or("");
                let tp = ThinkingProfile::parse(tp_raw)
                    .unwrap_or_else(|| default_thinking_profile_for_agent(name));
                if let Some(obj) = json_profile.as_object_mut() {
                    if !obj.contains_key("thinking_profile") {
                        obj.insert("thinking_profile".into(), json!(tp.as_str()));
                    }
                    if !obj.contains_key("tier") {
                        obj.insert("tier".into(), json!(tp.legacy_tier()));
                    }
                }
                let tier_s = json_profile
                    .get("tier")
                    .and_then(|v| v.as_str())
                    .unwrap_or(tp.legacy_tier());
                if tp != ThinkingProfile::Deterministic && !is_none_tier(tier_s) {
                    all_none = false;
                }
                dominant = Some(dominant.map_or(tp, |cur| cur.max(tp)));
                map.insert(name.clone(), json_profile);
            }
            None => {
                let inferred = default_thinking_profile_for_agent(name);
                if is_deterministic_agent(name) {
                    map.insert(
                        name.clone(),
                        json!({
                            "thinking_profile": "deterministic",
                            "tier": "none",
                        }),
                    );
                } else {
                    all_none = false;
                }
                dominant = Some(dominant.map_or(inferred, |cur| cur.max(inferred)));
            }
        }
    }
    let resolved_dominant = dominant.unwrap_or(if all_none {
        ThinkingProfile::Deterministic
    } else {
        ThinkingProfile::Execution
    });
    (Value::Object(map), resolved_dominant, all_none)
}

pub fn enrich_simulated_agent_entry(repo: &Path, delegates: &[Value], entry: &mut Value) {
    let agents = agent_names(delegates);
    let (_profiles, thinking_profile, all_none) = load_llm_profiles_with_thinking(repo, &agents);
    entry["agent_ids"] = json!(agents);
    entry["thinking_profile"] = json!(thinking_profile.as_str());
    if all_none {
        entry["llm_provider"] = json!("deterministic-none");
        entry["resolved_model"] = Value::Null;
        entry["provider_mode"] = json!(ProviderMode::Deterministic.as_str());
    } else {
        if entry.get("llm_provider").is_none() {
            entry["llm_provider"] = json!("cursor");
        }
        if entry.get("resolved_model").is_none() {
            entry["resolved_model"] = Value::Null;
        }
        if entry.get("provider_mode").is_none() {
            entry["provider_mode"] = json!(ProviderMode::InteractiveRelay.as_str());
        }
    }
}

fn truthy_state_flag(state: &Value, key: &str) -> bool {
    match state.get(key) {
        Some(Value::Bool(b)) => *b,
        Some(Value::String(s)) => matches!(s.to_lowercase().as_str(), "true" | "1" | "yes" | "on"),
        Some(Value::Number(n)) => n.as_i64().unwrap_or(0) != 0,
        _ => false,
    }
}

/// Copia flags nativos PPR (#125) al payload del CLI agente (L-STATE-FWD / PPR #136 residual).
fn inject_runtime_evidence_from_state(payload: &mut Value, state: &Value) {
    let Some(obj) = payload.as_object_mut() else {
        return;
    };

    let git = truthy_state_flag(state, "git_manager_invoked");
    let formal = truthy_state_flag(state, "formal_execute_process")
        || truthy_state_flag(state, "tech_triage_formal");

    if git {
        obj.insert("git_manager_invoked".into(), json!(true));
    }
    if formal {
        obj.insert("formal_execute_process".into(), json!(true));
        obj.insert("tech_triage_formal".into(), json!(true));
    }
    if let Some(checks) = state.get("tech_checks") {
        obj.insert("tech_checks".into(), checks.clone());
    }

    let mut runtime_evidence = json!({
        "git_manager_invoked": git,
        "formal_execute_process": formal,
        "tech_triage_formal": truthy_state_flag(state, "tech_triage_formal"),
    });
    if let Some(checks) = state.get("tech_checks") {
        if let Some(re) = runtime_evidence.as_object_mut() {
            re.insert("tech_checks".into(), checks.clone());
        }
    }
    if git || formal || state.get("tech_checks").is_some() {
        obj.insert("runtime_evidence".into(), runtime_evidence);
    }
}

/// Ola 1: fricción de agente en fases no-aduana (no F2/F4/F5) no colapsa PPR.
fn finish_agent_entry(mut entry: Value, process_name: &str, phase_name: &str) -> Value {
    if process_name == "pull-request-review"
        && matches!(phase_name, "Triaje documental" | "Cosecha Kaizen")
        && entry.get("status").and_then(|v| v.as_str()) == Some("failed")
    {
        entry["fail_soft"] = json!(true);
    }
    entry
}

const MCP_BACKEND_UNSUPPORTED: &str = "MCP_BACKEND_UNSUPPORTED";

/// JSON-RPC `initialize` contra el binario del Workspace Server (stdio).
fn smoke_mcp_stdio_server(command: &str, args: &[String]) -> Result<(), String> {
    let init = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {},
            "clientInfo": {"name": "sddia-agent-runtime", "version": "1.0.0"},
        },
    });
    let line = format!("{}\n", serde_json::to_string(&init).map_err(|e| e.to_string())?);
    let mut cmd = Command::new(command);
    cmd.args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    let mut child = cmd.spawn().map_err(|e| format!("mcp spawn: {e}"))?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(line.as_bytes())
            .map_err(|e| format!("mcp stdin: {e}"))?;
    }
    let out = child
        .wait_with_output()
        .map_err(|e| format!("mcp wait: {e}"))?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    let resp_line = stdout.lines().find(|l| !l.trim().is_empty()).unwrap_or("");
    if resp_line.is_empty() {
        return Err("mcp initialize: empty stdout".into());
    }
    let resp: Value = serde_json::from_str(resp_line)
        .map_err(|e| format!("mcp initialize: invalid json: {e}"))?;
    let caps = resp
        .get("result")
        .and_then(|r| r.get("capabilities"))
        .ok_or_else(|| "mcp initialize: missing capabilities".to_string())?;
    if caps.get("tools").is_none() || caps.get("resources").is_none() {
        return Err("mcp initialize: capabilities.tools/resources missing".into());
    }
    Ok(())
}

fn mcp_workspace_cache_root() -> PathBuf {
    if let Ok(v) = std::env::var("SDDIA_AGENT_MCP_WS_ROOT") {
        let t = v.trim();
        if !t.is_empty() {
            return PathBuf::from(t);
        }
    }
    std::env::var("HOME")
        .map(|h| PathBuf::from(h).join(".cache/sddia-agent-mcp"))
        .unwrap_or_else(|_| std::env::temp_dir().join("sddia-agent-mcp"))
}

fn workspace_server_binary(repo: &Path) -> PathBuf {
    let release = repo.join("SddIA/target/release/sddia-workspace-server");
    let debug = repo.join("SddIA/target/debug/sddia-workspace-server");
    let mtime = |p: &Path| p.metadata().ok().and_then(|m| m.modified().ok());
    match (release.is_file(), debug.is_file()) {
        (true, false) => release,
        (false, true) => debug,
        (true, true) => match (mtime(&release), mtime(&debug)) {
            (Some(rt), Some(dt)) if dt > rt => debug,
            _ => release,
        },
        _ => debug,
    }
}

fn agent_runtime_cwd(repo: &Path, project_root: Option<&str>, execution_id: Option<&str>) -> PathBuf {
    if project_root.map(str::trim).is_some_and(|s| !s.is_empty()) {
        let eid = execution_id
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or("default");
        let ws = mcp_workspace_cache_root().join(eid);
        let _ = std::fs::create_dir_all(&ws);
        return ws;
    }
    repo.to_path_buf()
}

fn resolve_runtime_executable(repo: &Path, bin: &str) -> PathBuf {
    let trimmed = bin.trim();
    let path = Path::new(trimmed);
    if path.is_absolute() {
        return path.to_path_buf();
    }
    repo.join(trimmed)
}

fn build_agent_command(
    repo: &Path,
    bin: &str,
    args: &[String],
    cwd: &Path,
    depth: u32,
) -> Command {
    let executable = resolve_runtime_executable(repo, bin);
    let mut cmd = Command::new(&executable);
    cmd.args(args)
        .current_dir(cwd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env(ENV_DEPTH, (depth + 1).to_string());

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        unsafe {
            cmd.pre_exec(|| {
                if libc::setpgid(0, 0) != 0 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
    }

    cmd
}

#[cfg(unix)]
fn kill_process_group(pid: u32) {
    let pgid = pid as i32;
    unsafe {
        libc::kill(-pgid, libc::SIGTERM);
    }
    thread::sleep(Duration::from_millis(KILL_GRACE_MS));
    unsafe {
        libc::kill(-pgid, libc::SIGKILL);
    }
}

#[cfg(not(unix))]
fn kill_process_group(_pid: u32) {
    // Best-effort en hosts no-Unix: el drop de Child intentará wait.
}

use std::sync::mpsc;

fn wait_child_with_timeout(
    child: std::process::Child,
    timeout: Duration,
) -> Result<Output, &'static str> {
    let pid = child.id();
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = tx.send(child.wait_with_output());
    });
    match rx.recv_timeout(timeout) {
        Ok(Ok(out)) => Ok(out),
        Ok(Err(_)) => Err("wait agent-runtime"),
        Err(mpsc::RecvTimeoutError::Timeout) => {
            kill_process_group(pid);
            Err("agent-runtime-timeout")
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => Err("wait agent-runtime"),
    }
}

fn classify_error_code_from_text(raw: &str, explicit_code: Option<&str>) -> Option<String> {
    if let Some(code) = explicit_code.map(str::trim).filter(|s| !s.is_empty()) {
        return Some(match code {
            "rate_limited" | "quota_exceeded" | ERR_PROVIDER_QUOTA_EXCEEDED => {
                ERR_PROVIDER_QUOTA_EXCEEDED.to_string()
            }
            "timeout" | "agent-runtime-timeout" | ERR_PROVIDER_TIMEOUT => {
                ERR_PROVIDER_TIMEOUT.to_string()
            }
            "upstream_unavailable" | "unavailable" | ERR_PROVIDER_UNAVAILABLE => {
                ERR_PROVIDER_UNAVAILABLE.to_string()
            }
            other => other.to_string(),
        });
    }
    let low = raw.to_ascii_lowercase();
    if low.contains("quota") || low.contains("rate_limit") || low.contains("rate-limit") || low.contains("429") {
        Some(ERR_PROVIDER_QUOTA_EXCEEDED.to_string())
    } else if low.contains("timeout") {
        Some(ERR_PROVIDER_TIMEOUT.to_string())
    } else if low.contains("unavailable") || low.contains("connection refused") {
        Some(ERR_PROVIDER_UNAVAILABLE.to_string())
    } else {
        None
    }
}

fn execute_cli_subcommand(
    provider_id: &str,
    command_raw: &str,
    req: &AgentInvocationRequest,
) -> AgentExecutionResult {
    let parts = match split_command(command_raw) {
        Ok(p) => p,
        Err(e) => {
            return AgentExecutionResult {
                status: "failed".into(),
                provider_id: provider_id.into(),
                provider_mode: ProviderMode::HeadlessCli,
                thinking_profile: req.thinking_profile,
                resolved_model: req.resolved_model.clone(),
                message: None,
                note: None,
                error_code: Some(ERR_PROVIDER_UNAVAILABLE.into()),
                error: Some(e),
                runtime_response: None,
                raw_stdout: None,
                timeout_secs: None,
                timeout_floor_applied: req.timeout_floor_applied,
            };
        }
    };
    let (bin, args) = match parts.split_first() {
        Some((b, a)) => (b.clone(), a.to_vec()),
        None => {
            return AgentExecutionResult {
                status: "failed".into(),
                provider_id: provider_id.into(),
                provider_mode: ProviderMode::HeadlessCli,
                thinking_profile: req.thinking_profile,
                resolved_model: req.resolved_model.clone(),
                message: None,
                note: None,
                error_code: Some(ERR_PROVIDER_UNAVAILABLE.into()),
                error: Some("comando runtime vacío".into()),
                runtime_response: None,
                raw_stdout: None,
                timeout_secs: None,
                timeout_floor_applied: req.timeout_floor_applied,
            };
        }
    };

    let mut child = match build_agent_command(&req.repo_root, &bin, &args, &req.cwd, req.depth).spawn()
    {
        Ok(c) => c,
        Err(e) => {
            return AgentExecutionResult {
                status: "failed".into(),
                provider_id: provider_id.into(),
                provider_mode: ProviderMode::HeadlessCli,
                thinking_profile: req.thinking_profile,
                resolved_model: req.resolved_model.clone(),
                message: None,
                note: None,
                error_code: Some(ERR_PROVIDER_UNAVAILABLE.into()),
                error: Some(format!("spawn agent-runtime: {e}")),
                runtime_response: None,
                raw_stdout: None,
                timeout_secs: None,
                timeout_floor_applied: req.timeout_floor_applied,
            };
        }
    };

    if let Some(mut stdin) = child.stdin.take() {
        let body = serde_json::to_string(&req.payload).unwrap_or_else(|_| "{}".into());
        if let Err(e) = stdin.write_all(body.as_bytes()) {
            return AgentExecutionResult {
                status: "failed".into(),
                provider_id: provider_id.into(),
                provider_mode: ProviderMode::HeadlessCli,
                thinking_profile: req.thinking_profile,
                resolved_model: req.resolved_model.clone(),
                message: None,
                note: None,
                error_code: None,
                error: Some(format!("stdin agent-runtime: {e}")),
                runtime_response: None,
                raw_stdout: None,
                timeout_secs: None,
                timeout_floor_applied: req.timeout_floor_applied,
            };
        }
    }

    let out = match wait_child_with_timeout(child, Duration::from_secs(req.timeout_secs)) {
        Ok(o) => o,
        Err("agent-runtime-timeout") => {
            return AgentExecutionResult {
                status: "failed".into(),
                provider_id: provider_id.into(),
                provider_mode: ProviderMode::HeadlessCli,
                thinking_profile: req.thinking_profile,
                resolved_model: req.resolved_model.clone(),
                message: None,
                note: None,
                error_code: Some(ERR_PROVIDER_TIMEOUT.into()),
                error: Some("agent-runtime-timeout".into()),
                runtime_response: None,
                raw_stdout: None,
                timeout_secs: Some(req.timeout_secs),
                timeout_floor_applied: req.timeout_floor_applied,
            };
        }
        Err(e) => {
            return AgentExecutionResult {
                status: "failed".into(),
                provider_id: provider_id.into(),
                provider_mode: ProviderMode::HeadlessCli,
                thinking_profile: req.thinking_profile,
                resolved_model: req.resolved_model.clone(),
                message: None,
                note: None,
                error_code: None,
                error: Some(e.to_string()),
                runtime_response: None,
                raw_stdout: None,
                timeout_secs: None,
                timeout_floor_applied: req.timeout_floor_applied,
            };
        }
    };

    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    let line = stdout
        .lines()
        .map(str::trim)
        .rfind(|l| !l.is_empty())
        .unwrap_or("");

    if line.is_empty() {
        let err_msg = if stderr.trim().is_empty() {
            format!(
                "agent-runtime sin stdout (exit {})",
                out.status.code().unwrap_or(1)
            )
        } else {
            stderr.trim().to_string()
        };
        let code = classify_error_code_from_text(&err_msg, None);
        return AgentExecutionResult {
            status: "failed".into(),
            provider_id: provider_id.into(),
            provider_mode: ProviderMode::HeadlessCli,
            thinking_profile: req.thinking_profile,
            resolved_model: req.resolved_model.clone(),
            message: None,
            note: None,
            error_code: code,
            error: Some(err_msg),
            runtime_response: None,
            raw_stdout: None,
            timeout_secs: None,
            timeout_floor_applied: req.timeout_floor_applied,
        };
    }

    let body: Value = match serde_json::from_str(line) {
        Ok(v) => v,
        Err(e) => {
            return AgentExecutionResult {
                status: "failed".into(),
                provider_id: provider_id.into(),
                provider_mode: ProviderMode::HeadlessCli,
                thinking_profile: req.thinking_profile,
                resolved_model: req.resolved_model.clone(),
                message: None,
                note: None,
                error_code: None,
                error: Some(format!("JSON agent-runtime: {e}")),
                runtime_response: None,
                raw_stdout: Some(line.to_string()),
                timeout_secs: None,
                timeout_floor_applied: req.timeout_floor_applied,
            };
        }
    };

    let success = body
        .get("success")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let data = body.get("data").cloned().unwrap_or(json!({}));
    let status = data
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or(if success { "executed" } else { "failed" });

    let mut normalized = match status {
        "executed" | "awaiting_agents" | "failed" | "blocked" => status.to_string(),
        "awaiting" => "awaiting_agents".to_string(),
        _ if success => "executed".to_string(),
        _ => "failed".to_string(),
    };

    let message = data
        .get("message")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let mut error = body
        .get("error")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string);
    if matches!(normalized.as_str(), "blocked" | "failed")
        && error.as_deref().unwrap_or("").is_empty()
    {
        if let Some(ref msg) = message {
            if !msg.is_empty() {
                error = Some(msg.clone());
            }
        }
    }
    if !out.status.success() && normalized == "executed" {
        normalized = "failed".to_string();
        error = Some(format!(
            "agent-runtime exit {} pese a status executed",
            out.status.code().unwrap_or(1)
        ));
    }
    let explicit_code = data
        .get("error_code")
        .or_else(|| body.get("error_code"))
        .and_then(|v| v.as_str());
    let combined_err = error
        .as_deref()
        .or(message.as_deref())
        .unwrap_or("");
    let error_code = if matches!(normalized.as_str(), "failed" | "blocked") {
        classify_error_code_from_text(combined_err, explicit_code)
    } else {
        None
    };
    let resolved_model = data
        .get("resolved_model")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .or_else(|| req.resolved_model.clone());

    AgentExecutionResult {
        status: normalized,
        provider_id: provider_id.into(),
        provider_mode: ProviderMode::HeadlessCli,
        thinking_profile: req.thinking_profile,
        resolved_model,
        message,
        note: None,
        error_code,
        error,
        runtime_response: Some(body),
        raw_stdout: None,
        timeout_secs: None,
        timeout_floor_applied: req.timeout_floor_applied,
    }
}

pub struct CursorAgentRuntimeAdapter {
    pub command: Option<String>,
}

impl AgentRuntimePort for CursorAgentRuntimeAdapter {
    fn provider_id(&self) -> &str {
        "cursor"
    }

    fn mode(&self) -> ProviderMode {
        if self.command.as_deref().map(str::trim).is_some_and(|s| !s.is_empty()) {
            ProviderMode::HeadlessCli
        } else {
            ProviderMode::InteractiveRelay
        }
    }

    fn supports_unattended(&self) -> bool {
        self.command.as_deref().map(str::trim).is_some_and(|s| !s.is_empty())
    }

    fn invoke(&self, req: &AgentInvocationRequest) -> AgentExecutionResult {
        if let Some(cmd) = self.command.as_deref().filter(|s| !s.trim().is_empty()) {
            return execute_cli_subcommand("cursor", cmd, req);
        }
        if req.unattended_policy == UnattendedPolicy::FailFast {
            return AgentExecutionResult {
                status: "failed".into(),
                provider_id: "cursor".into(),
                provider_mode: ProviderMode::InteractiveRelay,
                thinking_profile: req.thinking_profile,
                resolved_model: req.resolved_model.clone(),
                message: None,
                note: None,
                error_code: Some(ERR_PROVIDER_UNATTENDED_UNSUPPORTED.into()),
                error: Some(format!(
                    "{ERR_PROVIDER_UNATTENDED_UNSUPPORTED}: cursor sin comando CLI bajo unattended_policy=fail_fast"
                )),
                runtime_response: None,
                raw_stdout: None,
                timeout_secs: None,
                timeout_floor_applied: req.timeout_floor_applied,
            };
        }
        AgentExecutionResult {
            status: "simulated".into(),
            provider_id: "cursor".into(),
            provider_mode: ProviderMode::InteractiveRelay,
            thinking_profile: req.thinking_profile,
            resolved_model: req.resolved_model.clone(),
            message: None,
            note: Some("agentes IDE; sin SDDIA_AGENT_RUNTIME_COMMAND".into()),
            error_code: None,
            error: None,
            runtime_response: None,
            raw_stdout: None,
            timeout_secs: None,
            timeout_floor_applied: req.timeout_floor_applied,
        }
    }
}

pub struct AntigravityAgentRuntimeAdapter {
    pub command: Option<String>,
    pub explicit_local_config: bool,
}

impl AgentRuntimePort for AntigravityAgentRuntimeAdapter {
    fn provider_id(&self) -> &str {
        "antigravity"
    }

    fn mode(&self) -> ProviderMode {
        if self.command.as_deref().map(str::trim).is_some_and(|s| !s.is_empty()) {
            ProviderMode::HeadlessCli
        } else {
            ProviderMode::InteractiveRelay
        }
    }

    fn supports_unattended(&self) -> bool {
        self.command.as_deref().map(str::trim).is_some_and(|s| !s.is_empty())
    }

    fn invoke(&self, req: &AgentInvocationRequest) -> AgentExecutionResult {
        if let Some(cmd) = self.command.as_deref().filter(|s| !s.trim().is_empty()) {
            return execute_cli_subcommand("antigravity", cmd, req);
        }
        if req.unattended_policy == UnattendedPolicy::FailFast {
            return AgentExecutionResult {
                status: "failed".into(),
                provider_id: "antigravity".into(),
                provider_mode: ProviderMode::InteractiveRelay,
                thinking_profile: req.thinking_profile,
                resolved_model: req.resolved_model.clone(),
                message: None,
                note: None,
                error_code: Some(ERR_PROVIDER_UNATTENDED_UNSUPPORTED.into()),
                error: Some(format!(
                    "{ERR_PROVIDER_UNATTENDED_UNSUPPORTED}: antigravity en modo interactive_relay bajo unattended_policy=fail_fast"
                )),
                runtime_response: None,
                raw_stdout: None,
                timeout_secs: None,
                timeout_floor_applied: req.timeout_floor_applied,
            };
        }
        let status = if self.explicit_local_config {
            "awaiting_agents"
        } else {
            "simulated"
        };
        AgentExecutionResult {
            status: status.into(),
            provider_id: "antigravity".into(),
            provider_mode: ProviderMode::InteractiveRelay,
            thinking_profile: req.thinking_profile,
            resolved_model: req.resolved_model.clone(),
            message: Some("relevo interactivo en proveedor antigravity".into()),
            note: Some("proveedor antigravity (interactive_relay)".into()),
            error_code: None,
            error: None,
            runtime_response: None,
            raw_stdout: None,
            timeout_secs: None,
            timeout_floor_applied: req.timeout_floor_applied,
        }
    }
}

pub struct LocalLlmAgentRuntimeAdapter {
    pub command: Option<String>,
    pub endpoint: Option<String>,
}

impl AgentRuntimePort for LocalLlmAgentRuntimeAdapter {
    fn provider_id(&self) -> &str {
        "local-llm"
    }

    fn mode(&self) -> ProviderMode {
        if self.command.as_deref().map(str::trim).is_some_and(|s| !s.is_empty()) {
            ProviderMode::HeadlessCli
        } else {
            ProviderMode::HeadlessApi
        }
    }

    fn supports_unattended(&self) -> bool {
        true
    }

    fn invoke(&self, req: &AgentInvocationRequest) -> AgentExecutionResult {
        if let Some(cmd) = self.command.as_deref().filter(|s| !s.trim().is_empty()) {
            return execute_cli_subcommand("local-llm", cmd, req);
        }
        let ep = self
            .endpoint
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let msg = match ep {
            Some(url) => format!(
                "{ERR_PROVIDER_UNAVAILABLE}: endpoint local-llm '{url}' sin transporte activo configurado"
            ),
            None => format!(
                "{ERR_PROVIDER_UNAVAILABLE}: proveedor local-llm sin endpoint ni comando configurado en .SddIA/llm-runtime.local.json"
            ),
        };
        AgentExecutionResult {
            status: "failed".into(),
            provider_id: "local-llm".into(),
            provider_mode: ProviderMode::HeadlessApi,
            thinking_profile: req.thinking_profile,
            resolved_model: req.resolved_model.clone(),
            message: None,
            note: None,
            error_code: Some(ERR_PROVIDER_UNAVAILABLE.into()),
            error: Some(msg),
            runtime_response: None,
            raw_stdout: None,
            timeout_secs: None,
            timeout_floor_applied: req.timeout_floor_applied,
        }
    }
}

pub struct ResolvedRuntimeContext {
    pub port: Arc<dyn AgentRuntimePort>,
    pub provider_id: String,
    pub resolved_model: Option<String>,
    pub unattended_policy: UnattendedPolicy,
    pub has_command: bool,
}

/// Factoría DI perezosa (`AgentRuntimeDiFactory`, AUT-122).
/// Solo se invoca cuando `thinking_profile != ThinkingProfile::Deterministic`.
pub struct AgentRuntimeDiFactory;

impl AgentRuntimeDiFactory {
    pub fn resolve(
        repo: &Path,
        inputs: &Value,
        thinking_profile: ThinkingProfile,
    ) -> Result<ResolvedRuntimeContext, (String, String)> {
        let env_cmd = std::env::var(ENV_CMD)
            .ok()
            .map(|v| v.trim().to_string())
            .filter(|s| !s.is_empty());
        if let Some(mock) = current_mock_agent_runtime() {
            let pid = mock.provider_id().to_string();
            let model = env_model_override(&pid, thinking_profile);
            return Ok(ResolvedRuntimeContext {
                port: mock,
                provider_id: pid,
                resolved_model: model,
                unattended_policy: UnattendedPolicy::PauseAwaitingAgents,
                has_command: false,
            });
        }

        let local_cfg = load_local_runtime_config(repo, inputs)
            .map_err(|e| (ERR_CAPABILITY_BINDING_MISSING.to_string(), e))?;
        let explicit_local = local_cfg.is_some();

        let active_provider = local_cfg
            .as_ref()
            .and_then(|c| c.get("active_provider").and_then(|v| v.as_str()))
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .or_else(|| {
                std::env::var("SDDIA_LLM_ACTIVE_PROVIDER")
                    .ok()
                    .map(|v| v.trim().to_string())
                    .filter(|s| !s.is_empty())
            })
            .unwrap_or_else(|| "cursor".to_string());

        let allowed_roles = allowed_agent_runtime_roles(repo);
        if !allowed_roles.contains(&active_provider) {
            return Err((
                ERR_CAPABILITY_BINDING_MISSING.to_string(),
                format!(
                    "{ERR_CAPABILITY_BINDING_MISSING}: proveedor '{active_provider}' no figura en roles de agent:runtime"
                ),
            ));
        }

        let unattended_policy = UnattendedPolicy::parse(
            local_cfg
                .as_ref()
                .and_then(|c| c.get("unattended_policy").and_then(|v| v.as_str())),
        );

        let provider_cfg = local_cfg
            .as_ref()
            .and_then(|c| c.get("providers"))
            .and_then(|p| p.get(&active_provider));

        let cfg_model = provider_cfg
            .and_then(|p| p.get("models"))
            .and_then(|m| m.get(thinking_profile.as_str()))
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string);

        let resolved_model = env_model_override(&active_provider, thinking_profile).or(cfg_model);

        if explicit_local
            && thinking_profile != ThinkingProfile::Deterministic
            && active_provider != "test"
        {
            let provider_block_present = provider_cfg.is_some()
                || local_cfg
                    .as_ref()
                    .and_then(|c| c.get("strict_models"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
            if provider_block_present && resolved_model.is_none() {
                return Err((
                    ERR_LLM_MODEL_UNRESOLVED.to_string(),
                    format!(
                        "{ERR_LLM_MODEL_UNRESOLVED}: sin modelo para perfil '{}' en proveedor '{}'",
                        thinking_profile.as_str(),
                        active_provider
                    ),
                ));
            }
        }

        let cfg_command = provider_cfg
            .and_then(|p| p.get("command"))
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .or(env_cmd);

        let cfg_endpoint = provider_cfg
            .and_then(|p| p.get("endpoint"))
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .or_else(|| {
                std::env::var("SDDIA_LOCAL_LLM_ENDPOINT")
                    .ok()
                    .map(|v| v.trim().to_string())
                    .filter(|s| !s.is_empty())
            });

        let has_command = cfg_command.is_some();
        let port: Arc<dyn AgentRuntimePort> = match active_provider.as_str() {
            "cursor" => Arc::new(CursorAgentRuntimeAdapter {
                command: cfg_command,
            }),
            "antigravity" => Arc::new(AntigravityAgentRuntimeAdapter {
                command: cfg_command,
                explicit_local_config: explicit_local,
            }),
            "local-llm" => Arc::new(LocalLlmAgentRuntimeAdapter {
                command: cfg_command,
                endpoint: cfg_endpoint,
            }),
            "test" => Arc::new(InMemoryAgentRuntimeMock {
                resolved_model_override: resolved_model.clone(),
                ..Default::default()
            }),
            other => {
                return Err((
                    ERR_CAPABILITY_BINDING_MISSING.to_string(),
                    format!("{ERR_CAPABILITY_BINDING_MISSING}: proveedor no soportado '{other}'"),
                ));
            }
        };

        Ok(ResolvedRuntimeContext {
            port,
            provider_id: active_provider,
            resolved_model,
            unattended_policy,
            has_command,
        })
    }
}

/// Invoca el puerto DI de runtime para una fase de agentes (`AgentRuntimePort`, AUT-122).
pub fn invoke_agent_phase(
    repo: &Path,
    process_name: &str,
    phase_name: &str,
    delegates: &[Value],
    inputs: &Value,
    state: &Value,
    di_binding: Option<Value>,
) -> Value {
    let agents = agent_names(delegates);
    let (llm_profiles, thinking_profile, all_none) = load_llm_profiles_with_thinking(repo, &agents);

    let mut entry = json!({
        "phase_name": phase_name,
        "delegates_to": delegates,
        "handler": "agent-runtime",
        "agent_ids": agents,
        "thinking_profile": thinking_profile.as_str(),
    });
    if let Some(di) = &di_binding {
        entry["di_binding"] = di.clone();
    }

    if lab_relay_active() {
        eprintln!("agent-runtime: lab-relay activo (bóveda ignorada)");
        entry["status"] = json!("simulated");
        entry["note"] = json!("SDDIA_AGENT_RELAY_IDE=1; relevo IDE");
        entry["llm_provider"] = json!("cursor");
        entry["resolved_model"] = Value::Null;
        entry["provider_mode"] = json!(ProviderMode::InteractiveRelay.as_str());
        return finish_agent_entry(entry, process_name, phase_name);
    }

    if agent_runtime_depth() >= 1 {
        entry["status"] = json!("simulated");
        entry["note"] = json!("reentry-guard: SDDIA_AGENT_RUNTIME_DEPTH>=1");
        entry["llm_provider"] = json!("cursor");
        entry["resolved_model"] = Value::Null;
        entry["provider_mode"] = json!(ProviderMode::InteractiveRelay.as_str());
        return finish_agent_entry(entry, process_name, phase_name);
    }

    // Invariante de evaluación perezosa: si todos los agentes son `deterministic`,
    // cortocircuita sin evaluar `.SddIA/llm-runtime.local.json` ni spawnear comando.
    if all_none {
        entry["status"] = json!("executed");
        entry["note"] = json!("deterministic-agent-no-llm");
        entry["llm_profiles"] = llm_profiles;
        entry["llm_provider"] = json!("deterministic-none");
        entry["resolved_model"] = Value::Null;
        entry["provider_mode"] = json!(ProviderMode::Deterministic.as_str());
        return finish_agent_entry(entry, process_name, phase_name);
    }

    let execution_id = resolve_execution_id(inputs, state);
    let persist_ref_val = resolve_persist_ref_value(inputs, state);
  // AUT-134: PPR audita PR sellado; prohibido mutar cascada en disco (reconcile execution_id).
    if process_name != "pull-request-review" {
        if let (Some(live_id), Some(persist)) = (
            execution_id.as_deref(),
            resolve_persist_dir_for_guard(repo, inputs, state),
        ) {
            reconcile_persist_execution_id_for_reentry(repo, &persist, live_id);
            if let Err(conflicts) = check_persist_execution_id_conflict(repo, &persist, live_id) {
                entry["status"] = json!("failed");
                entry["error"] = json!("persist-execution-id-conflict");
                entry["conflict_paths"] = json!(conflicts);
                entry["llm_provider"] = json!("cursor");
                entry["resolved_model"] = Value::Null;
                entry["provider_mode"] = json!(ProviderMode::InteractiveRelay.as_str());
                return finish_agent_entry(entry, process_name, phase_name);
            }
        }
    }

    let resolved_ctx = match AgentRuntimeDiFactory::resolve(repo, inputs, thinking_profile) {
        Ok(ctx) => ctx,
        Err((code, msg)) => {
            entry["status"] = json!("failed");
            entry["error_code"] = json!(code);
            entry["error"] = json!(msg);
            entry["llm_provider"] = json!("unresolved");
            entry["resolved_model"] = Value::Null;
            entry["provider_mode"] = json!(ProviderMode::InteractiveRelay.as_str());
            return finish_agent_entry(entry, process_name, phase_name);
        }
    };

    entry["llm_provider"] = json!(resolved_ctx.provider_id);
    entry["resolved_model"] = match &resolved_ctx.resolved_model {
        Some(m) => json!(m),
        None => Value::Null,
    };
    entry["provider_mode"] = json!(resolved_ctx.port.mode().as_str());

    let branch_name = inputs
        .get("branch_name")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| json!(s))
        .or_else(|| {
            inputs
                .get("pr_branch")
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|s| json!(s))
        })
        .unwrap_or(Value::Null);

    let project_root: Option<String> = state
        .get("project_root")
        .and_then(|v| v.as_str())
        .or_else(|| {
            state
                .get("workspace")
                .and_then(|w| w.get("project_root"))
                .and_then(|v| v.as_str())
        })
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .or_else(|| {
            inputs
                .get("project_slug")
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|s| !s.is_empty())?;
            project_binding::bind(repo, inputs)
                .ok()
                .flatten()
                .map(|b| b.project_root.to_string_lossy().replace('\\', "/"))
        });

    let mcp_servers = if resolved_ctx.has_command {
        if let Some(ref root) = project_root {
            let command = workspace_server_binary(repo).display().to_string();
            let repo_s = repo.display().to_string();
            let mut args = vec![
                "--root".to_string(),
                root.clone(),
                "--sddia-repo".to_string(),
                repo_s.clone(),
                "--instance-root".to_string(),
                repo_s,
            ];
            if let Some(ws) = state
                .get("workspace_path")
                .or_else(|| state.get("workspace").and_then(|w| w.get("workspace_path")))
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|s| !s.is_empty())
            {
                args.push("--workspace-path".to_string());
                args.push(ws.to_string());
            }
            let env_keys: Vec<&str> = if inputs
                .get("correlation_id")
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .is_some()
            {
                vec!["SDDIA_CORRELATION_ID"]
            } else {
                vec![]
            };
            Some(json!([{
                "name": "sddia-workspace-server",
                "command": command,
                "args": args,
                "env_keys": env_keys,
            }]))
        } else {
            None
        }
    } else {
        None
    };

    if let Some(servers) = mcp_servers.as_ref().and_then(|v| v.as_array()) {
        if !env_truthy("SDDIA_LAB_SKIP_MCP_SMOKE") {
            for srv in servers {
                let command = srv.get("command").and_then(|v| v.as_str());
                if command.is_none() {
                    entry["status"] = json!("failed");
                    entry["error_code"] = json!(MCP_BACKEND_UNSUPPORTED);
                    entry["error"] = json!("mcp_servers.command missing");
                    return finish_agent_entry(entry, process_name, phase_name);
                }
                let args: Vec<String> = srv
                    .get("args")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|x| x.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                if let Err(e) = smoke_mcp_stdio_server(command.unwrap(), &args) {
                    entry["status"] = json!("failed");
                    entry["error_code"] = json!(MCP_BACKEND_UNSUPPORTED);
                    entry["error"] = json!(format!("{MCP_BACKEND_UNSUPPORTED}: {e}"));
                    return finish_agent_entry(entry, process_name, phase_name);
                }
            }
        }
    }

    let mut payload = json!({
        "operation": "AGENT_PHASE",
        "process_name": process_name,
        "phase_name": phase_name,
        "agents": agents,
        "thinking_profile": thinking_profile.as_str(),
        "llm_provider": resolved_ctx.provider_id,
        "resolved_model": resolved_ctx.resolved_model,
        "llm_profiles": llm_profiles,
        "persist_ref": persist_ref_val,
        "branch_name": branch_name,
        "execution_id": execution_id,
        "correlation_id": inputs.get("correlation_id"),
        "pbi_ref": inputs.get("pbi_ref"),
        "inputs": inputs,
        "workspace_path": state.get("workspace_path")
            .or_else(|| state.get("workspace").and_then(|w| w.get("workspace_path"))),
        "repo_root": repo.display().to_string(),
    });
    if let Some(ref root) = project_root {
        payload["project_root"] = json!(root);
    }
    if let Some(servers) = mcp_servers {
        payload["mcp_servers"] = servers;
    }
    inject_runtime_evidence_from_state(&mut payload, state);
    if let Some(di) = di_binding {
        if let Some(obj) = payload.as_object_mut() {
            obj.insert("di_binding".into(), di);
        }
    }

    if phase_name == "Verificación" && resolved_ctx.has_command {
        let branch = branch_name
            .as_str()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        if branch.is_none() || execution_id.is_none() {
            entry["status"] = json!("failed");
            entry["error_code"] = json!("verification-handoff-incomplete");
            entry["error"] = json!(
                "handoff de Verificación sin branch o execution_id; no se escribe validacion.md"
            );
            return finish_agent_entry(entry, process_name, phase_name);
        }
        if let Err(e) = ensure_verification_branch(repo, branch.expect("branch")) {
            entry["status"] = json!("failed");
            entry["error_code"] = json!("verification-branch-checkout");
            entry["error"] = json!(e);
            return finish_agent_entry(entry, process_name, phase_name);
        }
    }

    let timeout_secs = resolve_timeout_secs(phase_name);
    let mut timeout_floor_applied = false;
    if execution_timeout_floor_applied(phase_name, timeout_secs) {
        let without_floor = parse_env_timeout_secs(ENV_TIMEOUT_EXEC)
            .or_else(|| parse_env_timeout_secs(ENV_TIMEOUT))
            .unwrap_or(DEFAULT_TIMEOUT_SECS);
        if without_floor < MIN_EXECUTION_TIMEOUT_SECS {
            timeout_floor_applied = true;
            entry["timeout_floor_applied"] = json!(true);
        }
    }

    let req = AgentInvocationRequest {
        repo_root: repo.to_path_buf(),
        process_name: process_name.to_string(),
        phase_name: phase_name.to_string(),
        agents,
        thinking_profile,
        resolved_model: resolved_ctx.resolved_model,
        unattended_policy: resolved_ctx.unattended_policy,
        payload,
        cwd: agent_runtime_cwd(repo, project_root.as_deref(), execution_id.as_deref()),
        timeout_secs,
        timeout_floor_applied,
        depth: agent_runtime_depth(),
    };

    let result = resolved_ctx.port.invoke(&req);
    entry["status"] = json!(result.status);
    entry["llm_provider"] = json!(result.provider_id);
    entry["provider_mode"] = json!(result.provider_mode.as_str());
    entry["thinking_profile"] = json!(result.thinking_profile.as_str());
    entry["resolved_model"] = match result.resolved_model {
        Some(m) => json!(m),
        None => Value::Null,
    };
    if let Some(msg) = result.message {
        entry["message"] = json!(msg);
    }
    if let Some(note) = result.note {
        entry["note"] = json!(note);
    }
    if let Some(code) = result.error_code {
        entry["error_code"] = json!(code);
    }
    if let Some(err) = result.error {
        entry["error"] = json!(err);
    }
    if let Some(to) = result.timeout_secs {
        entry["timeout_secs"] = json!(to);
    }
    if let Some(raw_out) = result.raw_stdout {
        entry["raw_stdout"] = json!(raw_out);
    }
    if let Some(resp) = result.runtime_response {
        entry["runtime_response"] = resp;
    }
    finish_agent_entry(entry, process_name, phase_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;
    fn env_lock() -> crate::engine::eda_port::test_lab::ParallelLabGuard {
        crate::engine::eda_port::test_lab::ParallelLabGuard::acquire()
    }

    fn clear_agent_env() {
        for key in [
            ENV_CMD,
            ENV_RELAY,
            ENV_DEPTH,
            ENV_TIMEOUT,
            ENV_TIMEOUT_EXEC,
        ] {
            std::env::remove_var(key);
        }
    }

    fn mock_script(dir: &Path, body: &str) -> PathBuf {
        let script = dir.join("mock-agent.sh");
        fs::write(&script, body).unwrap();
        let mut perms = fs::metadata(&script).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script, perms).unwrap();
        script
    }

    #[test]
    fn not_configured_returns_simulated() {
        let _guard = env_lock();
        clear_agent_env();
        let entry = invoke_agent_phase(
            Path::new("."),
            "bug-fix",
            "Diseño del fix",
            &[json!("agent:dedalo")],
            &json!({}),
            &json!({}),
            None,
        );
        assert_eq!(entry["status"], "simulated");
    }

    #[test]
    fn relay_flag_forces_simulated_even_with_command() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-relay-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = mock_script(
            &dir,
            "#!/bin/sh\necho '{\"success\":true,\"data\":{\"status\":\"executed\"}}'\n",
        );
        std::env::set_var(ENV_CMD, script.display().to_string());
        std::env::set_var(ENV_RELAY, "1");
        let entry = invoke_agent_phase(
            &dir,
            "feature",
            "Ejecución",
            &[json!("agent:tekton")],
            &json!({}),
            &json!({}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        std::env::remove_var(ENV_RELAY);
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(entry["status"], "simulated");
        assert!(entry["note"]
            .as_str()
            .unwrap_or("")
            .contains("SDDIA_AGENT_RELAY_IDE"));
    }

    #[test]
    fn reentry_guard_skips_spawn() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-depth-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = mock_script(
            &dir,
            "#!/bin/sh\necho '{\"success\":true,\"data\":{\"status\":\"executed\"}}'\n",
        );
        std::env::set_var(ENV_CMD, script.display().to_string());
        std::env::set_var(ENV_DEPTH, "1");
        let entry = invoke_agent_phase(
            &dir,
            "feature",
            "Ejecución",
            &[json!("agent:tekton")],
            &json!({}),
            &json!({}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        std::env::remove_var(ENV_DEPTH);
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(entry["status"], "simulated");
        assert!(entry["note"].as_str().unwrap_or("").contains("reentry-guard"));
    }

    #[test]
    fn timeout_kills_hanging_command() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-to-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = mock_script(&dir, "#!/bin/sh\nsleep 30\n");
        std::env::set_var(ENV_CMD, script.display().to_string());
        std::env::set_var(ENV_TIMEOUT, "1");
        let entry = invoke_agent_phase(
            &dir,
            "feature",
            "Estabilización de Requisitos",
            &[json!("agent:mayeuta")],
            &json!({}),
            &json!({}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        std::env::remove_var(ENV_TIMEOUT);
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(entry["status"], "failed");
        assert_eq!(entry["error"], "agent-runtime-timeout");
        assert_eq!(entry["phase_name"], "Estabilización de Requisitos");
        assert_eq!(entry["timeout_secs"], 1);
    }

    #[test]
    fn resolve_timeout_secs_honors_hook_env_180() {
        let _guard = env_lock();
        clear_agent_env();
        std::env::set_var(ENV_TIMEOUT, "180");
        assert_eq!(resolve_timeout_secs("Local QA"), 180);
        std::env::remove_var(ENV_TIMEOUT);
        assert_eq!(resolve_timeout_secs("Local QA"), DEFAULT_TIMEOUT_SECS);
    }

    #[test]
    fn execution_phase_timeout_floor_2400() {
        let _guard = env_lock();
        clear_agent_env();
        std::env::remove_var(ENV_TIMEOUT_EXEC);
        std::env::remove_var(ENV_TIMEOUT);
        assert_eq!(resolve_timeout_secs("Ejecución"), MIN_EXECUTION_TIMEOUT_SECS);
        std::env::set_var(ENV_TIMEOUT, "600");
        assert_eq!(resolve_timeout_secs("Ejecución"), MIN_EXECUTION_TIMEOUT_SECS);
        std::env::set_var(ENV_TIMEOUT, "3600");
        assert_eq!(resolve_timeout_secs("Ejecución"), 3600);
    }

    #[test]
    fn parse_env_timeout_secs_ignores_empty() {
        let _guard = env_lock();
        std::env::set_var(ENV_TIMEOUT, "   ");
        assert_eq!(parse_env_timeout_secs(ENV_TIMEOUT), None);
        std::env::remove_var(ENV_TIMEOUT);
    }

    #[test]
    fn configured_cli_marks_executed() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-rt-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = mock_script(
            &dir,
            "#!/bin/sh\ncat >/dev/null\necho '{\"success\":true,\"data\":{\"status\":\"executed\",\"message\":\"ok\"}}'\n",
        );

        std::env::set_var(ENV_CMD, script.display().to_string());
        let entry = invoke_agent_phase(
            &dir,
            "bug-fix",
            "Diseño del fix",
            &[json!("agent:dedalo")],
            &json!({"persist_ref": "docs/fixes/x", "correlation_id": "corr"}),
            &json!({}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        let _ = fs::remove_dir_all(&dir);

        assert_eq!(entry["status"], "executed", "{entry}");
        assert_eq!(entry["handler"], "agent-runtime");
    }

    #[test]
    fn execution_id_reentry_refreshes_stale_frontmatter() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-conf-{}", uuid::Uuid::new_v4()));
        let persist = dir.join("docs/features/x");
        fs::create_dir_all(&persist).unwrap();
        fs::write(
            persist.join("plan.md"),
            "---\nexecution_id: deadbeef-dead-beef-dead-beefdeadbeef\n---\n\nbody\n",
        )
        .unwrap();
        let script = mock_script(
            &dir,
            "#!/bin/sh\necho '{\"success\":true,\"data\":{\"status\":\"executed\"}}'\n",
        );
        std::env::set_var(ENV_CMD, script.display().to_string());
        let live = "80a3ca0d-80c5-4662-ab12-2afe757478c8";
        let entry = invoke_agent_phase(
            &dir,
            "feature",
            "Diseño de Blueprint",
            &[json!("agent:dedalo")],
            &json!({"persist_ref": "docs/features/x", "execution_id": live}),
            &json!({"execution_id": live}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        let plan = fs::read_to_string(persist.join("plan.md")).unwrap();
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(entry["status"], "executed", "{entry}");
        assert!(plan.contains(live));
        assert!(!plan.contains("deadbeef-dead-beef-dead-beefdeadbeef"));
    }

    #[test]
    fn reconcile_persist_execution_id_skips_handoff() {
        let dir = std::env::temp_dir().join(format!("sddia-reconcile-{}", uuid::Uuid::new_v4()));
        let persist = dir.join("docs/fixes/x");
        fs::create_dir_all(&persist).unwrap();
        fs::write(
            persist.join("spec.md"),
            "---\nexecution_id: old-old-old-old-old-old-old-old\n---\n",
        )
        .unwrap();
        fs::write(
            persist.join("_agent_handoff.md"),
            "---\nexecution_id: old-old-old-old-old-old-old-old\n---\n",
        )
        .unwrap();
        let live = "80a3ca0d-80c5-4662-ab12-2afe757478c8";
        let n = reconcile_persist_execution_id_for_reentry(&dir, "docs/fixes/x", live);
        assert_eq!(n, 1);
        let spec = fs::read_to_string(persist.join("spec.md")).unwrap();
        let handoff = fs::read_to_string(persist.join("_agent_handoff.md")).unwrap();
        let _ = fs::remove_dir_all(&dir);
        assert!(spec.contains(live));
        assert!(handoff.contains("old-old-old"));
    }

    #[test]
    fn configured_cli_can_await() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-rt-aw-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = mock_script(
            &dir,
            "#!/bin/sh\ncat >/dev/null\necho '{\"success\":true,\"data\":{\"status\":\"awaiting_agents\"}}'\n",
        );

        std::env::set_var(ENV_CMD, script.display().to_string());
        let entry = invoke_agent_phase(
            &dir,
            "feature",
            "Ejecución",
            &[json!("agent:tekton")],
            &json!({}),
            &json!({}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        let _ = fs::remove_dir_all(&dir);

        assert_eq!(entry["status"], "awaiting_agents");
    }

    #[test]
    fn configured_cli_can_block() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-rt-bl-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = mock_script(
            &dir,
            "#!/bin/sh\ncat >/dev/null\necho '{\"success\":true,\"data\":{\"status\":\"blocked\"}}'\n",
        );

        std::env::set_var(ENV_CMD, script.display().to_string());
        let entry = invoke_agent_phase(
            &dir,
            "bug-fix",
            "Diseño del fix",
            &[json!("agent:dedalo")],
            &json!({"persist_ref": "docs/fixes/x"}),
            &json!({}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        let _ = fs::remove_dir_all(&dir);

        assert_eq!(entry["status"], "blocked", "{entry}");
        assert!(entry.get("error").is_none() || entry["error"].as_str() == Some(""));
    }

    #[test]
    fn blocked_propagates_data_message_to_entry_error() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-rt-blmsg-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = mock_script(
            &dir,
            "#!/bin/sh\ncat >/dev/null\necho '{\"success\":true,\"data\":{\"status\":\"blocked\",\"message\":\"motivo-x\"}}'\n",
        );

        std::env::set_var(ENV_CMD, script.display().to_string());
        let entry = invoke_agent_phase(
            &dir,
            "refine-item",
            "Refinamiento Mayeuta",
            &[json!("agent:mayeuta")],
            &json!({"item_ref": "AUT-37", "project_slug": "core"}),
            &json!({"workspace_path": dir.join("ws")}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        let _ = fs::remove_dir_all(&dir);

        assert_eq!(entry["status"], "blocked", "{entry}");
        assert_eq!(entry["message"], "motivo-x");
        assert_eq!(entry["error"], "motivo-x");
    }

    #[test]
    fn verification_handoff_without_branch_does_not_spawn() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-vh-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let marker = dir.join("spawned");
        let script = mock_script(
            &dir,
            &format!("#!/bin/sh\ntouch {}\nexit 0\n", marker.display()),
        );
        std::env::set_var(ENV_CMD, script.display().to_string());
        let entry = invoke_agent_phase(
            &dir,
            "bug-fix",
            "Verificación",
            &[json!("agent:argos")],
            &json!({"persist_ref": "docs/fixes/x", "execution_id": "live-id"}),
            &json!({}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        assert_eq!(entry["error_code"], "verification-handoff-incomplete", "{entry}");
        assert!(!marker.exists());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn verification_handoff_without_execution_id_does_not_spawn() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-ve-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let marker = dir.join("spawned");
        let script = mock_script(
            &dir,
            &format!("#!/bin/sh\ntouch {}\nexit 0\n", marker.display()),
        );
        std::env::set_var(ENV_CMD, script.display().to_string());
        let entry = invoke_agent_phase(
            &dir,
            "refactorization",
            "Verificación",
            &[json!("agent:argos")],
            &json!({"persist_ref": "docs/features/x", "branch_name": "feat/x"}),
            &json!({}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        assert_eq!(entry["status"], "failed", "{entry}");
        assert!(!marker.exists());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn verification_missing_local_ref_fails_before_spawn_when_checkout_unavailable() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-vc-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let init = std::process::Command::new("git")
            .args(["init"])
            .current_dir(&dir)
            .output()
            .expect("git init");
        assert!(init.status.success());
        let marker = dir.join("spawned");
        let script = mock_script(
            &dir,
            &format!("#!/bin/sh\ntouch {}\nexit 0\n", marker.display()),
        );
        std::env::set_var(ENV_CMD, script.display().to_string());
        let entry = invoke_agent_phase(
            &dir,
            "bug-fix",
            "Verificación",
            &[json!("agent:argos")],
            &json!({
                "persist_ref": "docs/fixes/x",
                "branch_name": "fix/absent-local",
                "execution_id": "live-id"
            }),
            &json!({}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        assert_eq!(entry["error_code"], "verification-branch-checkout", "{entry}");
        assert!(!marker.exists());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn branch_name_coalesces_from_pr_branch() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-rt-br-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = mock_script(
            &dir,
            r#"#!/bin/sh
python3 -c 'import json,sys; doc=json.load(sys.stdin); assert doc.get("branch_name")=="feat/from-pr", doc; print(json.dumps({"success":True,"data":{"status":"executed","message":"branch-ok"}}))'
"#,
        );

        std::env::set_var(ENV_CMD, script.display().to_string());
        let entry = invoke_agent_phase(
            &dir,
            "pull-request-review",
            "Triaje documental",
            &[json!("agent:argos")],
            &json!({"pr_branch": "feat/from-pr", "persist_ref": "docs/features/x"}),
            &json!({}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        let _ = fs::remove_dir_all(&dir);

        assert_eq!(entry["status"], "executed", "{entry}");
    }

    #[test]
    fn payload_includes_execution_id() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-eid-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = mock_script(
            &dir,
            r#"#!/bin/sh
python3 -c 'import json,sys; doc=json.load(sys.stdin); assert doc.get("execution_id")=="live-id", doc; print(json.dumps({"success":True,"data":{"status":"executed"}}))'
"#,
        );

        std::env::set_var(ENV_CMD, script.display().to_string());
        let entry = invoke_agent_phase(
            &dir,
            "feature",
            "Ejecución",
            &[json!("agent:tekton")],
            &json!({"execution_id": "live-id"}),
            &json!({}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(entry["status"], "executed", "{entry}");
    }

    #[test]
    fn runtime_evidence_forwards_native_state_flags() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-rt-ev-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = mock_script(
            &dir,
            r#"#!/bin/sh
python3 -c '
import json,sys
doc=json.load(sys.stdin)
assert doc.get("git_manager_invoked") is True, doc
assert doc.get("formal_execute_process") is True, doc
assert doc.get("tech_triage_formal") is True, doc
re=doc.get("runtime_evidence") or {}
assert re.get("git_manager_invoked") is True, re
assert re.get("formal_execute_process") is True, re
assert (re.get("tech_checks") or {}).get("TECH_FORMAL_EXECUTE_PROCESS")=="APTO", re
print(json.dumps({"success":True,"data":{"status":"executed","message":"evidence-fwd-ok"}}))
'
"#,
        );

        std::env::set_var(ENV_CMD, script.display().to_string());
        let entry = invoke_agent_phase(
            &dir,
            "pull-request-review",
            "Verificación",
            &[json!("agent:argos")],
            &json!({"persist_ref": "docs/features/x", "branch_name": "feat/x", "execution_id": "live-id"}),
            &json!({
                "git_manager_invoked": true,
                "tech_triage_formal": true,
                "tech_checks": {"TECH_FORMAL_EXECUTE_PROCESS": "APTO"}
            }),
            None,
        );
        std::env::remove_var(ENV_CMD);
        let _ = fs::remove_dir_all(&dir);

        assert_eq!(entry["status"], "executed", "{entry}");
        assert_eq!(entry["message"], "evidence-fwd-ok");
    }

    #[test]
    fn persist_ref_falls_back_from_workspace_state() {
        let from_ws = resolve_persist_ref_value(
            &json!({"persist_ref": ""}),
            &json!({"workspace": {"persist_ref": "docs/features/from-ws"}}),
        );
        assert_eq!(from_ws, json!("docs/features/from-ws"));
        let workspace_wins = resolve_persist_ref_value(
            &json!({"persist_ref": "docs/features/top"}),
            &json!({"workspace": {"persist_ref": "docs/features/from-ws"}}),
        );
        assert_eq!(workspace_wins, json!("docs/features/from-ws"));
        let missing = resolve_persist_ref_value(&json!({}), &json!({}));
        assert_eq!(missing, Value::Null);
    }

    #[test]
    fn ppr_doc_triage_agent_failed_is_fail_soft() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-agent-rt-fs-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = mock_script(
            &dir,
            r#"#!/bin/sh
python3 -c 'import json; print(json.dumps({"success":False,"data":{"status":"failed","message":"timeout"}}))'
"#,
        );
        std::env::set_var(ENV_CMD, script.display().to_string());
        let soft = invoke_agent_phase(
            &dir,
            "pull-request-review",
            "Triaje documental",
            &[json!("agent:argos")],
            &json!({"pr_branch": "feat/x"}),
            &json!({}),
            None,
        );
        let hard = invoke_agent_phase(
            &dir,
            "pull-request-review",
            "Veredicto y bloqueo",
            &[json!("agent:argos")],
            &json!({"pr_branch": "feat/x"}),
            &json!({}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(soft["status"], "failed");
        assert_eq!(soft["fail_soft"], true);
        assert_eq!(hard["status"], "failed");
        assert!(hard.get("fail_soft").is_none());
    }

    #[test]
    fn llm_profile_high_is_injected_in_payload() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-llm-high-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(dir.join("SddIA/agents")).unwrap();
        fs::write(
            dir.join("SddIA/agents/dedalo.md"),
            "---\nname: dedalo\nllm_profile:\n  tier: high\n  description: arquitectura\n---\n",
        )
        .unwrap();
        let script = mock_script(
            &dir,
            r#"#!/bin/sh
python3 -c 'import json,sys
doc=json.load(sys.stdin)
assert doc.get("repo_root")
p=(doc.get("llm_profiles") or {}).get("dedalo") or {}
assert p.get("tier")=="high", doc
print(json.dumps({"success":True,"data":{"status":"executed","message":"tier-ok"}}))'
"#,
        );
        std::env::set_var(ENV_CMD, script.display().to_string());
        let entry = invoke_agent_phase(
            &dir,
            "feature",
            "Diseño de Blueprint",
            &[json!("agent:dedalo")],
            &json!({"persist_ref": "docs/features/x"}),
            &json!({}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(entry["status"], "executed", "{entry}");
        assert_eq!(entry["message"], "tier-ok");
    }

    #[test]
    fn llm_profile_none_does_not_spawn() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-llm-none-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(dir.join("SddIA/agents")).unwrap();
        fs::write(
            dir.join("SddIA/agents/cerbero.md"),
            "---\nname: cerbero\nllm_profile:\n  tier: none\n---\n",
        )
        .unwrap();
        let marker = dir.join("spawned");
        let script = mock_script(
            &dir,
            &format!(
                "#!/bin/sh\ntouch {}\necho '{{\"success\":true,\"data\":{{\"status\":\"executed\"}}}}'\n",
                marker.display()
            ),
        );
        std::env::set_var(ENV_CMD, script.display().to_string());
        let entry = invoke_agent_phase(
            &dir,
            "pull-request-review",
            "Certificación RBAC",
            &[json!("agent:cerbero")],
            &json!({}),
            &json!({}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        let spawned = marker.is_file();
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(entry["status"], "executed", "{entry}");
        assert_eq!(entry["note"], "deterministic-agent-no-llm");
        assert_eq!(entry["llm_profiles"]["cerbero"]["tier"], "none");
        assert!(!spawned);
    }

    #[test]
    fn agent_runtime_cwd_is_outside_repo_when_project_bound() {
        let repo = std::env::temp_dir().join(format!("sddia-cwd-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&repo).unwrap();
        let cwd = agent_runtime_cwd(&repo, Some("/tmp/client-project"), Some("exec-test-id"));
        assert_ne!(cwd, repo);
        assert!(cwd.ends_with("exec-test-id"));
        assert!(!cwd.starts_with(&repo));
        let _ = fs::remove_dir_all(&repo);
    }

    #[test]
    fn workspace_mcp_smoke_initialize_when_binary_present() {
        let repo = find_repo_root_for_tests();
        let bin = repo.join("SddIA/target/debug/sddia-workspace-server");
        let release = repo.join("SddIA/target/release/sddia-workspace-server");
        let command = if bin.is_file() {
            bin.display().to_string()
        } else if release.is_file() {
            release.display().to_string()
        } else {
            return;
        };
        let td = tempfile::tempdir().unwrap();
        let args = vec![
            "--root".to_string(),
            td.path().display().to_string(),
            "--sddia-repo".to_string(),
            repo.display().to_string(),
        ];
        smoke_mcp_stdio_server(&command, &args).expect("workspace server initialize smoke");
    }

    fn find_repo_root_for_tests() -> PathBuf {
        let mut cur = std::env::current_dir().unwrap();
        for _ in 0..8 {
            if cur.join("SddIA/core/cumulo.paths.json").is_file() {
                return cur;
            }
            if !cur.pop() {
                break;
            }
        }
        std::env::current_dir().unwrap()
    }

    #[test]
    fn llm_profile_missing_agent_md_is_fail_soft() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-llm-miss-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();
        let script = mock_script(
            &dir,
            r#"#!/bin/sh
python3 -c 'import json,sys
doc=json.load(sys.stdin)
assert "dedalo" not in (doc.get("llm_profiles") or {}), doc
print(json.dumps({"success":True,"data":{"status":"executed","message":"soft-ok"}}))'
"#,
        );
        std::env::set_var(ENV_CMD, script.display().to_string());
        let entry = invoke_agent_phase(
            &dir,
            "feature",
            "Diseño de Blueprint",
            &[json!("agent:dedalo")],
            &json!({}),
            &json!({}),
            None,
        );
        std::env::remove_var(ENV_CMD);
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(entry["status"], "executed", "{entry}");
        assert_eq!(entry["message"], "soft-ok");
    }

    #[test]
    fn thinking_profile_taxonomy_and_multi_agent_precedence() {
        let dir = std::env::temp_dir().join(format!("sddia-tp-prec-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(dir.join("SddIA/agents")).unwrap();
        fs::write(
            dir.join("SddIA/agents/mayeuta.md"),
            "---\nname: mayeuta\nllm_profile:\n  thinking_profile: reasoning\n  tier: high\n---\n",
        )
        .unwrap();
        fs::write(
            dir.join("SddIA/agents/argos.md"),
            "---\nname: argos\nllm_profile:\n  thinking_profile: audit\n  tier: medium\n---\n",
        )
        .unwrap();
        fs::write(
            dir.join("SddIA/agents/tekton.md"),
            "---\nname: tekton\nllm_profile:\n  thinking_profile: execution\n  tier: low\n---\n",
        )
        .unwrap();
        fs::write(
            dir.join("SddIA/agents/cerbero.md"),
            "---\nname: cerbero\nllm_profile:\n  thinking_profile: deterministic\n  tier: none\n---\n",
        )
        .unwrap();

        let (_, tp_reason, all_none_1) = load_llm_profiles_with_thinking(
            &dir,
            &["tekton".into(), "argos".into(), "mayeuta".into()],
        );
        assert_eq!(tp_reason, ThinkingProfile::Reasoning);
        assert!(!all_none_1);

        let (_, tp_audit, all_none_2) =
            load_llm_profiles_with_thinking(&dir, &["tekton".into(), "argos".into()]);
        assert_eq!(tp_audit, ThinkingProfile::Audit);
        assert!(!all_none_2);

        let (_, tp_exec, all_none_3) =
            load_llm_profiles_with_thinking(&dir, &["cerbero".into(), "tekton".into()]);
        assert_eq!(tp_exec, ThinkingProfile::Execution);
        assert!(!all_none_3);

        let (_, tp_det, all_none_4) =
            load_llm_profiles_with_thinking(&dir, &["cerbero".into(), "cumulo".into()]);
        assert_eq!(tp_det, ThinkingProfile::Deterministic);
        assert!(all_none_4);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn di_factory_lazy_deterministic_ignores_broken_local_config() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-lazy-det-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(dir.join(".SddIA")).unwrap();
        // Broken JSON in .SddIA/llm-runtime.local.json must NOT be evaluated for deterministic agents
        fs::write(
            dir.join(".SddIA/llm-runtime.local.json"),
            "{ invalid json !!!",
        )
        .unwrap();
        let entry = invoke_agent_phase(
            &dir,
            "pull-request-review",
            "Certificación RBAC",
            &[json!("agent:cerbero")],
            &json!({}),
            &json!({}),
            None,
        );
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(entry["status"], "executed", "{entry}");
        assert_eq!(entry["thinking_profile"], "deterministic");
        assert_eq!(entry["llm_provider"], "deterministic-none");
        assert_eq!(entry["provider_mode"], "deterministic");
        assert!(entry["resolved_model"].is_null());
    }

    #[test]
    fn di_factory_resolves_models_and_rejects_missing_model_or_binding() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-di-cfg-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(dir.join(".SddIA")).unwrap();

        // 1. Unknown provider -> CAPABILITY_BINDING_MISSING
        fs::write(
            dir.join(".SddIA/llm-runtime.local.json"),
            r#"{"active_provider":"unknown-ide","providers":{"unknown-ide":{"models":{"reasoning":"m1"}}}}"#,
        )
        .unwrap();
        let bad_binding = invoke_agent_phase(
            &dir,
            "feature",
            "Diseño de Blueprint",
            &[json!("agent:dedalo")],
            &json!({}),
            &json!({}),
            None,
        );
        assert_eq!(bad_binding["status"], "failed");
        assert_eq!(bad_binding["error_code"], ERR_CAPABILITY_BINDING_MISSING);

        // 2. Missing model for reasoning in configured provider -> LLM_MODEL_UNRESOLVED
        fs::write(
            dir.join(".SddIA/llm-runtime.local.json"),
            r#"{"active_provider":"antigravity","providers":{"antigravity":{"models":{"execution":"flash-only"}}}}"#,
        )
        .unwrap();
        let bad_model = invoke_agent_phase(
            &dir,
            "feature",
            "Diseño de Blueprint",
            &[json!("agent:dedalo")],
            &json!({}),
            &json!({}),
            None,
        );
        assert_eq!(bad_model["status"], "failed");
        assert_eq!(bad_model["error_code"], ERR_LLM_MODEL_UNRESOLVED);

        // 3. Valid antigravity interactive relay config -> awaiting_agents + resolved_model
        fs::write(
            dir.join(".SddIA/llm-runtime.local.json"),
            r#"{"active_provider":"antigravity","unattended_policy":"pause_awaiting_agents","providers":{"antigravity":{"models":{"reasoning":"gemini-2.5-pro"}}}}"#,
        )
        .unwrap();
        let ok_agy = invoke_agent_phase(
            &dir,
            "feature",
            "Diseño de Blueprint",
            &[json!("agent:dedalo")],
            &json!({}),
            &json!({}),
            None,
        );
        assert_eq!(ok_agy["status"], "awaiting_agents", "{ok_agy}");
        assert_eq!(ok_agy["llm_provider"], "antigravity");
        assert_eq!(ok_agy["thinking_profile"], "reasoning");
        assert_eq!(ok_agy["resolved_model"], "gemini-2.5-pro");
        assert_eq!(ok_agy["provider_mode"], "interactive_relay");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn no_auto_failover_on_quota_timeout_or_unavailable_and_mock_injection() {
        let _guard = env_lock();
        clear_agent_env();
        let dir = std::env::temp_dir().join(format!("sddia-nofailover-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(dir.join(".SddIA")).unwrap();

        // 1. local-llm without endpoint fails with PROVIDER_UNAVAILABLE (no fallback to cursor/antigravity)
        fs::write(
            dir.join(".SddIA/llm-runtime.local.json"),
            r#"{"active_provider":"local-llm","providers":{"local-llm":{"endpoint":"","models":{"execution":"qwen-coder"}}}}"#,
        )
        .unwrap();
        let local_res = invoke_agent_phase(
            &dir,
            "feature",
            "Ejecución",
            &[json!("agent:tekton")],
            &json!({}),
            &json!({}),
            None,
        );
        assert_eq!(local_res["status"], "failed");
        assert_eq!(local_res["llm_provider"], "local-llm");
        assert_eq!(local_res["error_code"], ERR_PROVIDER_UNAVAILABLE);

        // 2. Cursor CLI quota exceeded -> PROVIDER_QUOTA_EXCEEDED without switching provider
        let quota_script = mock_script(
            &dir,
            "#!/bin/sh\ncat >/dev/null\necho '{\"success\":false,\"data\":{\"status\":\"failed\",\"error_code\":\"rate_limited\",\"message\":\"429 quota exceeded\"}}'\n",
        );
        fs::write(
            dir.join(".SddIA/llm-runtime.local.json"),
            serde_json::to_string(&json!({
                "active_provider": "cursor",
                "providers": {
                    "cursor": {
                        "command": quota_script.display().to_string(),
                        "models": { "execution": "composer-2.5" }
                    },
                    "antigravity": {
                        "models": { "execution": "gemini-2.5-flash" }
                    }
                }
            }))
            .unwrap(),
        )
        .unwrap();
        let quota_res = invoke_agent_phase(
            &dir,
            "feature",
            "Ejecución",
            &[json!("agent:tekton")],
            &json!({}),
            &json!({}),
            None,
        );
        assert_eq!(quota_res["status"], "failed");
        assert_eq!(quota_res["llm_provider"], "cursor");
        assert_eq!(quota_res["error_code"], ERR_PROVIDER_QUOTA_EXCEEDED);

        // 3. In-memory mock injection without SDDIA_LAB_*
        let mock = Arc::new(InMemoryAgentRuntimeMock {
            provider: "mock".into(),
            mode: ProviderMode::HeadlessApi,
            status: "executed".into(),
            resolved_model_override: Some("mock-audit-v1".into()),
            message: Some("mock-ok".into()),
            error_code: None,
            error: None,
        });
        let mock_res = with_mock_agent_runtime(mock, || {
            invoke_agent_phase(
                &dir,
                "feature",
                "Verificación",
                &[json!("agent:argos")],
                &json!({}),
                &json!({}),
                None,
            )
        });
        assert_eq!(mock_res["status"], "executed");
        assert_eq!(mock_res["llm_provider"], "mock");
        assert_eq!(mock_res["thinking_profile"], "audit");
        assert_eq!(mock_res["resolved_model"], "mock-audit-v1");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn switch_llm_provider_persists_local_config_and_rejects_unknown_provider() {
        let dir = std::env::temp_dir().join(format!("sddia-switch-llm-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&dir).unwrap();

        let err = switch_llm_provider(&dir, &json!({"provider": "invalid-provider"})).unwrap_err();
        assert!(err.contains(ERR_CAPABILITY_BINDING_MISSING));

        let ok = switch_llm_provider(
            &dir,
            &json!({
                "provider": "antigravity",
                "unattended_policy": "pause_awaiting_agents",
                "models": {
                    "reasoning": "gemini-2.5-pro",
                    "audit": "gemini-2.5-pro",
                    "execution": "gemini-2.5-flash"
                }
            }),
        )
        .expect("switch-llm-provider should succeed");
        assert_eq!(ok["active_provider"], "antigravity");
        assert_eq!(ok["config_path"], ".SddIA/llm-runtime.local.json");
        let saved: Value = serde_json::from_str(
            &fs::read_to_string(dir.join(".SddIA/llm-runtime.local.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(saved["active_provider"], "antigravity");
        assert_eq!(
            saved["providers"]["antigravity"]["models"]["reasoning"],
            "gemini-2.5-pro"
        );
        let _ = fs::remove_dir_all(&dir);
    }
}
