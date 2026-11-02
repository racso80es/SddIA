---

feature_name: aut-62-ppr-qa-gate
created: "2026-11-02"
process: bug-fix
phase: blueprint
agents: dedalo
phases: T0-T5
branch_name: Fix_Events_Multiple_02
sighting_branch: Fix_Events_Multiple_02
persist_ref: docs/fixes/aut-62-ppr-qa-gate
pbi_ref: AUT-62
document_id: PBI-c4293e98
uuid: 69228e55-be56-4578-8bf3-1207b4eb31a2
olas:
  - A1
  - V1
execution_id: "9554806d-2d2e-4944-89b7-25e2f81cc27a"
---
# Plan — aut-62-ppr-qa-gate

Orden: T0 evidencia baseline → T1 A1 Cerbero+Radamanto → T2 evolution → T3 verificación PPR/atestación → T4 gate DCC → T5 cascada Argos/DCC. Este commit de diseño **solo** sella `spec.md` + `plan.md`. Mutación de instancia y verificación = Ejecución (Tekton).

## T0 — Baseline (antes de mutar)

1. Capturar extracto de `.SddIA/cerbero/revoked_entities.json` (`revoked.pull-request-review` + `permanent`).
2. Capturar bucket `pull-request-review` de `.SddIA/radamanto/stats.json`.
3. Anotar en `execution.md` sección *Baseline* (timestamps, `since`, sample count/KO).
4. Confirmar umbrales leídos: `SddIA/agents/radamanto.thresholds.json` v1.3.0 sin intención de editar.

## T1 — A1 instancia (Cerbero + Radamanto)

1. Quitar clave `pull-request-review` de `revoked` (y de `permanent` si existe).
2. Reset stats PPR: `status: healthy`, `recovery_attempts: 0`, `degraded_at: null`, poda samples `exit_code≠0`, stamp `rehab_laudo` AUT-62 / `PBI-c4293e98`.
3. Evidencia *Después* en `execution.md` (diff conceptual o JSON post).
4. Git: no forzar commit de `.SddIA/` si gitignore; la evidencia vive en persist_ref + evolution.

## T2 — Evolution canónica

1. Crear `SddIA/evolution/69228e55-be56-4578-8bf3-1207b4eb31a2.md` con contrato vigente.
2. Campos mínimos: `id_cambio` = UUID, descripción rehab PPR AUT-62, `relacionado`: AUT-62, `PBI-c4293e98`, `episode_since: 2026-10-02T18:42:59Z`, `persist_ref`.
3. Hash integrity según contrato evolution (Tekton/herramienta canónica; no inventar éxito si el hash no se calcula).

## T3 — Verificación PPR / atestación (V1a)

1. Disparo lab: `Local_QA_Requested` **o** `./sddia-run.sh --process pull-request-review` con inputs de lab acordados (rama de trabajo del ciclo).
2. Criterio duro: ausencia de `FAIL_F4_RBAC` / `FAIL_F5_VERDICT` por entidad revocada.
3. Si `verdict: aprobado`: comprobar `{proofs}/qa-attestations/{branch_slug}.json` → `schema: qa-attestation/1.0`, `tree_sha` = HEAD, TTL vigente.
4. Si falla por causa **distinta** de revocación: documentar; no declarar AC2/AC3 OK.
5. Releer Cerbero: PPR sigue ∉ revoked/permanent (anti-re-revoke inmediata).

## T4 — Gate pre-PR QA (V1b)

1. Con atestación válida in situ, ejercer fase *Gate pre-PR QA* de `delivery-close-cycle` (lab parcial o inspección del handler) **sin** `SDDIA_LAB_SKIP_PRE_PR_QA_GATE`.
2. Esperado: no `friction_id: qa_attestation_required`.
3. Si DCC propio está revocado y el lab no puede correr el proceso completo: verificar el predicado de atestación (existencia + schema + tree_sha + verdict) y documentar el límite; no fingir DCC completo.

## T5 — Cascada documental / cierre

1. `implementation.md` + completar `execution.md`.
2. Argos → `validacion.md` (APTO solo con AC1–AC6 evidentes).
3. Tracker AUT-62: actualización de estado vía canal tracker (no escribir TODO markdown bajo `docs/todos/`).
4. `delivery-close-cycle` del propio fix **después** de validacion APTO (fuera de esta parada Dedalo).

## Contingencia L-HOLLOW

Si post-T3 Radamanto re-revoca PPR por `abrupt_success_rate_drop` / KO de hijo detached:

1. No ampliar genoma en silencio.
2. Emitir faro vía Cúmulo / `Kaizen_Alert_Required`.
3. Referenciar jurisprudencia #190 (`is_survival_hollow`) en el faro; este plan no implementa A2 código por defecto.

## Cierre de esta parada Dedalo

Sellados: `docs/fixes/aut-62-ppr-qa-gate/spec.md`, `docs/fixes/aut-62-ppr-qa-gate/plan.md`.
Pendiente Ejecución: T0–T5.
