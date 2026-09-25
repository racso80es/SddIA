---
feature_name: bundle-consumer-tormentosa-chain
created: "2026-09-25"
process: bug-fix
branch_name: fix/bundle-consumer-tormentosa-chain
persist_ref: docs/fixes/bundle-consumer-tormentosa-chain
pbi_document_id: PBI-FIX-BUNDLE-CONSUMER-TORMENTOSA-CHAIN
execution_id: "4480e892-4647-48c5-a1de-934800e768d7"
agents: dedalo
---

# Plan — bundle-consumer-tormentosa-chain

| ID | Fase | Touchpoints | Done |
|----|------|-------------|------|
| **T0** | Bundle | `build-release-bundle.sh`: 13 ELF, crate `skills/`, scan `skill:` + `aiua-stimulus-processing.md`, núcleo `llm-router`/`thought-graph-access`, ONBOARDING | CA-BINS, CA-SKILL-CRATE, CA-FAILCLOSED |
| **T1** | Smokes bundle | `test-build-release-bundle-filtro-c.sh` 4 ELF + `.md`; `test-daemon-binary-resolver.sh` intacto | CA-SMOKE-FILTRO-C, CA-RESOLVER |
| **T2** | Creator genoma | `entity-manager` bump `instance-creator` 1.3.0 → 1.4.0 (fase Topologia + `codex_slug`) | CA-GOV |
| **T3** | Creator motor | `instance_creator.rs`: `materialize_llm_registry`, `materialize_domain_profile`; tests 4 casos perfil + registry + authority | CA-CREATOR-REGISTRY, CA-CREATOR-PROFILE, CA-AUTHORITY |
| **T4** | Email | `sddia-email-watcher@.service.template` + `test-instance-root-resolver.sh` | CA-EMAIL-LOGRATE |
| **T5** | Cierre | evolution, `implementation.md`/`execution.md`/`validacion.md` PENDIENTE-CI, PBI `done/` en rama, DCC → PR; APTO solo con CI verde | CA-BUILD, CA-DOC |

## Orden

1. Commit de esta planificación (sin motor).
2. T0 → T1 → T4 (scripts/plantillas; no genoma).
3. T2 `entity-manager` **antes** de T3 (DA-2).
4. T3 tests `cargo test -p execute-process`.
5. Evolution + docs de ejecución. `delivery-close-cycle`. `validacion.md` no APTO hasta check verde. Entonces `accept-pr`.

## Delegaciones

| Necesidad | Vía |
|-----------|-----|
| Scripts, plantillas, handler Rust, tests | Tekton directo |
| Bump `instance-creator.md` | `./sddia-run.sh --process entity-manager` |
| Evolution | bisturí bajo `directories.evolution` + `gate-evolution --range` |
| Git / PR | `delivery-close-cycle` / `accept-pr` |
