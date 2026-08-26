---
document_id: PBI-FIX-WASI-RUNTIME-SMOKE
uuid: "a8f3c2e1-4b5d-6a7e-8f9c-0d1e2f3a4b5c"
title: "[FIX] wasi-runtime-smoke — build WASI workspace falla (openssl-sys / email-watcher)"
format: markdown
version: "1.1.0"
status: done
priority: alta
process: bug-fix
persist_ref: docs/fixes/eda-bus-e2e-smoke-wasi-build-block
created: "2026-08-26"
updated: "2026-08-26"
pbi_archived: true
closed_by_pr: https://github.com/racso80es/SddIA/pull/195
unified_with: PBI-FIX-EDA-BUS-E2E-SMOKE-WASI-BUILD
derived_from:
  - PBI-KAIZEN-CI-WASI-RUNTIME-BUILD
  - docs/features/ci-wasi-runtime-validation
incident_ref: "PR #194 — job wasi-runtime-smoke FAIL exit 101 en Build WASI workspace"
audit_ref: docs/fixes/bundle-consumer-telegram-gateway/validacion.md
workflow: .github/workflows/sddia-index-qa.yml
job_name: wasi-runtime-smoke
ci_run_id: "32964510993"
ci_job_id: "98163957252"
friction_ids:
  - F-CI-WASI-OPENSSL
tech_debt_ids:
  - DT-WASI-NATIVE-DAEMON-SPLIT
blocks_on: []
---

# [FIX] `wasi-runtime-smoke` — build WASI workspace falla (`openssl-sys` / `email-watcher`)

## 0. Contexto

Empiría **2026-08-26** tras merge PR #194 (`fix/bundle-consumer-telegram-gateway`). El fix de bundle **no** toca workflow CI ni dependencias WASI; expuso regresión ya latente en jobs que compilan el workspace completo para `wasm32-wasip1`.

| Job CI | Estado PR #194 | Paso que falla | Nunca alcanzado |
|--------|----------------|----------------|-----------------|
| `verify-tools-index` | **PASS** | — | — |
| `eda-iota-smoke-simulate` | **PASS** | — | — |
| `eda-iota-physical` | **PASS** | — | — |
| **`wasi-runtime-smoke`** | **FAIL** (exit 101) | `Build WASI workspace` | `WASI CI smoke`, `evolution gate` |
| **`eda-bus-e2e-smoke`** | **FAIL** (exit 101) | `Build WASI workspace` | `EDA E2E lab`, `event-sweeper --once` |

Workflow: `.github/workflows/sddia-index-qa.yml` · runner `ubuntu-latest` · target `wasm32-wasip1`.

Histórico: job `wasi-runtime-smoke` introducido en PR #84 (`ci-wasi-runtime-validation`) para **obligar** ejecución física WASI (`SDDIA_CI_REQUIRE_WASI=1`). Job `eda-bus-e2e-smoke` tenía fallback Python/wasmtime documentado en PR #83; ambos comparten el mismo paso `cargo build --workspace --target wasm32-wasip1`.

---

## 1. Fricción

| ID | Síntoma | Causa raíz | Ad-hoc (no persistir) | Acción |
|----|---------|------------|----------------------|--------|
| **F-CI-WASI-OPENSSL** | CI `wasi-runtime-smoke` y `eda-bus-e2e-smoke` fallan en ~40–50s con exit 101 | `cargo build --workspace --target wasm32-wasip1` incluye `email-watcher` (`daemons/*` en workspace). Dependencias `imap` + `native-tls` → `openssl-sys` 0.9.117, incompatible con cross-compile a WASI (sin OpenSSL/pkg-config para `wasm32-wasip1`) | Omitir jobs en merge manual; no build WASI en CI | **DT-WASI-NATIVE-DAEMON-SPLIT:** excluir daemons nativos del grafo WASI en CI + documentar contrato |

---

## 2. Comportamiento producido (log CI)

### 2.1 Comando que falla

```yaml
# sddia-index-qa.yml — job wasi-runtime-smoke
- name: Build WASI workspace
  run: cd SddIA && cargo build --workspace --target wasm32-wasip1
```

### 2.2 Cadena de dependencias (causa)

```text
SddIA/Cargo.toml  members = [ …, "daemons/*", … ]
  → email-watcher/Cargo.toml
      imap = "2.4"
      native-tls = "0.2"
  → openssl-sys v0.9.117  (build-script-main exit 101)
```

`email-watcher` es centinela **nativo** (IMAP/TLS en host); no es candidato WASI. El workspace glob `daemons/*` lo arrastra al build cross-target.

