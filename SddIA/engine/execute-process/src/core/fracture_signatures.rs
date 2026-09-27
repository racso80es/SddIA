//! Catálogo SSOT `fracture-signatures.json` — match DCC / Mayeuta.

use crate::core::paths::load_paths_config;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

#[derive(Debug, Clone, Deserialize)]
pub struct CatalogFile {
    pub version: String,
    pub signatures: Vec<SignatureEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SignatureEntry {
    pub id: String,
    pub consumers: Vec<String>,
    #[serde(default)]
    pub scope: Option<ScopeFilter>,
    #[serde(rename = "match")]
    pub match_ast: MatchAst,
    #[serde(default)]
    pub match_surface: Option<String>,
    #[serde(default)]
    pub excludes: Vec<String>,
    #[serde(default)]
    pub refine: Vec<String>,
    #[serde(default)]
    pub fracture_policy: Option<String>,
    #[serde(default)]
    pub phase_status: Option<String>,
    #[serde(default)]
    pub operator_hint: Option<String>,
    #[serde(default)]
    pub report_friction_ids: Vec<String>,
    #[serde(default)]
    pub report_error_codes: Vec<String>,
    #[serde(default)]
    pub diagnosis: Option<Diagnosis>,
    /// Solo evaluable vía `refine` del padre.
    #[serde(default)]
    pub refine_only: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScopeFilter {
    #[serde(default)]
    pub process_name: Vec<String>,
    #[serde(default)]
    pub attempted_action: Vec<String>,
    #[serde(default)]
    pub status: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MatchAst {
    #[serde(default)]
    pub scope_only: bool,
    #[serde(default)]
    pub all: Vec<String>,
    #[serde(default)]
    pub any: Vec<String>,
    #[serde(default)]
    pub none: Vec<String>,
    #[serde(default)]
    pub any_groups: Vec<MatchGroup>,
    #[serde(default)]
    pub genomic_context_any: Vec<String>,
    #[serde(default)]
    pub genomic_token_any: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MatchGroup {
    #[serde(default)]
    pub all: Vec<String>,
    #[serde(default)]
    pub any: Vec<String>,
    #[serde(default)]
    pub none: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Diagnosis {
    pub root_cause: String,
    pub verdict: String,
    pub proposal: String,
}

#[derive(Debug, Clone)]
pub struct MatchSurfaces {
    pub error_trace_norm: String,
    pub hook_blob: String,
    pub blob: String,
}

#[derive(Clone)]
pub struct FractureCatalog {
    pub file: CatalogFile,
    id_index: std::collections::HashMap<String, usize>,
    catalog_ids: HashSet<String>,
}

impl FractureCatalog {
    pub fn load(repo: &Path) -> Result<Self, String> {
        let path = resolve_catalog_path(repo)?;
        let text = fs::read_to_string(&path).map_err(|e| format!("fracture-signatures: {e}"))?;
        let file: CatalogFile = serde_json::from_str(&text).map_err(|e| e.to_string())?;
        let mut id_index = std::collections::HashMap::new();
        let mut catalog_ids = HashSet::new();
        for (i, sig) in file.signatures.iter().enumerate() {
            id_index.insert(sig.id.clone(), i);
            catalog_ids.insert(sig.id.clone());
        }
        Ok(Self {
            file,
            id_index,
            catalog_ids,
        })
    }

    pub fn catalog_contains_id(&self, id: &str) -> bool {
        self.catalog_ids.contains(id)
    }

    pub fn get(&self, id: &str) -> Option<&SignatureEntry> {
        self.id_index.get(id).map(|i| &self.file.signatures[*i])
    }

    pub fn signatures(&self) -> &[SignatureEntry] {
        &self.file.signatures
    }
}

fn resolve_catalog_path(repo: &Path) -> Result<PathBuf, String> {
    if let Ok(cfg) = load_paths_config(repo) {
        if let Some(rel) = cfg
            .get("core")
            .and_then(|c| c.get("fractureSignatures"))
            .and_then(|v| v.as_str())
        {
            let p = repo.join(rel);
            if p.is_file() {
                return Ok(p);
            }
        }
    }
    let default = repo.join("SddIA/core/fracture-signatures.json");
    if default.is_file() {
        return Ok(default);
    }
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../core/fracture-signatures.json");
    if manifest.is_file() {
        return Ok(manifest);
    }
    Err("fracture-signatures.json no encontrado".into())
}

pub fn shared_default_catalog() -> &'static FractureCatalog {
    static ONCE: OnceLock<FractureCatalog> = OnceLock::new();
    ONCE.get_or_init(|| {
        let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
        load_fracture_catalog(&repo).expect("fracture-signatures.json")
    })
}

pub fn dcc_suppresses_any(
    catalog: &FractureCatalog,
    ctx: &DccMatchContext<'_>,
    surfaces: &MatchSurfaces,
) -> bool {
    catalog
        .signatures()
        .iter()
        .any(|sig| signature_matches_dcc_suppress(catalog, sig, ctx, surfaces))
}

pub fn load_fracture_catalog(repo: &Path) -> Result<FractureCatalog, String> {
    FractureCatalog::load(repo)
}

/// Normaliza traza para match Mayeuta (K2): quita hints y líneas SKIPPED pre-push.
pub fn normalize_error_trace_for_match(error_trace: &str) -> String {
    let mut out = Vec::new();
    for line in error_trace.lines() {
        let t = line.trim();
        let lower = t.to_lowercase();
        if lower.starts_with("sddia pre-push: skipped") {
            continue;
        }
        if lower.starts_with("hint:")
            || lower.starts_with("ayuda:")
            || lower.starts_with("help:")
            || lower.starts_with("sugerencia:")
        {
            continue;
        }
        out.push(line);
    }
    out.join("\n").to_lowercase()
}

pub fn build_match_surfaces(
    process_name: &str,
    error_trace: &str,
    attempted_action: &str,
    normalize: bool,
) -> MatchSurfaces {
    let raw_norm = if normalize {
        normalize_error_trace_for_match(error_trace)
    } else {
        error_trace.to_lowercase()
    };
    let hook_blob = format!("{error_trace}\n{attempted_action}").to_lowercase();
    let blob = format!("{error_trace}\n{attempted_action}\n{process_name}").to_lowercase();
    MatchSurfaces {
        error_trace_norm: raw_norm,
        hook_blob,
        blob,
    }
}

fn scope_matches(scope: &Option<ScopeFilter>, ctx: &DccMatchContext<'_>) -> bool {
    let scope = scope.as_ref();
    if scope.is_none() {
        return true;
    }
    let s = scope.unwrap();
    if !s.process_name.is_empty()
        && !s
            .process_name
            .iter()
            .any(|p| p.eq_ignore_ascii_case(ctx.process_name))
    {
        return false;
    }
    if !s.attempted_action.is_empty()
        && !s
            .attempted_action
            .iter()
            .any(|a| a == ctx.attempted_action)
    {
        return false;
    }
    if !s.status.is_empty()
        && !ctx.status.is_empty()
        && !s.status.iter().any(|st| st == ctx.status)
    {
        return false;
    }
    true
}

fn hay_contains(hay: &str, token: &str) -> bool {
    hay.contains(&token.to_lowercase())
}

fn group_matches(hay: &str, g: &MatchGroup) -> bool {
    if !g.all.iter().all(|t| hay_contains(hay, t)) {
        return false;
    }
    if !g.any.is_empty() && !g.any.iter().any(|t| hay_contains(hay, t)) {
        return false;
    }
    if g.none.iter().any(|t| hay_contains(hay, t)) {
        return false;
    }
    true
}

fn ast_matches(hay: &str, ast: &MatchAst, surfaces: &MatchSurfaces) -> bool {
    if ast.scope_only {
        return true;
    }
    if !ast.any_groups.is_empty() && ast.all.is_empty() && ast.any.is_empty() {
        return ast.any_groups.iter().any(|g| group_matches(hay, g));
    }
    if !ast.genomic_context_any.is_empty() || !ast.genomic_token_any.is_empty() {
        let ctx_ok = ast
            .genomic_context_any
            .iter()
            .any(|t| hay_contains(&surfaces.blob, t));
        let tok_ok = ast
            .genomic_token_any
            .iter()
            .any(|t| hay_contains(&surfaces.blob, t));
        return ctx_ok && tok_ok;
    }
    if !ast.all.iter().all(|t| hay_contains(hay, t)) {
        return false;
    }
    if !ast.all.is_empty() {
        // `all` acotado arriba; `any` / `any_groups` adicionales siguen abajo.
    } else if !ast.any.is_empty() && ast.any.iter().any(|t| hay_contains(hay, t)) {
        return true;
    } else if !ast.any_groups.is_empty()
        && ast.any_groups.iter().any(|g| group_matches(hay, g))
    {
        return true;
    } else if !ast.any.is_empty() || !ast.any_groups.is_empty() {
        return false;
    }
    if !ast.all.is_empty() && !ast.any.is_empty() && !ast.any.iter().any(|t| hay_contains(hay, t)) {
        return false;
    }
    if ast.none.iter().any(|t| hay_contains(hay, t)) {
        return false;
    }
    if !ast.any_groups.is_empty()
        && !ast.any_groups.iter().any(|g| group_matches(hay, g))
        && ast.all.is_empty()
        && ast.any.is_empty()
    {
        return false;
    }
    if !ast.any_groups.is_empty()
        && !ast.all.is_empty()
        && !ast.any_groups.iter().any(|g| group_matches(hay, g))
    {
        // snapshot: all + any_groups OR semantics from legacy
        return false;
    }
    if !ast.any_groups.is_empty()
        && ast.all.is_empty()
        && ast.any.is_empty()
        && ast.any_groups.iter().any(|g| group_matches(hay, g))
    {
        return true;
    }
    if !ast.any_groups.is_empty() && !ast.all.is_empty() {
        let all_ok = ast.all.iter().all(|t| hay_contains(hay, t));
        let groups_ok = ast.any_groups.iter().any(|g| group_matches(hay, g));
        return all_ok && groups_ok;
    }
    true
}

fn surface_text<'a>(sig: &SignatureEntry, surfaces: &'a MatchSurfaces) -> &'a str {
    match sig.match_surface.as_deref() {
        Some("hook_blob") => &surfaces.hook_blob,
        Some("blob") => &surfaces.blob,
        _ => &surfaces.error_trace_norm,
    }
}

pub struct DccMatchContext<'a> {
    pub process_name: &'a str,
    pub attempted_action: &'a str,
    pub status: &'a str,
    pub error_trace: &'a str,
    pub report_friction_id: Option<&'a str>,
    pub report_error_code: Option<&'a str>,
}

pub fn signature_matches_mayeuta(
    catalog: &FractureCatalog,
    sig: &SignatureEntry,
    surfaces: &MatchSurfaces,
    ctx: &DccMatchContext<'_>,
    matched_ids: &HashSet<String>,
) -> bool {
    if !sig.consumers.iter().any(|c| c == "mayeuta") {
        return false;
    }
    if !scope_matches(&sig.scope, ctx) {
        return false;
    }
    if sig.excludes.iter().any(|id| matched_ids.contains(id)) {
        return false;
    }
    let hay = surface_text(sig, surfaces);
    ast_matches(hay, &sig.match_ast, surfaces)
}

pub fn signature_matches_dcc_suppress(
    catalog: &FractureCatalog,
    sig: &SignatureEntry,
    ctx: &DccMatchContext<'_>,
    surfaces: &MatchSurfaces,
) -> bool {
    if !sig.consumers.iter().any(|c| c == "dcc_suppress") {
        return false;
    }
    if sig.fracture_policy.as_deref() != Some("suppress") {
        return false;
    }
    if !scope_matches(&sig.scope, ctx) {
        return false;
    }
    if !sig.report_friction_ids.is_empty() {
        if let Some(fid) = ctx.report_friction_id {
            if sig.report_friction_ids.iter().any(|x| x == fid) {
                return true;
            }
        }
    }
    if !sig.report_error_codes.is_empty() {
        if let Some(code) = ctx.report_error_code {
            if sig.report_error_codes.iter().any(|x| x == code) {
                return true;
            }
        }
    }
    let hay = surface_text(sig, surfaces);
    ast_matches(hay, &sig.match_ast, surfaces)
}

pub fn resolve_refined_diagnosis(
    catalog: &FractureCatalog,
    parent: &SignatureEntry,
    surfaces: &MatchSurfaces,
    ctx: &DccMatchContext<'_>,
) -> (String, Diagnosis) {
    for child_id in &parent.refine {
        if let Some(child) = catalog.get(child_id) {
            let hay = surface_text(child, surfaces);
            if ast_matches(hay, &child.match_ast, surfaces) {
                if let Some(d) = child.diagnosis.clone() {
                    return (child.id.clone(), d);
                }
            }
        }
    }
    let d = parent
        .diagnosis
        .clone()
        .unwrap_or_else(|| Diagnosis {
            root_cause: parent.id.clone(),
            verdict: "process_fix".into(),
            proposal: "Revisar catálogo.".into(),
        });
    (parent.id.clone(), d)
}

pub fn extract_evidence_lines(error_trace_norm: &str, max_lines: usize) -> Vec<String> {
    error_trace_norm
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .take(max_lines)
        .map(str::to_string)
        .collect()
}
