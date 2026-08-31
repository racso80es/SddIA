---
feature_name: kaizen-lancedb-ciclo-fricciones
created: "2026-08-31"
process: feature
branch_name: feat/kaizen-lancedb-ciclo-fricciones
persist_ref: docs/features/kaizen-lancedb-ciclo-fricciones
execution_id: "b97c39ce-f5d6-4e26-92c6-68de26eedcf0"
pbi_ref: docs/todos/done/[KAIZEN] Post-ciclo LanceDB — PAT-workflow, Kintsugi saltado, Mayeuta ciego y correlato evolution.md
document_id: PBI-KAIZEN-LANCEDB-CICLO-FRICCIONES
uuid: "12250eca-49c6-4008-ac50-5c5722a7fe91"
items:
  - mayeuta/credential_workflow_scope
  - mayeuta/remote_branch_absent
  - dcc/stamp_workflow_scope
  - dcc/halt_post_push
  - evolution-register/suggest_relacionado
  - norms/ca3_core_da4
  - process/memory-evolution-ingest_1.2.0
  - library-norm/features-documentation-pattern_ca8
  - pbi/01c9040df256_archive
---

# Implementation — kaizen-lancedb-ciclo-fricciones

## Touchpoints

| Artefacto | Cambio | Vía |
|-----------|--------|-----|
| `enrich_fracture_pbi_kaizen.rs` | Cubos `credential_workflow_scope` / `remote_branch_absent` antes de hook y de `failed`→`prompt_adjustment` | IDE |
| `delivery_close.rs` | `stamp_dcc_workflow_scope_block` (`blocked`, `F-DCC-WORKFLOW-SCOPE`); suppress fracture; halt `prior_push_not_ok` | IDE |
| `sddia-evolution-register/src/lib.rs` | `suggest_relacionado_complements`; veredicto UNREGISTERED incluye `suggested_relacionado` | IDE (cápsula, no `.md`) |
| `obediencia-procesos.md` v1.3 | Credencial `workflow` ≠ Core; veto raw post-DCC blocked | DA-4 (Core `directories.norms`; EM `norm` solo escribe `library_norms`) |
| `external-ai-constraints.md` v1.6.2 | DA-7 veto post-DCC `failed`/`blocked` | DA-4 (idem) |
| `memory-evolution-ingest.md` v1.2.0 | Intent + cuerpo `EvolutionStore` / `{paths.vectorStore}/lancedb/` | entity-manager |
| `features-documentation-pattern.md` | CA de CI: no `APTO` sin `run_id`/URL; `PENDIENTE-CI` | entity-manager (`tactical_norm_version` 1.2.1: replacements no bumpan SemVer en fichero) |
| PBI `01c9040df256` | Diagnóstico §1+§1b; propuesta Mayeuta inválida | IDE (todos) |

## Contrato

- Traza `without workflow scope` y specimen Head-sha + Apertura **no** son cubo hook.
- Publicación remota workflow-scope: `blocked`, sin `System_Fracture_Detected`.
- Halt: Apertura / Sello / Higiene `skipped` si push `failed`/`blocked`. Fail-soft post-`pr_url` intacto.
- Helper no exime `Cargo.lock` del gate.
- Prohibido reimplementar `SDDIA_HOOK_DELIVERY_CLOSE`.

## Jurisdicción EM vs Core norms

`entity-manager` `entity_class: norm` forja `SddIA/library/norms/`. `obediencia-procesos` y `external-ai-constraints` viven en `directories.norms`. Parche DA-4 + correlato evolution (precedente `a8f3c2e1`). Plan L4 listaba EM para esos dos; el creator no tiene destino Core.
