---

feature_name: aut-62-ppr-qa-gate
created: "2026-11-02"
process: bug-fix
phase: design
agents: dedalo
base: main
scope: rehab-ppr-cerbero + radamanto-anti-re-revoke + gate-pre-pr-qa
branch_name: Fix_Events_Multiple_02
sighting_branch: Fix_Events_Multiple_02
persist_ref: docs/fixes/aut-62-ppr-qa-gate
pbi_ref: AUT-62
document_id: PBI-c4293e98
uuid: 69228e55-be56-4578-8bf3-1207b4eb31a2
version_spec: "1.0.0"
status: dedalo_locked
parent: AUT-20
sighting: AUT-50 / PR 432
episode_since: "2026-10-02T18:42:59Z"
olas:
  - A1
  - V1
execution_id: "9554806d-2d2e-4944-89b7-25e2f81cc27a"
---
# Spec — aut-62-ppr-qa-gate

## 1. Misión

Rehabilitar el proceso `pull-request-review` (episodio Cerbero `since 2026-10-02T18:42:59Z`, `reason: abrupt_success_rate_drop`) para que el camino canónico de atestación local (`pre-push` → `Local_QA_Requested` → `pull-request-review`) vuelva a sellar `qa-attestation/1.0` y el **Gate pre-PR QA** de `delivery-close-cycle` deje de responder `friction_id: qa_attestation_required`.

Padre: AUT-20. Sighting: fricción AUT-50 (rama `Fix_Events_Multiple_02`, PR 432). Etiquetas Linear: `pbi`, `kaizen`.

## 2. Problema verificado

| Hecho | Evidencia |
|-------|-----------|
| PPR ∈ `revoked` | `.SddIA/cerbero/revoked_entities.json` → `revoked.pull-request-review` · `entity_type: process` · `reason: abrupt_success_rate_drop` · `since: 2026-10-02T18:42:59Z` |
| ∉ `permanent` | `permanent: {}` (sin clave PPR) |
| Fallo aduana | Argos *Veredicto y bloqueo* → `FAIL_F4_RBAC` / `FAIL_F5_VERDICT`; `delivery_state: failed` |
| Gate DCC | `delivery-close-cycle` fase *Gate pre-PR QA* exige `qa-attestation/1.0` con `verdict: aprobado` + `tree_sha` = HEAD; sin ella → `status: blocked` / `qa_attestation_required` |
| Stats huecas | `.SddIA/radamanto/stats.json` → `pull-request-review.status: healthy` pero 2 samples `exit_code: 1` (~207–228 s) — ventana de re-revocación post-rehab |
| Umbrales vigentes | `SddIA/agents/radamanto.thresholds.json` v1.3.0: `success_rate_min_by_entity_type.process: 0.70`, `abrupt_drop_min_samples: 3`, `max_recovery_attempts: 3` |

No hay bypass manual de la aduana: mientras PPR siga revocado, el operador no cierra el ciclo por el camino canónico.

## 3. Genealogía (episodio nuevo)

| Episodio | Ciclo | Reason / bucket | `since` | Estado |
|----------|-------|-----------------|---------|--------|
| #124/#125 | `ppr-rehab-revoked-entities` | latency / registry | 2026-07 | done |
| #174+#177 | `radamanto-process-threshold-rehab` | `success_rate_below_threshold` + umbrales `process` | 2026-08-15 | done |
| #190 | `ppr-revoked-registry-rehab-ppr190` | permanent `max_recovery_attempts_exceeded` + revoked `abrupt_success_rate_drop` + hollow detached | 2026-08-25 | done |
| Aduana evolution / restore CI | `kaizen-aduana-evolution-local`, `restore-pbi-kaizen-ci-step-archive` | olas rehab / anti-recurrencia | 2026-08..09 | done |
| **AUT-62 (este)** | `aut-62-ppr-qa-gate` | revoked · `abrupt_success_rate_drop` | **2026-10-02T18:42:59Z** | **abierto** |

## 4. Laudos

