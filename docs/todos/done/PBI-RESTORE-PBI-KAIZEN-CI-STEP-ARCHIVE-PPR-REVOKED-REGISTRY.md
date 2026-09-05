---
document_id: PBI-RESTORE-PBI-KAIZEN-CI-STEP-ARCHIVE-PPR-REVOKED-REGISTRY
title: "[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (restore-pbi-kaizen-ci-step-archive)"
format: markdown
version: "1.2.0"
created: "2026-09-01"
updated: "2026-09-05T12:00:00Z"
status: done
refinement_status: implemented
pbi_archived: true
persist_ref: docs/features/ppr-revoked-registry-rehab-restore-kaizen-ci-step
branch_name: refactor/ppr-revoked-registry-rehab-restore-kaizen-ci-step
pr_url: https://github.com/racso80es/SddIA/pull/259
priority: alta
process: refactorization
executor_vehicle: feature
type: refactorization
dispatch: false
uuid: e2f8a1c4-7b3d-4e9f-a612-8c5d0b9e4f17
entity: pull-request-review
entity_type: process
olas:
  - A1
suggested_branch: refactor/ppr-revoked-registry-rehab-restore-kaizen-ci-step
persist_ref_suggested: docs/features/ppr-revoked-registry-rehab-restore-kaizen-ci-step
source_branch: fix/restore-pbi-kaizen-ci-step-archive
source_correlation_id: "AU1AzkrREQVTRhGHexuqiumPXPw8iP2SgCSLB7AcFKfc"
source_pr_url: https://github.com/racso80es/SddIA/pull/247
feature_ref: docs/fixes/restore-pbi-kaizen-ci-step-archive
parent_pbi: docs/todos/done/PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY.md
incident_ref: "REVOKED_ENTITY_ALERT_PULL_REQUEST_REVIEW — pull-request-review degradado en Radamanto y revocado en Cerbero (abrupt_success_rate_drop since 2026-08-29T05:01:52Z; rehabilitated_at instancia 2026-08-29T04:47:57Z; PR #220 mergedAt GitHub 2026-08-29T04:51:00Z oid c1007a51; rehab_laudo residual PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY; ventana FIFO 20 = 10 exit 0 / 10 exit 1; rate 0.50 vs umbral process 0.70; sightings 2026-09-01..04 F4 RBAC_PROCESS_REGISTRY → handoff accept-pr bloqueado)"
blocked_by:
  - "refactorization ∈ revoked since 2026-08-20T05:48:56Z (dedup done PPR #186) → vehículo DCC = feature (L-VEHICLE)"
  - "feature ∈ revoked since 2026-08-31T07:49:12Z — L-VEHICLE-DUAL: vehículo sigue siendo feature; no es gate F4 de este PBI (L-LATERAL)"
  - "delivery-close-cycle ∈ revoked since 2026-08-29T14:23:29Z — invocación DCC operativa en cosecha 2026-09-04; F4 bloqueante = PPR, no el emisor (L-LATERAL)"
related:
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/library/codexes/codex-software-engineering/process/pull-request-review.md
  - docs/todos/done/PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY.md
  - docs/todos/done/PBI-PPR-ANTI-RECURRENCE-HOLLOW-A2-KAIZEN-ADUANA-EVOLUTION.md
  - docs/fixes/restore-pbi-kaizen-ci-step-archive/validacion.md
  - docs/fixes/ignition-pre-push-guard/validacion.md
  - docs/ppr-cosecha-kaizen-20260904/validacion.md
source_audits:
  - docs/fixes/restore-pbi-kaizen-ci-step-archive/validacion.md
  - docs/fixes/ignition-pre-push-guard/validacion.md
  - docs/ppr-cosecha-kaizen-20260904/validacion.md
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
audit_verified_at: "2026-09-05T11:50:00Z"
---

# [ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (restore-pbi-kaizen-ci-step-archive)

## 0. Diagnóstico Forense

### 0.1 Stub semilla (v1.0 → v1.1)

