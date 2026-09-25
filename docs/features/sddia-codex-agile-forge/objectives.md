---
feature_name: sddia-codex-agile-forge
created: "2026-09-25"
process: feature
branch_name: feat/sddia-codex-agile-forge
persist_ref: docs/features/sddia-codex-agile-forge
pbi_ref: docs/todos/pending/Desacople_ingenieria_software_de_core_a_codice.md
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-04
execution_id: e0333bae-1c81-4879-994b-63dbf5eba294
delivery_mode: branch_pr
delivery_mode_source: inputs
agents: mayeuta
---

# Objetivos — sddia-codex-agile-forge

## Misión

Desacoplar la fabricación de software del Core: proyectos en carpeta propia con git propio, configuración validada por contrato, y modo de entrega `branch_pr` | `trunk_direct` por proyecto y por ejecución. El conocimiento nuevo vive en `codex-software-engineering` (alias `codex-agile-forge`). El Core conserva el bus, la validación de forma y el ciclo auto-hospedado en `branch_pr`.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Contrato `project-config-contract` v1.0.0 + validador en `workspace_init` | Poda física de normas/ECST históricos (L-PRUNE → ABSTRACT-04b) |
| Índice Core + manifiesto cliente, `uuid` cruzado, `.git` propio, anti-anidamiento | NFT, GitLab/Gitea, migración Paciente 0 |
| `delivery_mode` con precedencia inputs > proyecto > default; `trunk_direct` sin PR | Cambiar el flujo del propio repo SddIA |
| `events_domain_roots`, suscripciones compuestas, `PBI_Forged`, `Delivery_Committed` | Reescribir `route-domain-event` |
| Rutas documentales ancladas a `project_root` cuando hay proyecto | Borrar `paths.featurePath` del Cúmulo |
| Proceso `forge-pbi` (tiers, sin modelo literal) + handler de sellado lab | Ejecución real de LLM en CI |
| Gate pre-push condicionado; aduana pre-commit obligatoria en `trunk_direct` | `SDDIA_SKIP_HOOKS` para IAs |
| Tests Rust de los AC de motor + evolution del uuid del PBI | |

## Criterios de aceptación (este ciclo)

Los AC del PBI v1.1.0, con L-PRUNE acotando AC-PRUNE: **no** se exige vaciar `SddIA/norms` ni `SddIA/events`. Se exige que lo nuevo de semántica software no nazca en el Core (contrato, suscripciones de dominio, ECST nuevos, `forge-pbi`).

## Ley aplicada

- Rutas vía `SddIA/core/cumulo.paths.json`.
- Git vía `skill:git-manager`.
- Alta de process/event/contrato de códice vía creator / `entity-manager`, no escritura IDE directa.
- Motor Rust (`SddIA/engine/`) editable directo.
- `features-documentation-pattern` v1.2.1. Cierre documental en rama, un PR.
- `global: APTO` solo con CI verde (CA de CI = `PENDIENTE-CI` hasta run verde).
- Jerarquía: Acción → Agente → Skill → Tools.
