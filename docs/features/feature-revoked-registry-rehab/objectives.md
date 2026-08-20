---
feature_name: feature-revoked-registry-rehab
created: "2026-08-20"
process: refactorization
branch_name: refactor/feature-revoked-registry-rehab
persist_ref: docs/features/feature-revoked-registry-rehab
pbi_ref: docs/todos/pending/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185).md
document_id: PBI-FEATURE-185-REVOKED-REGISTRY
uuid: c8f4e2a1-7b3d-4e59-9f6a-2d1e0c9b8a7f
phase: mayeuta-stabilization
agents: mayeuta
source_correlation_id: 17043d6d-c978-4245-b554-2c5edcf94422
parent_pbi: docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
olas:
  - A1
  - A2
  - A3
---

# Objetivos — feature-revoked-registry-rehab

## Objetivo

Rehabilitar el proceso `feature` tras revocación permanente (`max_recovery_attempts_exceeded`) **y** impedir la re-muerte: fail-soft del padre cuando el DCC hijo ya cruzó umbral físico, y poda de samples de laboratorio huecos en el batch de supervivencia Radamanto. Un `persist_ref`, un PR.

## Alcance

1. **A1 — Saneamiento estructural (instancia):** eliminar `permanent.feature` de Cerbero; verificar `revoked.feature` ausente; reset absoluto del bucket raíz `feature` en stats (`healthy`, `recovery_attempts: 0`, `degraded_at: null`, `rehab_laudo: PBI-FEATURE-185-REVOKED-REGISTRY`, `rehabilitated_at`, ventana recortada). Evidencia en `execution.md`. Prohibido versionar `.SddIA/cerbero/` / `.SddIA/radamanto/` como cierre del PR.
2. **A2 — Fail-soft padre (Kintsugi de fase):** si el DCC hijo cruzó umbral físico (`pr_url` o `delivery_push`) y falla cola secundaria ya cubierta o I/O de telemetría no causal, el padre `feature` marca `fail_soft` y no colapsa el `exit_code` global. Git/init, snapshot, push, apertura de PR, Argos `block` y fallos reales de agentes 2–5 permanecen causales. `aggregate_execution_terminal` intacto.
3. **A3 — Poda de telemetría hueca:** `Raw_Execution_Finished` debe portar `cycle_phase` (y `lab_hollow` si aplica). `radamanto-batch` no ingiere runs `initialized` / `awaiting_agents` / lab-skip de cierre / `lab_hollow: true` como supervivencia. PEC sigue emitiéndose. Filtro extensible a lifecycle que comparte peaje; **sin** rehab de `bug-fix`.
4. **Ontología:** conservar `entity_type: process` (ya correcto en instancia).
5. **Umbrales:** tabla Radamanto 1.1.0 intacta; no reabrir `success_rate` ni `max_recovery_attempts`.
6. **Cierre documental single-PR:** cascada bajo `persist_ref` + PBI en `docs/todos/done/` + `validacion.md` APTO (`pbi_archived: true`).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-A1 | `feature` ∉ `permanent` ni `revoked`; stats raíz `healthy`; `recovery_attempts: 0`; laudo y timestamp de rehab; ventana recortada; evidencia A1 en `execution.md`. |
| AC-ONTO | `entity_type: process` conservado. |
| AC-A2 | DCC hijo post-umbral físico + cola secundaria → padre `fail_soft` + éxito operativo; causales (git/snapshot/push/PR/Argos/agentes reales) intactos; agregador sin mutar. |
| AC-A3 | Payload de supervivencia porta `cycle_phase`; batch ignora huecos y no muta contadores por ellos; PEC permanece. |
| AC-THRESH | Umbrales 1.1.0 intactos. |
| AC-DOC | Cascada `features-documentation-pattern`; PBI en `done/`; `validacion.md` con `global: APTO`, `pbi_archived: true`, `branch` coherente. |

## Fuera de alcance

- Residual Kalma2 Shell / `git-manager` (dedup OPERATIVO PPR #136 done).
- Lab IMAP/Telegram vivo (`LAB_*_LIVE: DIFERIDO` en feature #185).
- Rehabilitación de `bug-fix` o `emit-pr-audited-event`.
- Reabrir umbrales `process: 0.70` / tabla 1.1.0 (SSOT post-#174+#177).
- Mutar `aggregate_execution_terminal` para «tolerar simulated».
- Troceo EDA de `feature` en eventos atómicos (faro Kaizen; Filtro C).
- Versionar mutaciones de instancia Cerbero/Radamanto en el PR de motor.

## Restricciones

- Git solo vía `skill:git-manager`. Rama canónica: `refactor/feature-revoked-registry-rehab`.
- Prohibido despachar ciclos `bug-fix` satélite.
- Cerbero no tiene estado `healthy`: rehab = borrar clave de `permanent` (y de `revoked` si reapareciera).
- Reset A1 no absoluto (`attempts` ≥ 3) reabre `Domain_Entity_Deprecated` al primer fallo.
- Mayeuta no diseña touchpoints de código; Dedalo los fija en `spec.md`.
- El cuerpo de este documento es el `refined_requirements` de entrada a Dedalo.

## Ley aplicada

- `features-documentation-pattern` v1.2.x (frontmatter + un `.md` por fase).
- Proceso `refactorization` v1.2.2 — fase Estabilización de alcance → Mayeuta.
- Cierre documental en rama (un PR): `task-closure-documental` / patrón v1.2.0+.
- Jurisprudencia `L-REHAB-INST` #174+#177.
- Jerarquía: Acción → Agente → Skill → Tools.
- `SddIA/norms/external-ai-constraints.md` (soberanía de rutas; forja gobernada).
