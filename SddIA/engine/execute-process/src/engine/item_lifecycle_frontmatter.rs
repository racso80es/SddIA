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
pub const COLUMN_IN_VALIDATION: &str = "in_validation";
pub const COLUMN_IN_REVIEW: &str = "in_review";
pub const COLUMN_VALIDATED: &str = "validated";
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

fn yaml_patch_column_and_lines(column: &str, lines: &str) -> YamlValue {
    let mut block = format!("{FM_TRACKER_COLUMN}: {column}");
    if !lines.trim().is_empty() {
        block.push('\n');
        block.push_str(lines.trim());
    }
    serde_yaml::from_str(&block).expect("patch lifecycle")
}

pub fn yaml_patch_forge_closed(forge_date: &str, forge_by: &str) -> YamlValue {
    yaml_patch_column_and_lines(
        COLUMN_IN_VALIDATION,
        &format!("forge_date: \"{forge_date}\"\nforge_by: \"{forge_by}\""),
    )
}

pub fn yaml_patch_validation_apto(validated_date: &str, validated_by: &str) -> YamlValue {
    yaml_patch_column_and_lines(
        COLUMN_VALIDATED,
        &format!(
            "validated_status: apto\nvalidated_date: \"{validated_date}\"\nvalidated_by: \"{validated_by}\""
        ),
    )
}

pub fn yaml_patch_validation_no_apto(validated_date: &str, validated_by: &str) -> YamlValue {
    yaml_patch_column_and_lines(
        COLUMN_IN_PROGRESS,
        &format!(
            "validated_status: no apto\nvalidated_date: \"{validated_date}\"\nvalidated_by: \"{validated_by}\""
        ),
    )
}

pub fn yaml_patch_pr_opened(in_review_date: &str, in_review_by: &str) -> YamlValue {
    yaml_patch_column_and_lines(
        COLUMN_IN_REVIEW,
        &format!("in_review_date: \"{in_review_date}\"\nin_review_by: \"{in_review_by}\""),
    )
}

pub fn yaml_patch_review_ci_green(done_date: &str, done_by: &str) -> YamlValue {
    yaml_patch_column_and_lines(
        COLUMN_DONE,
        &format!("done_status: ok\ndone_date: \"{done_date}\"\ndone_by: \"{done_by}\""),
    )
}

pub fn yaml_patch_review_ci_red(done_date: &str, done_by: &str) -> YamlValue {
    yaml_patch_column_and_lines(
        COLUMN_IN_PROGRESS,
        &format!("done_status: false\ndone_date: \"{done_date}\"\ndone_by: \"{done_by}\""),
    )
}

pub fn yaml_patch_forge_started() -> YamlValue {
    yaml_patch_tracker_column(COLUMN_IN_PROGRESS)
}

pub fn yaml_patch_cancelled() -> YamlValue {
    serde_yaml::from_str(&format!(
        "{FM_TRACKER_COLUMN}: {COLUMN_CANCELLED}\n{FM_REFINEMENT}: {REFINEMENT_REFINED}"
    ))
    .expect("patch cancelado")
}

/// Convierte descripciones de tracker (p. ej. bloque `yaml` de Linear) al formato `---` / cuerpo.
pub fn normalize_item_markdown(raw: &str) -> Result<String, String> {
    let trimmed = raw.trim_start();
    if trimmed.starts_with("---") {
        return Ok(raw.to_string());
    }
    if !trimmed.starts_with("```") {
        return Err("ITEM_FM: sin frontmatter reconocible".into());
    }
    let after_ticks = trimmed
        .strip_prefix("```")
        .ok_or_else(|| "ITEM_FM: fence inválido".to_string())?;
    let yaml_start = after_ticks
        .strip_prefix("yaml")
        .or_else(|| after_ticks.strip_prefix("yml"))
        .ok_or_else(|| "ITEM_FM: fence sin yaml".to_string())?
        .trim_start_matches(['\r', '\n']);
    let close = yaml_start
        .find("```")
        .ok_or_else(|| "ITEM_FM: fence sin cierre".to_string())?;
    let fm_text = yaml_start[..close].trim();
    let body = yaml_start[close + 3..].trim_start_matches(['\r', '\n']);
    if fm_text.is_empty() {
        return Err("ITEM_FM: yaml vacío".into());
    }
    if body.is_empty() {
        Ok(format!("---\n{fm_text}\n---\n"))
    } else {
        Ok(format!("---\n{fm_text}\n---\n\n{body}"))
    }
}

/// Fusiona `patch` en el frontmatter y elimina `status` legado (pending / refinado / refinada).
pub fn seal_markdown(raw: &str, patch: &YamlValue) -> Result<String, String> {
    super::tracker_item_document::seal_item_description(
        raw,
        patch,
        super::tracker_item_document::TrackerDescriptionDialect::FileMarkdown,
    )
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

    #[test]
    fn seal_markdown_accepts_linear_yaml_fence() {
        let raw = "```yaml\ndocument_id: X\ntracker_column: backlog\nrefinement: pending\n```\n\n# cuerpo\n";
        let patch = yaml_patch_refinement_refined("2026-10-26");
        let out = seal_markdown(raw, &patch).unwrap();
        assert!(out.starts_with("---"));
        assert!(out.contains("tracker_column: todo"));
        assert!(out.contains("refinement: refined"));
        assert!(out.contains("# cuerpo"));
    }
}
