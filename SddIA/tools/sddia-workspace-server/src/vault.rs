use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

pub const VAULT_PRECEDENCE_KEYS: &[&str] = &[
    "SDDIA_LAB_SIMULATE_IOTA",
    "SDDIA_IOTA_TIMEOUT_SECONDS",
];

fn parse_dotenv_file(path: &Path) -> Result<HashMap<String, String>, String> {
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut out = HashMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            let key = k.trim();
            let val = v.trim().trim_matches('"').trim_matches('\'');
            if !key.is_empty() {
                out.insert(key.to_string(), val.to_string());
            }
        }
    }
    Ok(out)
}

/// Entorno efímero para cápsulas hijas: SO gana salvo `VAULT_PRECEDENCE_KEYS`.
pub fn child_capsule_env(
    project_root: &Path,
    env_ref_rel: Option<&str>,
    instance_root: Option<&Path>,
) -> Result<HashMap<String, String>, String> {
    let mut vault = HashMap::new();

    if let Some(inst) = instance_root {
        merge_env_file(&mut vault, &inst.join(".dev/.env"))?;
        merge_env_file(&mut vault, &inst.join(".SddIA/.dev/.env"))?;
    }
    if let Some(rel) = env_ref_rel {
        let p = project_root.join(rel);
        merge_env_file(&mut vault, &p)?;
    }

    let mut child = HashMap::new();
    for (k, v) in &vault {
        if env::var(k).is_err() {
            child.insert(k.clone(), v.clone());
        }
    }
    for key in VAULT_PRECEDENCE_KEYS {
        if let Some(v) = vault.get(*key) {
            child.insert(key.to_string(), v.clone());
        }
    }
    Ok(child)
}

fn merge_env_file(target: &mut HashMap<String, String>, path: &Path) -> Result<(), String> {
    if path.is_file() {
        target.extend(parse_dotenv_file(path)?);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn os_env_wins_over_vault_except_precedence_keys() {
        let proj = TempDir::new().unwrap();
        let inst = TempDir::new().unwrap();
        fs::create_dir_all(proj.path().join(".SddIA/.dev")).unwrap();
        fs::write(
            proj.path().join(".SddIA/.dev/.env"),
            "SECRET_FROM_VAULT=from_vault\nSDDIA_LAB_SIMULATE_IOTA=1\n",
        )
        .unwrap();
        env::set_var("SECRET_FROM_VAULT", "from_os");
        let child = child_capsule_env(proj.path(), Some(".SddIA/.dev/.env"), Some(inst.path()))
            .unwrap();
        assert_eq!(child.get("SECRET_FROM_VAULT"), None);
        assert_eq!(
            child.get("SDDIA_LAB_SIMULATE_IOTA").map(String::as_str),
            Some("1")
        );
        env::remove_var("SECRET_FROM_VAULT");
    }
}
