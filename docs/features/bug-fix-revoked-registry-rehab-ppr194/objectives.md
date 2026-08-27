---
feature_name: bug-fix-revoked-registry-rehab-ppr194
created: "2026-08-27"
process: refactorization
branch_name: refactor/bug-fix-revoked-registry-rehab-ppr194
persist_ref: docs/features/bug-fix-revoked-registry-rehab-ppr194
pbi_ref: docs/todos/pending/[ARQUITECTURA] bug-fix — rehabilitación revoked_entities (PPR #194).md
document_id: PBI-PPR-194-BUG-FIX-REVOKED-REGISTRY
uuid: 8a4b0d3f-5c2e-4f9b-8d6a-7e8f9a0b1c2d
phase: mayeuta-stabilization
agents: mayeuta
source_correlation_id: "59606407-eed3-4da8-ac13-3cf6205b2147"
source_pr_url: https://github.com/racso80es/SddIA/pull/194
feature_ref: docs/fixes/bundle-consumer-telegram-gateway
incident_ref: "REVOKED_ENTITY_ALERT_BUG_FIX — abrupt_success_rate_drop since 2026-08-16T16:09:32Z; entity_type tool (misclassified)"
parent_pbi: docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md
olas:
  - A1
---

# Objetivos — bug-fix-revoked-registry-rehab-ppr194

## Objetivo

Rehabilitar el proceso `bug-fix` tras revocación `abrupt_success_rate_drop` (PPR #194 / since `2026-08-16T16:09:32Z`) etiquetada erróneamente como `entity_type: tool`: A1 Yunque Rúnico Cerbero/Radamanto + corrección ontológica `tool`→`process` (jurisprudencia #174) + cascada documental single-PR. Un `persist_ref`, un PR.

## Alcance

1. **A1 — Saneamiento de instancia (Yunque Rúnico):** eliminar `revoked.bug-fix`; verificar `permanent.bug-fix` ausente; materializar/reset absoluto del bucket raíz `bug-fix` en stats (`healthy`, `recovery_attempts: 0`, `degraded_at: null`, `rehab_laudo: PBI-PPR-194-BUG-FIX-REVOKED-REGISTRY`, `rehabilitated_at`, `samples: []` — bucket actualmente ausente). Evidencia en `execution.md`. Prohibido versionar `.SddIA/cerbero/` / `.SddIA/radamanto/` en el diff del PR.
2. **Ontología:** fijar `entity_type: process` en stats rehab; prohibido conservar/reintroducir `tool`. Verificar que la resolución tipológica vigente (`L-TYPE-RESOLVE` #174) sigue mapeando `bug-fix`→`process`; si el motor aún estampa `tool`, Dedalo escala A2 (no inventado aquí).
3. **Umbrales:** tabla Radamanto 1.1.0 intacta; no reabrir `success_rate` ni `abrupt_drop_min_samples`.
4. **Cierre documental single-PR:** cascada bajo `persist_ref` + PBI canónico en `docs/todos/done/` + `validacion.md` APTO (`pbi_archived: true`).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC-A1 | `bug-fix` ∉ `revoked` ni `permanent`; stats raíz presentes y `healthy`; `recovery_attempts: 0`; laudo y timestamp de rehab; `samples` vacíos o solo OK runtime; evidencia A1 en `execution.md`. |
| AC-GIT-CLEAN | Instancia Cerbero/Radamanto ausente del diff del PR. |
| AC-ONTO | `entity_type: process` post-A1; cero `tool` residual para esta entidad. |
| AC-TYPE-VERIFY | Resolución tipológica vigente: `bug-fix`→`process` (o A2 motor si falla la verificación). |
| AC-THRESH | Umbrales 1.1.0 intactos. |
| AC-DOC | Cascada `features-documentation-pattern`; PBI en `done/`; `validacion.md` con `global: APTO`, `pbi_archived: true`, `branch` coherente. |

## Fuera de alcance

- Rehabilitación de `accept-pr`, `refactorization` o `emit-pr-audited-event`.
- Inventar olas A2/A3 motor (payload, handoff, fail-soft, hollow) sin vector empírico; hollow #185 ya cubre peaje lifecycle de `bug-fix`.
- Mutar `radamanto.thresholds.json` v1.1.0 sin laudo.
- Versionar mutaciones de instancia Cerbero/Radamanto en el PR.
- Escribir semillas Kaizen / TODOs bajo `docs/todos/` (Cúmulo / evento Kaizen).

## Restricciones

- Git solo vía `skill:git-manager`. Rama canónica: `refactor/bug-fix-revoked-registry-rehab-ppr194`.
- Cerbero no tiene estado `healthy`: rehab = borrar clave de `revoked` (y de `permanent` si apareciera).
- Bucket Radamanto ausente ⇒ materializar sano; no inventar samples históricos.
- Mayeuta no diseña touchpoints de código; Dedalo los fija en `spec.md` solo si L-TYPE-VERIFY falla.
- El cuerpo de este documento es el `refined_requirements` de entrada a Dedalo.

## Ley aplicada

- `features-documentation-pattern` v1.2.x (frontmatter + un `.md` por fase).
- Proceso `refactorization` — fase Estabilización de alcance → Mayeuta.
- Cierre documental en rama (un PR): `task-closure-documental` / patrón v1.2.0+.
- Jurisprudencia `L-REHAB-INST` / `L-ONTOLOGY` / `L-TYPE-RESOLVE` (#174+#177; anti-recurrencia #185/#186/#187/#190).
- SSOT proceso: `SddIA/library/codexes/codex-software-engineering/process/bug-fix.md`.
- Jerarquía: Acción → Agente → Skill → Tools.
- `SddIA/norms/external-ai-constraints.md` (soberanía de rutas; forja gobernada).
