---
document_id: PBI-ARQUITECTURA-INSTALLER-V3-IO-CONTRACT
uuid: "cb5483e7-4e39-4fb6-9c11-4a8285b957b7"
title: "[ARQUITECTURA] Installer v3 — contrato de entrada-salida del comando (motor, fachada, progreso)"
format: markdown
version: "1.0.0"
status: done
pr_url: https://github.com/racso80es/SddIA/pull/303
closed: "2026-09-26"
priority: alta
type: arquitectura
process: refactorization
dispatch: false
feature_name: sddia-installer-v3-io-contract
historia_ref: docs/todos/historias/[ARQUITECTURA] Installer v3 — contrato de entrada-salida por comando y UX de Deploy - Eliminar Cliente.md
historia_document_id: HU-INSTALLER-V3-IO-CONTRACT-UX
historia_uuid: "489f5b85-fa2f-4f0e-ae52-7278e039dff1"
created: "2026-09-26"
author: tekton
installer_contract_ref: SddIA/library/norms/sddia-installer-contract.md
installer_contract_uuid: "b1327ef3-5f07-4fba-9073-a72c5fdf97e2"
installer_contract_version_actual: "1.1.0"
installer_contract_version_objetivo: "1.2.0"
pbi_antecesores:
  - document_id: PBI-ARQUITECTURA-INSTALLER-V2-DESPLIEGUE-LIMPIO
    uuid: "bb30e934-7f1f-44cb-a51e-21c28ccf426b"
    pr: "https://github.com/racso80es/SddIA/pull/302"
unblocks:
  - PBI-ARQUITECTURA-INSTALLER-V3-UX-EJECUTABLES
  - PBI-DT-PACIENTE0-DEPLOY-PROCESS
  - PBI-DT-PACIENTE0-UNDEPLOY-PROCESS
related:
  - SddIA/scripts/sddia-installer.sh
  - sddia-installer.sh
  - SddIA/scripts/qa/test-sddia-installer.sh
  - SddIA/norms/capsule-json-io.md
  - SddIA/tools/tools-contract.md
  - SddIA/engine/execute-process/src/engine/progress_trace.rs
baseline_decisiones:
  - "D2: logs en clave Cúmulo instance.installer_logs → {FORGE}/.SddIA/logs/installer/, retención 20"
  - "D3: envelope siempre; test-sddia-installer.sh lee result.plan; sin flag plan-legacy"
  - "D4: progress auto = fd 3 si abierto, si no stderr con prefijo @sddia-progress "
  - "D5: motor bash; sin reescritura Rust"
  - "D1 corregida: esquemas junto a la norma, no en capability-contracts"
  - "D7 aplazada: no se forja entidad tool en este PBI"
---

# Installer v3 — contrato de entrada-salida del comando

Historia madre: `HU-INSTALLER-V3-IO-CONTRACT-UX` §3–§4.4, fases F0–F3, AC-1..AC-8, AC-13, AC-14, AC-15. El presentador y los atajos son `PBI-ARQUITECTURA-INSTALLER-V3-UX-EJECUTABLES` y no se tocan aquí.

## 0. Filtro A — correcciones al borrador de la historia

| Afirmación de la historia | Corrección medida |
|---------------------------|-------------------|
| D1: esquemas en `directories.capability_contracts` | Ese directorio (`SddIA/library/norms/capability-contracts/`) guarda schemas de **salida de capacidades** (`doc.closure`, `fs.persist`, …) que `execute-process` valida tras DI. El installer no es un proveedor de capacidad. Meter ahí `installer.request` contaminaría el gate. **Baseline:** `SddIA/library/norms/sddia-installer-request.schema.json` y `sddia-installer-result.schema.json`, referenciados desde la norma 1.2.0. |
| D7: forjar `sddia-installer` como `tool` vía `tool-creator` | `tools-contract` §1 define tool como capacidad de **dominio de un workspace**, invocada por Cúmulo/Cerbero, no por comando crudo. `tool-creator` materializa cápsula bajo `execution_capsules.tools` cuando hay `execution_logic`. El motor ya existe en `SddIA/scripts/sddia-installer.sh` y los atajos lo invocan por CLI. Forjarlo como tool duplicaría el artefacto y contradiría la UX. `io_mode` no existe en `tools-contract` 1.5.0 (lo pide la historia MCP, no esta). **Baseline: D7 aplazada.** El envelope 2.0 se cumple sin entidad `tool`. |
| Códigos 1–5 «no cambian» | Cierto para el **número**. Hoy varios fallos distintos salen como `1` sin código semántico. Este PBI conserva el número y añade `error.code`. |

## 1. Intención

Que `deploy` y `teardown` acepten una petición estructurada y emitan **un único** envelope `capsule-json-io` 2.0 por stdout, con pasos declarados, errores tipados y un log del intento. Motor y fachada siguen sin prompts y sin HTTP (I-DEP-CEGUERA). Un consumidor futuro (`paciente0-deploy`, Kalma2) lee `result` y `error.code`; no parsea stderr.

## 2. Baseline de decisiones (HU §7)

