---
document_id: HU-INSTALLER-V3-IO-CONTRACT-UX
uuid: "489f5b85-fa2f-4f0e-ae52-7278e039dff1"
title: "[ARQUITECTURA] Installer v3 — contrato E/S por comando y UX de los ejecutables Deploy / Eliminar Cliente"
format: markdown
version: "1.1.0"
status: "historia"
type: historia
priority: alta
context: "Forja SddIA / despliegue y retirada de instancias cliente (installer + atajos de escritorio)"
created: "2026-09-26"
updated: "2026-09-26"
source: "Petición del Vértice Biológico 2026-09-26, refinada contra el repo por Tekton"
process_candidate: refactorization
process_candidate_alt: feature
executables_observed:
  facade: sddia-installer.sh
  motor: SddIA/scripts/sddia-installer.sh
  shortcut_deploy: /home/racso/Aplicaciones/SddIA/SddIA_Deploy.sh
  shortcut_teardown: /home/racso/Aplicaciones/SddIA/SddIA_Eliminar_Cliente.sh
installer_contract_ref: SddIA/library/norms/sddia-installer-contract.md
installer_contract_uuid: "b1327ef3-5f07-4fba-9073-a72c5fdf97e2"
installer_contract_version_actual: "1.1.0"
installer_contract_version_objetivo: "1.2.0"
depends_on:
  - PBI-ARQUITECTURA-INSTALLER-V2-DESPLIEGUE-LIMPIO (done, PR #302)
unblocks:
  - PBI-DT-PACIENTE0-DEPLOY-PROCESS (process_candidate paciente0-deploy)
  - PBI-DT-PACIENTE0-UNDEPLOY-PROCESS (process_candidate paciente0-undeploy)
  - HU-KALMA2-PROJECT-WORKSPACE-SERVER-1xN (F5, redeploy gobernado de la instancia forjadora)
derived_from:
  - PBI-ARQUITECTURA-INSTALLER-V2-DESPLIEGUE-LIMPIO
  - PBI-ARQUITECTURA-DESTILACION-DEPLOY-DETERMINISTA
  - PBI-ARQUITECTURA-DEPLOY-DETERMINISTA
related_features:
  - docs/features/sddia-installer-v2-clean-deploy
  - docs/features/sddia-deterministic-installer
  - docs/features/sddia-deterministic-teardown
  - docs/features/destilacion-sddia-installer
ssot_refs:
  - SddIA/norms/capsule-json-io.md (envelope 2.0)
  - SddIA/tools/tools-contract.md (§5 feedback[], error, reglas de secretos)
  - SddIA/library/norms/sddia-installer-contract.md
  - SddIA/core/cumulo.paths.json (instance.host_registry, directories.progress)
  - SddIA/engine/execute-process/src/engine/progress_trace.rs (formato PTC)
  - SddIA/scripts/qa/test-sddia-installer.sh
decisions_status: "baseline adoptado en los PBI hijos; D1 y D7 corregidas (Filtro A del PBI de contrato). Laudo residual del Vértice antes del init."
spawned_pbis:
  - document_id: PBI-ARQUITECTURA-INSTALLER-V3-IO-CONTRACT
    uuid: "cb5483e7-4e39-4fb6-9c11-4a8285b957b7"
    path: docs/todos/pending/[ARQUITECTURA] Installer v3 — contrato de entrada-salida del comando.md
    process: refactorization
    fases: "F0–F3"
  - document_id: PBI-ARQUITECTURA-INSTALLER-V3-UX-EJECUTABLES
    uuid: "8cb95b4a-52f0-4030-91c1-8b937a425589"
    path: docs/todos/pending/[ARQUITECTURA] Installer v3 — UX de Deploy y Eliminar Cliente.md
    process: feature
    fases: "F4–F5"
    blocked_by: PBI-ARQUITECTURA-INSTALLER-V3-IO-CONTRACT
changelog:
  - "1.1.0: descomposición en dos PBI (§11). Correcciones que el PBI de contrato fija como baseline: D1 schemas junto a la norma (capability-contracts es salida de capacidades); D7 aplazada (tools-contract §1 y tool-creator no caben con el CLI). Norma partida: 1.2.0 contrato, 1.3.0 UX."
---

# Installer v3 — comando con contrato de entrada/salida y ejecutables amigables

## 1. Historia de usuario

**Como** Vértice Biológico (Racso), operador que despliega y retira instancias cliente SddIA desde la forja,
**quiero** (a) que `deploy` y `teardown` sean **comandos con contrato**: una petición estructurada de entrada y un único resultado estructurado de salida, con pasos, códigos de error y referencias trazables; y (b) que los **ejecutables de escritorio** `SddIA_Deploy` / `SddIA_Eliminar_Cliente` me informen del **paso en curso** sin ruido técnico, terminen con un **resumen** claro y **no cierren la ventana** hasta que pulse una tecla,
**para** que en el futuro el despliegue pueda invocarse desde el entorno (proceso Core, Kalma2, cola de tareas) sin *parsear* texto libre, y para que hoy, como humano, sepa qué está pasando y cómo ha acabado sin tener que leer cientos de líneas de `cargo`.

## 2. Contexto real verificado (estado a 2026-09-26)

### 2.1 Cadena actual de ejecución

```text
[ doble clic / terminal ]
  ~/Aplicaciones/SddIA/SddIA_Deploy.sh            (fuera de git)  ── exec ──▶
  ~/Aplicaciones/SddIA/SddIA_Eliminar_Cliente.sh  (fuera de git)  ── exec ──▶
                                                                            │
  {FORGE}/sddia-installer.sh  (fachada, 110 líneas)                        ◀┘
     ├─ parse --no-verify; resto passthru
     ├─ MOTOR "$@"
     ├─ deploy:   instance-health-verify (sddia-run.sh) → verdict → exit 5 si NO-APTO
     └─ deploy/teardown --force: emite Instance_Deployed / Instance_Torn_Down (route-domain-event)
                                                                            │
  {FORGE}/SddIA/scripts/sddia-installer.sh  (motor, 628 líneas, Ceguera)   ◀┘
     ├─ _parse argv → CMD, ROOT_FLAG, VAULT_FLAG, CODEX_FLAG, FORCE, SKIP_BUILD, DRY_RUN, ALLOW_SHARED_MAILBOX
     ├─ deploy:   validate_host → registry_reconcile → [do_teardown si live+force] → stage_vault
     │            → build-release-bundle.sh (cargo) → instance-creator (sddia-run.sh)
     │            → check_shared_mailbox → enable_units → registry_upsert
     └─ teardown: signal_instance_procs → stop/disable/reset-failed unidades @ESC → daemon-reload
                  → stop_lock_residuals → signal → registry_remove → templates_if_last → rm -rf ROOT
```

### 2.2 Lo que ya existe y se reutiliza

| Pieza | Estado real | Evidencia |
|-------|-------------|-----------|
| Contrato installer | `sddia-installer-contract` **1.1.0** (invariantes I-DEP-* / I-TEAR-*, códigos 1–5, registro host). | `SddIA/library/norms/sddia-installer-contract.md` |
| Plan estructurado | Solo en `--dry-run`: JSON plano en stdout con `command, root, esc, bundle_profile, vault_set, force, skip_build, live, vault_root_source, vault_instance_source, wui_port, port_source, channel_keys_present`. | motor §`emit_plan` |
| Códigos de salida | `1` args/host/ROOT inseguro · `2` destino vivo sin `--force` · `3` teardown sin `--force` · `4` buzón compartido · `5` verify NO-APTO (fachada). Fallos de hijos (`cargo`, `instance-creator`) propagan por `set -e` con el código del hijo. | motor `_die`, fachada |
| Registro host | `{FORGE}/.SddIA/instances.json` (`instance.host_registry`). | Cúmulo línea 131 |
| Verificación post-deploy | Proceso `instance-health-verify` → acta `docs/audits/instance-deploy-{ESC}-{ts}.md`, `data.verdict`. | fachada §deploy |
| Eventos de dominio | `Instance_Deployed`, `Instance_Torn_Down` con suscriptor Cúmulo (DLT). | fachada `_emit_instance_event` |
| Envelope canónico | `capsule-json-io` 2.0 (`meta`, `success`, `exitCode`, `message`, `result`, `durationMs`) y `tools-contract` §5 (`feedback[]` con fase/nivel/timestamp/message; `error{}`; prohibición de secretos). | normas |
| Trazas de progreso Kalma2 | PTC en `./.events/progress/{correlation_id}/` (`trace_id, correlation_id, timestamp, phase, severity, source_agent, message, metadata`), servidas por `kalma2-bridge /api/progress/stream` (SSE). | `progress_trace.rs`, `kalma2-bridge/src/main.rs` |
| Smoke | `SddIA/scripts/qa/test-sddia-installer.sh` (dry-run, live-gate, abort forja) + job CI `sddia-installer-smoke`. | QA |
| Helpers shell | `sddia_shell_lib.sh` (`_sddia_load_vault`, `_sddia_stop_lock_pid`, …). | `SddIA/scripts/common/` |

### 2.3 Brechas reales

| ID | Brecha | Detalle medido |
|----|--------|----------------|
| G1 | **Sin contrato de salida en modo real** | En ejecución no-dry-run, stdout mezcla: JSON de `execute-process` (`instance-creator`, `route-domain-event`), stderr passthru de `cargo` (cientos de líneas) y líneas `[installer] …`. No hay envelope final. El JSON del plan `--dry-run` no cumple `capsule-json-io` (sin `meta/success/exitCode`). |
| G2 | **Sin contrato de entrada** | Solo `argv` + `SDDIA_INSTALL_ROOT`. No hay forma de inyectar una petición JSON (stdin / `--request-file` / `SDDIA_CAPSULE_REQUEST`) como exige `capsule-json-io` para ser invocable por `execute-process`, `task-queue-manager` o Kalma2. |
| G3 | **Errores no tipados** | El código de salida es la única señal; el texto `[installer] ERROR: …` es libre. Un consumidor no puede distinguir `ROOT inseguro` de `cargo ausente` (ambos `1`). Fallos de hijos heredan códigos arbitrarios. |
| G4 | **Sin noción de paso** | El motor no declara sus fases; el humano ve un volcado continuo. No hay `k/N`, ni duración por paso, ni resumen. |
| G5 | **Ventana efímera y sin acta local** | Los atajos hacen `exec`; al terminar, el emulador de terminal cierra. `WORK` (`mktemp`) se borra en `EXIT`: no queda log del intento. El único vestigio es el acta de `instance-health-verify` (solo deploy, solo si se llegó a verify). |
| G6 | **Atajos fuera de git y ya divergentes** | `SddIA_Deploy.sh` / `SddIA_Eliminar_Cliente.sh` viven en `~/Aplicaciones/SddIA/` sin plantilla en la forja. Documentación previa cita `SddIA_Teardown.sh` (nombre anterior): deriva ya materializada. |
| G7 | **Borrado destructivo por doble clic** | `SddIA_Eliminar_Cliente.sh` sin argumentos ejecuta `teardown --force` directamente: `rm -rf ROOT` sin confirmación ni vista previa del destino. Cumple I-TEAR-FORCE (consentimiento no interactivo en el motor), pero el atajo *es* la vía interactiva y no pide consentimiento. |
| G8 | **Procesos Core `paciente0-deploy` / `paciente0-undeploy` no forjables** | Ambas deudas (`PBI-DT-PACIENTE0-*`) fijan como candidato un proceso Core que envuelva el installer. Sin G1–G3 resueltas, ese proceso tendría que *scrapear* stderr. |

### 2.4 Precisiones (Filtro A)

| Riesgo de lectura | Corrección |
|-------------------|-----------|
| "Convertir el installer en cápsula Rust" | No es el objetivo de esta historia. El motor bash es Delivery aceptado por `sddia-installer-contract`; `tools-contract` §2 separa Interfaz (Core) de Delivery (workspace). Lo que se contrata es la **interfaz** (petición/resultado/feedback). La migración a Rust es decisión residual (§7 D5). |
| "Hacer interactivo el motor" | Prohibido: I-DEP-*/I-TEAR-* exigen cadena sin prompts. Toda interacción (confirmación, tecla final) vive en una capa **presentadora** separada, que invoca al motor con flags no interactivos. |
| "Reutilizar el plan `--dry-run` como salida" | Se **conserva** como `result.plan` (compatibilidad con `test-sddia-installer.sh` vía `--output plan-legacy` o equivalente, §7 D3) y se envuelve en el envelope. |
| "El resumen debe mostrar el log completo" | El Vértice pide *sin entrar en detalle*: el resumen muestra estado por paso y punteros (acta, log); el detalle queda en fichero. |

## 3. Tesis arquitectónica

Tres capas con responsabilidades disjuntas y un único formato de intercambio:

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ PRESENTADOR (UX humana)   SddIA/scripts/installer/sddia-installer-ui.sh      │
│   • lee progreso JSONL del motor/fachada                                     │
│   • pinta "Paso k/N — Título" (sin detalle), estado ✓/✗/omitido              │
│   • confirmación de teardown (solo TTY), resumen final, espera tecla         │
│   • invocado por los atajos ~/Aplicaciones/SddIA/*.sh (generados desde forja)│
└──────────────▲────────────────────────────────────────────────┬──────────────┘
               │ progreso (fd 3 / JSONL)                        │ request JSON
┌──────────────┴────────────────────────────────────────────────▼──────────────┐
│ FACHADA  ./sddia-installer.sh                                                │
│   • normaliza argv → request (o acepta request JSON directo)                 │
│   • MOTOR + verify + eventos, cada uno como paso declarado                   │
│   • stdout: UN envelope capsule-json-io 2.0 (entityKind tool, sddia-installer)│
└──────────────▲────────────────────────────────────────────────┬──────────────┘
               │ progreso JSONL                                 │ request JSON
┌──────────────┴────────────────────────────────────────────────▼──────────────┐
│ MOTOR  SddIA/scripts/sddia-installer.sh (Ceguera de Ejecución intacta)       │
│   • pasos declarados (STEP begin/end) con id estable                         │
│   • hijos ruidosos (cargo, execute-process) → log de intento; JSON hijo → result│
│   • stdout: UN envelope; stderr: solo diagnóstico humano; errores tipados    │
└──────────────────────────────────────────────────────────────────────────────┘
```

Principios:

1. **Un envelope, un stdout.** El motor y la fachada emiten exactamente un objeto JSON en stdout (`capsule-json-io` 2.0). Nada más toca stdout. Los hijos se redirigen al log del intento; su JSON (plan de `instance-creator`, `verdict` de verify, `event_id` de eventos) se **captura e incorpora** a `result`.
2. **Progreso como canal separado.** Los pasos se emiten como JSONL por un canal distinto de stdout (fd 3 si está abierto; si no, stderr con prefijo fijo). Formato alineado con PTC (`timestamp, phase, severity, message, metadata`) para que en el futuro Kalma2 los sirva por `/api/progress/stream` sin traducción.
3. **Errores tipados.** Cada `_die` lleva `error.code` estable (enumerado en el contrato) y el `exitCode` que ya define el contrato. Fallo de hijo → `error.code = STEP_FAILED`, `error.step`, `error.child_exit`.
4. **Interactividad solo en el presentador.** Motor y fachada permanecen sin prompts. Confirmación de borrado y "pulsa una tecla" viven en la capa UX y se desactivan cuando no hay TTY.
5. **Atajos versionados.** Los ejecutables de `~/Aplicaciones/SddIA/` se generan desde plantillas de la forja; un comando los (re)instala. El nombre y el comportamiento dejan de derivar a mano.

## 4. Contratos propuestos (borrador para Dédalo)

### 4.1 Contrato de entrada — `request`

Vías de entrada (precedencia: `--request-file` > stdin JSON > `SDDIA_CAPSULE_REQUEST` > `argv`). Los flags actuales se **conservan** y se normalizan a este objeto; ningún flag existente cambia de semántica.

```json
{
  "meta": { "schemaVersion": "2.0", "entityKind": "tool", "entityId": "sddia-installer" },
  "request": {
    "command": "deploy",
    "root": "/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA",
    "vault": null,
    "codex": null,
    "force": false,
    "skip_build": false,
    "dry_run": false,
    "allow_shared_mailbox": false,
    "verify": true,
    "correlation_id": null,
    "progress": "auto",
    "log_dir": null
  }
}
```

| Campo | Tipo | Regla |
|-------|------|-------|
| `command` | `deploy` \| `teardown` | obligatorio |
| `root` | string | opcional; misma precedencia actual (`--root` > `SDDIA_INSTALL_ROOT` > last-resort) |
| `vault`, `codex` | string | solo `deploy`; semántica de `--vault` / `--codex` |
| `force`, `skip_build`, `dry_run`, `allow_shared_mailbox` | bool | semántica de los flags homónimos |
| `verify` | bool | fachada; `false` ≡ `--no-verify` |
| `correlation_id` | uuid | opcional; si viene, el progreso se duplica como PTC en `directories.progress/{correlation_id}/` |
| `progress` | `auto` \| `fd3` \| `stderr` \| `none` | canal del JSONL de pasos |
| `log_dir` | string | opcional; por defecto ruta Cúmulo nueva `instance.installer_logs` (§7 D2) |

Ningún campo del `request` transporta secretos; la bóveda sigue entrando por ficheros (`vault`) y entorno.

### 4.2 Contrato de salida — envelope

```json
{
  "meta": { "schemaVersion": "2.0", "entityKind": "tool", "entityId": "sddia-installer" },
  "success": true,
  "exitCode": 0,
  "message": "deploy APTO en /home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA (WUI 8766)",
  "durationMs": 184233,
  "feedback": [
    { "phase": "validate_host", "level": "info", "timestamp": "2026-09-26T15:40:01Z", "message": "Validar host", "status": "ok", "durationMs": 120 }
  ],
  "result": {
    "command": "deploy",
    "root": "/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA",
    "esc": "home-racso-Aplicaciones-Asistencia_Tormentosa_SddIA",
    "profile": "full-node",
    "dry_run": false,
    "plan": { "…campos actuales de emit_plan…": "…" },
    "steps": [
      { "id": "validate_host", "index": 1, "total": 11, "title": "Validar host", "status": "ok", "durationMs": 120 },
      { "id": "teardown_previous", "index": 3, "total": 11, "title": "Retirar instancia previa", "status": "skipped", "reason": "not_live" }
    ],
    "units": {
      "enabled": ["sddia-event-watcher@…", "sddia-kalma2-bridge@…"],
      "skipped": [ { "unit": "sddia-telegram-watcher@…", "missing": ["TELEGRAM_BOT_TOKEN", "TELEGRAM_ALLOWED_CHAT_ID"] } ]
    },
    "wui_port": 8766,
    "verify": { "verdict": "APTO", "audit_ref": "docs/audits/instance-deploy-…-20260926T154012Z.md" },
    "events": [ { "event_type": "Instance_Deployed", "event_id": "…" } ],
    "registry": { "ref": "instance.host_registry", "action": "upsert" },
    "log_ref": ".SddIA/logs/installer/deploy-…-20260926T153958Z.log",
    "error": null
  }
}
```

Reglas: `exitCode === 0 ⟺ success`; `feedback[]` = proyección de `steps[]` en el formato de `tools-contract` §5; `error` presente ⟺ `success:false`; **cero valores de bóveda** en cualquier campo (solo nombres de clave en `units.skipped[].missing` y `plan.channel_keys_present`).

### 4.3 Errores tipados

| `error.code` | `exitCode` | Origen actual |
|--------------|------------|---------------|
| `INVALID_ARGS` | 1 | `_parse` |
| `UNSAFE_ROOT` | 1 | `resolve_root` (`/`, `$HOME`, forja) |
| `HOST_TOOL_MISSING` | 1 | `validate_host`, `escape_root` |
| `VAULT_SOURCE_MISSING` | 1 | `stage_vault` |
| `ROOT_LIVE_REQUIRES_FORCE` | 2 | live-gate deploy |
| `TEARDOWN_REQUIRES_FORCE` | 3 | teardown sin `--force` |
| `MAILBOX_SHARED` | 4 | `check_shared_mailbox` |
| `VERIFY_NOT_APTO` | 5 | fachada |
| `STEP_FAILED` | **6** (nuevo) | hijo no-cero (`build-release-bundle`, `instance-creator`, `systemctl enable`); `error.step`, `error.child_exit` |
| `REQUEST_INVALID` | **7** (nuevo) | request JSON mal formado / campo desconocido |

Los códigos 1–5 no cambian (CA-REGRESION de v2 intacto).

### 4.4 Canal de progreso (JSONL)

Una línea por transición, sin detalle técnico:

```json
{"kind":"step","timestamp":"2026-09-26T15:40:01Z","id":"build_bundle","index":5,"total":11,"title":"Construir bundle full-node","moment":"begin"}
{"kind":"step","timestamp":"2026-09-26T15:42:37Z","id":"build_bundle","index":5,"total":11,"title":"Construir bundle full-node","moment":"end","status":"ok","durationMs":156000}
```

Pasos declarados (ids estables, títulos legibles, sin rutas ni valores):

| `deploy` | `teardown` |
|----------|------------|
| 1 `validate_host` Validar host | 1 `resolve_target` Comprobar destino |
| 2 `resolve_target` Comprobar destino | 2 `signal_procs` Detener procesos de la instancia |
| 3 `teardown_previous` Retirar instancia previa (omitido si no viva) | 3 `stop_units` Detener y deshabilitar servicios |
| 4 `stage_vault` Preparar bóveda | 4 `clean_residuals` Limpiar residuos |
| 5 `build_bundle` Construir bundle | 5 `registry_remove` Dar de baja en el registro |
| 6 `materialize_instance` Materializar instancia | 6 `remove_templates` Retirar plantillas (si última) |
| 7 `check_mailbox` Comprobar buzón | 7 `wipe_root` Borrar directorio |
| 8 `enable_units` Habilitar servicios | 8 `emit_event` Notificar (fachada) |
| 9 `registry_upsert` Registrar instancia | |
| 10 `verify_health` Verificar salud (fachada) | |
| 11 `emit_event` Notificar (fachada) | |

Con `correlation_id`, cada línea se replica como PTC (`phase` ← `id`, `severity` ← `status`, `source_agent: "sddia-installer"`) en `directories.progress/{correlation_id}/`.

### 4.5 Presentador (UX)

Comportamiento con stdout/stdin TTY:

```text
SddIA — Desplegar cliente
Destino: /home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA

  ✓  1/11  Validar host                    (0.1 s)
  ✓  2/11  Comprobar destino               (0.0 s)
  –  3/11  Retirar instancia previa        omitido
  ✓  4/11  Preparar bóveda                 (0.4 s)
  ⟳  5/11  Construir bundle …

────────────────────────────── Resumen ──────────────────────────────
Resultado        APTO
Puerto WUI       8766
Servicios        4 activos · 3 omitidos (telegram-watcher, email-watcher, iota-publish-relay)
Acta             docs/audits/instance-deploy-…-20260926T154012Z.md
Log completo     .SddIA/logs/installer/deploy-…-20260926T153958Z.log
Duración         3 min 04 s

Pulsa cualquier tecla para cerrar…
```

Reglas:

- **Sin detalle**: nunca se muestra salida de `cargo`, `systemctl` ni JSON. En fallo: línea del paso en rojo + `error.code` + una frase + puntero al log.
- **Teardown**: antes de invocar al motor muestra destino, `esc` y unidades activas (`is_live`) y pide `Escribe ELIMINAR para continuar` (solo TTY; `--yes` lo omite). Tras confirmar, invoca motor con `--force`. El motor no cambia.
- **Espera de tecla**: `read -rsn1` al final, tanto en éxito como en fallo, si stdin es TTY y no se pasó `--no-hold` / `SDDIA_INSTALLER_HOLD=0`. Sin TTY → no espera (CI, cron, Kalma2).
- **Sin TTY** (pipe, `execute-process`): el presentador no interviene; fachada/motor emiten solo el envelope.
- Colores solo si `TERM` lo permite y `NO_COLOR` no está definido.

### 4.6 Atajos generados

Plantillas en `SddIA/scripts/installer/shortcuts/` (`SddIA_Deploy.sh`, `SddIA_Eliminar_Cliente.sh`, opcional `.desktop` con `Terminal=true`) que invocan `sddia-installer-ui.sh deploy|teardown "$@"`. Comando `./sddia-installer.sh shortcuts --dest ~/Aplicaciones/SddIA` los materializa (idempotente, sobrescribe si `sha256` difiere, como `enable_units` con plantillas systemd). Ruta de la forja se inyecta al generar, no se cablea en la plantilla.

## 5. Alcance

### Incluido

- **A. Motor con pasos y envelope**: `STEP begin/end` en `do_deploy` / `do_teardown`; redirección de hijos al log del intento; captura del JSON de `instance-creator`; `_die` con `error.code`; envelope único en stdout; `--dry-run` envuelto (`result.plan`) con modo legado para el smoke (§7 D3).
- **B. Contrato de entrada**: parser `request` (fichero / stdin / env) + normalización de `argv`; validación → `REQUEST_INVALID`.
- **C. Fachada alineada**: verify y eventos como pasos; envelope compuesto; `route-domain-event`/`instance-health-verify` capturados (no a stdout).
- **D. Canal de progreso** JSONL + réplica PTC opcional por `correlation_id`.
- **E. Log persistente del intento** en ruta Cúmulo nueva (`instance.installer_logs`), rotación simple (últimos N).
- **F. Presentador** `sddia-installer-ui.sh`: pasos, confirmación teardown, resumen, espera de tecla, degradación sin TTY.
- **G. Atajos versionados** + comando `shortcuts`.
- **H. Norma** `sddia-installer-contract` **1.1.0 → 1.2.0** (vía `entity-manager`): §Contrato E/S, tabla `error.code`, códigos 6 y 7, canal de progreso, invariante **I-UX-NOPROMPT-MOTOR** (motor/fachada sin prompts; interacción solo en presentador), **I-DEP-SINGLE-STDOUT**.
- **I. QA**: `test-sddia-installer.sh` ampliado (envelope válido, `exitCode⟺success`, error tipado por caso 1/2/3/4, request por stdin equivalente a argv, progreso JSONL bien formado, presentador sin TTY = passthrough).

### Fuera de alcance

- Forjar los procesos Core `paciente0-deploy` / `paciente0-undeploy` (siguen en `pending/`; esta historia los **desbloquea**).
- Migrar el motor a Rust (§7 D5).
- Wizard de onboarding / edición de bóveda (`DT-CONFIG-UX-ONBOARDING`).
- UI gráfica (zenity/GTK); solo terminal.
- Cambiar la semántica de I-DEP-*/I-TEAR-* v1.1.0 ni los códigos 1–5.
- Rollback automático tras `STEP_FAILED` o `VERIFY_NOT_APTO`.

## 6. Fases de implementación (blueprint para Dédalo)

| Fase | Entregable | Cierra |
|------|-----------|--------|
| **F0 — Contrato** | Borrador §4 consolidado en `sddia-installer-contract` 1.2.0 (draft en rama); esquema JSON `installer.request.schema.json` / `installer.result.schema.json` bajo `directories.capability_contracts` o ruta equivalente (§7 D1); clave Cúmulo `instance.installer_logs`. | G2, G3 (normativo) |
| **F1 — Motor** | Pasos declarados, log del intento, hijos silenciados, `_die` tipado, envelope en stdout, `--dry-run` envuelto + modo legado, parser `request`. Smoke verde. | G1, G2, G3, G4 (motor) |
| **F2 — Fachada** | verify/eventos como pasos; captura de JSON hijo; envelope compuesto; código 5/6 coherentes. | G1 (fachada) |
| **F3 — Progreso** | JSONL fd3/stderr; réplica PTC por `correlation_id`; smoke que valida formato. | G4, G8 (precondición) |
| **F4 — Presentador** | `sddia-installer-ui.sh`: render de pasos, confirmación teardown, resumen, espera de tecla, degradación sin TTY. | G5, G7 |
| **F5 — Atajos** | Plantillas + `shortcuts`; regeneración en `~/Aplicaciones/SddIA/`; documentación de operador. | G6 |
| **F6 — Cierre** | `test-sddia-installer.sh` ampliado; CI `sddia-installer-smoke` verde; norma 1.2.0 publicada vía cadena autorizada; `validacion.md` APTO; evolución con `uuid` de esta historia. | — |

## 7. Decisiones (baseline en los PBI; laudo residual)

El detalle adoptado y las dos correcciones están en el PBI de contrato §0 y §2. Resumen: D2, D3, D4, D5 y D6 siguen la opción preferente. D1 se corrige (schemas junto a la norma, no en `capability-contracts`). D7 se aplaza (no se forja entidad `tool`). La norma se parte: **1.2.0** la publica el PBI de contrato; **1.3.0** el de UX.

## 7.1 Tabla original (v1.0.0)

| ID | Decisión | Opción preferente (Tekton) | Alternativa |
|----|----------|----------------------------|-------------|
| D1 | Dónde viven los esquemas JSON del request/result | `directories.capability_contracts` (junto a `doc.closure` etc.), validables por `execute-process` cuando exista el proceso envolvente. | Inline en la norma 1.2.0 sin schema formal. |
| D2 | Ruta del log del intento | Clave Cúmulo `instance.installer_logs` → `{FORGE}/.SddIA/logs/installer/` (hermana de `instance.host_registry`). Retención: últimos 20. | `~/.local/state/sddia/installer/` (XDG, fuera de la forja). |
| D3 | Compatibilidad del `--dry-run` legado | Envelope siempre; `test-sddia-installer.sh` migra a leer `result.plan`. Flag `--output plan-legacy` **no** se añade (menos superficie). | Mantener JSON plano bajo `--output plan-legacy` durante una versión. |
| D4 | Canal de progreso por defecto | `auto`: fd 3 si abierto, si no stderr con prefijo `@sddia-progress `. | Siempre fichero `progress.jsonl` junto al log. |
| D5 | Sustrato del motor | Mantener bash (Delivery contratado; cambio de interfaz, no de motor). Revisar migración a Rust cuando se forje `paciente0-deploy`. | Reescribir motor en Rust ahora (fuera de alcance por coste/riesgo). |
| D6 | Confirmación de teardown en el atajo | Palabra `ELIMINAR` en TTY; `--yes` para saltarla. El motor sigue exigiendo `--force` (I-TEAR-FORCE intacto). | `s/N` simple. |
| D7 | Declarar `sddia-installer` como entidad `tool` (`{name}.md` con `uuid`, `io_mode: capsule-json-io`) | Sí, en F0 vía `tool-creator`, con `implementation_path_ref` al motor bash (tools-contract §2 lo permite: Delivery no normativo). Alinea con `io_mode` de `HU-KALMA2-PROJECT-WORKSPACE-SERVER-1xN` D2. | Posponer hasta que exista el proceso envolvente. |

## 8. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-1 | `./sddia-installer.sh deploy --dry-run --root /tmp/x` escribe en stdout **exactamente un** objeto JSON válido con `meta.entityId=sddia-installer`, `success:true`, `exitCode:0`, `result.plan` con los 14 campos actuales de `emit_plan`. | smoke |
| AC-2 | Mismo `request` por `--request-file`, por stdin y por `argv` equivalentes → `result.plan` idéntico (diff vacío salvo `timestamp`/`durationMs`). | smoke |
| AC-3 | Casos de error 1/2/3/4 actuales devuelven el mismo `exitCode` que en v2 **y** `error.code` de la tabla §4.3; `success:false`. | smoke (matriz) |
| AC-4 | Ejecución real de `deploy` (lab, `--skip-build`, root temporal): stdout contiene un único JSON; ninguna línea de `cargo`, `systemctl` ni JSON de `execute-process` en stdout; todas ellas presentes en `result.log_ref`. | inspección stdout + log |
| AC-5 | Hijo forzado a fallar (`build-release-bundle` con `--profile` inválido en lab) → `exitCode 6`, `error.code=STEP_FAILED`, `error.step=build_bundle`, `error.child_exit` = código real; `steps[]` marca el paso `failed` y los posteriores `not_run`. | smoke |
| AC-6 | Canal de progreso emite `begin`/`end` para cada paso ejecutado, `index`/`total` coherentes, sin rutas absolutas de bóveda ni valores de clave. | smoke (parser JSONL) |
| AC-7 | Con `correlation_id`, aparecen ficheros PTC en `directories.progress/{cid}/` con `source_agent=sddia-installer` y `phase` ∈ ids de §4.4. | inspección |
| AC-8 | `rg` de claves de la lista negra R-VAULT-2 sobre envelope, progreso y log → 0 valores (solo nombres de clave). | test negativo |
| AC-9 | Presentador con TTY: muestra `k/N — título` por paso, resumen final con Resultado/Puerto/Servicios/Acta/Log/Duración, y no termina hasta `read -rsn1`. Con `--no-hold` termina solo. | ejecución manual en `script(1)` / `expect` |
| AC-10 | Presentador sin TTY (`| cat`): no imprime UI ni espera; stdout = envelope de la fachada; `exitCode` propagado. | smoke |
| AC-11 | `SddIA_Eliminar_Cliente.sh` con TTY exige `ELIMINAR`; cualquier otra entrada → `exitCode 3` (`TEARDOWN_REQUIRES_FORCE`) sin invocar al motor; con `--yes` invoca motor con `--force`. | `expect` |
| AC-12 | `./sddia-installer.sh shortcuts --dest /tmp/atajos` genera los dos `.sh` ejecutables; segunda ejecución sin cambios = no-op; plantilla modificada → sobrescritura. | smoke |
| AC-13 | Motor y fachada: `rg -n "read -|select |zenity|whiptail"` = 0 coincidencias (I-UX-NOPROMPT-MOTOR); `rg "curl|ss "` = 0 en motor (CA-CEGUERA de v2 intacto). | grep gate |
| AC-14 | `sddia-installer-contract` 1.2.0 publicada con hash regenerado vía cadena autorizada; códigos 6/7 y `error.code` documentados. | index-integrity |
| AC-15 | `test-sddia-installer.sh` + CI `sddia-installer-smoke` verdes; invariantes I-DEP-*/I-TEAR-* 1.1.0 sin regresión. | CI |

## 9. Riesgos

| Riesgo | Mitigación |
|--------|-----------|
| Silenciar hijos oculta fallos reales de `cargo` / `instance-creator`. | `STEP_FAILED` con `child_exit` + últimas 20 líneas del log en `error.detail_tail` (sin secretos); log persistente siempre. |
| Envelope en stdout rompe consumidores que parseaban el plan plano. | Único consumidor conocido: `test-sddia-installer.sh`; se migra en la misma rama (D3). |
| Confirmación por palabra irrita en uso frecuente. | `--yes` en atajo o alias; el motor no cambia. |
| Presentador falla y bloquea la ventana indefinidamente. | `read` con `-t 600` como red de seguridad; en fallo del propio presentador, imprime envelope crudo y termina. |
| Deriva de nuevo entre atajos y forja. | Atajos generados; `shortcuts` idempotente; documentado en norma 1.2.0. |
| Duplicar formato de progreso con PTC de `execute-process`. | JSONL propio mínimo + réplica PTC literal; no se inventa un tercer esquema. |

## 10. Mandato de ejecución

- Toda mutación de genoma (norma 1.2.0 y 1.3.0, clave Cúmulo `instance.installer_logs`) vía `execute-process` → `entity-manager` / `norm-creator`; prohibida la edición manual. No se forja entidad `tool` (D7 aplazada, §7).
- Motor y fachada conservan Ceguera de Ejecución (sin HTTP/LLM) y ausencia de prompts.
- Cierre documental en rama única (`task-closure-documental`): PBI a `done/`, `validacion.md` APTO.
- Registro en `SddIA/evolution/` vinculando `uuid` de esta historia y de las entidades tocadas.

## 11. Descomposición en PBIs

Dos PBI, dos ramas, dos PR. F6 (cierre documental) va dentro de cada uno, no es un tercer PBI. `forge-pbi` no aplica: exige `project_slug` de un proyecto cliente.

| PBI | Proceso | Fases | AC | Norma |
|-----|---------|-------|----|-------|
| `PBI-ARQUITECTURA-INSTALLER-V3-IO-CONTRACT` | `refactorization` | F0–F3 | AC-1..AC-8, AC-13, AC-14, AC-15 | 1.1.0 → 1.2.0 |
| `PBI-ARQUITECTURA-INSTALLER-V3-UX-EJECUTABLES` | `feature` | F4–F5 | AC-9..AC-12, AC-13, AC-15 | 1.2.0 → 1.3.0 |

El segundo está `blocked_by` el primero. Rutas en `spawned_pbis` del frontmatter.
