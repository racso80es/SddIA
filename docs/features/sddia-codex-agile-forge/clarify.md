---
feature_name: sddia-codex-agile-forge
created: "2026-09-25"
process: feature
purpose: clarificacion
branch_name: feat/sddia-codex-agile-forge
persist_ref: docs/features/sddia-codex-agile-forge
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-04
execution_id: e0333bae-1c81-4879-994b-63dbf5eba294
agents: mayeuta
---

# Clarify — sddia-codex-agile-forge

Relevo IDE de la fase Estabilización (`agent:mayeuta` simulated). Semilla: `.tmp/feature-sddia-codex-agile-forge.json`. Init con `SDDIA_LAB_ALLOW_DIRTY=1` porque el único dirt era el traslado kitchen→pending del propio PBI (dos paths; `pbi_ref` solo cubre uno).

## Decisiones cerradas (laudos 2026-09-25)

| ID | Decisión |
|----|----------|
| L-CODEX | Extender `codex-software-engineering`. `codex-agile-forge` = alias. Prohibido segundo códice. |
| L-REG | Índice Core `.SddIA/projects/{slug}.md` (básico, linkado) + manifiesto `{project_root}/.SddIA/project.md` (detalle). `uuid` cruzado. |
| L-CONTRACT | El Core valida forma del manifiesto contra `project-config-contract` v1.0.0 en `workspace_init`. No interpreta semántica de negocio. Fallo → `exitCode: 1` + `System_Fracture_Detected`. |
| L-TRUNK | `trunk_direct` omite por completo `pull-request-review` y `accept-pr`. Sin auditoría post-commit. Única aduana: gate pre-commit. |
| L-PREC | `delivery_mode`: `inputs` > manifiesto > `branch_pr`. Trazar `delivery_mode_source` en `objectives.md`. |
| L-SELF | El repo SddIA sigue `branch_pr` auto-hospedado. Sin `project_slug`, el init actual no cambia. |
| L-PRUNE | La semántica software nueva (contrato de proyecto, suscripciones de dominio, contratos ECST `PBI_Forged` y `Delivery_Committed`) nace en el códice. Las normas ya residentes en Core (`git-operations`, `pull-request-orchestration`, `pr-acceptance-protocol`, `features-documentation-pattern`) **no se mueven en este ciclo**: son dependencia de AC-CORE-SELF y de cientos de referencias. Se versiona en ellas solo la cláusula de vigencia por `delivery_mode`. Poda física = deuda explícita ABSTRACT-04b si el Vértice lo ordena. |
| L-EVENTS | `directories.events_domain_roots` análogo a `process_domain_roots`. Contratos nuevos bajo el códice. Los ECST ya en `SddIA/events/` permanecen (misma razón que L-PRUNE). |
| L-SUBS | Tabla efectiva = Core ∪ `subscriptions.json` del códice si el perfil tiene autoridad software. Sin autoridad, las claves exclusivas del códice no entran: `PBI_Forged` → dead-letter `reason: no_subscriber`, sin panic ni fractura. |
| L-PATHS | Con `project_root` resuelto, `featurePath`/`fixPath`/`todos` se anclan a ese root. Sin proyecto, siguen relativas al repo (comportamiento actual). El default documental vive en el contrato del códice, no se borra de Cúmulo en este ciclo (evita romper AC-CORE-SELF); Cúmulo marca esas claves `resolution: project_root_or_repo`. |
| L-FORGE | `forge-pbi` se alta en el packing del códice (`process_jurisdiction: domain`). Fases por tier de agente, sin nombre de modelo literal. En lab/relevo, un handler sella el `.md` en `{project_root}/docs/todos/pending/` y emite `PBI_Forged`. |
| L-HOOKS | La decisión «¿vetar push a main?» es función pura: veto si el repo es Core o `delivery_mode != trunk_direct`. `pre_push_gate.sh` la consulta. El Core no gana `trunk_direct`. |

## Fuera de este ciclo

NFT comercial, multi-remoto no-`gh`, migración GesFer/Paciente 0, poda física de normas/eventos históricos (L-PRUNE).
