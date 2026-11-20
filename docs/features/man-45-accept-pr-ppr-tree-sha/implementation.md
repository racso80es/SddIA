---
feature_name: man-45-accept-pr-ppr-tree-sha
created: "2026-11-20"
updated: "2026-11-20"
process: feature
purpose: Registro de implementación — aduana PPR tree_sha en accept-pr (MAN-45 / KZ-2)
branch_name: feat/man-45-accept-pr-ppr-tree-sha
persist_ref: docs/features/man-45-accept-pr-ppr-tree-sha
pbi_ref: MAN-45
parent_ref: MAN-43
historia_ref: MAN-43
document_id: PBI-75d12fd0
agent: tekton
phase: ejecucion
status: forged
norm_ref: features-documentation-pattern@1.3.2
kaizen_ref: KZ-2
execution_id: "73910788-b7e7-4a61-9ac2-72fedd024b2a"
---
# Implementación — man-45-accept-pr-ppr-tree-sha

## Touchpoints de código

| Archivo | Cambio | Propósito |
|---------|--------|-----------|
| `SddIA/engine/execute-process/src/engine/qa_attestation.rs` | `issuer_process` en `try_write_from_ppr`; exigencia en `validate_attestation_value`; `AttestationFault` + friction_ids; `validate_for_accept_pr_with_state` con tree pre-merge (`source_branch` HEAD^{tree}), `delta_class` fail-closed→active, rechazo dedupe; lab `SDDIA_LAB_SKIP_PPR_ATTESTATION_ACCEPT`; tests `ka_ppr_1`…`ka_ppr_4`, canónico OK, lab skip | Aduana atestación PPR (D-EMITTER / D-TREE-PRE / D-DELTA / D-DEDUPE / D-FRICTION-IDS / D-LAB) |
| `SddIA/engine/execute-process/src/engine/accept_pr.rs` | «Auditoría Genómica» invoca validación **bloqueante**; Presented solo trazabilidad; test `ka_ppr_1_genomic_audit_aborts_without_attestation` | Puerta pre-fusión (D-GATE / L-GENOMIC / L-ABORT / D-NO-FRACTURE) |
| `SddIA/engine/execute-process/src/engine/residual_runner.rs` | Sin cambio de contrato | Ya llama `try_write_from_ppr` tras PPR exitoso; el nuevo campo `issuer_process` se materializa en la emisión |
| `SddIA/engine/execute-process/src/engine/route_domain_core.rs` | Sin mutación de lógica | Dedupe sigue siendo exit 0 sin atestación; accept-pr ya no lo trata como QA |

## Touchpoints normativos y documentales

| Archivo | Cambio |
|---------|--------|
| `SddIA/library/codexes/codex-software-engineering/process/accept-pr.md` | Fase 1 documenta aduana PPR; nota MAN-45; `hash_signature` pendiente de refresh local (ver `execution.md`) |
| `docs/features/man-45-accept-pr-ppr-tree-sha/_fixtures/ka-ppr-{1,2,3,4}/` | Fixtures documentales + stubs JSON/inputs |
| `docs/features/man-45-accept-pr-ppr-tree-sha/ka-ppr-5-counterfactual.md` | Contrafactual KZ-2 / AC-5 |

## Call sites inventariados (H0.3)

| Sitio | Rol |
|-------|-----|
| `accept_pr.rs` «Auditoría Genómica» | Consumidor bloqueante |
| `qa_attestation::try_write_from_ppr` ← `residual_runner` (PPR success) | Emisor canónico |
| `qa_attestation::validate_attestation_for_branch_head` | DCC pre-PR (ahora fail-closed `active`) |
| `route_domain_core::try_acquire_ppr_dedupe` | Emite `skipped-ppr-dedupe`; no escribe atestación |

## D-EMITTER-LINEAGE

No se añadió contraste de `execution_id` contra proof store PPR: no hay API estable consultable en esta ola. Defensa = sello `issuer_process` en emisión + rechazo fail-closed de JSON forjado (**D-EMITTER** solo).
