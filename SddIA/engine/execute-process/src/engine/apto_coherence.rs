//! Aduana de coherencia documental APTO (MAN-44 / KZ-1).
//! Contrasta `validacion.md` ↔ `execution.md` ↔ `plan.md` del mismo `persist_ref`.

use crate::core::parser::load_frontmatter_yaml;
use serde_yaml::Value as YamlValue;
use std::fs;
use std::path::{Path, PathBuf};

/// Skip lab-only del contraste (no usar en CI productivo).
pub const LAB_SKIP_ENV: &str = "SDDIA_LAB_SKIP_APTO_COHERENCE";

pub const FRICTION_EXECUTION_BLOCKED: &str = "apto_coherence_execution_blocked";
pub const FRICTION_OPEN_PHASES: &str = "apto_coherence_open_phases";
pub const FRICTION_RESOLUTION: &str = "apto_coherence_resolution";

const OPERATOR_HINT: &str =
    "Corregir execution/plan o bajar global a NO_APTO antes del sello/fusión";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoherenceFault {
    pub friction_id: String,
    pub message: String,
    pub reasons: Vec<String>,
}

impl CoherenceFault {
    pub fn operator_hint(&self) -> &'static str {
        OPERATOR_HINT
    }

    /// Mensaje corto para Err de accept-pr / validate_for_accept_pr_merge.
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
    let base = persist_ref
        .trim()
        .trim_start_matches("./")
        .trim_end_matches('/');
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

fn fm_str(fm: &std::collections::HashMap<String, YamlValue>, key: &str) -> Option<String> {
    fm.get(key).and_then(yaml_str).filter(|s| !s.is_empty())
}

fn is_apto_global(fm: &std::collections::HashMap<String, YamlValue>) -> bool {
    fm_str(fm, "global")
        .map(|g| g.eq_ignore_ascii_case("APTO"))
        .unwrap_or(false)
}

fn ci_eq(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b)
}

/// `validate_apto_coherence(repo, persist_ref)` — no-op si no hay `global: APTO` o lab skip.
pub fn validate_apto_coherence(repo: &Path, persist_ref: &str) -> Result<(), CoherenceFault> {
    if lab_skip() {
        return Ok(());
    }
    let persist = persist_ref
        .trim()
        .trim_start_matches("./")
        .trim_end_matches('/');
    if persist.is_empty() {
        return Ok(());
    }

    let validacion_path = persist_join(repo, persist, "validacion.md");
    if !validacion_path.is_file() {
        return Ok(());
    }
    let validacion_fm = load_frontmatter_yaml(&validacion_path).map_err(|e| CoherenceFault {
        friction_id: FRICTION_EXECUTION_BLOCKED.into(),
        message: format!("no se pudo leer validacion.md: {e}"),
        reasons: vec!["validacion_unreadable".into()],
    })?;
    if !is_apto_global(&validacion_fm) {
        return Ok(());
    }

    let mut reasons: Vec<String> = Vec::new();
    let mut first: Option<CoherenceFault> = None;

    // C2 — ejecución no cerrada
    if let Some(fault) = check_c2_execution(repo, persist, &validacion_fm) {
        reasons.extend(fault.reasons.iter().cloned());
        first.get_or_insert(fault);
    }

    // C3 — fases abiertas
    if let Some(fault) = check_c3_open_phases(repo, persist) {
        reasons.extend(fault.reasons.iter().cloned());
        first.get_or_insert(fault);
    }

    // C4 — resolución no canónica (frontmatter de validacion + execution)
    if let Some(fault) = check_c4_resolution(repo, persist, &validacion_fm) {
        reasons.extend(fault.reasons.iter().cloned());
        first.get_or_insert(fault);
    }

    if let Some(mut fault) = first {
        // Dedup reasons preserving order
        let mut seen = std::collections::HashSet::new();
        fault.reasons = reasons
            .into_iter()
            .filter(|r| seen.insert(r.clone()))
            .collect();
        return Err(fault);
    }
    Ok(())
}

