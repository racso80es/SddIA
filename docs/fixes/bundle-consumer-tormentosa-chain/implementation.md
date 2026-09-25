---
feature_name: bundle-consumer-tormentosa-chain
created: "2026-09-25"
process: bug-fix
branch_name: fix/bundle-consumer-tormentosa-chain
persist_ref: docs/fixes/bundle-consumer-tormentosa-chain
pbi_document_id: PBI-FIX-BUNDLE-CONSUMER-TORMENTOSA-CHAIN
execution_id: "4480e892-4647-48c5-a1de-934800e768d7"
---

# Implementación

| Archivo | Cambio |
|---------|--------|
| `SddIA/scripts/build-release-bundle.sh` | `CONSUMER_BINS` = 13; crate `skills/`; scan `skill:` + `aiua-stimulus-processing.md`; núcleo `llm-router`/`thought-graph-access`; `SDDIA_BUNDLE_SKIP_WITNESS` para smoke Filtro C; ONBOARDING Tormentosa |
| `SddIA/scripts/qa/test-build-release-bundle-filtro-c.sh` | 4 ELF + `.md` + skip-witness |
| `SddIA/scripts/qa/test-instance-root-resolver.sh` | `LogRateLimit*` en plantilla email |
| `SddIA/templates/systemd/sddia-email-watcher@.service.template` | `LogRateLimitIntervalSec=30s` / `LogRateLimitBurst=500` |
| `SddIA/process/instance-creator.md` + `index.md` | v1.4.0 vía `entity-manager` (`execution_id` `1b3d53e3-5171-4e6f-8a53-1e6dc665458b`) |
| `SddIA/engine/execute-process/src/engine/handlers/instance_creator.rs` | `materialize_llm_registry`, `materialize_domain_profile`; 5 tests nuevos |
| `SddIA/evolution/806c9463-4c82-4216-a246-d5650a8553e9.md` | Alta CANONICO + fila en `Evolution_log` |
