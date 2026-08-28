//! Resolutor de PBI de fractura — identidad en genoma YAML (ceguera nominal).

use super::paths::load_paths_config;
use regex::Regex;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

const DEFAULT_PENDING: &str = "docs/todos/pending";
const DEFAULT_DONE: &str = "docs/todos/done";

/// Paridad `execute-action.py::_slugify_process_name`.
pub fn slugify_process_name(name: &str) -> String {
    static RE_NON_WORD: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    static RE_DASHES: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    let re_non = RE_NON_WORD.get_or_init(|| Regex::new(r"[^\w\-]+").expect("regex"));
    let re_dash = RE_DASHES.get_or_init(|| Regex::new(r"-+").expect("regex"));
    let lower = name.trim().to_lowercase();
    let slug = re_non.replace_all(&lower, "-");
    let slug = re_dash.replace_all(&slug, "-");
    let slug = slug.trim_matches('-');
    if slug.is_empty() {
        "fracture".to_string()
    } else if slug.len() > 48 {
        slug[..48].to_string()
    } else {
        slug.to_string()
    }
}

/// Paridad `execute-action.py::_fracture_trace_hash`.
pub fn fracture_trace_hash(error_trace: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(error_trace.trim().as_bytes());
    format!("{:x}", hasher.finalize())[..12].to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FractureBucket {
    Pending,
    Done,
}

#[derive(Debug, Clone)]
pub struct FracturePbiRecord {
    pub rel_path: String,
    pub fracture_hash: String,
    pub fracture_process: String,
    pub document_id: String,
    pub status_open: bool,
    pub bucket: FractureBucket,
    pub regression_of: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FractureLedgerScan {
    pub pending: Vec<FracturePbiRecord>,
    pub done: Vec<FracturePbiRecord>,
    pub docs_scanned: u32,
    pub bytes_read: u64,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterializeReason {
    AlreadyOpen,
    DedupedByProcess,
    RegressionOpened,
    Materialized,
}

impl MaterializeReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlreadyOpen => "already_open",
            Self::DedupedByProcess => "deduped_by_process",
            Self::RegressionOpened => "regression_opened",
            Self::Materialized => "materialized",
        }
    }
}

#[derive(Debug, Clone)]
pub struct MaterializeResolution {
    pub reason: MaterializeReason,
    pub target_path: String,
    pub canonical_ref: Option<String>,
    pub regression_n: Option<u32>,
    pub predecessor_document_id: Option<String>,
}

pub fn resolve_todos_pending_rel(repo: &Path) -> Result<String, String> {
    resolve_todos_rel(repo, "pending", DEFAULT_PENDING)
}

pub fn resolve_todos_done_rel(repo: &Path) -> Result<String, String> {
    resolve_todos_rel(repo, "done", DEFAULT_DONE)
}

fn resolve_todos_rel(repo: &Path, key: &str, fallback: &str) -> Result<String, String> {
    let cfg = load_paths_config(repo)?;
    let rel = cfg
        .get("paths")
        .and_then(|p| p.get("todos"))
        .and_then(|t| t.get(key))
        .and_then(|v| v.as_str())
        .map(|s| s.trim().trim_end_matches('/').replace('\\', "/"))
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| fallback.to_string());
    Ok(rel)
}

fn fm_string(fm: &HashMap<String, String>, key: &str) -> Option<String> {
    fm.get(key).cloned()
}

fn record_status_open(fm: &HashMap<String, String>, bucket: FractureBucket) -> bool {
    if let Some(s) = fm_string(fm, "status") {
        if is_status_open(&s) {
            return true;
        }
        if is_status_closed(&s) {
            return false;
        }
    }
    bucket == FractureBucket::Pending
}

fn is_status_open(raw: &str) -> bool {
    matches!(
        raw.trim().trim_matches('"').trim_matches('\'').to_lowercase().as_str(),
        "abierto" | "open" | "pendiente" | "pending"
    )
}

fn is_status_closed(raw: &str) -> bool {
    matches!(
        raw.trim().trim_matches('"').trim_matches('\'').to_lowercase().as_str(),
        "cerrado" | "closed" | "done"
    )
}

fn parse_fracture_frontmatter_fields(text: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    let Some(rest) = text.strip_prefix("---") else {
        return out;
    };
    let fm_block = rest.split("---").next().unwrap_or("");
    for line in fm_block.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, val)) = line.split_once(':') else {
            continue;
        };
        let key = key.trim();
        if !matches!(
            key,
            "fracture_hash" | "fracture_process" | "document_id" | "status" | "regression_of"
        ) {
            continue;
        }
        let val = val.trim().trim_matches('"').trim_matches('\'');
        if !val.is_empty() {
            out.insert(key.to_string(), val.to_string());
        }
    }
    out
}

fn read_frontmatter_bytes(path: &Path) -> Result<(HashMap<String, String>, u64), String> {
    let raw = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let bytes = raw.len() as u64;
    let fm = parse_fracture_frontmatter_fields(&raw);
    Ok((fm, bytes))
}

