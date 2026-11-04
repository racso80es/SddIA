---
feature_name: aut-75-lifecycle-state-actions
created: "2026-11-04"
process: bug-fix
persist_ref: docs/fixes/aut-75-lifecycle-state-actions
pbi_ref: AUT-75
---

# D1 — Inventario diff narrativa vs código (AUT-75)

| Artefacto | Antes (writer según MD) | Después (paridad código) |
| --- | --- | --- |
| `process/feature.md` § handoff 2–3 | `Work_Initiated` + `tracker-stamp` → In Progress; `Validation_Started` → In Validation | `lifecycle_phase_action` forge_started / forge_closed / validación / PR / CI |
| `process/bug-fix.md` § handoff 2–3 | Igual AUT-65/66 | Igual tabla AUT-75 |
| `process/refactorization.md` § handoff 2–3 | Igual AUT-65/66 | Igual tabla AUT-75 |
| `events/domain/work-initiated.md` | Consumidor principal tracker-stamp | Writer fase; stamp no-op columna |
| `events/domain/validation-started.md` | Post-Tekton → in_validation vía stamp | forge_closed en Verificación; emisión Tekton obsoleta |
| `events/domain/pull-request-presented.md` | tracker_ref → stamp in_review | pr_opened en fase DCC |
| `events/domain/pull-request-merged.md` | stamp done | review_ci_green en accept-pr |
| `core/event-domain-subscriptions.json` | Intents sello in_progress / in_validation / in_review / done | AUT-75 no-op columna + sellado en origen |

**Vía de mutación:** parche directo en rama (Cerbero revoca `entity-manager` en instancia); equivalente semántico a seeds `markdown_body_replacements` + bump SemVer en cuerpo/frontmatter. YAML `phases:` sin cambio.
