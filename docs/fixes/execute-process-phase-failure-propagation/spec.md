---
feature_name: execute-process-phase-failure-propagation
created: "2026-08-11"
updated: "2026-08-13"
process: bug-fix
branch_name: fix/execute-process-phase-failure-propagation
persist_ref: docs/fixes/execute-process-phase-failure-propagation
pbi_ref: docs/todos/done/[FIX] execute-process — fallo de fase debe fallar ejecución global (EV-AUD-005).md
document_id: 04f8f435-450b-477a-970a-4a05dd0224cb
finding: EV-AUD-005
uuid: 7e2a9c4f-1b58-4d3a-a6e0-9f8c2d5b7a14
scope: phase-failure-propagation
base: main
correlation_id: dcb9efed-2268-4298-8108-7a55cf4db323
argos_verdict_session: APTO
clarify_ref: docs/fixes/execute-process-phase-failure-propagation/clarify.md
---

# Spec — fallo de fase debe fallar ejecución global (EV-AUD-005)

## Problema

Ejecución `62b201cf-0d82-4153-8c7d-8223233cf476` (`evolution-audit`): la fase `Persistencia oficial` terminó `failed` con `CERBERO_CONFIG_ERROR`, pero el envelope global certificó `success:true`, `status_code:0`, `exitCode:0`.

Hallazgo: `EV-AUD-005` (`docs/audits/evolution/2026-08-11.md`).

## Causa raíz

| Pieza | Hecho |
|-------|--------|
| Enrutado | `evolution-audit` no está en el conjunto tipado de `executor` (`feature` / `bug-fix` / `refactorization`) → cae en `residual_runner::run`. |
| Agregación residual (defecto) | `residual_runner::run_generic` fijaba `success = !blocked` solo con `argos_verdict == "block"`; **no** inspeccionaba `phase_reports[].status == "failed"`. |
| Contraste sano | `executor::run_generic` (y rutas afines) sí agregaban `any(status == failed)` → asimetría contractual entre runners. |
| Estado termodinámico | Si residual no materializa `state["phase_reports"]` antes del peaje, PEC/telemetría carecen de fases y reciben el mismo `success` incorrecto. |
| Fallo local correcto | DI/Cerbero ya marcan la fase `failed` + código (`cerbero_di_code` / gate / envelope); el defecto es la **agregación terminal**, no el marcado local. |

## Solución

### L1 — Agregador terminal único

Helper puro en el crate `execute-process` (módulo dedicado, p. ej. `phase_terminal`):

```text
TerminalVerdict {
  success: bool,
  status_code: i32,                 // 0 | != 0
  failed_phase: Option<FailedPhaseRef>,
  error: Option<String>,            // diagnóstico agregable
}

FailedPhaseRef {
  phase_name: String,
  code: Option<String>,             // primera clave tipada no vacía
  error: Option<String>,            // error original de la fase
  handler: Option<String>,
}
```

**Precedencia de agregación:**

1. Primera fase con `status ∈ {failed, blocked}` **sin** autorización fail-soft contractual → fallo global; conservar nombre, código y `error` de esa causal.
2. `state.argos_verdict == "block"` → fallo global (`blocked` / mensaje Argos).
3. `status ∈ {executed, skipped, simulated, awaiting, awaiting_agents}` → **neutrales** (no inducen fallo).
4. Fail-soft del **peaje termodinámico / I/O** (D3.13) no altera el veredicto de negocio ya decidido.
5. Fail-soft de **fase**: solo si el `process-contract` (o declaración explícita de la fase en genoma) lo autoriza. Hoy **no** hay semántica de fase fail-soft en genoma → **prohibido** blanquear `failed` obligatorio con un flag ad-hoc no contractual. Si el helper admite `fail_soft: true` en el report, debe documentarse como forward-compat **condicionado a contrato**; sin contrato, toda fase `failed`/`blocked` es hard-fail.

**Claves de código (orden de extracción):**
`cerbero_di_code` → `cerbero_envelope_di_code` → `di_gate_code` → `di_resolve_code` → `error_code`.

**Consumidores obligatorios:**

- `residual_runner::run_generic` — fix primario EV-AUD-005.
- `executor::run_generic` — parity; misma semántica.

**Consumidores opcionales (coherencia):**

- `delivery_close` / `capsule_invoke_smoke` si duplican agregación a mano — unificar o dejar nota de deuda si el diff se infla.

### L2 — Envelope de respuesta

Si hay fallo de fase (o Argos block):

| Campo | Valor |
|-------|--------|
| `success` | `false` |
| `status_code` | `!= 0` (canónico `1`) |
| `exitCode` / `exit_code` | igual a `status_code` |
| `error` | diagnóstico que incluya fase causal y código si existe |
| `data.failed_phase` | nombre de fase causal |
| `data.failed_phase_code` (o espejo) | p. ej. `CERBERO_CONFIG_ERROR` |
| `data.failed_phase_error` | diagnóstico original de la fase |
| `execution_report.phases` | sin reescritura; la fase causal permanece `failed` |

