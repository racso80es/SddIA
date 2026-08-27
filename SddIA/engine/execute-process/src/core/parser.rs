use serde_yaml::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn parse_frontmatter(md_path: &Path) -> Result<HashMap<String, Value>, String> {
    let text = fs::read_to_string(md_path).map_err(|e| e.to_string())?;
    let parts: Vec<&str> = text.split("---").collect();
    if parts.len() < 3 {
        return Ok(HashMap::new());
    }
    let fm: Value = serde_yaml::from_str(parts[1].trim()).map_err(|e| e.to_string())?;
    match fm {
        Value::Mapping(m) => {
            let mut out = HashMap::new();
            for (k, v) in m {
                if let Some(key) = k.as_str() {
                    out.insert(key.to_string(), v);
                }
            }
            Ok(out)
        }
        _ => Ok(HashMap::new()),
    }
}

pub fn load_frontmatter_yaml(md_path: &Path) -> Result<HashMap<String, Value>, String> {
    parse_frontmatter(md_path)
}

/// Parsea frontmatter YAML desde el contenido crudo de un `.md` (misma regla que `parse_frontmatter`).
pub fn parse_frontmatter_from_str(text: &str) -> Result<HashMap<String, Value>, String> {
    let parts: Vec<&str> = text.split("---").collect();
    if parts.len() < 3 {
        return Ok(HashMap::new());
    }
    let fm: Value = serde_yaml::from_str(parts[1].trim()).map_err(|e| e.to_string())?;
    match fm {
        Value::Mapping(m) => {
            let mut out = HashMap::new();
            for (k, v) in m {
                if let Some(key) = k.as_str() {
                    out.insert(key.to_string(), v);
                }
            }
            Ok(out)
        }
        _ => Ok(HashMap::new()),
    }
}

pub fn frontmatter_yaml_to_json(map: &HashMap<String, Value>) -> serde_json::Value {
    let mut obj = serde_json::Map::new();
    for (k, v) in map {
        obj.insert(k.clone(), serde_json::to_value(v).unwrap_or(serde_json::Value::Null));
    }
    serde_json::Value::Object(obj)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_kalma2_frontmatter() {
        let root = crate::core::repo::find_repo_root().unwrap();
        let path = root.join("SddIA/process/kalma2-interact.md");
        let fm = parse_frontmatter(&path).unwrap();
        assert_eq!(
            fm.get("name").and_then(|v| v.as_str()),
            Some("kalma2-interact")
        );
    }
}
