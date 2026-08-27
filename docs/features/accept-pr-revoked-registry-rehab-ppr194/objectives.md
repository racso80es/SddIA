---
feature_name: accept-pr-revoked-registry-rehab-ppr194
created: "2026-08-27"
process: refactorization
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr194
persist_ref: docs/features/accept-pr-revoked-registry-rehab-ppr194
pbi_ref: docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #194).md
document_id: PBI-PPR-194-ACCEPT-PR-REVOKED-REGISTRY
uuid: 7f3a9c2e-4b1d-4e8a-9c5f-6d7e8a9b0c1d
phase: mayeuta-stabilization
agents: mayeuta
source_correlation_id: "59606407-eed3-4da8-ac13-3cf6205b2147"
source_pr_url: https://github.com/racso80es/SddIA/pull/194
feature_ref: docs/fixes/bundle-consumer-telegram-gateway
satellite_fix_pbi: docs/todos/pending/[FIX] accept-pr delete_branch payload vs git-manager.md
incident_ref: "REVOKED_ENTITY_ALERT_ACCEPT_PR — abrupt_success_rate_drop since 2026-08-26T11:42:26Z"
olas:
  - A1
  - A2
  - A3
---

# Objetivos — accept-pr-revoked-registry-rehab-ppr194

## Objetivo

Rehabilitar el proceso `accept-pr` tras revocación `abrupt_success_rate_drop` (PPR #194 / since `2026-08-26T11:42:26Z`) **y** cortar re-muerte: A1 Yunque Rúnico + A2 payload `delete_branch` alineado a `accept-pr.md` + A3 handoff PPR/F5 que no miente merge ausente. Un `persist_ref`, un PR; FIX satélite de payload absorbido.

## Alcance

1. **A1 — Saneamiento de instancia (Yunque Rúnico):** eliminar `revoked.accept-pr`; verificar `permanent.accept-pr` ausente; reset absoluto del bucket raíz `accept-pr` en stats (`healthy`, `recovery_attempts: 0`, `degraded_at: null`, `rehab_laudo: PBI-PPR-194-ACCEPT-PR-REVOKED-REGISTRY`, `rehabilitated_at`, poda de `samples` a vacío o ≤3 OK runtime). Evidencia en `execution.md`. Prohibido versionar `.SddIA/cerbero/` / `.SddIA/radamanto/` en el diff del PR.
2. **A2 — Contrato delete_branch (absorbe FIX satélite):** `delete_branch_hygiene` emite dos invocaciones canónicas (local `remote: false` / remoto `remote: true`, ambas con `force: false`); cero `"remote": "origin"` string; `hygiene_failure` auditable; fail-soft por op post-merge+push; una verdad frozen I/O ↔ cápsula (`delete_branch` declarado o recorte consciente).
3. **A3 — Veracidad handoff PPR/F5:** con merge ausente, no afirmar merge consumado; distinguir handoff pendiente / consumado / bloqueado (p. ej. `accept-pr` revoked); prohibido usar `accept_pr_handoff: true` como eufemismo de éxito sin merge soberano.
4. **Ontología:** conservar `entity_type: process`.
5. **Umbrales:** tabla Radamanto 1.1.0 intacta; no reabrir `success_rate` ni `abrupt_drop_min_samples`.
6. **Cierre documental single-PR:** cascada bajo `persist_ref` + PBI canónico **y** FIX satélite en `docs/todos/done/` + `validacion.md` APTO (`pbi_archived: true`).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-A1 | `accept-pr` ∉ `revoked` ni `permanent`; stats raíz `healthy`; `recovery_attempts: 0`; laudo y timestamp de rehab; ventana `samples` podada; evidencia A1 en `execution.md`. |
| AC-GIT-CLEAN | Instancia Cerbero/Radamanto ausente del diff del PR. |
| AC-ONTO | `entity_type: process` conservado. |
| AC-A2 | Payloads `delete_branch` canónicos; remoto ausente no tumba éxito post-merge+push; `hygiene_failure` visible; frozen I/O coherente. |
| AC-A3 | Merge ausente ≠ merge afirmado; handoff pendiente ≠ consumado; bloqueo por revoked explícito. |
| AC-SMOKE | Lab: delete local OK; remoto ausente no fuerza fallo global post-merge+push. |
| AC-THRESH | Umbrales 1.1.0 intactos. |
| AC-DOC | Cascada `features-documentation-pattern`; PBI canónico + FIX satélite en `done/`; `validacion.md` con `global: APTO`, `pbi_archived: true`, `branch` coherente. |

## Fuera de alcance

- Rehabilitación de `bug-fix`, `refactorization` o `emit-pr-audited-event`.
- Despachar `bug-fix` satélite aparte para el payload (absorbido aquí).
- Bypass `gh` / `git` crudo; `force: true` como default; remotes ≠ `origin`.
- Reabrir síntoma silencio Python del FIX #37.
- Mutar `radamanto.thresholds.json` v1.1.0 sin laudo.
- Versionar mutaciones de instancia Cerbero/Radamanto en el PR de motor.

## Restricciones

- Git solo vía `skill:git-manager`. Rama canónica: `refactor/accept-pr-revoked-registry-rehab-ppr194`.
- Prohibido despachar ciclos `bug-fix` satélite.
- Cerbero no tiene estado `healthy`: rehab = borrar clave de `revoked` (y de `permanent` si apareciera).
- Reset A1 sin poda de ventana KO reabre `abrupt_success_rate_drop` al primer fallo.
- Homónimo `remote` string (push) ≠ bool (`delete_branch`) — no unificar.
- Mayeuta no diseña touchpoints de código; Dedalo los fija en `spec.md`.
- El cuerpo de este documento es el `refined_requirements` de entrada a Dedalo.

## Ley aplicada

- `features-documentation-pattern` v1.2.x (frontmatter + un `.md` por fase).
- Proceso `refactorization` — fase Estabilización de alcance → Mayeuta.
- Cierre documental en rama (un PR): `task-closure-documental` / patrón v1.2.0+.
- Jurisprudencia `L-REHAB-INST` / anti-recurrencia (#174+#177, #185, #187, #190).
- SSOT proceso: `SddIA/library/codexes/codex-software-engineering/process/accept-pr.md` § Fase 4.
- Jerarquía: Acción → Agente → Skill → Tools.
- `SddIA/norms/external-ai-constraints.md` (soberanía de rutas; forja gobernada).