### L3 — Telemetría y orquestación

Antes de `thermodynamic::run`:

1. `state["phase_reports"] = phase_reports`.
2. Pasar `success` / `exit_code` del agregador (nunca `!blocked` a ciegas).
3. `Process_Execution_Completed.payload.status` = `"failed"` cuando el envelope falla; `exit_code` coherente. Incluir `failed_phase` / código en payload **solo** si el esquema de peaje lo admite sin romper consumidores; mínimo obligatorio: `status` + `exit_code` alineados con el envelope.

### L4 — Tests (mínimo)

| Caso | Expectativa |
|------|-------------|
| DI gate fail | fase `failed` → envelope `success:false` |
| Cerbero RBAC / `CERBERO_CONFIG_ERROR` | igual + `failed_phase` + código |
| Cápsula invoke fail | igual |
| Agente runtime fail | igual |
| Persistencia / `requires_capability` fail | igual (regresión patrón `62b201cf`) |
| Solo `skipped` / `simulated` / `awaiting_agents` | `success:true` si no hay `failed` |
| Argos `block` | `success:false` |
| Peaje I/O fail-soft | no altera veredicto de negocio ya decidido |

Regresión: fixture o test unitario que reproduzca el patrón `62b201cf-…` (fase tipo `Persistencia oficial` + `CERBERO_CONFIG_ERROR` → global fail).

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA1 | Cualquier fase obligatoria `failed` ⇒ ejecución global fallida (`success:false`, códigos ≠ 0). |
| CA2 | `skipped` / `simulated` / `awaiting_agents` no inducen fallo global por sí solos. |
| CA3 | Fail-soft del peaje no blanquea errores de fase no autorizados. |
| CA3b | Fail-soft de **fase** solo si el genoma/`process-contract` lo declara; sin contrato, `fail_soft` en report no puede ser la única vía de certificación (forward-compat debe quedar explícita o eliminarse del camino feliz). |
| CA4 | Respuesta incluye `failed_phase`, código y diagnóstico original. |
| CA5 | Telemetría + `Process_Execution_Completed` reflejan el mismo estado terminal. |
| CA6 | Tests cubren DI gate, Cerbero RBAC, cápsula, agente y persistencia fallidos. |
| CA7 | Regresión del patrón `62b201cf-…` pasa en rojo→verde **con evidencia física** (`cargo test -p execute-process --lib phase_terminal` o equivalente orquestado) citada en `validacion.md`. |

## Pendientes post-Verificación Kalma2/Argos (2026-08-11 → 2026-08-13)

Fuente: `validacion.md` (`global: NO_APTO`) + `clarify.md` (auditoría de motivos).  
Estado de código L1–L4: **implementado**; estado de **certificación de entrega**: **pendiente**.

### P1 — Evidencia runtime de tests (bloqueante)

| Campo | Valor |
|-------|--------|
| Checks Argos | `CARGO_TEST_PHASE_TERMINAL`, `CA7_regression_62b201cf_runtime` |
| Problema | En la sesión Argos, Shell IDE Rejected `cargo test`; sin stdout no se certifica. |
| Hecho host 2026-08-13 | `cargo test -p execute-process --lib phase_terminal` → **13 passed** (incl. `t9_regression_62b201cf_persistencia_oficial`). |
| Done | Re-auditoría Argos que cite ese stdout (o ejecución vía `execute-process` / lab formal) y actualice CA7 + `CARGO_TEST_*` a APTO. |

### P2 — Higiene de alcance / WIP (bloqueante)

| Campo | Valor |
|-------|--------|
| Check Argos | `SCOPE_WIP_CONTAMINATION` |
| Problema | Diffs ajenos a EV-AUD-005 en el mismo árbol: `kalma2.rs` (debug), `task_queue_manager` (suggested_branch), `event-watcher` async, `kalma2-bridge`/`app.js` (poll), docs `evolution-contract-index-v11`, `.cursor/debug-*.log`. |
| Done | PR/commit **solo** touchpoints EV-AUD-005 (`phase_terminal` + consumers listados abajo). Ola Kalma2 (poll/`awaiting_agents`, watcher async, TQM slug) → ciclo/PR distinto o laudo explícito de agrupación. |

### P3 — Dual `persist_ref` (bloqueante)

| Campo | Valor |
|-------|--------|
| Check Argos | `PERSIST_REF_DUAL_TREE` |
| Problema | Coexisten `docs/fixes/execute-process-phase-failure-propagation/` (canónico) y `docs/fixes/execute-processfallodefasedebefallarejecucinglobalev-aud-005/` (slug legado, mismo `correlation_id`). |
| Done | Un solo árbol canónico; eliminar o archivar el slug legado; `validacion.md` sin alias activo conflictivo. |