| Ref | Decisión |
|-----|----------|
| **L-SCOPE** | Solo `pull-request-review`. No rehabilitar en este ciclo `bug-fix`, `delivery-close-cycle`, `feature`, `entity-manager`, `event-creator`, `refactorization` (también ∈ `revoked`) salvo que bloqueen el mismo gate de atestación — no lo hacen: el bloqueo es PPR → ausencia de atestación. |
| **L-REHAB-INST** | A1 de instancia vía ciclo autorizado; evidencia en `execution.md` + evolution. Prohibido «parche de sesión» sin correlato. `.SddIA/cerbero/` y `.SddIA/radamanto/` son estado de instancia (gitignore); no van al diff del PR salvo política explícita del orquestador. |
| **L-CERBERO** | Quitar clave `pull-request-review` de `revoked`. Si reaparece en `permanent`, quitarla también. |
| **L-STATS** | Reset bucket raíz `pull-request-review` en `stats.json`: `status: healthy`, `recovery_attempts: 0`, `degraded_at: null`, poda de samples `exit_code≠0` (KO que alimentan `abrupt_success_rate_drop` hueco), `rehab_laudo: AUT-62` / `PBI-c4293e98`. Conservar ≤3 samples OK si existen; si no, samples vacíos hasta el primer disparo post-rehab. |
| **L-THRESH** | No mutar `radamanto.thresholds.json` v1.3.0 en este ciclo: umbrales `process` y `abrupt_drop_min_samples: 3` ya están. La anti-re-revocación inmediata es **higiene de samples + primer disparo limpio**, no nuevo umbral. |
| **L-HOLLOW** | No reabrir ola #190 (`is_survival_hollow` / `detached_child`) salvo evidencia post-rehab de re-muerte por el mismo vector hollow. Si el primer disparo post-rehab re-revoca por KO de fases `agent:` / detach, abrir faro Kaizen (Cúmulo), no ampliar este fix sin laudo. |
| **L-VERIFY-PPR** | Tras A1: disparo lab `Local_QA_Requested` o `./sddia-run.sh --process pull-request-review` debe terminar **sin** `FAIL_F4_RBAC` / `FAIL_F5_VERDICT` por entidad revocada y, si veredicto `aprobado`, materializar `{proofs}/qa-attestations/{branch_slug}.json` (`schema: qa-attestation/1.0`, `tree_sha` = HEAD). |
| **L-VERIFY-DCC** | Con atestación válida (`verdict: aprobado`, `tree_sha` = HEAD, TTL vigente), fase *Gate pre-PR QA* de `delivery-close-cycle` **no** bloquea por `qa_attestation_required`. Lab: no usar `SDDIA_LAB_SKIP_PRE_PR_QA_GATE` como sustituto del fix. |
| **L-EVOL** | Alta canónica `SddIA/evolution/69228e55-be56-4578-8bf3-1207b4eb31a2.md` vinculada a AUT-62 y al `since` del episodio. |
| **L-KAIZEN** | Semillas Kaizen solo vía agent:cumulo / `Kaizen_Alert_Required`. Este ítem **no** escribe TODOs bajo `docs/todos/`. |
| **L-GIT** | Mutaciones git solo `skill:git-manager` / `./sddia-run.sh --tool git-manager`. |

## 5. Solución (olas)

### A1 — Instancia Cerbero + Radamanto

1. Retirar `pull-request-review` de `revoked` (y de `permanent` si aplica).
2. Alinear telemetría: reset stats según **L-STATS** para que un disparo post-rehab no re-revoque de inmediato por el mismo `abrupt_success_rate_drop` sobre KO previos (patrón olas #174/#190).
3. Documentar evidencia machine-readable en `execution.md` (antes/después JSON o extractos).

### V1 — Verificación del gate (post-A1)

1. Disparo PPR / `Local_QA_Requested` en lab → sin fallo RBAC/veredicto por revocación; atestación `qa-attestation/1.0` si `aprobado`.
2. Comprobar ausencia de PPR en `revoked` y `permanent`.
3. Ejercer *Gate pre-PR QA* (o inspección contractual equivalente) con atestación válida → no `qa_attestation_required`.
4. Materializar evolution AUT-62.

## 6. Touchpoints

| Artefacto | Mutación |
|-----------|----------|
| `.SddIA/cerbero/revoked_entities.json` | A1 — quitar `pull-request-review` |
| `.SddIA/radamanto/stats.json` | A1 — reset + poda KO |
| `SddIA/evolution/69228e55-be56-4578-8bf3-1207b4eb31a2.md` | Alta correlato AUT-62 |
| `docs/fixes/aut-62-ppr-qa-gate/*` | Cascada bug-fix (spec/plan/implementation/execution/validacion) |
| `SddIA/agents/radamanto.thresholds.json` | **sin cambio** (L-THRESH) |
| `radamanto_batch_core.rs` / genoma PPR | **sin cambio** salvo regresión post-rehab (L-HOLLOW) |

## 7. Fuera de alcance

- Rehabilitar otros procesos ∈ `revoked` en el mismo ciclo.
- Bypass `SDDIA_LAB_SKIP_PRE_PR_QA_GATE` como remedio permanente.
- Reescritura del genoma `pull-request-review.md` o del contrato DCC (se consume).
- Cerrar AUT-50 / PR 432 (sighting; ciclo ajeno).
- Escribir semillas bajo `docs/todos/`.

## 8. Criterios de aceptación

| ID | Criterio |
|----|----------|
| AC1 | `pull-request-review` ausente de `revoked` y `permanent` en `revoked_entities.json`. |
| AC2 | Disparo post-rehab de PPR / `Local_QA_Requested` termina sin `FAIL_F4_RBAC` / `FAIL_F5_VERDICT` por entidad revocada. |
| AC3 | Si veredicto `aprobado`, existe `{proofs}/qa-attestations/{branch_slug}.json` con `schema: qa-attestation/1.0` y `tree_sha` = HEAD. |
| AC4 | `delivery-close-cycle` *Gate pre-PR QA* con esa atestación no bloquea por `qa_attestation_required`. |
| AC5 | Evolution `69228e55-be56-4578-8bf3-1207b4eb31a2` referencia AUT-62 y `since: 2026-10-02T18:42:59Z`. |
| AC6 | Stats PPR post-A1: `healthy`, `recovery_attempts: 0`, sin samples KO residuales pre-rehab. |

## 9. Métrica de mejora

Alineada a AC1–AC5 del manifiesto Kaizen AUT-62; AC6 es anti-recurrencia inmediata del vector `abrupt_success_rate_drop` hueco.
