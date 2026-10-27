//! Definición de Ready determinista por tipo de ítem (AUT-19).

use std::collections::HashMap;

pub fn required_sections(item_type: &str) -> &'static [&'static str] {
    match item_type {
        "hu" => &[
            "Rol",
            "Necesidad",
            "Valor",
            "Criterios de aceptación",
        ],
        "fix" => &[
            "Síntoma",
            "Reproducción",
            "Impacto",
            "Criterio de cierre",
        ],
        "kaizen" => &["Fricción", "Propuesta", "Métrica de mejora"],
        "deuda" => &["Origen", "Coste de mantenerla", "Propuesta"],
        "spike" => &["Pregunta", "Timebox", "Entregable"],
        _ => &[
            "Objetivo",
            "Alcance",
            "Criterios de aceptación",
            "Dependencias",
        ],
    }
}

fn section_body(body: &str, title: &str) -> Option<String> {
    let title_l = title.to_lowercase();
    let mut in_section = false;
    let mut lines: Vec<String> = Vec::new();
    for line in body.lines() {
        let t = line.trim();
        if t.starts_with('#') {
            let heading = t.trim_start_matches('#').trim().to_lowercase();
            if in_section {
                break;
            }
            if heading == title_l {
                in_section = true;
                continue;
            }
        } else if in_section {
            lines.push(line.to_string());
        }
    }
    if !in_section {
        return None;
    }
    let joined = lines.join("\n");
    let trimmed = joined.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

/// Huecos de DoR: títulos canónicos ausentes o con cuerpo vacío.
pub fn dor_gaps(item_type: &str, body: &str) -> Vec<String> {
    let mut gaps = Vec::new();
    for sec in required_sections(item_type) {
        if section_body(body, sec).is_none() {
            gaps.push(sec.to_string());
        }
    }
    gaps
}

pub fn dor_ok(item_type: &str, body: &str) -> bool {
    dor_gaps(item_type, body).is_empty()
}

/// Tipo: frontmatter `type` → labels conocidas → `pbi`.
pub fn resolve_item_type(
    fm_type: Option<&str>,
    labels: &[String],
) -> String {
    if let Some(t) = fm_type.map(str::trim).filter(|s| !s.is_empty()) {
        return t.to_string();
    }
    let order = [
        "hu",
        "fix",
        "kaizen",
        "deuda",
        "spike",
        "pbi",
    ];
    let label_set: HashMap<String, bool> = labels
        .iter()
        .map(|l| (l.to_lowercase(), true))
        .collect();
    for key in order {
        if label_set.contains_key(key) {
            return key.to_string();
        }
    }
    "pbi".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dor_pbi_requires_four_sections() {
        let body = "## Objetivo\n\nx\n## Alcance\n\ny\n";
        assert!(!dor_ok("pbi", body));
        let full = "## Objetivo\n\nx\n## Alcance\n\ny\n## Criterios de aceptación\n\nz\n## Dependencias\n\nw\n";
        assert!(dor_ok("pbi", full));
    }

    #[test]
    fn type_from_labels() {
        assert_eq!(resolve_item_type(None, &["fix".into()]), "fix");
        assert_eq!(resolve_item_type(Some("spike"), &["pbi".into()]), "spike");
    }
}
