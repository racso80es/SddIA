//! Aduana de cierre parental (MAN-46 / KZ-3).
//! Hito/hijo no cierra PBI padre ni tracker: Done/success parental exige AC verdes
//! del mismo `persist_ref` **o** refine sellado de reducción de alcance previo.

use crate::core::parser::load_frontmatter_yaml;
use serde_yaml::Value as YamlValue;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub const LAB_SKIP_ENV: &str = "SDDIA_LAB_SKIP_PARENTAL_CLOSE_GATE";
pub const FRICTION_ROLLUP: &str = "parental_close_rollup_denied";
pub const FRICTION_SUCCESS: &str = "parental_close_success_denied";
pub const FRICTION_NARRATIVE: &str = "parental_close_narrative_denied";
pub const FRICTION_SCOPE_CUT: &str = "parental_close_scope_cut_denied";

const OPERATOR_HINT: &str = "Cerrar AC del persist_ref padre o sellar refine de reducción de alcance antes del Done/success parental; no narrar ciclo cerrado en el padre";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentalCloseIntent {
    Rollup,
    SuccessSeal,
    NarrativeCheck,
}

#[derive(Debug, Clone, Default)]
pub struct ParentalCloseEvidence {
    pub merge_or_stamp_at: Option<String>,
    pub intent: Option<ParentalCloseIntent>,
    pub child_ref: Option<String>,
    pub same_pass_validacion_rewrite: bool,
}

