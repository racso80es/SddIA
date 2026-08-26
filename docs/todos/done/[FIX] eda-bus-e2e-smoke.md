---
document_id: PBI-FIX-EDA-BUS-E2E-SMOKE-WASI-BUILD
uuid: "b9e4d3f2-5c6a-7b8e-9f0d-1e2f3a4b5c6d"
title: "[FIX] eda-bus-e2e-smoke — build WASI workspace bloquea E2E lab"
format: markdown
version: "1.1.0"
status: done
type: bug-fix
priority: alta
process: bug-fix
persist_ref: docs/fixes/eda-bus-e2e-smoke-wasi-build-block
created: "2026-08-26"
updated: "2026-08-26"
pbi_archived: true
derived_from:
  - PBI-KAIZEN-EDA-BUS-E2E-WASMTIME-FALLBACK
  - PBI-FIX-WASI-RUNTIME-SMOKE
incident_ref: "PR #194 — job eda-bus-e2e-smoke FAIL exit 101; E2E lab nunca ejecutado"
audit_ref: docs/fixes/bundle-consumer-telegram-gateway/validacion.md
workflow: .github/workflows/sddia-index-qa.yml
job_name: eda-bus-e2e-smoke
ci_run_id: "32964510993"
ci_job_id: "98163957132"
related_pbi: docs/todos/pending/[Fix] wasi-runtime-somke.md
friction_ids:
  - F-CI-EDA-E2E-WASI-BUILD
tech_debt_ids:
  - DT-WASI-NATIVE-DAEMON-SPLIT
blocks_on: []
---

# [FIX] `eda-bus-e2e-smoke` — build WASI workspace bloquea E2E lab

## 0. Contexto

Empiría **2026-08-26** en PR #194 (`fix/bundle-consumer-telegram-gateway`). El job `eda-bus-e2e-smoke` falla en el **mismo paso** que `wasi-runtime-smoke`: compilación WASI del workspace completo. El fix de bundle telegram-gateway **no** modifica workflow ni `email-watcher`.

### Rol del job en la arquitectura CI

