use execute_process::engine::handlers::instance_creator::run;
use std::fs;
use serde_json::Value;
use std::path::Path;
    use serde_json::json;

    #[test]
    fn creates_topology_and_skips_smoke() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/templates/systemd")).unwrap();
        fs::write(
            repo.join("SddIA/templates/systemd/sddia-email-watcher@.service.template"),
            "[Service]\nWorkingDirectory=%f\nExecStart=%f/SddIA/daemons/email-watcher.sh\n",
        )
        .unwrap();
        fs::write(
            repo.join("SddIA/templates/systemd/sddia-daemon@.service.template"),
            "ExecStart=%f/SddIA/scripts/daemons/@@DAEMON_NAME@@.sh\nWorkingDirectory=%f\n",
        )
        .unwrap();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(repo.join("SddIA/core/cumulo.paths.json"), "{}").unwrap();
        fs::create_dir_all(repo.join("SddIA/tools")).unwrap();
        fs::write(
            repo.join("SddIA/tools/send-telegram-notification.md"),
            "---\nname: send-telegram-notification\n---\n",
        )
        .unwrap();

        let instance = repo.join("cliente-a");
        let env = run(
            repo,
            &json!({
                "instance_root": instance.display().to_string(),
                "runtime_profile": "consumer",
                "skip_smoke": true,
                "skip_ignition": true,
            }),
        )
        .unwrap();
        assert!(env.success);
        assert!(instance.join(".SddIA/.dev/.env").is_file());
        assert!(instance.join(".SddIA/local.paths.json").is_file());
        let paths_txt = fs::read_to_string(instance.join(".SddIA/local.paths.json")).unwrap();
        assert_ne!(paths_txt.trim(), "{}");
        assert!(paths_txt.contains("local_tools"));
        let unit = fs::read_to_string(
            instance.join(".SddIA/systemd/sddia-email-watcher@.service"),
        )
        .unwrap();
        let inst = instance.display().to_string();
        assert!(
            unit.contains("ExecStart=%f/SddIA/daemons/email-watcher.sh"),
            "ExecStart universal %f, unit={unit}"
        );
        assert!(
            !unit.contains(&inst),
            "ExecStart no debe hornear instance_root absoluto, unit={unit}"
        );
        assert!(
            !unit.contains(&format!("{}/SddIA/daemons", repo.display())),
            "ExecStart no debe ser repo forjador"
        );
        assert!(!instance.join(".events").exists());
        assert!(instance
            .join(".SddIA/systemd/sddia-email-watcher@.service")
            .is_file());
        let ew = fs::read_to_string(
            instance.join(".SddIA/systemd/sddia-event-watcher@.service"),
        )
        .unwrap();
        assert!(ew.contains("ExecStart=%f/SddIA/scripts/daemons/event-watcher.sh"));
        assert!(ew.contains("WorkingDirectory=%f"));
        assert!(!ew.contains("@@DAEMON_NAME@@"));
        assert!(!ew.contains(&inst));
        assert!(instance
            .join(".SddIA/systemd/sddia-event-sweeper@.service")
            .is_file());
        assert!(instance
            .join(".SddIA/systemd/sddia-kalma2-bridge@.service")
            .is_file());
        assert!(!instance
            .join(".SddIA/systemd/sddia-daemon@.service")
            .is_file());
    }

    #[test]
    fn smoke_native_without_skip() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/templates/systemd")).unwrap();
        fs::write(
            repo.join("SddIA/templates/systemd/sddia-email-watcher@.service.template"),
            "[Service]\nWorkingDirectory=%f\n",
        )
        .unwrap();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(repo.join("SddIA/core/cumulo.paths.json"), "{}").unwrap();
        fs::create_dir_all(repo.join("SddIA/tools")).unwrap();
        fs::write(
            repo.join("SddIA/tools/send-telegram-notification.md"),
            "---\nname: send-telegram-notification\n---\n",
        )
        .unwrap();

        let instance = repo.join("cliente-smoke");
        let env = run(
            repo,
            &json!({
                "instance_root": instance.display().to_string(),
                "skip_ignition": true
            }),
        )
        .unwrap();
        assert!(env.success, "{:?}", env.error);
        let smoke = env.data.as_ref().unwrap().get("smoke").unwrap();
        assert_eq!(smoke["mode"], "native-topology+local-qa");
        assert_eq!(smoke["success"], true);
        assert_eq!(smoke["checks"]["local_qa_emitted"], false);
        assert!(
            !instance.join(".events").exists(),
            "F-SMOKE-01: sin topología FS .events en instancia hija"
        );
    }

    #[test]
    fn replaces_empty_local_paths_stub() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        fs::create_dir_all(repo.join("SddIA/templates/systemd")).unwrap();
        fs::write(
            repo.join("SddIA/templates/systemd/sddia-email-watcher@.service.template"),
            "[Service]\nWorkingDirectory=%f\nExecStart=%f/SddIA/daemons/email-watcher.sh\n",
        )
        .unwrap();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(repo.join("SddIA/core/cumulo.paths.json"), "{}").unwrap();
        fs::create_dir_all(repo.join("SddIA/tools")).unwrap();
        fs::write(
            repo.join("SddIA/tools/send-telegram-notification.md"),
            "---\nname: send-telegram-notification\n---\n",
        )
        .unwrap();

        let instance = repo.join("cliente-stub");
        fs::create_dir_all(instance.join(".SddIA")).unwrap();
        fs::write(instance.join(".SddIA/local.paths.json"), "{}\n").unwrap();

        let env = run(
            repo,
            &json!({
                "instance_root": instance.display().to_string(),
                "skip_smoke": true,
                "skip_ignition": true,
            }),
        )
        .unwrap();
        assert!(env.success);
        let paths_txt = fs::read_to_string(instance.join(".SddIA/local.paths.json")).unwrap();
        assert_ne!(paths_txt.trim(), "{}");
        assert!(paths_txt.contains("local_tools"));
    }

    fn scaffold_creator_repo(repo: &Path) {
        fs::create_dir_all(repo.join("SddIA/templates/systemd")).unwrap();
        fs::write(
            repo.join("SddIA/templates/systemd/sddia-email-watcher@.service.template"),
            "[Service]\nWorkingDirectory=%f\nExecStart=%f/SddIA/daemons/email-watcher.sh\n",
        )
        .unwrap();
        fs::write(
            repo.join("SddIA/templates/systemd/sddia-daemon@.service.template"),
            "ExecStart=%f/SddIA/scripts/daemons/@@DAEMON_NAME@@.sh\nWorkingDirectory=%f\n",
        )
        .unwrap();
        fs::create_dir_all(repo.join("SddIA/core")).unwrap();
        fs::write(repo.join("SddIA/core/cumulo.paths.json"), "{}").unwrap();
        fs::create_dir_all(repo.join("SddIA/tools")).unwrap();
        fs::write(
            repo.join("SddIA/tools/send-telegram-notification.md"),
            "---\nname: send-telegram-notification\n---\n",
        )
        .unwrap();
        fs::create_dir_all(repo.join("SddIA/scripts/starter-kit/.SddIA")).unwrap();
        fs::write(
            repo.join("SddIA/scripts/starter-kit/.SddIA/llm-registry.example.json"),
            r#"{"registry_version":"1.0.0","oracles":{"oracle-agy":{"adapter_ref":"skill:antigravity-cli-executor","affinity":["aiua"],"fallback":"oracle-gemini","status":"active"}}}"#,
        )
        .unwrap();
    }

    #[test]
    fn materializes_llm_registry_once() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        scaffold_creator_repo(repo);
        let instance = repo.join("cliente-reg");
        let env = run(
            repo,
            &json!({
                "instance_root": instance.display().to_string(),
                "runtime_profile": "consumer",
                "skip_smoke": true,
                "skip_ignition": true,
            }),
        )
        .unwrap();
        assert!(env.success);
        let dest = instance.join(".SddIA/llm-registry.json");
        assert!(dest.is_file());
        let first = fs::read_to_string(&dest).unwrap();
        assert!(first.contains("oracle-agy"));
        fs::write(&dest, "{\"keep\":true}\n").unwrap();
        let env2 = run(
            repo,
            &json!({
                "instance_root": instance.display().to_string(),
                "runtime_profile": "consumer",
                "skip_smoke": true,
                "skip_ignition": true,
            }),
        )
        .unwrap();
        assert!(env2.success);
        assert_eq!(fs::read_to_string(&dest).unwrap().trim(), r#"{"keep":true}"#);
        assert_eq!(
            env2.data.as_ref().unwrap()["llm_registry"]["reason"],
            "already-present"
        );
    }

    #[test]
    fn domain_profile_from_manifest_consumer() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        scaffold_creator_repo(repo);
        let instance = repo.join("cliente-prof");
        fs::create_dir_all(&instance).unwrap();
        fs::write(
            instance.join("MANIFEST.json"),
            r#"{"codex":"codex-kalma2-assistant"}"#,
        )
        .unwrap();
        let env = run(
            repo,
            &json!({
                "instance_root": instance.display().to_string(),
                "runtime_profile": "consumer",
                "skip_smoke": true,
                "skip_ignition": true,
            }),
        )
        .unwrap();
        assert!(env.success);
        let dest = instance.join(".SddIA/active-domain-profile.json");
        let v: Value = serde_json::from_str(&fs::read_to_string(&dest).unwrap()).unwrap();
        assert_eq!(v["codex_slug"], "codex-kalma2-assistant");
        assert_eq!(v["git_required"], false);
        let profile = execute_process::engine::domain_profile::resolve_execution_profile(&instance, &json!({}));
        assert!(!execute_process::engine::domain_authority::has_software_authority(&profile));
    }

    #[test]
    fn engineering_domain_profile_software_forge_opt_in() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        scaffold_creator_repo(repo);
        let instance = repo.join("eng-forge");
        fs::create_dir_all(&instance).unwrap();
        let env = run(
            repo,
            &json!({
                "instance_root": instance.display().to_string(),
                "runtime_profile": "engineering",
                "software_forge": true,
                "skip_smoke": true,
                "skip_ignition": true,
            }),
        )
        .unwrap();
        assert!(env.success);
        let v: Value =
            serde_json::from_str(&fs::read_to_string(instance.join(".SddIA/active-domain-profile.json")).unwrap())
                .unwrap();
        assert_eq!(v["software_forge"], true);

        let instance2 = repo.join("eng-default");
        fs::create_dir_all(&instance2).unwrap();
        let env2 = run(
            repo,
            &json!({
                "instance_root": instance2.display().to_string(),
                "runtime_profile": "engineering",
                "skip_smoke": true,
                "skip_ignition": true,
            }),
        )
        .unwrap();
        assert!(env2.success);
        let v2: Value =
            serde_json::from_str(&fs::read_to_string(instance2.join(".SddIA/active-domain-profile.json")).unwrap())
                .unwrap();
        assert_eq!(v2["software_forge"], false);
    }

    #[test]
    fn domain_profile_codex_unknown_without_manifest() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        scaffold_creator_repo(repo);
        let instance = repo.join("cliente-noslug");
        let env = run(
            repo,
            &json!({
                "instance_root": instance.display().to_string(),
                "runtime_profile": "consumer",
                "skip_smoke": true,
                "skip_ignition": true,
            }),
        )
        .unwrap();
        assert!(env.success);
        assert!(!instance.join(".SddIA/active-domain-profile.json").is_file());
        assert_eq!(
            env.data.as_ref().unwrap()["domain_profile"]["reason"],
            "codex-unknown"
        );
    }

    #[test]
    fn domain_profile_skipped_on_forge_profile() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        scaffold_creator_repo(repo);
        let instance = repo.join("forja");
        fs::create_dir_all(&instance).unwrap();
        fs::write(
            instance.join("MANIFEST.json"),
            r#"{"codex":"codex-kalma2-assistant"}"#,
        )
        .unwrap();
        let env = run(
            repo,
            &json!({
                "instance_root": instance.display().to_string(),
                "runtime_profile": "forge",
                "skip_smoke": true,
                "skip_ignition": true,
            }),
        )
        .unwrap();
        assert!(env.success);
        assert!(!instance.join(".SddIA/active-domain-profile.json").is_file());
        assert_eq!(
            env.data.as_ref().unwrap()["domain_profile"]["reason"],
            "profile-unhandled"
        );
    }

    #[test]
    fn domain_profile_never_overwrites() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        scaffold_creator_repo(repo);
        let instance = repo.join("cliente-keep");
        fs::create_dir_all(instance.join(".SddIA")).unwrap();
        fs::write(
            instance.join(".SddIA/active-domain-profile.json"),
            r#"{"codex_slug":"keep-me","git_required":true}"#,
        )
        .unwrap();
        fs::write(
            instance.join("MANIFEST.json"),
            r#"{"codex":"codex-kalma2-assistant"}"#,
        )
        .unwrap();
        let env = run(
            repo,
            &json!({
                "instance_root": instance.display().to_string(),
                "runtime_profile": "consumer",
                "skip_smoke": true,
                "skip_ignition": true,
            }),
        )
        .unwrap();
        assert!(env.success);
        let raw = fs::read_to_string(instance.join(".SddIA/active-domain-profile.json")).unwrap();
        assert!(raw.contains("keep-me"));
        assert_eq!(
            env.data.as_ref().unwrap()["domain_profile"]["reason"],
            "already-present"
        );
    }