Adoptado para ejecutar. Revocable por laudo del Vértice **antes** del init del `refactorization`.

| ID | Baseline de este PBI |
|----|----------------------|
| D1 | Schemas junto a la norma (§0). |
| D2 | Clave Cúmulo `instance.installer_logs` → `{FORGE}/.SddIA/logs/installer/`. Retención: 20 ficheros. |
| D3 | Envelope siempre. El smoke migra a `result.plan`. Sin `--output plan-legacy`. |
| D4 | `progress: auto` → fd 3 si está abierto; si no, stderr con prefijo `@sddia-progress `. |
| D5 | Bash. |
| D7 | No se forja `{name}.md`. |

## 3. Requisitos

### 3.1 Entrada (G2)

| ID | Requisito |
|----|-----------|
| R-IN-1 | Precedencia: `--request-file` > stdin si el primer byte no blanco es `{` > `SDDIA_CAPSULE_REQUEST` > `argv`. Flags actuales (`--root`, `--vault`, `--codex`, `--force`, `--skip-build`, `--dry-run`, `--allow-shared-mailbox`, `--no-verify`) conservan semántica y se normalizan al `request` de HU §4.1. |
| R-IN-2 | JSON mal formado, campo desconocido o `command` ∉ {`deploy`,`teardown`} → `REQUEST_INVALID`, exit **7**, envelope con `success:false`. |
| R-IN-3 | Ningún campo del `request` transporta secretos. La bóveda sigue entrando por fichero (`vault`) y entorno. |

### 3.2 Salida (G1, G3)

| ID | Requisito |
|----|-----------|
| R-OUT-1 | Motor y fachada escriben en stdout **exactamente un** objeto JSON (`meta.schemaVersion=2.0`, `meta.entityKind=tool`, `meta.entityId=sddia-installer`, `success`, `exitCode`, `message`, `durationMs`, `feedback[]`, `result`). `exitCode === 0 ⟺ success`. |
| R-OUT-2 | `result` incluye `command`, `root`, `esc`, `profile`, `dry_run`, `plan` (los 14 campos actuales de `emit_plan`), `steps[]`, `units`, `wui_port`, `verify`, `events`, `registry`, `log_ref`, `error`. |
| R-OUT-3 | Tabla `error.code` de HU §4.3. Códigos de proceso 1–5 idénticos a v1.1.0. Nuevo **6** `STEP_FAILED` (`error.step`, `error.child_exit`, `error.detail_tail` = últimas 20 líneas del log del paso, sin secretos). Nuevo **7** `REQUEST_INVALID`. |
| R-OUT-4 | Hijos (`build-release-bundle`, `sddia-run.sh` / `instance-creator`, `instance-health-verify`, `route-domain-event`) no escriben en el stdout del installer. Su JSON se captura y entra en `result` (`plan` del creator, `verify.verdict`, `events[].event_id`). Su texto va al log del intento. |
| R-OUT-5 | Cero **valores** de la lista negra R-VAULT-2 en envelope, progreso y log. Nombres de clave permitidos (`units.skipped[].missing`, `plan.channel_keys_present`). |

### 3.3 Pasos y progreso (G4)

| ID | Requisito |
|----|-----------|
| R-STEP-1 | Ids y títulos de HU §4.4, estables. Cada paso emite `begin` y `end` (`ok` \| `failed` \| `skipped`). Los no alcanzados quedan `not_run` en `steps[]`. |
| R-STEP-2 | Canal JSONL según D4. Campos: `kind=step`, `timestamp`, `id`, `index`, `total`, `title`, `moment`, y en `end`: `status`, `durationMs`. Sin rutas de bóveda ni valores de clave. |
| R-STEP-3 | Si `request.correlation_id` viene, cada transición se replica como PTC en `directories.progress/{correlation_id}/` (`phase` ← id, `source_agent=sddia-installer`, formato de `progress_trace.rs`). |
| R-STEP-4 | `feedback[]` es la proyección de `steps[]` al formato `tools-contract` §5 (`phase`, `level`, `timestamp`, `message`). |

### 3.4 Log (G5, lado máquina)

| ID | Requisito |
|----|-----------|
| R-LOG-1 | Cada intento escribe un log en `instance.installer_logs` y lo referencia en `result.log_ref` (ruta relativa a la forja). Rotación: conservar 20. |
| R-LOG-2 | El `mktemp` de trabajo se sigue borrando en `EXIT`. El log no vive ahí. |

### 3.5 Norma

| ID | Requisito |
|----|-----------|
| R-NORM-1 | `sddia-installer-contract` **1.1.0 → 1.2.0** vía `entity-manager` / `norm-creator`. Contenido nuevo: contrato de entrada, envelope, tabla `error.code`, códigos 6 y 7, canal de progreso, clave `instance.installer_logs`, invariantes **I-DEP-SINGLE-STDOUT** y **I-UX-NOPROMPT-MOTOR** (cero `read`/`select`/`zenity`/`whiptail` en motor y fachada). I-DEP-*/I-TEAR-* 1.1.0 intactos. Hash regenerado por la cadena. |
| R-NORM-2 | La cláusula de presentador, confirmación `ELIMINAR` y atajos **no** entra en 1.2.0. Es 1.3.0 del PBI de UX. |

