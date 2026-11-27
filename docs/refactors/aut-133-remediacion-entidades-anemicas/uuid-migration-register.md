---
feature_name: aut-133-remediacion-entidades-anemicas
tracker_ref: AUT-133
created: 2026-11-27
historia_ref: AUT-125
remediation_ref: AUT-133
pbi_ref: PBI-81e91c67
execution_id: 5ea79c56-a610-4223-b2b3-7498a510d239
source_inventory: docs/refactors/gate-anti-anemia-entidades-dominio/audit-inventory.json
kind: uuid-migration-register
---

# UUID migration register — AUT-133

Registro de las 25 entidades con `UUID_NOT_V4` / `UUID_INVALID` congeladas desde la semilla `audit-inventory.json` (tip semilla `fd7ef31b…`; tip actual `3aaaa266…`; live rescan pendiente).

| path | family | uuid_old | code | disposition | uuid_new | index_sync | eda_coverage_sync | notes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `SddIA/actions/emit-pr-audited-event.md` | action | `d4e5f6a7-b8c9-4d0e-1f2a-3b4c5d6e7f8a` | UUID_NOT_V4 | migrate |  | pending | pending | UUID presente no-v4; migrar a v4 en T4 |
| `SddIA/actions/emit-user-preference-change-requested.md` | action | `c3d4e5f6-a7b8-4901-c234-567890ab004` | UUID_INVALID | migrate |  | pending | pending | UUID inválido/truncado; migrar a v4 en T4 |
| `SddIA/actions/enrich-fracture-pbi-kaizen.md` | action | `c4d5e6f7-a8b9-4012-c345-678901234567` | UUID_NOT_V4 | migrate |  | pending | pending | UUID presente no-v4; migrar a v4 en T4 |
| `SddIA/actions/sync-entity-index.md` | action | `a3f8c2e1-4b5d-6a7e-8f90-1a2b3c4d5e6f` | UUID_NOT_V4 | migrate |  | pending | pending | UUID presente no-v4; migrar a v4 en T4 |
| `SddIA/agents/cerbero.md` | agent | `e1f2a3b4-c5d6-7e8f-9a0b-1c2d3e4f5a6b` | UUID_NOT_V4 | migrate |  | pending | pending | UUID presente no-v4; migrar a v4 en T4 |
| `SddIA/agents/radamanto.md` | agent | `4d5e6f7a-8b9c-4d0e-1f2a-3b4c5d6e7f8a` | UUID_NOT_V4 | migrate |  | pending | pending | UUID presente no-v4; migrar a v4 en T4 |
| `SddIA/agents/tekton.md` | agent | `b3a4c5d6-7e8f-9a0b-1c2d-3e4f5a6b7c8d` | UUID_NOT_V4 | migrate |  | pending | pending | UUID presente no-v4; migrar a v4 en T4 |
| `SddIA/events/domain/domain-entity-deprecated.md` | event | `9c3d4e5f-6a7b-4c8d-0e1f-2a3b4c5d6e7f` | UUID_NOT_V4 | migrate |  | pending | pending | UUID presente no-v4; migrar a v4 en T4 |
| `SddIA/events/domain/kaizen-idea-captured.md` | event | `b7c8d9e0-f1a2-4b3c-d4e5-f6a7b8c9d0e1` | UUID_NOT_V4 | migrate |  | pending | pending | UUID presente no-v4; migrar a v4 en T4 |
| `SddIA/events/domain/manual-task-requested.md` | event | `a6b7c8d9-e0f1-4a2b-c3d4-e5f6a7b8c9d0` | UUID_NOT_V4 | migrate |  | pending | pending | UUID presente no-v4; migrar a v4 en T4 |
| `SddIA/events/domain/telegram-message-received.md` | event | `e1f2a3b4-c5d6-47e8-f9a0-b1c2d3e4f5a6` | UUID_NOT_V4 | migrate |  | pending | pending | UUID presente no-v4; migrar a v4 en T4 |
| `SddIA/events/domain/user-preference-change-requested.md` | event | `a1b2c3d4-e5f6-4789-a012-3456789ab02` | UUID_INVALID | migrate |  | pending | pending | UUID inválido/truncado; migrar a v4 en T4 |
| `SddIA/events/domain/user-preference-changed.md` | event | `b2c3d4e5-f6a7-4890-b123-456789ab003` | UUID_INVALID | migrate |  | pending | pending | UUID inválido/truncado; migrar a v4 en T4 |
| `SddIA/library/codexes/codex-software-engineering/events/hu-forged.md` | event | `c4e5f6a7-8b9c-4d0e-1f2a-3b4c5d6e7f80` | UUID_NOT_V4 | migrate |  | pending | pending | UUID presente no-v4; migrar a v4 en T4 |
| `SddIA/process/action-creator.md` | process | `d0e1f2a3-b4c5-46d7-e8f9-0a1b2c3d4e5f` | UUID_NOT_V4 | migrate |  | pending | pending | UUID presente no-v4; migrar a v4 en T4 |
| `SddIA/process/cerbero-governance-react.md` | process | `3b4c5d6e-7f8a-4b9c-0d1e-2f3a4b5c6d7e` | UUID_NOT_V4 | migrate |  | pending | pending | UUID presente no-v4; migrar a v4 en T4 |
| `SddIA/process/fix-tool-process.md` | process | `4c5d6e7f-8a9b-4c0d-1e2f-3a4b5c6d7e8f` | UUID_NOT_V4 | migrate |  | pending | pending | UUID presente no-v4; migrar a v4 en T4 |
| `SddIA/process/route-domain.md` | process | `d4e5f6a7-b8c9-4012-d3e4-f5a6b7c8d9e0` | UUID_NOT_V4 | migrate |  | pending | pending | UUID presente no-v4; migrar a v4 en T4 |
| `SddIA/process/route-orchestration.md` | process | `c3d4e5f6-a7b8-4901-c2d3-e4f5a6b7c8d9` | UUID_NOT_V4 | migrate |  | pending | pending | UUID presente no-v4; migrar a v4 en T4 |
| `SddIA/process/telegram-fallback-responder.md` | process | `c9d0e1f2-a3b4-4c5d-6e7f-8a9b0c1d2e3f` | UUID_NOT_V4 | migrate |  | pending | pending | UUID presente no-v4; migrar a v4 en T4 |
| `SddIA/process/user-preference-ingest.md` | process | `d4e5f6a7-b8c9-4d0e-1f2a-3b4c5ab005` | UUID_INVALID | migrate |  | pending | pending | UUID inválido/truncado; migrar a v4 en T4 |
| `SddIA/skills/cryptography-manager.md` | skill | `a1f2e3d4-c5b6-4789-a012-3456789abc0` | UUID_INVALID | migrate |  | pending | pending | UUID inválido/truncado; migrar a v4 en T4 |
| `SddIA/skills/filesystem-manager.md` | skill | `f4a5b6c7-d8e9-4f0a-1b2c-3d4e5f6a7b8c` | UUID_NOT_V4 | migrate |  | pending | pending | UUID presente no-v4; migrar a v4 en T4 |
| `SddIA/skills/user-preference-store.md` | skill | `f1a2b3c4-d5e6-4789-a012-3456789ab01` | UUID_INVALID | migrate |  | pending | pending | UUID inválido/truncado; migrar a v4 en T4 |
| `SddIA/suites/fail-fast-lab.md` | suite | `c3d4e5f6-a7b8-4901-c123-456789abcdef` | UUID_NOT_V4 | migrate |  | pending | pending | UUID presente no-v4; migrar a v4 en T4 |

_Total filas: 25 (UUID_NOT_V4=19, UUID_INVALID=6)._
