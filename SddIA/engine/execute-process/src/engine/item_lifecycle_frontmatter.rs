//! Frontmatter del ítem de backlog: columna de tracker vs refinamiento (AUT-5 / MAN-14).

use serde_yaml::Value as YamlValue;

pub const FM_TRACKER_COLUMN: &str = "tracker_column";
pub const FM_REFINEMENT: &str = "refinement";
pub const FM_REFINED_DATE: &str = "refined";

pub const REFINEMENT_PENDING: &str = "pending";
pub const REFINEMENT_REFINED: &str = "refined";

pub const COLUMN_BACKLOG: &str = "backlog";
pub const COLUMN_TODO: &str = "todo";
pub const COLUMN_IN_PROGRESS: &str = "in_progress";
pub const COLUMN_IN_REVIEW: &str = "in_review";
pub const COLUMN_DONE: &str = "done";
pub const COLUMN_CANCELLED: &str = "cancelled";

/// Líneas YAML (sin `---`) para el alta `forge-pbi`.
pub fn forge_frontmatter_lines() -> String {
    format!(
        "{FM_TRACKER_COLUMN}: {COLUMN_BACKLOG}\n{FM_REFINEMENT}: {REFINEMENT_PENDING}"
    )
}

pub fn yaml_patch_refinement_refined(refined_date: &str) -> YamlValue {
    serde_yaml::from_str(&format!(
        "{FM_TRACKER_COLUMN}: {COLUMN_TODO}\n{FM_REFINEMENT}: {REFINEMENT_REFINED}\n{FM_REFINED_DATE}: \"{refined_date}\""
    ))
    .expect("patch refinado")
}

pub fn yaml_patch_tracker_column(column: &str) -> YamlValue {
    serde_yaml::from_str(&format!("{FM_TRACKER_COLUMN}: {column}"))
        .expect("patch columna")
}

pub fn yaml_patch_cancelled() -> YamlValue {
    serde_yaml::from_str(&format!(
        "{FM_TRACKER_COLUMN}: {COLUMN_CANCELLED}\n{FM_REFINEMENT}: {REFINEMENT_REFINED}"
    ))
    .expect("patch cancelado")
}

fn split_frontmatter(raw: &str) -> Result<(String, String), String> {
    let mut lines = raw.lines();
    if lines.next() != Some("---") {
        return Err("ITEM_FM: sin frontmatter".into());
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
    Err("ITEM_FM: frontmatter sin cierre".into())
}

fn strip_legacy_status(fm: &mut YamlValue) {
    if let Some(map) = fm.as_mapping_mut() {
        map.remove("status");
    }
}

/// Fusiona `patch` en el frontmatter y elimina `status` legado (pending / refinado / refinada).
pub fn seal_markdown(raw: &str, patch: &YamlValue) -> Result<String, String> {
    let (fm_text, body) = split_frontmatter(raw)?;
    let mut fm: YamlValue = serde_yaml::from_str(&fm_text).map_err(|e| e.to_string())?;
    strip_legacy_status(&mut fm);
    if let Some(map) = patch.as_mapping() {
        for (k, v) in map {
            if let Some(key) = k.as_str() {
                fm[key] = v.clone();
            }
        }
    }
    let new_fm = serde_yaml::to_string(&fm).map_err(|e| e.to_string())?;
    let body = body.trim_start_matches('\n');
    Ok(format!("---\n{new_fm}---\n\n{body}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forge_lines_no_legacy_status() {
        let lines = forge_frontmatter_lines();
        assert!(lines.contains("tracker_column: backlog"));
        assert!(!lines.contains("status:"));
    }

    #[test]
    fn refine_strips_status_and_sets_column() {
        let raw = "---\ndocument_id: X\nstatus: pending\n---\n\n# body\n";
        let patch = yaml_patch_refinement_refined("2026-10-25");
        let out = seal_markdown(raw, &patch).unwrap();
        assert!(!out.contains("status: pending"));
        assert!(out.contains("tracker_column: todo"));
        assert!(out.contains("refinement: refined"));
    }

    #[test]
    fn column_patch_in_progress() {
        let raw = "---\ntracker_column: todo\nrefinement: refined\n---\n\n# x\n";
        let out = seal_markdown(raw, &yaml_patch_tracker_column(COLUMN_IN_PROGRESS)).unwrap();
        assert!(out.contains("tracker_column: in_progress"));
        assert!(out.contains("refinement: refined"));
    }
}