## 4. Plan de ejecución

1. Init `refactorization` `sddia-installer-v3-io-contract` con este PBI como `pbi_ref`. Raw Kernel donde haya mutación de norma o de `cumulo.paths.json`.
2. **F0.** Clave Cúmulo `instance.installer_logs`. Schemas JSON (§0) y bump de norma a 1.2.0 por cadena autorizada. No editar `SddIA/library/norms/` a mano.
3. **F1.** Motor: parser de `request`, `STEP begin/end`, `_die` tipado, hijos al log, envelope único, `--dry-run` dentro de `result.plan`.
4. **F2.** Fachada: verify y eventos como pasos `verify_health` y `emit_event`; captura de su JSON; exit 5 = `VERIFY_NOT_APTO` dentro del envelope.
5. **F3.** JSONL de progreso + réplica PTC.
6. Migrar `test-sddia-installer.sh` a `result.plan` y añadir la matriz de AC de §5.
7. Cierre documental en la misma rama: `validacion.md` APTO, este PBI a `done/`, evolución con `uuid` de este PBI y de la historia.

## 5. Criterios de aceptación

Heredan el texto de la historia. Verificación en este PBI:

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-1 | Dry-run: un solo JSON en stdout; `meta.entityId=sddia-installer`; `success:true`; `exitCode:0`; `result.plan` con los 14 campos de `emit_plan`. | smoke |
| AC-2 | Mismo request por `--request-file`, stdin y `argv` → `result.plan` idéntico salvo `timestamp`/`durationMs`. | smoke |
| AC-3 | Errores 1/2/3/4 conservan `exitCode` de v2 y añaden `error.code` de §3.2; `success:false`. Request inválido → exit 7, `REQUEST_INVALID`. | matriz |
| AC-4 | Deploy de lab (`--skip-build`, root temporal): stdout = un JSON; cero líneas de `cargo`, `systemctl` o JSON de `execute-process` en stdout; presentes en `result.log_ref`. | inspección |
| AC-5 | Hijo forzado a fallar → exit 6, `STEP_FAILED`, `error.step=build_bundle`, `error.child_exit` real; pasos posteriores `not_run`. | smoke |
| AC-6 | Progreso: `begin`/`end` por paso ejecutado; `index`/`total` coherentes; sin rutas de bóveda ni valores de clave. | parser JSONL |
| AC-7 | Con `correlation_id`, PTC en `directories.progress/{cid}/`, `source_agent=sddia-installer`, `phase` ∈ ids de HU §4.4. | inspección |
| AC-8 | Lista negra R-VAULT-2: 0 valores en envelope, progreso y log. | test negativo |
| AC-13 | `rg` de `read -`, `select `, `zenity`, `whiptail` en motor y fachada = 0. `rg` de `curl` y `ss ` en el motor = 0. | grep gate |
| AC-14 | Norma 1.2.0 con hash de la cadena; códigos 6 y 7 documentados. | index-integrity |
| AC-15 | `test-sddia-installer.sh` y job `sddia-installer-smoke` verdes. I-DEP-*/I-TEAR-* 1.1.0 sin regresión (exit 1/2/3, abort si root = forja). | CI |

AC-9, AC-10, AC-11 y AC-12 no aplican: viven en el PBI de UX.

## 6. Fuera de alcance

- `sddia-installer-ui.sh`, plantillas de atajos, comando `shortcuts`, confirmación `ELIMINAR`, espera de tecla.
- Forjar `paciente0-deploy` / `paciente0-undeploy`.
- Entidad `tool` `sddia-installer` (D7 aplazada, §0).
- Migración del motor a Rust.
- Rollback automático tras `STEP_FAILED` o `VERIFY_NOT_APTO`.
- Cambio de semántica de los códigos 1–5 o de I-DEP-*/I-TEAR-* 1.1.0.

## 7. Riesgos

| Riesgo | Mitigación |
|--------|-----------|
| Silenciar `cargo` oculta el fallo real. | `STEP_FAILED` + `child_exit` + `detail_tail` (20 líneas) + log persistente. |
| El smoke actual lee el JSON plano de `--dry-run`. | Se migra en esta misma rama (D3). No hay otro consumidor en el repo. |
| `set -e` aborta antes de emitir el envelope. | El envelope de error se escribe en un `trap` antes del `exit`. Un crash sin trap es defecto, no modo degradado silencioso. |
| Réplica PTC duplica el formato de `execute-process`. | Mismos campos que `progress_trace.rs`. No se inventa un tercer esquema. |

## 8. Dependencias

- Antecesor: `PBI-ARQUITECTURA-INSTALLER-V2-DESPLIEGUE-LIMPIO` (done, PR #302).
- Bloquea: `PBI-ARQUITECTURA-INSTALLER-V3-UX-EJECUTABLES`.
- Desbloquea la forja futura de `paciente0-deploy` / `paciente0-undeploy` (esos PBI siguen en `pending/` y no se ejecutan aquí).
