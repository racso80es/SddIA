---
feature_name: accept-pr-revoked-registry-rehab-ppr200
created: "2026-08-27"
process: refactorization
branch_name: refactor/accept-pr-revoked-registry-rehab-ppr200
persist_ref: docs/features/accept-pr-revoked-registry-rehab-ppr200
pbi_ref: docs/todos/pending/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #200).md
document_id: PBI-PPR-200-ACCEPT-PR-REVOKED-REGISTRY
uuid: a8f3c1e2-9b4d-4e7a-8c5f-1d2e3f4a5b6c
phase: mayeuta-stabilization
agents: mayeuta
source_correlation_id: "7c215675-2ad2-436a-9749-ff635c52c8b3"
source_pr_url: https://github.com/racso80es/SddIA/pull/200
feature_ref: docs/features/accept-pr-revoked-registry-rehab-ppr194
parent_pbi: docs/todos/done/[ARQUITECTURA] accept-pr — rehabilitación revoked_entities (PPR #194).md
incident_ref: "REVOKED_ENTITY_ALERT_ACCEPT_PR — re-revoked post-#194 abrupt_success_rate_drop since 2026-08-27T11:31:15Z"
olas:
  - A1
  - A2
---

# Objetivos — accept-pr-revoked-registry-rehab-ppr200

## Objetivo

Rehabilitar el proceso `accept-pr` tras **re-revocación** `abrupt_success_rate_drop` post-cierre #194 (PPR #200 / since `2026-08-27T11:31:15Z`) **y** cortar re-muerte: A1 Yunque Rúnico + A2 fail_soft del sello `PullRequest_Merged` cuando `merge_commit_hash` ya cruzó. Un `persist_ref`, un PR.

## Alcance

1. **A1 — Saneamiento de instancia (Yunque Rúnico):** eliminar `revoked.accept-pr`; verificar `permanent.accept-pr` ausente; reset absoluto del bucket raíz `accept-pr` en stats (`healthy`, `recovery_attempts: 0`, `degraded_at: null`, `rehab_laudo: PBI-PPR-200-ACCEPT-PR-REVOKED-REGISTRY`, `rehabilitated_at`, poda de `samples` a vacío o ≤3 OK runtime). Limpiar residuales `rehab_laudo`/`rehabilitated_at` del ciclo #194. Evidencia en `execution.md`. Prohibido versionar `.SddIA/cerbero/` / `.SddIA/radamanto/` en el diff del PR.
2. **A2 — Fail_soft sello post-merge (anti-recurrencia):** si `merge_commit_hash` está presente y la fase «Sello Criptográfico de Fusión» queda `failed`/`blocked` (p. ej. dead-letter `PullRequest_Merged`), el report lleva `fail_soft: true` **antes** de `aggregate_execution_terminal` → survival global (`success` / `exit_code: 0`) sin silenciar el error del sello. Sin `merge_commit_hash` el sello KO permanece causal. Agregador intacto (simetría #187).
3. **Ontología:** conservar `entity_type: process`.
4. **Umbrales:** tabla Radamanto 1.1.0 intacta; no reabrir `success_rate` ni `abrupt_drop_min_samples`.
5. **Cierre documental single-PR:** cascada bajo `persist_ref` + PBI canónico en `docs/todos/done/` + `validacion.md` APTO (`pbi_archived: true`).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-A1 | `accept-pr` ∉ `revoked` ni `permanent`; stats raíz `healthy`; `recovery_attempts: 0`; laudo `PBI-PPR-200-ACCEPT-PR-REVOKED-REGISTRY` y timestamp de rehab; ventana `samples` podada; evidencia A1 en `execution.md`. |
| AC-GIT-CLEAN | Instancia Cerbero/Radamanto ausente del diff del PR. |
| AC-ONTO | `entity_type: process` conservado. |
| AC-A2 | Sello KO + `merge_commit_hash` → `fail_soft` + agregador success; sello KO sin hash → `exit_code: 1`; error de sello visible; agregador no mutado. |
| AC-TESTS | Cobertura producto A2 + no regresión causal sin hash; regresiones #194 intactas si el motor se toca. |
| AC-THRESH | Umbrales 1.1.0 intactos. |
| AC-DOC | Cascada `features-documentation-pattern`; PBI en `done/`; `validacion.md` con `global: APTO`, `pbi_archived: true`, `branch` coherente. |

## Fuera de alcance

- Rehabilitación de `refactorization`, `emit-pr-audited-event` o `bug-fix`.
- Reabrir A2 payload `delete_branch` / A3 handoff truth del ciclo #194 sin regresión empírica nueva.
- Bypass `gh` / `git` crudo; mutar `radamanto.thresholds.json` v1.1.0 sin laudo.
- Silenciar dead-letter del sello (debe seguir auditable).
- Versionar mutaciones de instancia Cerbero/Radamanto en el PR de motor.
- Escribir semillas/TODOs bajo `docs/todos/` (Cúmulo/Kaizen).

## Restricciones

- Git solo vía `skill:git-manager`. Rama canónica: `refactor/accept-pr-revoked-registry-rehab-ppr200`.
- Cerbero no tiene estado `healthy`: rehab = borrar clave de `revoked` (y de `permanent` si apareciera).
- Reset A1 sin poda de ventana KO reabre `abrupt_success_rate_drop` al primer fallo.
- `fail_soft` es runtime en `phase_report`, no YAML estático del proceso.
- Mayeuta no diseña touchpoints de código; Dedalo los fija en `spec.md` y verifica cobertura punta a punta frente al dead-letter empírico (`c24d84a7…` @ `11:31:11Z`).
- El cuerpo de este documento es el `refined_requirements` de entrada a Dedalo.

## Ley aplicada

- `features-documentation-pattern` v1.2.x (frontmatter + un `.md` por fase).
- Proceso `refactorization` — fase Estabilización de alcance → Mayeuta.
- Cierre documental en rama (un PR): `task-closure-documental` / patrón v1.2.0+.
- Jurisprudencia `L-REHAB-INST` / anti-recurrencia (#174+#177, #185, #187, #190, #194).
- Simetría fail_soft post-umbral físico: #187 DCC → #200 accept-pr sello.
- SSOT proceso: `SddIA/library/codexes/codex-software-engineering/process/accept-pr.md` (fase sello).
- Jerarquía: Acción → Agente → Skill → Tools.
- `SddIA/norms/external-ai-constraints.md` (soberanía de rutas; forja gobernada).