| Aspecto | `eda-bus-e2e-smoke` | `wasi-runtime-smoke` |
|---------|---------------------|----------------------|
| **Propósito** | Smoke **EDA bus** end-to-end: emisión domain → route → suscriptores (lab simulado) | Certificación **ejecución WASI física** (`SDDIA_CI_REQUIRE_WASI=1`) |
| **Runtime WASM** | Fallback permitido (Python / nativo si wasmtime falla — PR #83) | WASI **obligatorio** |
| **Paso compartido que falla** | `Build WASI workspace` | `Build WASI workspace` |
| **Smoke propio** | `sddia-qa run-eda-e2e-lab --entity-class tool --json` | `sddia-qa run-wasi-ci-smoke --json` |
| **Extra** | `event-sweeper --once` | `gate-evolution --range` |

Ambos jobs comparten cache key `wasi-${{ runner.os }}-${{ hashFiles('SddIA/Cargo.lock') }}` y el comando:

```bash
cd SddIA && cargo build --workspace --target wasm32-wasip1
```

**Relación con fix anterior (PR #83):** el Kaizen `eda-bus-e2e-wasmtime-fallback` resolvió `wasmtime` ausente en **runtime** (`cryptography-manager` fallback). El fallo actual es en **compilación** previa: el workspace WASI no llega a construirse.

---

## 1. Fricción

| ID | Síntoma | Causa raíz | Ad-hoc (no persistir) | Acción |
|----|---------|------------|----------------------|--------|
| **F-CI-EDA-E2E-WASI-BUILD** | Job `eda-bus-e2e-smoke` FAIL ~43s; cero evidencia E2E | `cargo build --workspace --target wasm32-wasip1` arrastra `email-watcher` → `openssl-sys` incompatible con WASI | Desactivar job en workflow temporalmente | Unificar fix con `PBI-FIX-WASI-RUNTIME-SMOKE` (mismo DT) |

---

## 2. Comportamiento producido (log CI)

### 2.1 Secuencia del job (esperada vs real)

| # | Step workflow | Esperado | Real PR #194 |
|---|---------------|----------|--------------|
| 1 | checkout + rust-toolchain (`wasm32-wasip1`) | OK | OK |
| 2 | Install wasmtime | OK | OK |
| 3 | cache restore | OK | OK |
| 4 | **Build WASI workspace** | ELF `.wasm` para skills/tools | **FAIL exit 101** |
| 5 | Build nativos (`event-watcher`, `execute-process`, `sddia-qa`) | — | **no ejecutado** |
| 6 | **EDA E2E lab (simulate)** | `run-eda-e2e-lab` JSON success | **no ejecutado** |
| 7 | **event-sweeper --once** | sweep idempotente | **no ejecutado** |

### 2.2 Error canónico (run `32964510993`, job `98163957132`)

Mismo bloqueo que `wasi-runtime-smoke`:

```text
   Compiling openssl-sys v0.9.117
error: failed to run custom build command for `openssl-sys v0.9.117`

Could not find openssl via pkg-config:
  pkg-config has not been configured to support cross-compilation.

$HOST = x86_64-unknown-linux-gnu
$TARGET = wasm32-wasip1
openssl-sys = 0.9.117

Error: Process completed with exit code 101.
```

Dependencias descargadas en el mismo job incluyen `imap`, `native-tls`, `openssl` — traza de `email-watcher`, no del smoke E2E en sí.

### 2.3 Qué **no** es este fallo

| Hipótesis | Evidencia contra |
|-----------|------------------|
| Falta `wasmtime` en PATH | wasmtime se instala; fallo antes de smoke |
| Payload ECST `emit-domain-mutation` | Fix previo `fix-eda-bus-e2e-smoke-entity-payload`; E2E lab no llegó a ejecutar |
| Fallback `cryptography-manager` | PR #83; runtime no alcanzado |
| Regresión `telegram-gateway` / bundle | Cambios solo en `build-release-bundle.sh`; no en workspace WASI |
| Topología local vs suscriptores | Kaizen histórico distinto; job no ejecuta sweep |

### 2.4 Impacto operativo

- PRs pueden mergearse vía `accept-pr` local con jobs WASI en rojo (observado PR #194).
- **No hay** señal CI de salud del bus EDA ni de `event-sweeper` en main hasta corregir el paso de build.
- Jobs nativos (`verify-tools-index`, `eda-iota-*`) siguen verdes → el defecto está acotado al grafo WASI del workflow.

---

## 3. Alcance del fix (propuesta)

**Unificar con `PBI-FIX-WASI-RUNTIME-SMOKE`** — un solo parche workflow/workspace recomendado.

| Artefacto | Cambio |
|-----------|--------|
| `.github/workflows/sddia-index-qa.yml` | En **ambos** jobs `wasi-runtime-smoke` y `eda-bus-e2e-smoke`: build WASI solo de paquetes migrados; excluir `email-watcher` y daemons nativos |
| Alternativa | Paso `Build WASI workspace` solo en `wasi-runtime-smoke`; `eda-bus-e2e-smoke` omite WASI build y confía en fallback (alineado con espíritu PR #83) — **validar** que `run-eda-e2e-lab` no exige `.wasm` frescos del workspace completo |
| `docs/fixes/eda-bus-e2e-wasmtime-fallback/` | Nota: fallback runtime ≠ build workspace |

---

## 4. Criterios de aceptación

- [x] Job `eda-bus-e2e-smoke` SUCCESS en PR de verificación.
- [x] `sddia-qa run-eda-e2e-lab --entity-class tool --json` ejecuta y acusa success.
- [x] `event-sweeper --once --json` ejecuta tras E2E lab.
- [x] Fix compartido con `wasi-runtime-smoke` documentado (un PR o dos PBIs cerrados en el mismo merge).
- [x] Sin regresión en fallback wasmtime (si se mantiene path sin WASI build en este job).

---

## 5. Referencias

| Ref | Uso |
|-----|-----|
| `docs/todos/pending/[Fix] wasi-runtime-somke.md` | Misma causa raíz; fix unificado |
| `docs/todos/done/[Kaizen] eda-bus-e2e-smoke — fallback cryptography-manager sin wasmtime.md` | Contexto fallback runtime |
| `docs/todos/done/[Kaizen] eda-bus-e2e-smoke — topología local vs suscriptores core y sweep vacío.md` | Historial E2E distinto |
| `docs/fixes/fix-eda-bus-e2e-smoke-entity-payload/` | Fix payload ECST (ya cerrado) |
| `.github/workflows/sddia-index-qa.yml` L116–153 | Definición job |
| PR #194 | Incidente |

---

## 6. Log crudo (referencia)

<details>
<summary>Fragmento PR #194 eda-bus-e2e-smoke (truncado)</summary>

```text
warning: profiles for the non root package will be ignored…
  Downloaded openssl-sys v0.9.117
  Downloaded imap v2.4.1
  Downloaded native-tls v0.2.18
  Could not find openssl via pkg-config:
  pkg-config has not been configured to support cross-compilation.
…
$TARGET = wasm32-wasip1
openssl-sys = 0.9.117
Error: Process completed with exit code 101.
```

</details>
