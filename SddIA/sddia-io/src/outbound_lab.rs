use std::env;
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub fn truthy_env(name: &str) -> bool {
    matches!(
        env::var(name).unwrap_or_default().trim().to_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

pub fn lab_mock_outbound_enabled() -> bool {
    truthy_env("SDDIA_LAB_MOCK_OUTBOUND") || truthy_env("SDDIA_LAB_SIMULATE_IOTA")
}

pub fn lab_simulate_iota_enabled() -> bool {
    truthy_env("SDDIA_LAB_SIMULATE_IOTA") || lab_mock_outbound_enabled()
}

pub fn lab_mock_iota_url() -> Option<String> {
    env::var("SDDIA_LAB_MOCK_IOTA_URL")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

pub fn lab_mock_telegram_url() -> Option<String> {
    env::var("SDDIA_LAB_MOCK_TELEGRAM_URL")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

pub fn load_iota_wallet_secret(repo: Option<&Path>) -> Option<String> {
    let from_env = env::var("IOTA_WALLET_SECRET")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    if from_env.is_some() {
        return from_env;
    }
    let Some(repo) = repo else {
        return None;
    };
    let wallet_path = repo.join(".SddIA/.dev/wallet.key");
    if !wallet_path.is_file() {
        return None;
    }
    fs::read_to_string(&wallet_path)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

pub fn lab_sim_digest(prefix: &str) -> String {
    format!("{prefix}-{}", &Uuid::new_v4().simple().to_string()[..24])
}

pub fn find_repo_root_from_cwd() -> Option<std::path::PathBuf> {
    let mut current = env::current_dir().ok()?;
    loop {
        if current.join("SddIA/core/cumulo.paths.json").is_file() {
            return Some(current);
        }
        current = current.parent()?.to_path_buf();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lab_sim_digest_has_prefix() {
        let digest = lab_sim_digest("lab-sim");
        assert!(digest.starts_with("lab-sim-"));
        assert!(digest.len() > "lab-sim-".len());
    }
}
