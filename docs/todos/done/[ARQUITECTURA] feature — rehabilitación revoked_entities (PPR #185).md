---
document_id: PBI-FEATURE-185-REVOKED-REGISTRY
title: "[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185)"
format: markdown
version: "1.2.0"
created: "2026-08-19"
updated: "2026-08-20T05:42:00Z"
status: done
priority: media
process: refactorization
dispatch: true
uuid: c8f4e2a1-7b3d-4e59-9f6a-2d1e0c9b8a7f
suggested_branch: refactor/feature-revoked-registry-rehab
persist_ref: docs/features/feature-revoked-registry-rehab
branch_name: refactor/feature-revoked-registry-rehab
pbi_ref: "docs/todos/done/[ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185).md"
source_correlation_id: 17043d6d-c978-4245-b554-2c5edcf94422
source_pr_url: https://github.com/racso80es/SddIA/pull/185
feature_ref: docs/features/kaizen-capsula-imap-triaje
incident_ref: "RBAC_PROCESS_SIGNER_REVOKED — process:feature ∈ revoked_entities.permanent since 2026-08-19T07:59:05Z (max_recovery_attempts_exceeded)"
entity: feature
parent_pbi: "docs/todos/done/[ARQUITECTURA] umbrales Radamanto process — rehabilitación revoked_entities (PPR #174+#177).md"
audit:
  date: "2026-08-20"
  verdict: "muerte por recovery_attempts; contrato de fases íntegro; rehab de registro sin fail-soft/reset reabre el vector"
  degraded_at: "2026-08-13T06:08:38Z"
  deprecated_at: "2026-08-19T07:59:05Z"
  recovery_attempts: 4
  max_recovery_attempts: 3
  window_success_rate: 0.727
olas:
  - id: A1
    name: saneamiento-estructural
    locus: instancia
  - id: A2
    name: fail-soft-kintsugi
    locus: engine
  - id: A3
    name: poda-telemetria-hueca
    locus: engine
related:
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/library/codexes/codex-software-engineering/process/feature.md
  - SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs
  - SddIA/engine/execute-process/src/engine/thermodynamic.rs
  - SddIA/engine/execute-process/src/engine/phase_terminal.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/engine/execute-process/src/engine/residual_runner.rs
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - SddIA/engine/execute-process/src/engine/cerbero_governance_react_core.rs
---

# [ARQUITECTURA] feature — rehabilitación revoked_entities (PPR #185)

## Mandato

Rehabilitar el proceso `feature` en `.SddIA/cerbero/revoked_entities.json` tras revocación **permanente** (`max_recovery_attempts_exceeded`).

Rehab de registro **sin** A1 absoluto + A2 + A3 **reabre el vector**. Ver § Auditoría y § Acciones.

| Campo | Valor |
|-------|--------|
| Entidad | `feature` |
| Registro | `.SddIA/cerbero/revoked_entities.json` → **`permanent.feature`** (no `revoked.feature`) |
| `entity_type` (instancia) | `process` (ya correcto; no es el vector activo) |
| `reason` | `max_recovery_attempts_exceeded` |
| Since | `2026-08-19T07:59:05Z` |
| Degradado desde | `2026-08-13T06:08:38Z` (`stats.feature.degraded_at`) |
| Check origen | `RBAC_PROCESS_SIGNER_REVOKED` (F4 Cerbero · alerta auditoría no bloqueante) |

## Sighting Cosecha

PPR #185 · CID `17043d6d-c978-4245-b554-2c5edcf94422` · firmante tekton vía cadena `feature → tekton (+ entity-manager T4/T5)` · artefactos materializados pre/post revocación.

La ejecución #185 **no es la causa** de la muerte: el proceso entregó (T1–T6, tests APTO). Cerbero alerta porque el firmante ya figuraba en `permanent`.

## Sightings adicionales

| Sighting | CID | Nota |
|----------|-----|------|
| Cosecha Kaizen dedup | `AicZf7SdgwpED4pyQq1KcmFUQHitACabDB8csNsmMTiC` | Re-run PPR #185 · seed ya materializada @ `17043d6d…` |

## Auditoría del proceso `feature` (2026-08-20)

Dictamen: el contrato de fases está íntegro. La muerte es termodinámica — peaje de un macro-proceso de 7 fases reducido a un `exit_code` binario, bucle self-healing sin reset de `recovery_attempts`, umbral `max_recovery_attempts: 3` superado (4).

#174+#177 (16-ago) añadió `process: 0.70` + exención latency y **prohibió tocar `feature`** (`permanent.feature` ya era lateral). El rate vigente **0,727 pasa** umbral `process`. Vector activo ≠ `success_rate_below_threshold`.

### Comportamiento de fases (causa de `exit_code: 1`)

Cadena V5 `feature.md` v1.3.2. Agregador `aggregate_execution_terminal`: cualquier fase `failed`/`blocked` **sin** `fail_soft` → éxito global falso.

| Fase | Handler | Causalidad |
|------|---------|------------|
| 1. Inicialización | `workspace-init` + `git-manager` | Fallo git → global 1. Sin `fail_soft`. |
| 2–5. Mayeuta → Dedalo → Tekton → Argos | agentes V5 o `simulated` | Lab sin runtime: `simulated` = **neutral** (éxito). El propio proceso declara: `success` no implica agentes ejecutados. |
| 6. Cierre documental | `feature-pbi-archive` | Exige `validacion.md` APTO + move PBI. Skip solo `SDDIA_LAB_SKIP_PBI_ARCHIVE`. |
| 7. Cierre de entrega | `feature-delivery-close` | Spawnea `delivery-close-cycle` entero. Hijo `success ≠ true` → fase padre failed. Sin `fail_soft`. |

PPR/DCC recibieron Kintsugi de fase en #174+#177. **`feature` no.** `residual_runner` no marca `fail_soft` en fallos de `feature`.

### Stats (bucket raíz `feature`)

| Campo | Valor |
|-------|--------|
| `status` | `deprecated` |
| `recovery_attempts` | **4** (`max`: 3) |
| `consecutive_success_count` | 0 |
| `structure_valid` | `true` (Argos validó; redención por racha no cerró) |
| Ventana | 11 samples · 8 OK / 3 KO · **rate 0,727** |

Fallos (runtime real, 6,5–13 min): `01a4edfb-…` 781 s · `1dfb704d-…` 632 s · `b2a13aba-…` 393 s.

Éxitos cortos (40 ms–2 s) = lab/`simulated`/skip. Recorte a runs de minutos: **2/5 = 40 %**. El lab infla el rate; el runtime lo hunde.

Tres buckets ontológicos:

| Clave | Estado | Rol |
|-------|--------|-----|
| `feature` (raíz) | `deprecated` | El que Cerbero revocó |
| `entities.feature` | `healthy` | Fósil · samples ~0 ms |
| `process:feature` | `healthy` | Prefijo `type:id` · no es el revocado |

### Cadena causal

```text
2026-08-13T06:08:38Z  Domain_Entity_Degraded
                      (aún tool 0.85 + latency 30 s; wall-clock feature 3–13 min)
        → Cerbero revoked.feature
        → self-healing: sandbox → Argos structure_valid → pending_redemption
        → Domain_Entity_Restored  (NO resetea recovery_attempts)
        → re-degradación (muestras mixtas / wall-clock / DCC anidado)
        → recovery_attempts = 4
2026-08-19T07:59:05Z  Domain_Entity_Deprecated
        → Cerbero: revoked → permanent.feature
```

`recovery_attempts` solo incrementa al degradar desde `healthy`. `Domain_Entity_Restored` limpia `degraded_at` / `consecutive_success_count`, **no** el contador. Cuatro ciclos healthy→degraded = muerte.

Antes del 16-ago, ≥5 samples con avg > 30 s degradaban por `latency_threshold` **aunque todos fueran exit 0**.

### Vectores de comportamiento (anti-recurrencia)

1. **Sin fail-soft propio.** Git init, archivo PBI o DCC hijo colapsan el `exit_code` global.
2. **DCC anidado sin aislamiento.** Fase 7 paga la cola causal del hijo (push / `gh pr` / snapshot). DCC ya rehab-eado; `feature` no.
3. **Contaminación lab/runtime.** Fases 2–5 `simulated` → peaje `exit_code: 0` + `cycle_phase: initialized`.
4. **Agregación binaria.** Un `exit_code` para 7 fases + subproceso.
5. **`structure_valid: true` + `deprecated`.** El contrato no está roto; el contador sí.

## Acciones del ciclo (olas A1–A3)

Un PR de motor (A2+A3) + mutación de instancia A1 evidenciada en `execution.md` (no entra al diff git; jurisprudencia `L-REHAB-INST` #174+#177). Prohibido despachar `bug-fix` satélite. Rehab de `bug-fix` / `emit-pr-audited-event` fuera de alcance.

Ajustes anti-alucinación aplicados a las tres órdenes de entrada:

| Orden cruda | Ajuste |
|-------------|--------|
| «pasar de revoked/deprecated a healthy» | Cerbero **no** tiene estado `healthy`. Rehab Cerbero = **borrar** la clave de `permanent` (y de `revoked` si reapareciera). `healthy` aplica solo a `stats.json`. |
| «refactorizar el agregador» para tolerar `simulated` | `aggregate_execution_terminal` **ya** trata `simulated` / `skipped` / `awaiting*` como neutrales. No tocarlo. El hueco es marcar `fail_soft` **antes** del agregador cuando el hijo DCC falla post-umbral físico. |
| «ignorar skipped en telemetría» | `skipped` de una fase periférica en un run `completed` no es ejecución hueca (`derive_cycle_phase`: executed+skipped → `completed`). Poda = runs `initialized` / `awaiting_agents` / lab-skip de cierre, no cualquier fase `skipped`. |
| «ignorar por completo el peaje» | PEC orquestación (`Process_Execution_Completed`) sigue emitiéndose (UI Kalma2). Lo que se poda es el **sample de supervivencia** en `radamanto-batch` (`Raw_Execution_Finished`). |

### A1 — Saneamiento estructural (rehabilitación explícita)

**Locus:** instancia `.SddIA/` (no genoma). Trazable. Absoluto.

| Parámetro | Valor obligatorio |
|-----------|-------------------|
| Cerbero | Eliminar `permanent.feature`. Verificar `revoked.feature` ausente. No reescribir como `revoked` residual. |
| Stats bucket | **Raíz** `feature` (el deprecado). No el fósil `entities.feature` ni `process:feature`. |
| `status` | `deprecated` → `healthy` |
| `recovery_attempts` | **0** (estricto) |
| `consecutive_success_count` | 0 |
| `degraded_at` | `null` |
| `rehab_laudo` | `PBI-FEATURE-185-REVOKED-REGISTRY` |
| `rehabilitated_at` | ISO de la intervención |
| Ventana `samples` | Recortar a vacío o a ≤ últimos éxitos runtime (`duration_ms` real, `exit_code: 0`). La ventana mixta actual (3 KO + éxitos de 40 ms) reabre `success_rate` en el siguiente fallo. |

Si el reseteo no es absoluto (`attempts` queda ≥ 3), el primer `exit_code ≠ 0` en degradado re-emite `Domain_Entity_Deprecated`. Evidencia A1 en `execution.md`; prohibido commitear `.SddIA/cerbero/` / `.SddIA/radamanto/` como cierre del PR.

### A2 — Fail-soft (Kintsugi de fase)

**Locus:** runner de `feature`, no el agregador compartido.

El agregador ya colapsa solo ante `failed`/`blocked` **sin** `fail_soft`. PPR/DCC ya marcan cicatriz; `feature` no (`residual_runner` L.703–706: `Err` → `failed` sin `fail_soft`). `invoke_process` del DCC hijo convierte cualquier `success ≠ true` en `Err` causal del padre, incluso si el hijo cruzó umbral físico y falló en cola secundaria.

**Duro (abortan `feature` de inmediato):**

- Fase 1 `workspace-init`: fetch/checkout/creación de rama.
- Snapshot final, push remoto, apertura de PR (fases causales del DCC hijo).
- Argos `block` / `argos_verdict: block`.
- Fases agente 2–5 cuando el runtime está configurado y el agente **falla** (Mayeuta/Dedalo/Tekton/Argos). `simulated` ya es neutral; no convertirlo en `fail_soft` (no es `failed`).
- Fase 6 `feature-pbi-archive` en runtime (DoD documental). Skip lab vía `SDDIA_LAB_SKIP_PBI_ARCHIVE` permanece `skipped` (neutral).

**Tolerancia (`fail_soft: true` + cicatriz; éxito operativo global):**

- Fase 7: si el DCC hijo cruzó umbral físico (`pr_url` o `delivery_push`) y el fallo es cola secundaria ya cubierta por `mark_fail_soft_if_secondary` (Higiene local / Impacto SddIA) o timeout de telemetría/`telemetry_receipt` no causal → el padre **no** propaga `Err` fatal. Usar `invoke_process_full`, inspeccionar envelope, marcar `fail_soft` en `feature-delivery-close`.
- I/O del peaje termodinámico del hijo (`telemetry_io_failed`) no decapita al padre.

**Touchpoints:** `phase_capsules.rs` (`capsule_feature_invoke_delivery_close`), `residual_runner.rs` (rama `feature` \| `bug-fix`: simetría de código, **sin** rehab de `bug-fix` este ciclo), reutilizar predicado DCC `mark_fail_soft_if_secondary`. Tests: hijo con `pr_url` + higiene failed → padre `fail_soft` + agregador `success`; hijo sin push/PR + snapshot failed → padre causal `exit_code: 1`.

### A3 — Poda de falsos positivos estadísticos

**Locus:** peaje `Raw_Execution_Finished` + consumidor `radamanto-batch`. No castrar PEC.

Hoy `cycle_phase` vive solo en orquestación (`derive_cycle_phase`: `simulated` → `initialized`; `awaiting*` → `awaiting_agents`; executed+skipped → `completed`). El payload de telemetría **no** lleva `cycle_phase`; Radamanto ingiere lab hueco como `exit_code: 0` / `duration_ms: 0`.

| Se poda del batch de supervivencia | No se poda |
|------------------------------------|------------|
| `cycle_phase` ∈ {`initialized`, `awaiting_agents`} | `cycle_phase: completed` (runtime real, agentes ejecutados) |
| Lab-skip de cierre: `SDDIA_LAB_SKIP_PBI_ARCHIVE` y/o `SDDIA_LAB_SKIP_DELIVERY_CLOSE` en el mismo run | Fase `skipped` aislada dentro de un `completed` (p. ej. impacto DCC condicional) |
| Payload `lab_hollow: true` (sello explícito) | Fallos reales (`exit_code: 1`, `cycle_phase: failed`) — el fuego cuenta |

**Implementación:**

1. `thermodynamic.rs`: copiar `cycle_phase` (y `lab_hollow` si aplica) al payload de `Raw_Execution_Finished`, no solo al PEC.
2. `radamanto_batch_core.rs`: si el sample es hueco → `delivery_state: skipped` / no empujar a `samples` / no mutar `recovery_attempts` ni `success_rate`. Marcar consumed para no reingestar.
3. Alcance de filtro: procesos lifecycle (`feature`, y de paso `bug-fix`/`refactorization` porque comparten peaje). No reabre rehab de `bug-fix`.

Objetivo: post-A1, la ventana se alimenta **solo** de fuego real. Lab no infla el rate; runtime no se esconde detrás de 40 ms.

## Criterio de cierre

- [x] A1: `feature` ∉ `permanent` ni `revoked` · stats raíz `healthy` · `recovery_attempts: 0` · `rehab_laudo: PBI-FEATURE-185-REVOKED-REGISTRY` · `rehabilitated_at` · ventana recortada
- [x] Ontología: `entity_type: process` (ya en instancia; no regresionar a `tool`)
- [x] A2: DCC hijo post-`pr_url` con cola secundaria → `fail_soft` en padre; git/snapshot/push/PR/Argos siguen causales; `aggregate_execution_terminal` intacto
- [x] A3: `Raw_Execution_Finished` porta `cycle_phase`; `radamanto-batch` ignora huecos; PEC sigue emitiéndose
- [x] Umbrales 1.1.0 intactos; redención **sin** reabrir `success_rate` ni `max_recovery_attempts`

## Fuera de alcance

- Residual Kalma2 Shell/`git-manager` (dedup OPERATIVO PPR #136 done).
- Lab IMAP/Telegram vivo (`LAB_*_LIVE: DIFERIDO` en feature #185).
- Reabrir umbrales `process: 0.70` / tabla 1.1.0 (ya SSOT post-#174+#177).
- Troceo EDA de `feature` en eventos atómicos (faro Kaizen; Filtro C).
