use serde_yaml::Value as Yaml;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub struct ProjectConfig {
    pub env_ref: Option<String>,
    pub allowed_executables: HashSet<String>,
    pub docs_paths: std::collections::HashMap<String, String>,
}

pub fn load_project_config(project_root: &Path) -> ProjectConfig {
    let path = project_root.join(".SddIA/project.md");
    let mut cfg = ProjectConfig {
        env_ref: None,
        allowed_executables: HashSet::new(),
        docs_paths: std::collections::HashMap::new(),
    };
    if !path.is_file() {
        return cfg;
    }
    let text = fs::read_to_string(&path).unwrap_or_default();
    let yaml = extract_frontmatter(&text);
    if let Some(Yaml::Mapping(map)) = yaml {
        if let Some(Yaml::String(s)) = map.get(&Yaml::String("env_ref".into())) {
            cfg.env_ref = Some(s.clone());
        }
        if let Some(Yaml::Mapping(mcp)) = map.get(&Yaml::String("mcp".into())) {
            if let Some(Yaml::Sequence(seq)) =
                mcp.get(&Yaml::String("allowed_executables".into()))
            {
                for item in seq {
                    if let Yaml::String(ex) = item {
                        cfg.allowed_executables.insert(ex.clone());
                    }
                }
            }
        }
        if let Some(Yaml::Mapping(docs)) = map.get(&Yaml::String("docs".into())) {
            for (k, v) in docs {
                if let (Yaml::String(key), Yaml::String(val)) = (k, v) {
                    cfg.docs_paths.insert(key.clone(), val.clone());
                }
            }
        }
    }
    cfg
}

fn extract_frontmatter(text: &str) -> Option<Yaml> {
    let text = text.strip_prefix("---")?;
    let rest = text.strip_prefix('\n').or_else(|| text.strip_prefix("\r\n"))?;
    let end = rest.find("\n---")?;
    let fm = &rest[..end];
    serde_yaml::from_str(fm).ok()
}