fn check_c2_execution(
    repo: &Path,
    persist: &str,
    _validacion_fm: &std::collections::HashMap<String, YamlValue>,
) -> Option<CoherenceFault> {
    let path = persist_join(repo, persist, "execution.md");
    if !path.is_file() {
        return Some(CoherenceFault {
            friction_id: FRICTION_EXECUTION_BLOCKED.into(),
            message: "global APTO sin execution.md (no hay evidencia de cierre)".into(),
            reasons: vec!["execution_missing".into()],
        });
    }
    let fm = match load_frontmatter_yaml(&path) {
        Ok(f) => f,
        Err(e) => {
            return Some(CoherenceFault {
                friction_id: FRICTION_EXECUTION_BLOCKED.into(),
                message: format!("execution.md ilegible: {e}"),
                reasons: vec!["execution_unreadable".into()],
            });
        }
    };

    let verdict = fm_str(&fm, "verdict").unwrap_or_default();
    let status = fm_str(&fm, "status").unwrap_or_default();

    let blocked_verdict = ["blocked", "NO_APTO", "no_apto", "fail", "failed"]
        .iter()
        .any(|v| ci_eq(&verdict, v));
    let blocked_status = ["partial", "blocked", "in_progress", "open"]
        .iter()
        .any(|s| ci_eq(&status, s));

    if blocked_verdict || blocked_status {
        let mut reasons = Vec::new();
        if blocked_verdict {
            reasons.push(format!("execution_verdict={verdict}"));
        }
        if blocked_status {
            reasons.push(format!("execution_status={status}"));
        }
        return Some(CoherenceFault {
            friction_id: FRICTION_EXECUTION_BLOCKED.into(),
            message: format!(
                "global APTO incoherente con execution.verdict={verdict} status={status}"
            ),
            reasons,
        });
    }
    None
}

fn phase_id_from_map(m: &serde_yaml::Mapping) -> Option<String> {
    for key in ["id", "name", "phase", "key"] {
        if let Some(YamlValue::String(s)) = m.get(YamlValue::String(key.into())) {
            let t = s.trim();
            if !t.is_empty() {
                return Some(t.to_string());
            }
        }
    }
    None
}

fn phase_status_from_map(m: &serde_yaml::Mapping) -> Option<String> {
    m.get(YamlValue::String("status".into()))
        .and_then(yaml_str)
}

