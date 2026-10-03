//! Perfil de aduana local (`qa_profile`) — laudo D-1 HU merge-thermodynamics.

use serde_json::Value;

pub const PROFILE_DOCS_ONLY: &str = "docs-only";
pub const PROFILE_FULL: &str = "full";

pub fn qa_profile_from_value(v: Option<&Value>) -> Option<String> {
    v.and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

pub fn qa_profile_from_inputs(inputs: &Value) -> Option<String> {
    qa_profile_from_value(inputs.get("qa_profile"))
}

pub fn qa_profile_from_event_payload(event: &Value) -> Option<String> {
    event
        .get("payload")
        .and_then(|p| qa_profile_from_value(p.get("qa_profile")))
}

pub fn is_docs_only_profile(inputs: &Value) -> bool {
    qa_profile_from_inputs(inputs).as_deref() == Some(PROFILE_DOCS_ONLY)
}

/// Fases solo-`agent:` que se omiten con `docs-only` (sin LLM).
pub fn ppr_agent_phase_skipped_by_docs_only(phase_name: &str) -> bool {
    matches!(
        phase_name,
        "Certificación RBAC" | "Veredicto y bloqueo" | "Cosecha Kaizen"
    )
}

pub fn skip_agent_phase_entry(process_name: &str, phase_name: &str, inputs: &Value) -> Option<Value> {
    if process_name != "pull-request-review" || !is_docs_only_profile(inputs) {
        return None;
    }
    if !ppr_agent_phase_skipped_by_docs_only(phase_name) {
        return None;
    }
    Some(serde_json::json!({
        "status": "executed",
        "handler": "qa-profile-skip",
        "note": "skipped-by-profile",
        "qa_profile": PROFILE_DOCS_ONLY,
        "phase_name": phase_name,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn docs_only_skips_rbac_phase() {
        let inputs = json!({"qa_profile": "docs-only"});
        assert!(skip_agent_phase_entry("pull-request-review", "Certificación RBAC", &inputs).is_some());
    }

    #[test]
    fn full_profile_does_not_skip() {
        let inputs = json!({"qa_profile": "full"});
        assert!(skip_agent_phase_entry("pull-request-review", "Certificación RBAC", &inputs).is_none());
    }

    #[test]
    fn event_payload_qa_profile() {
        let ev = json!({"payload": {"branch": "feat/x", "qa_profile": "docs-only"}});
        assert_eq!(
            qa_profile_from_event_payload(&ev).as_deref(),
            Some(PROFILE_DOCS_ONLY)
        );
    }
}