### P4 — Cierre documental / PBI (bloqueante post-APTO)

| Campo | Valor |
|-------|--------|
| Check Argos | `PBI_ARCHIVED` |
| Problema | PBI sigue en `docs/todos/pending/`; `pbi_archived: false` (correcto bajo rechazo). |
| Done | Tras `global: APTO`: mover PBI a `docs/todos/done/` en la rama del PR; `validacion.md` con `pbi_archived: true` (norma `task-closure-documental` / features-documentation-pattern v1.2.0). |

### P5 — CA3b fail-soft de fase (no bloqueante, deuda contractual)

| Campo | Valor |
|-------|--------|
| Check Argos | `CA3b_failsoft_phase_field` |
| Problema | Helper acepta `fail_soft: true` en phase report sin declaración en genoma. |
| Done | O (A) laudo Dedalo que formalice fail-soft de fase en contrato, o (B) restringir el helper a peaje/I/O y fallar hard toda fase `failed` sin contrato. |

### P6 — Deuda Kalma2 (fuera del núcleo EV-AUD-005; no mezclar en el mismo sellado)

| ID | Deuda | Motivo observado en ciclo `dcb9efed` |
|----|--------|--------------------------------------|
| K1 | Poll UI no debe cortar en `initialized`/`awaiting_agents` | UX congelada en “inicializado” |
| K2 | Early PEC TQM = `awaiting_agents` | Alinea sondeo con espera de agentes |
| K3 | Watcher rutas async | Evita inanición domain tras pending largo |
| K4 | TQM respeta `suggested_branch` del PBI | Evita dual persist_ref por slug |
| K5 | Single-flight / idempotencia por `correlation_id` | Evita doble `bug-fix` concurrente |

**PBI operativo:** `docs/todos/pending/[OPERATIVO] Kalma2 — ola mejora post-auditoría EV-AUD-005 (K1–K5).md`  
**Retoma:** `docs/fixes/execute-process-phase-failure-propagation/procedimiento-retoma.md`

**CA de entrega (higiene) — obligatorios para APTO de este fix:**

| ID | Criterio |
|----|----------|
| CA8 | Working tree / PR del fix sin diffs ajenos a la lista de touchpoints EV-AUD-005 (salvo laudo). |
| CA9 | Un único `persist_ref` canónico para el `document_id`. |
| CA10 | `validacion.md` con evidencia física de tests CA6/CA7 (comando + resultado). |
| CA11 | PBI archivado en `done/` en la rama del PR cuando `global: APTO`. |

## Fuera de alcance

- Corregir la causa de `CERBERO_CONFIG_ERROR` en la auditoría (config Cerbero / bindings).
- Mutar genoma `evolution-audit.md` u otras entidades vía forja manual.
- Diffs ajenos sin causalidad EV-AUD-005 (`kalma2.rs`, `task_queue_manager.rs`, `event-watcher`, UI Kalma2, etc.) **salvo** ola/PR dedicada (P6).
- Forjar entidad `process` nueva; no se instancia blueprint genómico.
- Mentir `pbi_archived: true` o `global: APTO` sin checks verdes.

## Nota de estado en rama (2026-08-13 · retoma Vía A)

- L1–L4 **materializados** (`phase_terminal` + cableado).
- P1–P4 / CA7–CA11: **cerrados** en retoma Vía A (tests 13/13 citados, WIP Kalma2 fuera del PR, slug legado podado, PBI en `done/`).
- P5 (CA3b fail-soft de fase): **deuda no bloqueante** — queda fuera del sellado APTO.
- P6 (ola Kalma2): **fuera de este PR**; stash `via-b-kalma2-ola-wip-post-ev-aud-005`.

## Touchpoints previstos (Tekton) — núcleo EV-AUD-005

- `SddIA/engine/execute-process/src/engine/phase_terminal.rs` — helper + tests.
- `SddIA/engine/execute-process/src/engine/mod.rs` — `pub mod phase_terminal`.
- `SddIA/engine/execute-process/src/engine/residual_runner.rs` — agregación.
- `SddIA/engine/execute-process/src/engine/executor.rs` — unificar helper.
- `SddIA/engine/execute-process/src/engine/delivery_close.rs` — paridad.
- `SddIA/engine/execute-process/src/engine/capsule_invoke_smoke.rs` — paridad.
- `SddIA/engine/execute-process/src/engine/thermodynamic.rs` — PEC/telemetría `failed_phase*` (sin mezclar cambios de early-PEC Kalma2 en el mismo commit si se podan).
- Documental: `implementation.md`, `execution.md`, `validacion.md`, `clarify.md`.
