---
feature_name: sddia-codex-agile-forge
created: "2026-09-25"
process: feature
branch_name: feat/sddia-codex-agile-forge
persist_ref: docs/features/sddia-codex-agile-forge
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-04
items:
  - project_binding
  - events_domain_roots
  - composed_subscriptions
  - forge-pbi
  - delivery_mode_trunk
  - pre_push_veto
agents: tekton
---

# Implementation — sddia-codex-agile-forge

| Touchpoint | Cambio |
|------------|--------|
| `engine/project_binding.rs` | Contrato v1.0.0, uuid cruzado, git propio, anti-anidamiento, precedencia `delivery_mode`, rutas bajo `project_root`, `plan_route` |
| `engine/workspace_init.rs` | `bind` antes del git; `trunk_direct` no reescribe prefijo; `anchor_persist`; fractura solo en `PROJECT_SCOPE_ESCAPE` |
| `engine/delivery_close.rs` | `trunk_direct` emite `Delivery_Committed` y omite PR |
| `engine/ecst_validation.rs` | Esquemas también desde `events_domain_roots` |
| `core/resolver.rs` | `resolve_event_contract` |
| `engine/route_domain_core.rs` | Tabla compuesta; dead-letter `no_subscriber` sin autoridad |
| `handlers/forge_pbi.rs` | Sellado en pending del proyecto + `PBI_Forged`. Tiers en el process, sin modelo literal |
| `cumulo.paths.json` 1.11.0 | `events_domain_roots`, `instance.projects`, `codex_subscriptions`, `paths.resolution` |
| Códex | alias `codex-agile-forge`, membership `forge-pbi`, contrato, eventos, `subscriptions.json` |
| Hooks | `main_push_veto`: Core siempre veta; cliente `trunk_direct` no |
| Normas | cláusula `delivery_mode` en `pull-request-orchestration` 1.1.0 y `pr-acceptance-protocol` 1.1.0 |

Poda física de normas y ECST históricos: fuera (L-PRUNE). Git del cliente distinto del repo orquestador no se redirige dentro de `git-manager` en este ciclo: el contrato de rama y el cierre `trunk_direct` sí.