| # | Alucinación / Incoherencia en el Stub | Realidad Empírica | Rectificación |
|---|----------------------------------------|-------------------|---------------|
| **H1** | `samples: 20 (0 exit 0 / 20 exit≠0)` y `success_rate 0.0`. | Ventana FIFO: **10 exit 0** (617321–1072785 ms) + **10 exit 1** (684–3642 ms). Rate **0.50**. | Caída bajo umbral process `0.70`, no fallo total. |
| **H2** | "Radamanto re-degradó ~14 min después con 20 muestras KO". | A `2026-08-29T05:01:52Z` la ventana tenía **3** muestras (exit 0 @ 801847 ms + 2×exit 1 @ 743/851 ms). `abrupt_drop_min_samples = 3`; rate `0.33 < 0.70`. | Transición `degraded` con n=3; las 17 restantes son FIFO posterior. |
| **H3** | Recidiva post-A2 / A2 no cubre `failed_phase_code`. | PR #221 mergedAt **`2026-08-29T10:48:06Z`** oid `5024a022` — ~6 h **después** de `degraded_at`. PR #221: sin mutación `.SddIA/cerbero/` ni `.SddIA/radamanto/`. | No hubo recidiva post-A2; hubo omisión de rehab de instancia tras desplegar el motor. |
| **H4** | A2 debía podar abortos PPR por gobernanza. | `is_governance_self_revoked_hollow` exige `failed_phase_code == "CERBERO_ENTITY_REVOKED"` y proveedor = entidad. PPR es orquestador, no proveedor DI. Abortos F4/F5 Argos: `FAIL_F4_RBAC` / `FAIL_F5_VERDICT` ≠ código motor. | A2 no aplica a veredicto de proceso; A1 (rehab instancia) es la vía. |
| **H5** | `FAIL_F4_RBAC` / `RBAC_PROCESS_REGISTRY` = bug de runtime. | Etiquetas de aduana Argos/Cerbero en PPR, no excepciones no controladas. | Paridad terminológica con códice y Cosecha. |

### 0.2 Hallazgos v1.2.0 (re-auditoría 2026-09-05)

| # | Inexactitud en v1.1.0 | Realidad Empírica 2026-09-05 | Rectificación |
|---|----------------------|------------------------------|---------------|
| **H6** | L-VEHICLE: «solo `refactorization` ∈ revoked → vehículo `feature`». | Cerbero `revoked`: `feature` since **`2026-08-31T07:49:12Z`**; `delivery-close-cycle` since **`2026-08-29T14:23:29Z`**. `validate_di_rbac` bloquea **proveedores**, no el `process_name`. Cosecha 2026-09-04 ejecutó DCC como emisor (`L-OUT`). | **L-VEHICLE-DUAL.** Vehículo documental permanece `feature` + `process_label: refactorization`. Dual-revocación es **L-LATERAL** (no rehab en este PBI). F4 bloqueante de este ciclo = `pull-request-review`, no el vehículo. |
| **H7** | Timeline: «PR #220 merge @ 04:47Z». | GitHub `mergedAt` **`2026-08-29T04:51:00Z`** oid `c1007a51`. `04:47:57Z` = `rehabilitated_at` de instancia (stats), no merge remoto. Delta instancia→`degraded_at`: **13 min 55 s**. Delta merge GitHub→`degraded_at`: **10 min 52 s**. | Separar sello de instancia y sello GitHub. |
| **H8** | Timeline: «PR #190 cerrado 2026-08-26». | GitHub `mergedAt` **`2026-08-25T12:52:07Z`**. | Fecha de merge = 2026-08-25. |
| **H9** | Duraciones `801.847 ms` (punto como miles ES). | Enteros canónicos en `stats.json`: `801847`, `617321`, … | Publicar **ms enteros**. Prohibido punto miles/decimal ambiguo. |
| **H10** | «F4 bloquea el proceso en seco» = 100 % de runs post-revoke son abortos cortos. | FIFO post-muestra 3 incluye **8× exit 0 largos** (617321–1072785 ms) intercalados con abortos cortos. Sightings **2026-09-01..04** sí son F4→F5 con `exitCode: 1` y `accept_pr_handoff: blocked`. | Dos regímenes en la misma ventana. El daño actual es **bloqueo de handoff** (Cosecha), no «cero éxitos post-revoke». Rehab A1 sigue siendo el único desbloqueo de `RBAC_PROCESS_REGISTRY`. |

