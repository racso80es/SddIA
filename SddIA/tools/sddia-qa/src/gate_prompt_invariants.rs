//! Oráculos mecánicos para invariantes de prompts y handoff (MAN-55: KZ-PRM-01..06).

#[cfg(test)]
mod tests {
    use execute_process::core::repo::find_repo_root;
    use std::fs;
    use std::path::Path;

    fn read_rel(repo: &Path, rel: &str) -> String {
        fs::read_to_string(repo.join(rel))
            .unwrap_or_else(|e| panic!("no se pudo leer {rel}: {e}"))
    }

    fn assert_contains_all(rel: &str, text: &str, needles: &[&str]) {
        for n in needles {
            assert!(
                text.contains(n),
                "{rel}: falta invariante obligatoria `{n}`"
            );
        }
    }

    #[test]
    fn prompt_invariants_ac1_oracle_ola0_present() {
        let repo = find_repo_root().expect("repo");
        let spec_tpl = read_rel(&repo, "SddIA/templates/spec-template/spec.md");
        assert_contains_all(
            "SddIA/templates/spec-template/spec.md",
            &spec_tpl,
            &["Oráculo mecánico", "Ola 0", "oracle-red.log", "grep"],
        );

        let doc_pat = read_rel(&repo, "SddIA/library/norms/features-documentation-pattern.md");
        assert_contains_all(
            "SddIA/library/norms/features-documentation-pattern.md",
            &doc_pat,
            &["Oráculo", "Ola 0", "oracle-red.log"],
        );

        let dedalo = read_rel(&repo, "SddIA/agents/dedalo.md");
        assert_contains_all(
            "SddIA/agents/dedalo.md",
            &dedalo,
            &["oráculo mecánico", "Ola 0"],
        );
    }

    #[test]
    fn prompt_invariants_ac2_anti_evasion_present() {
        let repo = find_repo_root().expect("repo");
        let constraints = read_rel(&repo, "SddIA/norms/external-ai-constraints.md");
        assert_contains_all(
            "SddIA/norms/external-ai-constraints.md",
            &constraints,
            &["concat!", "VIRTUAL_PATH_ALLOW", "clarify.md"],
        );

        let tekton = read_rel(&repo, "SddIA/agents/tekton.md");
        assert_contains_all(
            "SddIA/agents/tekton.md",
            &tekton,
            &["concat!", "VIRTUAL_PATH_ALLOW", "clarify.md"],
        );
    }

    #[test]
    fn prompt_invariants_ac3_hermetic_and_entity_manager_present() {
        let repo = find_repo_root().expect("repo");
        let doc_pat = read_rel(&repo, "SddIA/library/norms/features-documentation-pattern.md");
        assert_contains_all(
            "SddIA/library/norms/features-documentation-pattern.md",
            &doc_pat,
            &[
                "git status --porcelain",
                "systemctl --user list-units",
                "SDDIA_EVENT_STORE",
            ],
        );

        let constraints = read_rel(&repo, "SddIA/norms/external-ai-constraints.md");
        assert_contains_all(
            "SddIA/norms/external-ai-constraints.md",
            &constraints,
            &[
                "handoff_hash_signature_new != handoff_hash_signature_old",
                "idempotent",
                "mermaid",
            ],
        );

        let tekton = read_rel(&repo, "SddIA/agents/tekton.md");
        assert_contains_all(
            "SddIA/agents/tekton.md",
            &tekton,
            &[
                "handoff_hash_signature_new != handoff_hash_signature_old",
                "idempotent",
            ],
        );
    }

    #[test]
    fn prompt_invariants_ac4_host_parity_and_role_separation_present() {
        let repo = find_repo_root().expect("repo");
        let constraints = read_rel(&repo, "SddIA/norms/external-ai-constraints.md");
        assert_contains_all(
            "SddIA/norms/external-ai-constraints.md",
            &constraints,
            &[
                "cargo build --workspace --release",
                "systemctl --user restart",
                "eda_event_token_hygiene",
                "global: APTO",
                "refine-item",
                "non_blocking_findings",
            ],
        );

        let tekton = read_rel(&repo, "SddIA/agents/tekton.md");
        assert_contains_all(
            "SddIA/agents/tekton.md",
            &tekton,
            &[
                "cargo build --workspace --release",
                "global: APTO",
                "refine-item",
            ],
        );

        let argos = read_rel(&repo, "SddIA/agents/argos.md");
        assert_contains_all(
            "SddIA/agents/argos.md",
            &argos,
            &["F3_TECH_GATE", "PARCIAL", "non_blocking_findings"],
        );

        let rule = read_rel(&repo, ".cursor/rules/task-closure-documental.mdc");
        assert_contains_all(
            ".cursor/rules/task-closure-documental.mdc",
            &rule,
            &["global: APTO", "refine-item", "non_blocking_findings"],
        );
    }
}