fn scan_bucket(
    repo: &Path,
    rel_dir: &str,
    bucket: FractureBucket,
    docs_scanned: &mut u32,
    bytes_read: &mut u64,
) -> Result<Vec<FracturePbiRecord>, String> {
    let dir = repo.join(rel_dir);
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    let mut entries: Vec<PathBuf> = fs::read_dir(&dir)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("md"))
        .collect();
    entries.sort();
    for path in entries {
        *docs_scanned += 1;
        let (fm, nbytes) = match read_frontmatter_bytes(&path) {
            Ok(v) => v,
            Err(_) => continue,
        };
        *bytes_read += nbytes;
        let fracture_hash = match fm_string(&fm, "fracture_hash") {
            Some(h) => h,
            None => continue,
        };
        let fracture_process = match fm_string(&fm, "fracture_process") {
            Some(p) => p,
            None => continue,
        };
        let document_id = fm_string(&fm, "document_id").unwrap_or_default();
        let rel_path = path
            .strip_prefix(repo)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        out.push(FracturePbiRecord {
            rel_path,
            fracture_hash,
            fracture_process,
            document_id,
            status_open: record_status_open(&fm, bucket),
            bucket,
            regression_of: fm_string(&fm, "regression_of"),
        });
    }
    Ok(out)
}

pub fn scan_fracture_ledger(repo: &Path) -> Result<FractureLedgerScan, String> {
    let start = Instant::now();
    let pending_rel = resolve_todos_pending_rel(repo)?;
    let done_rel = resolve_todos_done_rel(repo)?;
    let mut docs_scanned = 0u32;
    let mut bytes_read = 0u64;
    let pending = scan_bucket(
        repo,
        &pending_rel,
        FractureBucket::Pending,
        &mut docs_scanned,
        &mut bytes_read,
    )?;
    let done = scan_bucket(
        repo,
        &done_rel,
        FractureBucket::Done,
        &mut docs_scanned,
        &mut bytes_read,
    )?;
    Ok(FractureLedgerScan {
        pending,
        done,
        docs_scanned,
        bytes_read,
        duration_ms: start.elapsed().as_millis() as u64,
    })
}

pub fn next_regression_n(scan: &FractureLedgerScan, fracture_hash: &str) -> u32 {
    static RE_R: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
    let re = RE_R.get_or_init(|| {
        Regex::new(r"^PBI-FIX-FRACTURE-[0-9a-f]{12}-R(\d+)$").expect("regex")
    });
    let mut max_n = 0u32;
    for rec in scan.pending.iter().chain(scan.done.iter()) {
        if rec.fracture_hash != fracture_hash {
            continue;
        }
        if let Some(caps) = re.captures(&rec.document_id) {
            if let Ok(n) = caps[1].parse::<u32>() {
                max_n = max_n.max(n);
            }
        }
    }
    max_n + 1
}

pub fn resolve_materialize(
    scan: &FractureLedgerScan,
    fracture_hash: &str,
    fracture_process: &str,
) -> MaterializeResolution {
    if let Some(rec) = scan
        .pending
        .iter()
        .find(|r| r.fracture_hash == fracture_hash && r.status_open)
    {
        return MaterializeResolution {
            reason: MaterializeReason::AlreadyOpen,
            target_path: rec.rel_path.clone(),
            canonical_ref: None,
            regression_n: None,
            predecessor_document_id: None,
        };
    }
    if let Some(rec) = scan
        .pending
        .iter()
        .find(|r| r.fracture_process == fracture_process && r.status_open)
    {
        return MaterializeResolution {
            reason: MaterializeReason::DedupedByProcess,
            target_path: rec.rel_path.clone(),
            canonical_ref: None,
            regression_n: None,
            predecessor_document_id: None,
        };
    }
    if let Some(rec) = scan.done.iter().find(|r| r.fracture_hash == fracture_hash) {
        let n = next_regression_n(scan, fracture_hash);
        return MaterializeResolution {
            reason: MaterializeReason::RegressionOpened,
            target_path: String::new(),
            canonical_ref: Some(rec.rel_path.clone()),
            regression_n: Some(n),
            predecessor_document_id: Some(rec.document_id.clone()),
        };
    }
    MaterializeResolution {
        reason: MaterializeReason::Materialized,
        target_path: String::new(),
        canonical_ref: None,
        regression_n: None,
        predecessor_document_id: None,
    }
}

/// Cascada Mayeuta: path explícito → hash abierto → process abierto.
pub fn resolve_enrich_target(
    repo: &Path,
    scan: &FractureLedgerScan,
    cumulo_pbi_path: Option<&str>,
    fracture_hash: &str,
    fracture_process: &str,
) -> Option<String> {
    if let Some(rel) = cumulo_pbi_path.map(str::trim).filter(|s| !s.is_empty()) {
        let path = repo.join(rel);
        if path.is_file() {
            return Some(rel.to_string());
        }
    }
    if let Some(rec) = scan
        .pending
        .iter()
        .find(|r| r.fracture_hash == fracture_hash && r.status_open)
    {
        return Some(rec.rel_path.clone());
    }
    if let Some(rec) = scan
        .pending
        .iter()
        .find(|r| r.fracture_process == fracture_process && r.status_open)
    {
        return Some(rec.rel_path.clone());
    }
    None
}