### 2.3 Error canónico (GitHub Actions run `32964510993`, job `98163957252`)

```text
error: failed to run custom build command for `openssl-sys v0.9.117`
  process didn't exit successfully: …/build-script-main` (exit status: 101)

Could not find openssl via pkg-config:
  pkg-config has not been configured to support cross-compilation.

Could not find directory of OpenSSL installation …

$HOST = x86_64-unknown-linux-gnu
$TARGET = wasm32-wasip1
openssl-sys = 0.9.117

warning: build failed, waiting for other jobs to finish…
Error: Process completed with exit code 101.
```

Variables de entorno WASI/OpenSSL **unset** en runner (`OPENSSL_DIR`, `WASM32_WASIP1_OPENSSL_*`, `PKG_CONFIG_ALLOW_CROSS`).

### 2.4 Pasos del job **no ejecutados** por el fallo

| Paso | Env / comando | Propósito |
|------|---------------|-----------|
| Build nativos | `cargo build -p event-watcher -p execute-process -p sddia-qa` | Orquestador + QA |
| WASI CI smoke | `SDDIA_CI_REQUIRE_WASI=1` · `sddia-qa run-wasi-ci-smoke --json` | Certificación cápsulas WASI vía wasmtime |
| evolution gate | `gate-evolution --json --range` | Delta evolución vs `origin/main` |

**Nota:** `wasmtime` **sí** se instala en el job; el fallo ocurre **antes** de cualquier invocación wasmtime.

### 2.5 Divergencia lab local

En forja local el mismo comando puede fallar con otro error (`clang` ausente, `ring`/`cc-rs`) según toolchain; en GHA el primer bloqueo observable es `openssl-sys` vía `email-watcher`. La regresión CI es reproducible y determinista en el runner Ubuntu.

---

## 3. Alcance del fix (propuesta)

| Artefacto | Cambio propuesto |
|-----------|------------------|
| `.github/workflows/sddia-index-qa.yml` | Sustituir `cargo build --workspace --target wasm32-wasip1` por lista explícita de paquetes WASI (skills/tools con `.wasm`), **excluyendo** `email-watcher`, `telegram-watcher`, centinelas nativos |
| `SddIA/Cargo.toml` | Opcional: `default-members` WASI vs nativos; o `exclude` / workspace anidado |
| `docs/features/ci-wasi-runtime-validation` | Actualizar spec: daemons sensoriales = nativos only |
| Smoke post-fix | `wasi-runtime-smoke` SUCCESS + `eda-bus-e2e-smoke` SUCCESS en PR de verificación |

**Fuera:** migrar `email-watcher` a WASI (IMAP en wasm no es objetivo); instalar OpenSSL cross en runner (frágil).

---

## 4. Criterios de aceptación

- [ ] Job `wasi-runtime-smoke` en `sddia-index-qa.yml` termina **SUCCESS** en PR de prueba.
- [ ] Paso `run-wasi-ci-smoke` con `SDDIA_CI_REQUIRE_WASI=1` ejecuta (no solo compila).
- [ ] `cargo build --workspace --target wasm32-wasip1` **no** incluye `email-watcher` en el grafo WASI del CI (o el comando ya no se usa para el workspace completo).
- [ ] Sin regresión: `verify-tools-index`, `eda-iota-smoke-simulate` siguen verdes.
- [ ] Documentación: contrato «daemon nativo ≠ miembro build WASI» en norma o feature CI.

---

## 5. Referencias

| Ref | Uso |
|-----|-----|
| `docs/features/ci-wasi-runtime-validation/spec.md` | Diseño original job `wasi-runtime-smoke` |
| `docs/todos/done/[Kaizen] eda-bus-e2e-smoke — fallback cryptography-manager sin wasmtime.md` | Fallback runtime (PR #83) — distinto de este fallo de **compilación** |
| `SddIA/daemons/email-watcher/Cargo.toml` | `imap` + `native-tls` |
| `SddIA/Cargo.toml` | `members = [ …, "daemons/*" ]` |
| PR #194 | Incidente observado (merge sin jobs WASI verdes) |

---

## 6. Log crudo (referencia)

<details>
<summary>Fragmento completo PR #194 wasi-runtime-smoke (truncado)</summary>

```text
Run cd SddIA && cargo build --workspace --target wasm32-wasip1
  Downloaded openssl-sys v0.9.117
  Downloaded imap v2.4.1
  Downloaded native-tls v0.2.18
   Compiling openssl-sys v0.9.117
warning: openssl-sys@0.9.117: Could not find directory of OpenSSL installation…
error: failed to run custom build command for `openssl-sys v0.9.117`
… (ver §2.3)
```

</details>