impl ParentalCloseEvidence {
    pub fn rollup(child_ref: Option<&str>) -> Self {
        Self {
            intent: Some(ParentalCloseIntent::Rollup),
            child_ref: child_ref.map(str::to_string),
            ..Default::default()
        }
    }
    pub fn success_seal() -> Self {
        Self {
            intent: Some(ParentalCloseIntent::SuccessSeal),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParentalCloseFault {
    pub friction_id: String,
    pub message: String,
    pub reasons: Vec<String>,
    pub parent_ref: Option<String>,
    pub child_ref: Option<String>,
}

impl ParentalCloseFault {
    pub fn operator_hint(&self) -> &'static str {
        OPERATOR_HINT
    }
    pub fn to_error_string(&self) -> String {
        if self.reasons.is_empty() {
            format!("{}: {}", self.friction_id, self.message)
        } else {
            format!(
                "{}: {} ({})",
                self.friction_id,
                self.message,
                self.reasons.join(",")
            )
        }
    }
}

fn lab_skip() -> bool {
    std::env::var(LAB_SKIP_ENV)
        .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

fn persist_join(repo: &Path, persist_ref: &str, file: &str) -> PathBuf {
    let base = persist_ref.trim().trim_start_matches("./").trim_end_matches('/');
    repo.join(base).join(file)
}

fn yaml_str(v: &YamlValue) -> Option<String> {
    match v {
        YamlValue::String(s) => Some(s.trim().to_string()),
        YamlValue::Bool(b) => Some(b.to_string()),
        YamlValue::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

fn fm_str(fm: &HashMap<String, YamlValue>, key: &str) -> Option<String> {
    fm.get(key).and_then(yaml_str).filter(|s| !s.is_empty())
}

fn ci_eq(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b)
}

fn looks_like_persist_path(s: &str) -> bool {
    let t = s.trim().trim_start_matches("./");
    t.starts_with("docs/features/")
        || t.starts_with("docs/fixes/")
        || t.contains("/features/")
        || t.contains("/fixes/")
}

pub fn resolve_persist_ref(repo: &Path, parent_ref_or_persist: &str) -> Option<String> {
    let raw = parent_ref_or_persist.trim().trim_start_matches("./").trim_end_matches('/');
    if raw.is_empty() {
        return None;
    }
    if looks_like_persist_path(raw)
        && (repo.join(raw).join("objectives.md").is_file()
            || repo.join(raw).join("validacion.md").is_file())
    {
        return Some(raw.to_string());
    }
    scan_persist_for_issue(repo, raw)
}

fn scan_persist_for_issue(repo: &Path, issue_ref: &str) -> Option<String> {
    let needle = issue_ref.trim();
    if needle.is_empty() {
        return None;
    }
    for root_name in ["docs/features", "docs/fixes"] {
        let root = repo.join(root_name);
        let Ok(entries) = fs::read_dir(&root) else {
            continue;
        };
        for ent in entries.flatten() {
            let path = ent.path();
            if !path.is_dir() {
                continue;
            }
            let name = ent.file_name().to_string_lossy().to_string();
            if name.starts_with('_') {
                continue;
            }
            for fname in ["objectives.md", "spec.md", "validacion.md", "clarify.md"] {
                let md = path.join(fname);
                if !md.is_file() {
                    continue;
                }
                let Ok(fm) = load_frontmatter_yaml(&md) else {
                    continue;
                };
                for key in ["pbi_ref", "tracker_ref", "parent_ref", "historia_ref"] {
                    if let Some(v) = fm_str(&fm, key) {
                        if ci_eq(&v, needle) {
                            let rel = path.strip_prefix(repo).ok()?.to_string_lossy();
                            return Some(rel.replace('\\', "/"));
                        }
                    }
                }
            }
        }
    }
    None
}

fn strip_frontmatter(raw: &str) -> String {
    let mut lines = raw.lines();
    if lines.next().map(|l| l.trim() == "---").unwrap_or(false) {
        for line in lines.by_ref() {
            if line.trim() == "---" {
                break;
            }
        }
        lines.collect::<Vec<_>>().join("\n")
    } else {
        raw.to_string()
    }
}

fn collect_ac_ids(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for line in body.lines() {
        let mut rest = line;
        while let Some(idx) = rest.find("AC-") {
            let slice = &rest[idx..];
            let digits: String = slice.chars().skip(3).take_while(|c| c.is_ascii_digit()).collect();
            if !digits.is_empty() {
                let id = format!("AC-{digits}");
                if seen.insert(id.clone()) {
                    out.push(id);
                }
            }
            rest = &slice[3..];
        }
    }
    out
}

fn check_status_pass(st: &str) -> bool {
    ["pass", "ok", "apto", "green", "closed", "done", "PASS", "APTO"]
        .iter()
        .any(|s| ci_eq(st, s))
        || st.contains('✅')
}

fn open_acs_from_objectives(repo: &Path, persist: &str) -> Result<Vec<String>, ParentalCloseFault> {
    let path = persist_join(repo, persist, "objectives.md");
    if !path.is_file() {
        return Ok(vec!["objectives_missing".into()]);
    }
    let raw = fs::read_to_string(&path).map_err(|e| ParentalCloseFault {
        friction_id: FRICTION_SUCCESS.into(),
        message: format!("objectives.md ilegible: {e}"),
        reasons: vec!["objectives_unreadable".into()],
        parent_ref: Some(persist.to_string()),
        child_ref: None,
    })?;
    let acs = collect_ac_ids(&strip_frontmatter(&raw));
    if acs.is_empty() {
        return Ok(Vec::new());
    }
    let validacion_path = persist_join(repo, persist, "validacion.md");
    let mut checks_pass: HashMap<String, bool> = HashMap::new();
    if validacion_path.is_file() {
        if let Ok(fm) = load_frontmatter_yaml(&validacion_path) {
            if let Some(YamlValue::Mapping(m)) = fm.get("checks") {
                for (k, v) in m {
                    if let YamlValue::String(key) = k {
                        checks_pass.insert(key.clone(), check_status_pass(&yaml_str(v).unwrap_or_default()));
                    }
                }
            }
        }
    }
    let validacion_body = if validacion_path.is_file() {
        fs::read_to_string(&validacion_path).map(|r| strip_frontmatter(&r)).unwrap_or_default()
    } else {
        String::new()
    };
    let mut open = Vec::new();
    for ac in acs {
        if checks_pass.get(&ac) == Some(&true) {
            continue;
        }
        let closed = validacion_body.lines().any(|line| {
            line.contains(&ac)
                && (line.to_ascii_lowercase().contains("pass")
                    || line.to_ascii_lowercase().contains("apto")
                    || line.to_ascii_lowercase().contains("ok")
                    || line.contains('✅'))
        });
        if !closed {
            open.push(ac);
        }
    }
    Ok(open)
}

fn parse_instant_approx(raw: &str) -> Option<i64> {
    let t = raw.trim();
    if let Ok(n) = t.parse::<i64>() {
        return Some(n);
    }
    let digits: String = t.chars().filter(|c| c.is_ascii_digit()).take(14).collect();
    if digits.len() >= 8 {
        digits.parse().ok()
    } else {
        None
    }
}

fn prior_scope_refine(
    repo: &Path,
    persist: &str,
    merge_or_stamp_at: Option<&str>,
) -> Result<bool, ParentalCloseFault> {
    for fname in ["objectives.md", "clarify.md", "execution.md", "validacion.md"] {
        let path = persist_join(repo, persist, fname);
        if !path.is_file() {
            continue;
        }
        let Ok(fm) = load_frontmatter_yaml(&path) else {
            continue;
        };
        let sealed = fm_str(&fm, "scope_refine_sealed_at")
            .or_else(|| fm_str(&fm, "scope_cut_sealed_at"))
            .or_else(|| fm_str(&fm, "refine_sealed_at"));
        let laudado = fm_str(&fm, "scope_cut_laudado")
            .or_else(|| fm_str(&fm, "scope_reduction_lauded"))
            .map(|v| matches!(v.to_lowercase().as_str(), "true" | "1" | "yes" | "on"))
            .unwrap_or(false);
        if let Some(ref sealed_at) = sealed {
            if let (Some(sealed_n), Some(merge_n)) = (
                parse_instant_approx(sealed_at),
                merge_or_stamp_at.and_then(parse_instant_approx),
            ) {
                if sealed_n < merge_n {
                    return Ok(true);
                }
                continue;
            }
            return Ok(true);
        }
        if laudado {
            return Ok(true);
        }
    }
    let proof = persist_join(repo, persist, "scope_refine_proof.md");
    if proof.is_file() {
        if let Ok(fm) = load_frontmatter_yaml(&proof) {
            let sealed = fm_str(&fm, "sealed_at").or_else(|| fm_str(&fm, "occurred_at"));
            let reduction = fm_str(&fm, "scope_reduction")
                .map(|v| matches!(v.to_lowercase().as_str(), "true" | "1" | "yes" | "lauded"))
                .unwrap_or(false)
                || fm_str(&fm, "verdict").map(|v| ci_eq(&v, "refined")).unwrap_or(false);
            if reduction {
                if let (Some(merge_n), Some(sealed_n)) = (
                    merge_or_stamp_at.and_then(parse_instant_approx),
                    sealed.as_deref().and_then(parse_instant_approx),
                ) {
                    return Ok(sealed_n < merge_n);
                }
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn milestone_elevator_signals(fm: &HashMap<String, YamlValue>) -> Vec<String> {
    let mut reasons = Vec::new();
    if let Some(res) = fm_str(fm, "resolution") {
        let upper = res.to_ascii_uppercase();
        if ci_eq(&res, "PASS_H1_MILESTONE")
            || (upper.starts_with("PASS_") && upper.ends_with("_MILESTONE"))
        {
            reasons.push(format!("resolution={res}"));
        }
    }
    if let Some(ms) = fm_str(fm, "milestone") {
        let lower = ms.to_ascii_lowercase();
        if let Some(rest) = lower.strip_suffix("-only") {
            if let Some(stripped) = rest.trim().strip_prefix('h') {
                let digits: String = stripped.chars().take_while(|c| c.is_ascii_digit()).collect();
                if !digits.is_empty() && stripped.len() == digits.len() {
                    reasons.push(format!("milestone={ms}"));
                }
            }
        }
    }
    for (k, v) in fm {
        let key_l = k.to_ascii_lowercase();
        if key_l.ends_with("_deferred") || key_l == "h2_h6_deferred" {
            let truthy = match v {
                YamlValue::Bool(true) => true,
                YamlValue::String(s) => {
                    matches!(s.to_lowercase().as_str(), "true" | "1" | "yes" | "on")
                }
                _ => false,
            };
            if truthy {
                reasons.push(format!("{k}=true"));
            }
        }
    }
    reasons
}

fn narrative_ciclo_cerrado(body: &str) -> bool {
    let lower = body.to_ascii_lowercase();
    lower.contains("ciclo cerrado") || (lower.contains("ciclo") && lower.contains("cerrado"))
}

fn delivery_state_success(fm: &HashMap<String, YamlValue>) -> bool {
    fm_str(fm, "delivery_state")
        .map(|s| ci_eq(&s, "success"))
        .unwrap_or(false)
}

fn fault(
    friction_id: &str,
    message: &str,
    reasons: Vec<String>,
    parent_ref: Option<String>,
    child_ref: Option<String>,
) -> ParentalCloseFault {
    ParentalCloseFault {
        friction_id: friction_id.into(),
        message: message.into(),
        reasons,
        parent_ref,
        child_ref,
    }
}

pub fn assert_parental_close_allowed(
    repo: &Path,
    parent_ref_or_persist: &str,
    evidence: &ParentalCloseEvidence,
) -> Result<(), ParentalCloseFault> {
    if lab_skip() {
        return Ok(());
    }
    let intent = evidence.intent.unwrap_or(ParentalCloseIntent::SuccessSeal);
    let child_ref = evidence.child_ref.clone();
    let persist = resolve_persist_ref(repo, parent_ref_or_persist).ok_or_else(|| {
        fault(
            match intent {
                ParentalCloseIntent::Rollup => FRICTION_ROLLUP,
                ParentalCloseIntent::NarrativeCheck => FRICTION_NARRATIVE,
                ParentalCloseIntent::SuccessSeal => FRICTION_SUCCESS,
            },
            "no se pudo resolver persist_ref del padre para contrastar AC/refine (fail-closed)",
            vec!["parent_persist_unresolved".into()],
            Some(parent_ref_or_persist.trim().to_string()),
            child_ref.clone(),
        )
    })?;
    let open_acs = open_acs_from_objectives(repo, &persist)?;
    let prior_refine = prior_scope_refine(repo, &persist, evidence.merge_or_stamp_at.as_deref())?;
    if evidence.same_pass_validacion_rewrite && !prior_refine {
        return Err(fault(
            FRICTION_SCOPE_CUT,
            "rewrite ad hoc de validacion.md en la misma pasada no sustituye refine sellado de reducción de alcance",
            vec!["same_pass_validacion_rewrite".into(), "no_prior_scope_refine".into()],
            Some(persist.clone()),
            child_ref.clone(),
        ));
    }
    let validacion_path = persist_join(repo, &persist, "validacion.md");
    let (validacion_fm, validacion_body) = if validacion_path.is_file() {
        let raw = fs::read_to_string(&validacion_path).unwrap_or_default();
        (
            load_frontmatter_yaml(&validacion_path).unwrap_or_default(),
            strip_frontmatter(&raw),
        )
    } else {
        (HashMap::new(), String::new())
    };
    let elevators = milestone_elevator_signals(&validacion_fm);
    let narrates_close = narrative_ciclo_cerrado(&validacion_body);
    let claims_success = delivery_state_success(&validacion_fm);
    let acs_open = !open_acs.is_empty();
    if !elevators.is_empty() && !prior_refine && (narrates_close || claims_success) {
        let mut reasons = elevators;
        reasons.push("no_prior_scope_refine".into());
        if narrates_close {
            reasons.push("narrative_ciclo_cerrado".into());
        }
        if claims_success {
            reasons.push("delivery_state_success".into());
        }
        return Err(fault(
            FRICTION_NARRATIVE,
            "narración/success parental con tokens de hito/diferimiento sin refine sellado previo",
            reasons,
            Some(persist.clone()),
            child_ref.clone(),
        ));
    }
    if narrates_close && acs_open && !prior_refine {
        let mut reasons: Vec<String> = open_acs.iter().map(|a| format!("parent_ac_open={a}")).collect();
        reasons.push("narrative_ciclo_cerrado".into());
        reasons.push("no_prior_scope_refine".into());
        return Err(fault(
            FRICTION_NARRATIVE,
            "prohibido narrar ciclo cerrado en el padre con AC abiertos sin refine de reducción",
            reasons,
            Some(persist.clone()),
            child_ref.clone(),
        ));
    }
    let closing = matches!(
        intent,
        ParentalCloseIntent::Rollup | ParentalCloseIntent::SuccessSeal
    ) || claims_success;
    if closing && acs_open && !prior_refine {
        let friction = match intent {
            ParentalCloseIntent::Rollup => FRICTION_ROLLUP,
            _ => FRICTION_SUCCESS,
        };
        let mut reasons: Vec<String> = open_acs.iter().map(|a| format!("parent_acs_open={a}")).collect();
        reasons.push("no_prior_scope_refine".into());
        return Err(fault(
            friction,
            "hito/hijo no cierra el PBI padre ni el tracker mientras haya AC abiertos sin refine de reducción",
            reasons,
            Some(persist),
            child_ref,
        ));
    }
    Ok(())
}

pub fn assert_rollup_allowed(
    repo: &Path,
    parent_issue_ref: &str,
    child_ref: Option<&str>,
) -> Result<(), ParentalCloseFault> {
    assert_parental_close_allowed(repo, parent_issue_ref, &ParentalCloseEvidence::rollup(child_ref))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_md(dir: &Path, name: &str, body: &str) {
        let p = dir.join(name);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent).expect("mkdir");
        }
        let mut f = fs::File::create(&p).expect("create");
        f.write_all(body.as_bytes()).expect("write");
    }

    fn parent_persist(repo: &Path) -> PathBuf {
        let p = repo.join("docs/features/ka-mile-parent");
        fs::create_dir_all(&p).expect("persist");
        p
    }

    static LAB_ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn ka_mile_1_rollup_denied_when_parent_acs_open() {
        let _guard = LAB_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        let persist = parent_persist(repo);
        write_md(&persist, "objectives.md", "---\npbi_ref: MAN-PARENT\n---\n\n| **AC-1** | x |\n| **AC-2** | y |\n");
        write_md(&persist, "validacion.md", "---\nglobal: NO_APTO\nchecks:\n  AC-1: pending\n  AC-2: pending\n---\n\n# abierto\n");
        let err = assert_rollup_allowed(repo, "docs/features/ka-mile-parent", Some("MAN-CHILD"))
            .expect_err("deny");
        assert_eq!(err.friction_id, FRICTION_ROLLUP);
    }

    #[test]
    fn ka_mile_2_narrative_denied_with_milestone_tokens() {
        let _guard = LAB_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        let persist = parent_persist(repo);
        write_md(&persist, "objectives.md", "---\npbi_ref: MAN-PARENT\n---\n\n| **AC-1** | x |\n");
        write_md(&persist, "validacion.md", "---\nglobal: NO_APTO\ndelivery_state: success\nmilestone: H1-only\nh2_h6_deferred: true\nchecks:\n  AC-1: pending\n---\n\nEl ciclo queda cerrado tras el hito.\n");
        let err = assert_parental_close_allowed(
            repo,
            "docs/features/ka-mile-parent",
            &ParentalCloseEvidence { intent: Some(ParentalCloseIntent::NarrativeCheck), ..Default::default() },
        ).expect_err("deny");
        assert_eq!(err.friction_id, FRICTION_NARRATIVE);
    }

    #[test]
    fn ka_mile_3_prior_refine_allows_success() {
        let _guard = LAB_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        let persist = parent_persist(repo);
        write_md(&persist, "objectives.md", "---\npbi_ref: MAN-PARENT\nscope_refine_sealed_at: \"2026-11-01T10:00:00Z\"\nscope_cut_laudado: true\n---\n\n| **AC-1** | laudeado |\n");
        write_md(&persist, "validacion.md", "---\nglobal: APTO\ndelivery_state: success\nchecks:\n  AC-1: PASS\n---\n\n# ok\n");
        write_md(&persist, "scope_refine_proof.md", "---\nverdict: refined\nscope_reduction: true\nsealed_at: \"2026-11-01T10:00:00Z\"\n---\n\n# refine\n");
        assert_parental_close_allowed(
            repo,
            "docs/features/ka-mile-parent",
            &ParentalCloseEvidence {
                intent: Some(ParentalCloseIntent::SuccessSeal),
                merge_or_stamp_at: Some("2026-11-10T12:00:00Z".into()),
                ..Default::default()
            },
        ).expect("allow");
    }

    #[test]
    fn ka_mile_3_same_pass_rewrite_denied() {
        let _guard = LAB_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        let persist = parent_persist(repo);
        write_md(&persist, "objectives.md", "---\npbi_ref: MAN-PARENT\n---\n\n| **AC-1** | x |\n");
        write_md(&persist, "validacion.md", "---\nglobal: APTO\ndelivery_state: success\nchecks:\n  AC-1: PASS\n---\n\n# rewrite\n");
        let err = assert_parental_close_allowed(
            repo,
            "docs/features/ka-mile-parent",
            &ParentalCloseEvidence {
                intent: Some(ParentalCloseIntent::SuccessSeal),
                same_pass_validacion_rewrite: true,
                ..Default::default()
            },
        ).expect_err("deny");
        assert_eq!(err.friction_id, FRICTION_SCOPE_CUT);
    }

    #[test]
    fn parental_close_lab_skip() {
        let _guard = LAB_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        let persist = parent_persist(repo);
        write_md(&persist, "objectives.md", "---\npbi_ref: MAN-PARENT\n---\n\n| **AC-1** | x |\n");
        std::env::set_var(LAB_SKIP_ENV, "1");
        let r = assert_rollup_allowed(repo, "docs/features/ka-mile-parent", None);
        std::env::remove_var(LAB_SKIP_ENV);
        r.expect("lab skip");
    }

    #[test]
    fn resolve_persist_by_pbi_ref() {
        let _guard = LAB_ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        let persist = parent_persist(repo);
        write_md(&persist, "objectives.md", "---\npbi_ref: MAN-43\n---\n\n# obj\n");
        assert_eq!(resolve_persist_ref(repo, "MAN-43").unwrap(), "docs/features/ka-mile-parent");
    }
}
