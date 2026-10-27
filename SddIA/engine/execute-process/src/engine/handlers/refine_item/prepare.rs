use super::dor;
use crate::engine::item_lifecycle_frontmatter::{
    COLUMN_BACKLOG, FM_REFINEMENT, REFINEMENT_PENDING,
};
use crate::engine::project_binding::TrackerConfig;
use serde_yaml::Value as YamlValue;
use uuid::Uuid;

#[derive(Clone)]
pub struct FetchSnapshot {
    pub description: String,
    pub identifier: String,
    pub state_name: String,
    pub labels: Vec<String>,
    pub updated_at: String,
}

pub fn column_from_linear_state(tc: &TrackerConfig, state_name: &str) -> String {
    for (col, linear) in &tc.state_map {
        if linear.eq_ignore_ascii_case(state_name) {
            return col.clone();
        }
    }
    COLUMN_BACKLOG.to_string()
}

fn split_frontmatter(raw: &str) -> Result<(String, String), String> {
    let mut lines = raw.lines();
    if lines.next() != Some("---") {
        return Err("sin frontmatter".into());
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
    Err("frontmatter sin cierre".into())
}

fn parse_fm(text: &str) -> Result<YamlValue, String> {
    let (fm, _) = split_frontmatter(text)?;
    serde_yaml::from_str(&fm).map_err(|e| e.to_string())
}

fn fm_str(fm: &YamlValue, key: &str) -> Option<String> {
    fm.get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

/// Si la descripción no tiene cabecera reconocible, genera la mínima y conserva el cuerpo.
pub fn ensure_canonical_markdown(
    raw_description: &str,
    snap: &FetchSnapshot,
    project_slug: &str,
    tc: Option<&TrackerConfig>,
) -> Result<String, String> {
    if let Ok(canonical) =
        crate::engine::item_lifecycle_frontmatter::normalize_item_markdown(raw_description)
    {
        return Ok(canonical);
    }
    let body = raw_description.trim();
    let uuid = Uuid::new_v4();
    let document_id = format!("PBI-{}", &uuid.to_string()[..8]);
    let tracker_column = tc
        .map(|c| column_from_linear_state(c, &snap.state_name))
        .unwrap_or_else(|| COLUMN_BACKLOG.to_string());
    let item_type = dor::resolve_item_type(None, &snap.labels);
    let mut lines = Vec::new();
    lines.push(format!("document_id: {document_id}"));
    lines.push(format!("uuid: \"{uuid}\""));
    lines.push(format!("tracker_column: {tracker_column}"));
    lines.push(format!("{FM_REFINEMENT}: {REFINEMENT_PENDING}"));
    lines.push(format!("type: {item_type}"));
    lines.push(format!("tracker_ref: {}", snap.identifier));
    lines.push(format!("project_slug: {project_slug}"));
    let fm = lines.join("\n");
    if body.is_empty() {
        Ok(format!("---\n{fm}\n---\n"))
    } else {
        Ok(format!("---\n{fm}\n---\n\n{body}\n"))
    }
}

pub fn parse_properties(canonical: &str) -> Result<(YamlValue, String), String> {
    let (fm_text, body) = split_frontmatter(canonical)?;
    let fm = serde_yaml::from_str(&fm_text).map_err(|e| e.to_string())?;
    Ok((fm, body))
}

pub fn item_type_from_fm(fm: &YamlValue, labels: &[String]) -> String {
    dor::resolve_item_type(fm_str(fm, "type").as_deref(), labels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_frontmatter_for_plain_body() {
        let snap = FetchSnapshot {
            description: "# Solo cuerpo\n\nTexto.\n".to_string(),
            identifier: "AUT-99".into(),
            state_name: "Todo".into(),
            labels: vec!["pbi".into()],
            updated_at: "t".into(),
        };
        let out = ensure_canonical_markdown(&snap.description, &snap, "core", None).unwrap();
        assert!(out.starts_with("---\n"));
        assert!(out.contains("tracker_ref: AUT-99"));
        assert!(out.contains("# Solo cuerpo"));
    }
}
