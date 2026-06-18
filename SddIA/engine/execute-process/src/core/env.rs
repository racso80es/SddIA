use super::env_parse::parse_dotenv_file;
use std::collections::HashMap;
use std::env;
use std::path::Path;

const CONFIG_LOG: &str =
    "[CONFIG] Jerarquía detectada: Aplicando SddIA/.dev/.env sobre .dev/.env";

const VAULT_PRECEDENCE_KEYS: &[&str] = &["SDDIA_LAB_SIMULATE_IOTA", "SDDIA_IOTA_TIMEOUT_SECONDS"];

/// Carga bóveda global → local; aplica al entorno del proceso (paridad `env_loader.py`).
pub fn load_hierarchical_env(repo_root: &Path) -> Result<HashMap<String, String>, String> {
    let global_path = repo_root.join(".dev/.env");
    let local_path = repo_root.join(".SddIA/.dev/.env");
    let mut merged: HashMap<String, String> = HashMap::new();
    let global_exists = global_path.is_file();
    let local_exists = local_path.is_file();

    if global_exists {
        merged.extend(parse_dotenv_file(&global_path)?);
    }
    if local_exists {
        if global_exists && local_exists {
            eprintln!("{CONFIG_LOG}");
        }
        merged.extend(parse_dotenv_file(&local_path)?);
    }

    apply_env(&merged);
    Ok(merged)
}

fn apply_env(merged: &HashMap<String, String>) {
    for (key, value) in merged {
        if env::var(key).is_err() {
            env::set_var(key, value);
        }
    }
    for key in VAULT_PRECEDENCE_KEYS {
        if let Some(value) = merged.get(*key) {
            env::set_var(key, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn local_overrides_global_in_merged_dict() {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join(".dev")).unwrap();
        fs::create_dir_all(root.join(".SddIA/.dev")).unwrap();
        fs::write(root.join(".dev/.env"), "FOO=global\n").unwrap();
        fs::write(root.join(".SddIA/.dev/.env"), "FOO=local\n").unwrap();
        let merged = load_hierarchical_env(root).unwrap();
        assert_eq!(merged.get("FOO"), Some(&"local".to_string()));
    }
}