fn collect_declared_phases(
    plan_fm: &std::collections::HashMap<String, YamlValue>,
    plan_body: &str,
) -> Vec<(String, Option<String>)> {
    let mut out: Vec<(String, Option<String>)> = Vec::new();
    let mut seen = std::collections::HashSet::new();

    let push = |id: String, st: Option<String>, out: &mut Vec<(String, Option<String>)>, seen: &mut std::collections::HashSet<String>| {
        let key = id.trim().to_string();
        if key.is_empty() || !seen.insert(key.clone()) {
            return;
        }
        out.push((key, st));
    };

    for key in ["phases", "children", "hijos"] {
        if let Some(v) = plan_fm.get(key) {
            match v {
                YamlValue::Sequence(seq) => {
                    for item in seq {
                        match item {
                            YamlValue::String(s) => push(s.clone(), None, &mut out, &mut seen),
                            YamlValue::Mapping(m) => {
                                if let Some(id) = phase_id_from_map(m) {
                                    push(id, phase_status_from_map(m), &mut out, &mut seen);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                YamlValue::Mapping(m) => {
                    for (k, val) in m {
                        if let YamlValue::String(id) = k {
                            let st = match val {
                                YamlValue::Mapping(inner) => phase_status_from_map(inner),
                                YamlValue::String(s) => Some(s.clone()),
                                _ => None,
                            };
                            push(id.clone(), st, &mut out, &mut seen);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // Cuerpo: encabezados #+ H\d+ / Ola N
    for line in plan_body.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix('#') {
            let rest = rest.trim_start_matches('#').trim();
            if let Some(id) = extract_heading_phase_id(rest) {
                push(id, None, &mut out, &mut seen);
            }
        }
    }

    out
}

fn extract_heading_phase_id(heading: &str) -> Option<String> {
    let heading = heading.trim();
    // H0, H1, H2A, …
    if heading.starts_with('H') {
        let mut end = 1;
        let bytes = heading.as_bytes();
        while end < bytes.len() && bytes[end].is_ascii_digit() {
            end += 1;
        }
        if end > 1 {
            let mut id_end = end;
            if id_end < bytes.len() && bytes[id_end].is_ascii_uppercase() {
                id_end += 1;
            }
            // boundary: end of string, whitespace, or punctuation
            if id_end == bytes.len()
                || bytes[id_end].is_ascii_whitespace()
                || matches!(heading[id_end..].chars().next(), Some('-' | ':' | '—' | '|'))
            {
                return Some(heading[..id_end].to_string());
            }
        }
    }
    // Ola N / OlaN
    let lower = heading.to_ascii_lowercase();
    if let Some(rest) = lower.strip_prefix("ola") {
        let rest = rest.trim_start();
        let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
        if !digits.is_empty() {
            return Some(format!("Ola {digits}"));
        }
    }
    None
}

fn closed_status(st: &str) -> bool {
    ["closed", "done", "forged", "complete", "apto"]
        .iter()
        .any(|s| ci_eq(st, s))
}

fn open_status(st: &str) -> bool {
    ["open", "pending", "blocked", "deferred"]
        .iter()
        .any(|s| ci_eq(st, s))
}

fn phase_closed_in_execution(
    exec_fm: &std::collections::HashMap<String, YamlValue>,
    exec_body: &str,
    phase_id: &str,
) -> bool {
    // items_applied / phases_closed / closed_phases
    for key in ["items_applied", "phases_closed", "closed_phases"] {
        if let Some(v) = exec_fm.get(key) {
            if sequence_or_str_contains(v, phase_id) {
                return true;
            }
        }
    }

    // phases.<id>.status
    if let Some(YamlValue::Mapping(phases)) = exec_fm.get("phases") {
        if let Some(val) = phases.get(YamlValue::String(phase_id.into())) {
            match val {
                YamlValue::Mapping(m) => {
                    if let Some(st) = phase_status_from_map(m) {
                        if closed_status(&st) {
                            return true;
                        }
                    }
                }
                YamlValue::String(s) if closed_status(s) => return true,
                _ => {}
            }
        }
    }

    // Cuerpo: línea con id + token closed|done|forged|APTO
    for line in exec_body.lines() {
        if !line.contains(phase_id) {
            continue;
        }
        let lower = line.to_ascii_lowercase();
        if lower.contains("closed")
            || lower.contains("done")
            || lower.contains("forged")
            || lower.contains("apto")
        {
            return true;
        }
    }
    false
}

fn sequence_or_str_contains(v: &YamlValue, needle: &str) -> bool {
    match v {
        YamlValue::String(s) => {
            s.split(|c: char| c.is_whitespace() || c == ',' || c == ';')
                .any(|p| p.trim() == needle || ci_eq(p.trim(), needle))
                || s.contains(needle)
        }
        YamlValue::Sequence(seq) => seq.iter().any(|item| match item {
            YamlValue::String(s) => s.trim() == needle || ci_eq(s.trim(), needle),
            YamlValue::Mapping(m) => phase_id_from_map(m).map(|id| id == needle).unwrap_or(false),
            _ => false,
        }),
        YamlValue::Mapping(m) => m
            .keys()
            .any(|k| matches!(k, YamlValue::String(s) if s == needle)),
        _ => false,
    }
}

fn check_c3_open_phases(repo: &Path, persist: &str) -> Option<CoherenceFault> {
    let plan_path = persist_join(repo, persist, "plan.md");
    if !plan_path.is_file() {
        // C3 no aplica sin plan
        return None;
    }
    let plan_raw = fs::read_to_string(&plan_path).ok()?;
    let plan_fm = load_frontmatter_yaml(&plan_path).ok()?;
    let plan_body = strip_frontmatter(&plan_raw);

    let declared = collect_declared_phases(&plan_fm, &plan_body);
    if declared.is_empty() {
        return None;
    }

    let exec_path = persist_join(repo, persist, "execution.md");
    let (exec_fm, exec_body) = if exec_path.is_file() {
        let raw = fs::read_to_string(&exec_path).unwrap_or_default();
        let fm = load_frontmatter_yaml(&exec_path).unwrap_or_default();
        (fm, strip_frontmatter(&raw))
    } else {
        (Default::default(), String::new())
    };

    let mut open: Vec<String> = Vec::new();
    for (id, plan_st) in declared {
        if let Some(ref st) = plan_st {
            if open_status(st) {
                open.push(id);
                continue;
            }
            if closed_status(st) {
                continue;
            }
        }
        if !phase_closed_in_execution(&exec_fm, &exec_body, &id) {
            open.push(id);
        }
    }

    if open.is_empty() {
        return None;
    }
    Some(CoherenceFault {
        friction_id: FRICTION_OPEN_PHASES.into(),
        message: format!(
            "global APTO con fases abiertas: {}",
            open.join(", ")
        ),
        reasons: open.into_iter().map(|id| format!("open_phase={id}")).collect(),
    })
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

fn resolution_is_denylisted(resolution: &str) -> bool {
    let r = resolution.trim();
    if r.is_empty() {
        return false;
    }
    if ci_eq(r, "PASS_H1_MILESTONE") {
        return true;
    }
    // PASS_*_MILESTONE
    let upper = r.to_ascii_uppercase();
    if upper.starts_with("PASS_") && upper.ends_with("_MILESTONE") {
        return true;
    }
    false
}

fn milestone_is_elevator(milestone: &str) -> bool {
    let m = milestone.trim();
    if m.is_empty() {
        return false;
    }
    // H1-only / H*-only (case-insensitive)
    let lower = m.to_ascii_lowercase();
    if let Some(rest) = lower.strip_suffix("-only") {
        let rest = rest.trim();
        if let Some(stripped) = rest.strip_prefix('h') {
            let digits: String = stripped.chars().take_while(|c| c.is_ascii_digit()).collect();
            if !digits.is_empty() && stripped.len() == digits.len() {
                return true;
            }
        }
    }
    false
}

fn deferred_elevator_keys(fm: &std::collections::HashMap<String, YamlValue>) -> Vec<String> {
    let mut out = Vec::new();
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
                out.push(k.clone());
            }
        }
    }
    out
}

fn check_c4_on_fm(
    fm: &std::collections::HashMap<String, YamlValue>,
    source: &str,
) -> Option<CoherenceFault> {
    let mut reasons = Vec::new();

    if let Some(res) = fm_str(fm, "resolution") {
        if resolution_is_denylisted(&res) {
            reasons.push(format!("{source}_resolution={res}"));
        }
    }
    if let Some(ms) = fm_str(fm, "milestone") {
        if milestone_is_elevator(&ms) {
            reasons.push(format!("{source}_milestone={ms}"));
        }
    }
    for k in deferred_elevator_keys(fm) {
        reasons.push(format!("{source}_{k}=true"));
    }

    if reasons.is_empty() {
        return None;
    }
    Some(CoherenceFault {
        friction_id: FRICTION_RESOLUTION.into(),
        message: format!(
            "global APTO elevado por resolución/hito no canónico ({source})"
        ),
        reasons,
    })
}

fn check_c4_resolution(
    repo: &Path,
    persist: &str,
    validacion_fm: &std::collections::HashMap<String, YamlValue>,
) -> Option<CoherenceFault> {
    let mut first: Option<CoherenceFault> = None;
    let mut all_reasons = Vec::new();

    if let Some(f) = check_c4_on_fm(validacion_fm, "validacion") {
        all_reasons.extend(f.reasons.iter().cloned());
        first.get_or_insert(f);
    }

    let exec_path = persist_join(repo, persist, "execution.md");
    if exec_path.is_file() {
        if let Ok(fm) = load_frontmatter_yaml(&exec_path) {
            if let Some(f) = check_c4_on_fm(&fm, "execution") {
                all_reasons.extend(f.reasons.iter().cloned());
                first.get_or_insert(f);
            }
        }
    }

    if let Some(mut fault) = first {
        fault.reasons = all_reasons;
        return Some(fault);
    }
    None
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

    fn persist_dir(root: &Path) -> PathBuf {
        let p = root.join("docs/features/_fixture-ka-close");
        fs::create_dir_all(&p).expect("persist");
        p
    }

    #[test]
    fn ka_close_1_execution_blocked_rejects_apto() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        let persist = persist_dir(repo);
        write_md(
            &persist,
            "validacion.md",
            "---\nglobal: APTO\n---\n\n# ok\n",
        );
        write_md(
            &persist,
            "execution.md",
            "---\nverdict: blocked\nstatus: partial\n---\n\n# exec\n",
        );
        write_md(
            &persist,
            "plan.md",
            "---\nphases: [H0]\n---\n\n# H0\n",
        );
        // Mark H0 closed so only C2 fires as primary
        write_md(
            &persist,
            "execution.md",
            "---\nverdict: blocked\nstatus: partial\nphases_closed: [H0]\n---\n\n# exec\nH0 closed\n",
        );

        let rel = "docs/features/_fixture-ka-close";
        let err = validate_apto_coherence(repo, rel).expect_err("must block");
        assert_eq!(err.friction_id, FRICTION_EXECUTION_BLOCKED);
        assert!(err.message.contains("blocked") || err.reasons.iter().any(|r| r.contains("blocked")));
    }

    #[test]
    fn ka_close_2_open_phases_rejects_apto() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        let persist = persist_dir(repo);
        write_md(
            &persist,
            "validacion.md",
            "---\nglobal: APTO\n---\n\n# ok\n",
        );
        write_md(
            &persist,
            "execution.md",
            "---\nverdict: ok\nstatus: complete\nphases_closed: [H0]\n---\n\n# H0 closed\n",
        );
        write_md(
            &persist,
            "plan.md",
            "---\nphases:\n  - H0\n  - H1\n  - H2\n---\n\n# Plan\n\n## H0 — done\n## H1 — open\n## H2 — open\n",
        );

        let rel = "docs/features/_fixture-ka-close";
        let err = validate_apto_coherence(repo, rel).expect_err("must block open phases");
        assert_eq!(err.friction_id, FRICTION_OPEN_PHASES);
        assert!(err.reasons.iter().any(|r| r.contains("H1") || r.contains("H2")));
    }

    #[test]
    fn ka_close_3_resolution_denylist_rejects_apto() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        let persist = persist_dir(repo);
        write_md(
            &persist,
            "validacion.md",
            "---\nglobal: APTO\nresolution: PASS_H1_MILESTONE\nmilestone: H1-only\nh2_h6_deferred: true\n---\n\n# ok\n",
        );
        write_md(
            &persist,
            "execution.md",
            "---\nverdict: ok\nstatus: complete\nphases_closed: [H0]\n---\n\n# H0 closed\n",
        );
        write_md(
            &persist,
            "plan.md",
            "---\nphases: [H0]\n---\n\n# H0\n",
        );

        let rel = "docs/features/_fixture-ka-close";
        let err = validate_apto_coherence(repo, rel).expect_err("must block resolution");
        assert_eq!(err.friction_id, FRICTION_RESOLUTION);
        assert!(err.reasons.iter().any(|r| r.contains("PASS_H1_MILESTONE") || r.contains("milestone") || r.contains("deferred")));
    }

    #[test]
    fn apto_coherence_noop_without_apto() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        let persist = persist_dir(repo);
        write_md(
            &persist,
            "validacion.md",
            "---\nglobal: NO_APTO\n---\n\n# no\n",
        );
        write_md(
            &persist,
            "execution.md",
            "---\nverdict: blocked\nstatus: partial\n---\n\n# exec\n",
        );
        let rel = "docs/features/_fixture-ka-close";
        validate_apto_coherence(repo, rel).expect("noop");
    }

    #[test]
    fn apto_coherence_lab_skip() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        let persist = persist_dir(repo);
        write_md(
            &persist,
            "validacion.md",
            "---\nglobal: APTO\n---\n\n# ok\n",
        );
        write_md(
            &persist,
            "execution.md",
            "---\nverdict: blocked\nstatus: partial\n---\n\n# exec\n",
        );
        std::env::set_var(LAB_SKIP_ENV, "1");
        let rel = "docs/features/_fixture-ka-close";
        let r = validate_apto_coherence(repo, rel);
        std::env::remove_var(LAB_SKIP_ENV);
        r.expect("lab skip");
    }

    #[test]
    fn coherent_apto_passes() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let repo = tmp.path();
        let persist = persist_dir(repo);
        write_md(
            &persist,
            "validacion.md",
            "---\nglobal: APTO\n---\n\n# ok\n",
        );
        write_md(
            &persist,
            "execution.md",
            "---\nverdict: ok\nstatus: complete\nphases_closed: [H0, H1]\n---\n\n# H0 closed\n# H1 forged\n",
        );
        write_md(
            &persist,
            "plan.md",
            "---\nphases: [H0, H1]\n---\n\n## H0\n## H1\n",
        );
        let rel = "docs/features/_fixture-ka-close";
        validate_apto_coherence(repo, rel).expect("coherent");
    }
}