Fuentes de H6–H10: `.SddIA/cerbero/revoked_entities.json`, `.SddIA/radamanto/stats.json`, `SddIA/agents/radamanto.thresholds.json` (`success_rate_min_by_entity_type.process: 0.70`, `abrupt_drop_min_samples: 3`), `gh pr view` #190/#220/#221/#247/#251/#253/#255.

---

## 1. Mandato

Rehabilitar el proceso `pull-request-review` en la instancia local (**Yunque Rúnico**):

1. Purgar `pull-request-review` de `.SddIA/cerbero/revoked_entities.json` (`revoked` y, si apareciera, `permanent`).
2. Reset absoluto del bucket raíz en `.SddIA/radamanto/stats.json`: `samples: []`, `status: healthy`, `structure_valid: true`, `recovery_attempts: 0`, `consecutive_success_count: 0`, `degraded_at: null`, `rehab_laudo` = este `document_id`, `rehabilitated_at` = ISO-8601 de A1.
3. Restaurar elegibilidad F4 (`RBAC_PROCESS_REGISTRY: APTO`) para que el handoff a `accept-pr` deje de nacer `blocked` por peaje de proceso revocado.

| Parámetro | Valor verificado 2026-09-05 |
|---|---|
| **Entidad** | `pull-request-review` |
| **Registro Cerbero** | `.SddIA/cerbero/revoked_entities.json` → `revoked.pull-request-review` |
| **Tipo** | `process` |
| **Motivo** | `abrupt_success_rate_drop` |
| **`since` / `degraded_at`** | `2026-08-29T05:01:52Z` |
| **`rehabilitated_at` residual** | `2026-08-29T04:47:57Z` (sello A1 instancia PR #220; obsoleto) |
| **PR #220 merge GitHub** | `2026-08-29T04:51:00Z` oid `c1007a51` |
| **Radamanto** | `degraded` · `structure_valid: false` · `recovery_attempts: 1` · `consecutive_success_count: 0` |
| **Laudo residual** | `PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL-PPR-REVOKED-REGISTRY` |
| **Emisores de sighting (no laterales a rehab)** | `github-bridge-watcher`, `git-hook-pre-push`, `delivery-close-cycle` |

---

## 2. Estado Empírico Verificado (`.SddIA/`)

### Cerbero — `.SddIA/cerbero/revoked_entities.json`

* `permanent`: `{}`.
* `revoked.pull-request-review`:
  ```json
  "pull-request-review": {
    "entity_type": "process",
    "reason": "abrupt_success_rate_drop",
    "since": "2026-08-29T05:01:52Z"
  }
  ```
* **Laterales fuera de alcance (L-LATERAL), snapshot 2026-09-05:** `bug-fix` (since `2026-08-28T16:18:17Z`), `delivery-close-cycle` (since `2026-08-29T14:23:29Z`), `entity-manager` (since `2026-08-29T14:19:57Z`), `feature` (since `2026-08-31T07:49:12Z`), `refactorization` (since `2026-08-20T05:48:56Z`).

### Radamanto — `stats.json` → `pull-request-review`

* `status`: `"degraded"` · `structure_valid`: `false` · `recovery_attempts`: `1` · `consecutive_success_count`: `0`
* `degraded_at`: `"2026-08-29T05:01:52Z"`
* **FIFO (orden físico, 20 muestras):**

| # | duration_ms | exit_code |
|---|-------------:|----------:|
| 1 | 801847 | 0 |
| 2 | 743 | 1 |
| 3 | 851 | 1 |
| 4 | 3642 | 1 |
| 5 | 617321 | 0 |
| 6 | 701514 | 0 |
| 7 | 1072785 | 0 |
| 8 | 805714 | 0 |
| 9 | 947407 | 0 |
| 10 | 727984 | 0 |
| 11 | 807587 | 0 |
| 12 | 729 | 1 |
| 13 | 969138 | 0 |
| 14 | 1029892 | 0 |
| 15 | 1045 | 1 |
| 16 | 684 | 1 |
| 17 | 739 | 1 |
| 18 | 697 | 1 |
| 19 | 977 | 1 |
| 20 | 828 | 1 |

* Rate: `10/20 = 0.50` (umbral process `0.70`).
* Disparo `abrupt_success_rate_drop`: muestras 1–3 (rate `1/3 ≈ 0.33`).

---

## 3. Genealogía y Cronología Causal

```mermaid
timeline
    title Ciclo de Vida pull-request-review (Agosto - Septiembre 2026)
    2026-08-25 : PR #190 merge GitHub 12:52:07Z (rehab incompleta, sin poda samples)
    2026-08-28 : kaizen-aduana detecta success_rate_below_threshold (0.25)
    2026-08-29 04:47:57Z : A1 instancia (rehabilitated_at stats; samples [])
    2026-08-29 04:51:00Z : PR #220 merge GitHub oid c1007a51
    2026-08-29 05:01:52Z : n=3 (1 OK + 2 KO) -> abrupt_success_rate_drop (0.33 < 0.70)
    2026-08-29 10:48:06Z : PR #221 merge oid 5024a022 (A2 motor; .SddIA/ intacto)
    2026-08-29 14:23:29Z : delivery-close-cycle entra en revoked (lateral)
    2026-08-31 07:49:12Z : feature entra en revoked (lateral; L-VEHICLE-DUAL)
    2026-09-01 : PR #247 Cosecha: F4 RBAC_PROCESS_REGISTRY; semilla este PBI
    2026-09-04 : PRs #251 #253 #255: F4/F5; handoff accept-pr blocked
    2026-09-05 : Refinamiento v1.2.0 (H6-H10)
```

1. **#190** (`mergedAt 2026-08-25T12:52:07Z`): rehab sin `L-SAMPLES` (**done**).
2. **A1 PR #220**: instancia @ `04:47:57Z`; merge GitHub @ `04:51:00Z` oid `c1007a51` (**done**).
3. **Degradación** `2026-08-29T05:01:52Z`: n=3, rate 0.33, `abrupt_success_rate_drop`.
4. **A2 PR #221** `2026-08-29T10:48:06Z` oid `5024a022`: motor `is_governance_self_revoked_hollow`; instancia fosilizada (**done**).
5. **Cosecha PR #247** (`mergedAt 2026-09-01T12:19:56Z`, CID `AU1Azkr…`): semilla este PBI.
6. **Sightings 2026-09-04:** PR #251 / #253 / #255 — mismo `since` PPR; F4/F5; handoff bloqueado.

---

## 4. Evidencia Empírica de Sightings (Cosecha Kaizen)

Hecho físico común en 2026-09-01..04: `pull-request-review ∈ revoked` → check `RBAC_PROCESS_REGISTRY: NO_APTO` → F5 `failed`/`blocked` → `accept_pr_handoff: false`.

No implica que todo sample Radamanto post-`degraded_at` sea aborto F4 (H10).

| Sighting / Fase | Correlación (CID) | PR / Contexto | Dictamen F4 / F5 | Nota Forense |
|---|---|---|---|---|
| **Cosecha Kaizen (Create)** | `AU1AzkrREQVTRhGHexuqiumPXPw8iP2SgCSLB7AcFKfc` | PR #247 (`restore-pbi-kaizen-ci-step-archive`) | `FAIL_F4_RBAC` · F5 `failed` | Semilla PBI (`a315ae3e…`). Merge GitHub `2026-09-01T12:19:56Z`. |
| **Cosecha Kaizen (Dedup)** | `600cd25c-7d3d-4be4-a53b-54a9ff64be51` | PR #247 (DCC) | `FAIL_F4_RBAC` · F5 `failed` | Emisor `delivery-close-cycle` (`ab27081e…`). |
| **Cosecha Kaizen (Dedup)** | `064918a2-af08-441f-a5b5-d34ad312c489` | PR #251 | `FAIL_F5_VERDICT` | Carrera sibling con `7dd9caa4…` (`d712f728…`). |
| **Cosecha Kaizen (Dedup)** | `2Wkh9xqgpu1C8LPAhWzfrvL8LQXdTa5Rz55r81GWReda` | PR #251 (`ignition-pre-push-guard`) | `FAIL_F5_VERDICT` | Emisor `github-bridge-watcher` (`7dd9caa4…`). |
| **Cosecha Kaizen (Dedup)** | `cf977edc-706b-4b01-ba70-4beec1fcca82` | PR #251 (`dcc-lab-residual-capsules`) | `FAIL_F5_VERDICT` | Emisor `delivery-close-cycle` (`95a54dc9…`). |
| **Cosecha Kaizen (Dedup)** | `c368985f-2c03-4852-a9aa-0bc363f6c94e` | PR #253 (`docs/ppr-cosecha-kaizen-20260904`) | `FAIL_F5_VERDICT` | Emisor `git-hook-pre-push` (`db1b9e3f…`). |
| **Cosecha Kaizen (Dedup)** | `2dYUXN7nq9DsKAs7xZ5vWgq5G3fkfErbwT2Ke2ffnv3Z` | PR #253 | `FAIL_F5_VERDICT` | Emisor `github-bridge-watcher` (`66954b4b…`). |
| **Cosecha Kaizen (Dedup)** | `2fad80c0-6ee1-42a2-8d6f-c1399113fbdc` | PR #253 | `FAIL_F5_VERDICT` | Emisor `delivery-close-cycle` (`72f5e494…`). |
| **Cosecha Kaizen (Dedup)** | `7293fada-4fbc-4aac-8881-8061e9c0583d` | PR #253 | `FAIL_F5_VERDICT` | Emisor `delivery-close-cycle` (`e21fc03d…`). |
| **Cosecha Kaizen (Dedup)** | `9c9cd653-dabe-4fe2-a54d-17f868cd427e` | PR #253 | `FAIL_F5_VERDICT` | Emisor `delivery-close-cycle` (`6362eb00…`). |
| **Cosecha Kaizen (Dedup)** | `74a57c11-6764-4a6a-92e6-7943faa48d35` | PR #253 (Pre-push) | `FAIL_F5_VERDICT` | Carrera gemela: exec `e431afdf…` y `8d2567b6…`. |
| **Cosecha Kaizen (Dedup)** | `DK5QuSSudtQmSiSMZikUXN83xiF7fwEHxGHGRCUBz1tm` | PR #255 (`local-ledger-20260904`) | `FAIL_F5_VERDICT` | Emisor `github-bridge-watcher` (`0b826e3b…`), sibling `d50a40ba…`. |
| **Cosecha Kaizen (Dedup)** | `e4c9970f-9e15-40fe-857f-07c44c1bada5` | PR #255 (`local-ledger-20260904`) | `FAIL_F5_VERDICT` | Emisor `delivery-close-cycle` (`d50a40ba…`), sibling `0b826e3b…`. |

---

## 5. Alcance Técnico (Ola A1 — Yunque Rúnico)

`.SddIA/cerbero/` y `.SddIA/radamanto/` están en `.gitignore`. Mutación de instancia **fuera del diff Git**.

### Acciones obligatorias

1. **Purga Cerbero:** DELETE `revoked["pull-request-review"]`. Assert `permanent["pull-request-review"]` ausente. Laterales intactos. `"permanent": {}` se mantiene.
2. **Reset Radamanto** (clave `"pull-request-review"`):
   ```json
   {
     "consecutive_success_count": 0,
     "degraded_at": null,
     "entity_type": "process",
     "recovery_attempts": 0,
     "rehab_laudo": "PBI-RESTORE-PBI-KAIZEN-CI-STEP-ARCHIVE-PPR-REVOKED-REGISTRY",
     "rehabilitated_at": "<ISO-8601-A1>",
     "samples": [],
     "status": "healthy",
     "structure_valid": true
   }
   ```
   **L-SAMPLES:** `samples: []`. Conservar FIFO actual (rate 0.50) re-dispara umbral al siguiente evento.
3. **Smoke post-rehab:** inyectar `pull-request-review` (lab: `SDDIA_AGENT_RELAY_IDE=1`, `SDDIA_LAB_SKIP_ACCEPT_PR_HANDOFF=1`). Acuse `exitCode: 0` + `detached: true`. Lectura inmediata: entidad ∉ `revoked`. No exigir Cosecha completa ni join al watcher (DA-5).
4. **Documental (diff Git):** cascada `persist_ref` + evolution UUID `e2f8a1c4-7b3d-4e9f-a612-8c5d0b9e4f17`. Sin umbrales, sin YAML PPR, sin motor A2.

---

## 6. Criterios de Aceptación

| ID | Criterio | Verificación |
|---|---|---|
| **CA1** | **Cerbero Sanado** | `"pull-request-review"` ausente de `revoked` y `permanent`. Laterales sin diff de claves/valores. |
| **CA2** | **Radamanto Reseteado (L-SAMPLES)** | `status: "healthy"`, `structure_valid: true`, `samples: []`, `recovery_attempts: 0`, `consecutive_success_count: 0`, `degraded_at: null`, `rehab_laudo` = este `document_id`. |
| **CA3** | **Aislamiento Git (AC-GIT-CLEAN)** | Diff sin `.SddIA/**` y sin `SddIA/agents/radamanto.thresholds.json`. |
| **CA4** | **Smoke de inyección** | Acuse PPR `success` + `detached: true`; post-acuse `pull-request-review` ∉ `revoked`. No join. |
| **CA5** | **Vehículo (L-VEHICLE-DUAL)** | DCC con `--process feature` y `process_label: refactorization`. No rehab de `feature`/`refactorization`/`delivery-close-cycle` en este ciclo. |
| **CA6** | **CI del PR documental** | Checks GitHub del PR de esta entrega en verde (`run_id` o URL). `validacion.md` no declara `global: APTO` sobre este CA sin evidencia de run. |

---

## 7. Laudos Operativos Vinculantes

* **L-REHAB-INST:** Mutación Cerbero/Radamanto = instancia ignorada. Prohibido versionar `.SddIA/cerbero/` ni `.SddIA/radamanto/`.
* **L-SAMPLES:** Rehab de proceso ⇒ `samples: []`.
* **L-LATERAL:** No asimila `bug-fix`, `delivery-close-cycle`, `feature`, `entity-manager`, `refactorization`.
* **L-VEHICLE / L-VEHICLE-DUAL:** Vehículo CLI = `feature`; nota = `refactorization`. Ambos ∈ `revoked`; no es precondición de A1 ni autoriza rehab lateral.
* **L-NO-THRESH:** Prohibido mutar `radamanto.thresholds.json`.
* **L-NO-A2:** Motor A2 ya **done** (PR #221). Este PBI no reabre `radamanto_batch_core.rs`.
* **L-TWO-REGIMES (H10):** FIFO mixto ≠ «F4 aborta el 100 %». El gate a sanar es `RBAC_PROCESS_REGISTRY` para handoff.

---

## 8. Criterio de Cierre y Transición

- [x] A1 instancia: `pull-request-review` eliminado de `revoked_entities.json`.
- [x] Stats reseteados: `healthy`, `structure_valid: true`, `samples: []` en A1, laudo este PBI.
- [x] Smoke PPR: acuse detached OK; entidad sigue fuera de `revoked`.
- [x] Cascada `persist_ref` + evolution UUID ciclo.
- [x] PR documental https://github.com/racso80es/SddIA/pull/259; CI verde run `33964399405` (CA6).
- [x] PBI en `docs/todos/done/PBI-RESTORE-PBI-KAIZEN-CI-STEP-ARCHIVE-PPR-REVOKED-REGISTRY.md` en la rama del PR; `validacion.md` `pbi_archived: true`.