pub fn display_filename_fix(process_name: &str, fracture_hash: &str) -> String {
    let slug = slugify_process_name(process_name);
    format!("[FIX] {slug} — fractura sistémica ({fracture_hash}).md")
}

pub fn display_filename_regression(
    process_name: &str,
    fracture_hash: &str,
    regression_n: u32,
) -> String {
    let slug = slugify_process_name(process_name);
    format!(
        "[REGRESIÓN] {slug} — fractura sistémica ({fracture_hash})-R{regression_n}.md"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn write_pbi(
        repo: &Path,
        rel: &str,
        hash: &str,
        process: &str,
        status: &str,
        document_id: &str,
        regression_of: Option<&str>,
    ) {
        let reg = regression_of
            .map(|r| format!("regression_of: {r}\n"))
            .unwrap_or_default();
        let body = format!(
            r#"---
document_id: {document_id}
fracture_hash: {hash}
fracture_process: {process}
status: "{status}"
process: bug-fix
{reg}---

# body
"#
        );
        let path = repo.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, body).unwrap();
    }

    fn setup_cumulo(repo: &Path) {
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(
            repo.join("SddIA/core/cumulo.paths.json"),
            r#"{"paths":{"todos":{"pending":"docs/todos/pending","done":"docs/todos/done"}}}"#,
        )
        .unwrap();
    }

    #[test]
    fn resolve_regression_when_closed_in_done() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        setup_cumulo(repo);
        write_pbi(
            repo,
            "docs/todos/done/zzz-canonical.md",
            "abc123def456",
            "route-domain-event",
            "cerrado",
            "PBI-FIX-FRACTURE-abc123def456",
            None,
        );
        let scan = scan_fracture_ledger(repo).unwrap();
        let res = resolve_materialize(&scan, "abc123def456", "route-domain-event");
        assert_eq!(res.reason, MaterializeReason::RegressionOpened);
        assert_eq!(res.regression_n, Some(1));
        assert!(res.canonical_ref.is_some());
    }

    #[test]
    fn resolve_already_open_same_hash() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        setup_cumulo(repo);
        write_pbi(
            repo,
            "docs/todos/pending/renamed.md",
            "deadbeef0001",
            "event-watcher",
            "abierto",
            "PBI-FIX-FRACTURE-deadbeef0001",
            None,
        );
        let scan = scan_fracture_ledger(repo).unwrap();
        let res = resolve_materialize(&scan, "deadbeef0001", "event-watcher");
        assert_eq!(res.reason, MaterializeReason::AlreadyOpen);
        assert_eq!(res.target_path, "docs/todos/pending/renamed.md");
    }

    #[test]
    fn resolve_deduped_by_process() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        setup_cumulo(repo);
        write_pbi(
            repo,
            "docs/todos/pending/open-daemon.md",
            "aaaaaaaaaaaa",
            "event-watcher",
            "abierto",
            "PBI-FIX-FRACTURE-aaaaaaaaaaaa",
            None,
        );
        let scan = scan_fracture_ledger(repo).unwrap();
        let res = resolve_materialize(&scan, "bbbbbbbbbbbb", "event-watcher");
        assert_eq!(res.reason, MaterializeReason::DedupedByProcess);
        assert_eq!(res.target_path, "docs/todos/pending/open-daemon.md");
    }

    #[test]
    fn scan_tolerates_malformed_frontmatter_duplicate_keys() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        setup_cumulo(repo);
        let body = r#"---

document_id: PBI-FIX-FRACTURE-abc123def456
status: cerrado
resolution_ref: first
fracture_process: route-domain-event
fracture_hash: abc123def456
resolution_ref: second

# body
"#;
        let path = repo.join("docs/todos/done/malformed.md");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, body).unwrap();
        let scan = scan_fracture_ledger(repo).unwrap();
        assert_eq!(scan.done.len(), 1);
        assert_eq!(scan.done[0].fracture_hash, "abc123def456");
        let res = resolve_materialize(&scan, "abc123def456", "route-domain-event");
        assert_eq!(res.reason, MaterializeReason::RegressionOpened);
    }

    #[test]
    fn enrich_finds_renamed_open_by_hash() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        setup_cumulo(repo);
        write_pbi(
            repo,
            "docs/todos/pending/renamed.md",
            "cafebabe0001",
            "event-watcher",
            "abierto",
            "PBI-FIX-FRACTURE-cafebabe0001",
            None,
        );
        let scan = scan_fracture_ledger(repo).unwrap();
        let target = resolve_enrich_target(repo, &scan, None, "cafebabe0001", "event-watcher");
        assert_eq!(target.as_deref(), Some("docs/todos/pending/renamed.md"));
    }
}
