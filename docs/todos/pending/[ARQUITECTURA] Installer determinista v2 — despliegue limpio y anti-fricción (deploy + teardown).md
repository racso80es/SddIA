---
document_id: PBI-ARQUITECTURA-INSTALLER-V2-DESPLIEGUE-LIMPIO
uuid: "bb30e934-7f1f-44cb-a51e-21c28ccf426b"
title: "[ARQUITECTURA] Installer determinista v2 — despliegue limpio y anti-fricción (deploy + teardown)"
format: markdown
version: "1.0.0"
status: pending
priority: alta
type: arquitectura
process: feature
dispatch: false
feature_name: sddia-installer-v2-clean-deploy
branch_name: feat/sddia-installer-v2-clean-deploy
persist_ref: docs/features/sddia-installer-v2-clean-deploy
created: "2026-09-26"
author: tekton
audit_ref: docs/audits/installer-deploy-aplicaciones-20260926T113533Z.md
audit_document_id: AUDIT-INSTALLER-DEPLOY-APLICACIONES-20260926T113533Z
pbi_antecesores:
  - document_id: PBI-ARQUITECTURA-DEPLOY-DETERMINISTA
    uuid: "c154bea0-c4c1-457e-b070-f7dfd3bc5f1b"
    pr: "https://github.com/racso80es/SddIA/pull/300"
  - document_id: PBI-ARQUITECTURA-DESTILACION-DEPLOY-DETERMINISTA
    uuid: "d239cb31-a937-448c-941e-3808a3a28874"
installer_contract_ref: SddIA/library/norms/sddia-installer-contract.md
installer_contract_uuid: "b1327ef3-5f07-4fba-9073-a72c5fdf97e2"
installer_contract_version_actual: "1.0.0"
installer_contract_version_objetivo: "1.1.0"
related:
  - docs/todos/pending/[DEUDA] Paciente 0 — prompt y proceso de despliegue.md
  - docs/todos/pending/[DEUDA] Paciente 0 — prompt de teardown.md
  - docs/todos/historias/[ARQUITECTURA] Gobierno de proyectos externos desde Kalma2 — Workspace Server (MCP) 1×N.md
  - SddIA/scripts/sddia-installer.sh
  - SddIA/scripts/build-release-bundle.sh
  - SddIA/process/instance-creator.md
  - SddIA/engine/execute-process/src/engine/handlers/instance_creator.rs
tech_debt_ids:
  - DT-INST-VAULT-STAGE
  - DT-INST-PORT-COLLISION
  - DT-INST-UNITS-CONDITIONAL
  - DT-INST-RESTART-STORM
  - DT-INST-STALE-TEMPLATES
  - DT-INST-POSTDEPLOY-VERIFY
  - DT-INST-DOMAIN-PROFILE
  - DT-INST-MAILBOX-DUP
  - DT-INST-DEPLOY-TRACE
blocks:
  - HU-KALMA2-PROJECT-WORKSPACE-SERVER-1xN (fase F5)
---

# [ARQUITECTURA] Installer determinista v2 — despliegue limpio y anti-fricción

## 0. Filtro A — hechos de partida (no supuestos)

| Hecho medido (auditoría 2026-09-26) | Evidencia |
|-------------------------------------|-----------|
| Instancia `/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA` materializada por `deploy` (PR #300): `MANIFEST 20260925T122059Z`, `full-node`, 34 ELF, `filtro_c:false`, `codex:null`. | `MANIFEST.json` |
| `kalma2-bridge@`, `telegram-watcher@`, `iota-publish-relay@` en `activating/auto-restart` con **15 924 reinicios** cada una en ~25 h. | `systemctl --user show -p NRestarts` |
| `kalma2-bridge`: `bind 8765: Address already in use`; puerto ocupado por bridge de la forja (`cwd=/home/racso/Proyectos/SddIA`). `8766` = SddIA_AP. | `journalctl --user`, `ss -ltnp` |
| `telegram-watcher`: `TELEGRAM_BOT_TOKEN / TELEGRAM_ALLOWED_CHAT_ID no configurados` (exit 2). | `journalctl --user` |
| `iota-publish-relay`: `hijo Node ausente` (`ROOT/.SddIA/services/iota-publish-relay` no existe; `SDDIA_IOTA_RELAY_DIR` no definido). | `journalctl --user` |
| `ROOT/.SddIA/.dev/.env` es copia byte a byte (1898 B) de `{FORGE}/.dev/.env` (bóveda **global**); contiene `SDDIA_EMAIL_IMAP_SECRET`, `GEMINI_API_KEY`, `SDDIA_AGENT_RUNTIME_*` de la forja; **sin** `SDDIA_CLIENT_PORT`, `TELEGRAM_*`, `SDDIA_IOTA_RELAY_DIR`. | `diff` de claves |
| Causa: `stage_vault()` itera `{FORGE}/.dev/.env` → `{FORGE}/.SddIA/.dev/.env` y `return 0` en el primero; lo stagea como `instance.SddIA.dev.env`; `root.dev.env` nunca se stagea. | `SddIA/scripts/sddia-installer.sh` §`stage_vault` |
| `email-watcher@` de Aplicaciones **y** de SddIA_AP sondean el mismo buzón IMAP. | bóvedas de ambas instancias |
| `.SddIA/active-domain-profile.json` ausente en Aplicaciones → perfil `default` (autoridad software por regla legado D4, no por declaración). `materialize_domain_profile` solo escribe para `consumer`. | `instance_creator.rs` §`materialize_domain_profile` |
| `enable_units()` copia plantillas `sddia-*@.service` a `~/.config/systemd/user/` **solo si no existen**; `teardown` no las borra (I-TEAR-SYSTEMD). Una plantilla corregida en bundle nunca sustituye a la instalada. | `sddia-installer.sh` §`enable_units` |
| Plantillas: `Restart=always`, `RestartSec=5`, sin `StartLimitIntervalSec/StartLimitBurst`, sin `ExecCondition`. | `~/.config/systemd/user/sddia-kalma2-bridge@.service` |
| `enable_units()` habilita las 7 unidades si existe el launcher, sin comprobar prerequisitos (claves, servicio Node, puerto). | `sddia-installer.sh` §`enable_units` |
| Ningún proceso verifica post-deploy que el asistente quede operativo en el canal *installer default*; el deploy live de Aplicaciones no tiene acta en `docs/audits` ni evolución hasta la auditoría citada. | I-DEP-CEGUERA; `docs/features/sddia-deterministic-installer/execution.md` («deploy live = fuera de gate») |

**Veredicto de partida:** contrato I-DEP-*/I-TEAR-* v1.0.0 **cumplido**; expectativa del Vértice Biológico («despliegue total del asistente») **no cumplida**. El contrato define "operativo" como cadena sin prompts; no define disponibilidad de canal ni higiene de bóveda.

## 1. Intención

Que `./sddia-installer.sh deploy` produzca, sin prompts y en un host donde ya conviven forja y otras instancias, una instancia **operativa y aislada**: WUI escuchando en un puerto propio, unidades habilitadas solo cuando sus prerequisitos existen, bóveda de instancia **compuesta y saneada** (sin secretos de canal ajenos), perfil de dominio **declarado**, verificación post-deploy con acta, y `teardown` que deje el host en estado idéntico al previo (plantillas incluidas cuando la instancia era la última). Ninguna de las nueve fricciones `DT-INST-*` debe reproducirse en un redeploy.

Principio invariante: el **motor** (`sddia-installer.sh`) mantiene Ceguera de Ejecución (sin HTTP/LLM). La verificación operativa vive en un **paso posterior separado** (proceso Core), invocado por la fachada, no dentro del motor.

## 2. Fricciones → requisitos

### 2.1 Bóveda (DT-INST-VAULT-STAGE, DT-INST-MAILBOX-DUP)

| ID | Requisito |
|----|-----------|
| R-VAULT-1 | `stage_vault` compone **dos** artefactos: `root.dev.env` ← `{FORGE}/.dev/.env` (global, sin secretos de canal) y `instance.SddIA.dev.env` ← `--vault` **o** plantilla `SddIA/scripts/starter-kit/.SddIA/.dev/.env.example` renderizada. Prohibido copiar `{FORGE}/.SddIA/.dev/.env` (instancia de forja) sin `--vault` explícito que la señale. |
| R-VAULT-2 | Lista negra de claves de canal que **nunca** se heredan implícitamente desde forja: `SDDIA_EMAIL_IMAP_*`, `TELEGRAM_BOT_TOKEN`, `TELEGRAM_ALLOWED_CHAT_ID`, `IOTA_WALLET_SECRET`, `IOTA_ANCHOR_PACKAGE_ID`, `GEMINI_API_KEY`, `CURSOR_API_KEY`. Se heredan solo vía `--vault`. |
| R-VAULT-3 | `--dry-run` imprime en el plan JSON: `vault_root_source`, `vault_instance_source`, `channel_keys_present` (booleanos por clave, **sin valores**). |
| R-VAULT-4 | Dos instancias con el mismo `SDDIA_EMAIL_IMAP_USER`+`HOST` en el host → `deploy` aborta con exit **4** salvo `--allow-shared-mailbox`. Detección por lectura de claves de bóvedas de instancias registradas en el host (§2.6), sin loguear valores. |

### 2.2 Puerto WUI (DT-INST-PORT-COLLISION)

| ID | Requisito |
|----|-----------|
| R-PORT-1 | `deploy` escribe `SDDIA_CLIENT_PORT` en `instance.SddIA.dev.env` si no viene en `--vault`. Asignación **determinista y sin red**: `8765 + (índice de instancia en el registro host §2.6)`, saltando puertos ya declarados por otras instancias registradas y por la forja (`8765` reservado a forja). Sin `ss`/`curl` en el motor (I-DEP-CEGUERA). |
| R-PORT-2 | `kalma2-bridge` (crate `SddIA/interfaces/kalma2-bridge`) distingue `bind` fallido con exit **code 78** (`EX_CONFIG`) en lugar de `1`, para que systemd pueda aplicar `RestartPreventExitStatus=78`. |
| R-PORT-3 | Plan `--dry-run` incluye `wui_port` y `port_source` (`vault` | `derived`). |

### 2.3 Unidades systemd (DT-INST-UNITS-CONDITIONAL, DT-INST-RESTART-STORM, DT-INST-STALE-TEMPLATES)

| ID | Requisito |
|----|-----------|
| R-SYS-1 | `enable_units` habilita una unidad solo si **launcher existe y prerequisitos declarados existen**. Tabla de prerequisitos (SSOT en `SddIA/daemons/{name}.md` o `daemons-contract`): `telegram-watcher` → `TELEGRAM_BOT_TOKEN`+`TELEGRAM_ALLOWED_CHAT_ID`; `email-watcher` → `SDDIA_EMAIL_IMAP_HOST`+`USER`+`SECRET`; `iota-publish-relay` → `SDDIA_IOTA_RELAY_DIR` o `ROOT/.SddIA/services/iota-publish-relay/`; `kalma2-bridge`, `event-watcher`, `event-sweeper`, `github-bridge-watcher` → sin prerequisitos de clave. Unidad no habilitada → línea `[installer] skip sddia-X@ESC (missing: KEY,…)` (nombres de clave, nunca valores). |
| R-SYS-2 | Plantillas incorporan `ExecCondition=` (script `SddIA/scripts/daemons/_precondition.sh NAME`) que devuelve 1 si faltan prerequisitos, de modo que una habilitación manual posterior tampoco entre en bucle. |
| R-SYS-3 | Plantillas: `StartLimitIntervalSec=300`, `StartLimitBurst=5`, `Restart=on-failure`, `RestartPreventExitStatus=2 78`. Objetivo: cero unidades con `NRestarts > 5` en 24 h. |
| R-SYS-4 | `enable_units` **sobrescribe** la plantilla en `~/.config/systemd/user/` si el `sha256` del bundle difiere del instalado, seguido de `daemon-reload`. Prohibido dejar plantilla fósil. |
| R-SYS-5 | `teardown` elimina las plantillas `sddia-*@.service` **solo si** ninguna otra instancia `@*` del host las usa (`systemctl --user list-units 'sddia-*@*'` vacío tras el stop). Mantiene I-TEAR-SYSTEMD (no rm si hay otras instancias). |

### 2.4 Perfil de dominio (DT-INST-DOMAIN-PROFILE)

| ID | Requisito |
|----|-----------|
| R-PROF-1 | `instance-creator` materializa `.SddIA/active-domain-profile.json` **para todo perfil**: `consumer` → `{codex_slug: <MANIFEST.codex>, git_required:false}` (actual); `engineering` → `{codex_slug: "codex-software-engineering", git_required: true}` salvo `inputs.codex_slug`. `reason: not-consumer` deja de existir. |
| R-PROF-2 | `deploy` acepta `--codex SLUG` que fija `inputs.codex_slug` y `build-release-bundle --codex`. |

### 2.5 Verificación post-deploy y trazabilidad (DT-INST-POSTDEPLOY-VERIFY, DT-INST-DEPLOY-TRACE)

| ID | Requisito |
|----|-----------|
| R-VER-1 | Proceso Core **`instance-health-verify`** (forjado vía `process-creator`; reutiliza `system-vitality-probe`/`daemon-heartbeat-audit` si cubren): inputs `instance_root`; comprueba `systemctl is-active` de las unidades **habilitadas**, `NRestarts ≤ 1`, `/api/status` de la WUI en `SDDIA_CLIENT_PORT` (aquí sí HTTP: es proceso, no motor), presencia de `active-domain-profile.json`, `vault_env_present`. Output: acta `docs/audits/instance-deploy-{ESC}-{ts}.md` en la **forja** con `verdict: APTO|NO-APTO` y tabla de unidades. |
| R-VER-2 | La fachada `./sddia-installer.sh deploy` invoca `instance-health-verify` tras `enable_units` salvo `--no-verify`; el motor no lo hace. Exit de la fachada = 0 solo con `APTO`; `NO-APTO` → exit **5** y la instancia queda desplegada (no rollback automático). |
| R-VER-3 | `deploy` emite `Instance_Deployed` y `teardown` emite `Instance_Torn_Down` al bus de la forja (`event-creator`), payload: `root`, `esc`, `profile`, `manifest_created_at`, `verdict` (deploy). Suscriptor Cúmulo → anclaje DLT (mismo patrón que `Kalma2_Process_Requested`). |

### 2.6 Registro de instancias del host

| ID | Requisito |
|----|-----------|
| R-REG-1 | Fichero `{FORGE}/.SddIA/instances.json` (clave Cúmulo nueva `instance.host_registry`): `{ "<ESC>": { root, profile, wui_port, mailbox_fingerprint (sha256 de HOST+USER), created_at } }`. `deploy` añade/actualiza; `teardown` elimina. Base para R-PORT-1, R-VAULT-4 y R-SYS-5. Sin secretos. |

## 3. Plan de ejecución (DA-2 — genoma vía cadena `entity-manager`)

1. **Init** `feature` `sddia-installer-v2-clean-deploy` con este PBI como `pbi_ref`. Raw Kernel; relé IDE.
2. **Dédalo**: decidir ubicación de `_precondition.sh` (¿`SddIA/scripts/daemons/` o `daemons-contract` con campo `preconditions`?), forma de `instances.json` y si `instance-health-verify` es proceso nuevo o extensión de `system-vitality-probe`.
3. **Motor** (`sddia-installer.sh`): R-VAULT-1..3, R-PORT-1/3, R-SYS-1/4/5, R-REG-1, exit 4/5. Actualizar `test-sddia-installer.sh` (dry-run debe cubrir: bóveda compuesta, puerto derivado, skip por prerequisito, plantilla sobrescrita, teardown última instancia).
4. **Plantillas systemd** (`SddIA/infrastructure` / fábrica de unidades): R-SYS-2/3.
5. **`instance-creator`** (`instance_creator.rs` + `instance-creator.md` bump): R-PROF-1/2.
6. **`kalma2-bridge`**: R-PORT-2 (exit 78 en bind).
7. **Procesos/eventos**: `instance-health-verify` (`process-creator`), `Instance_Deployed` / `Instance_Torn_Down` (`event-creator`), suscripción Cúmulo, `eda-coverage`.
8. **Norma**: `sddia-installer-contract` → **1.1.0** (`entity-manager` update): tabla de códigos 4 (buzón compartido) y 5 (verify NO-APTO); I-DEP-VAULT, I-DEP-PORT, I-DEP-COND, I-DEP-VERIFY-FACADE; I-TEAR-TEMPLATES-LAST; sección «Registro host».
9. **Laudo operativo previo al redeploy real de Aplicaciones** (fuera del PR, requiere Vértice Biológico): `teardown --force` de la instancia actual en bucle, o `stop/disable` de las 3 unidades. No se ejecuta dentro del ciclo `feature`.
10. **Cierre**: un PR; `validacion.md` APTO; PBI a `done/`; evolución con `uuid` de este PBI.

## 4. Criterios de aceptación

* [ ] **CA-VAULT-COMPOSE:** `deploy --dry-run` en host con forja poblada muestra `vault_root_source={FORGE}/.dev/.env`, `vault_instance_source=starter-kit|--vault`, y `channel_keys_present` todo `false` sin `--vault`. Test unitario: bóveda de instancia resultante **no** contiene ninguna clave de la lista negra R-VAULT-2 salvo vía `--vault`.
* [ ] **CA-VAULT-NOSHARE:** dos instancias con mismo `mailbox_fingerprint` → exit 4; con `--allow-shared-mailbox` → continúa.
* [ ] **CA-PORT:** con forja y otra instancia registrada, `deploy --dry-run` deriva `wui_port ∉ {8765, puertos registrados}`; la bóveda resultante contiene `SDDIA_CLIENT_PORT` igual al plan.
* [ ] **CA-BIND-EXIT:** `kalma2-bridge` con puerto ocupado → exit 78; unidad no reinicia (`RestartPreventExitStatus`).
* [ ] **CA-COND:** sin `TELEGRAM_*` en bóveda, `deploy` **no** habilita `telegram-watcher@`; log `skip … (missing: TELEGRAM_BOT_TOKEN,TELEGRAM_ALLOWED_CHAT_ID)`; sin relay Node, no habilita `iota-publish-relay@`. `systemctl --user enable --now` manual de esa unidad → `ExecCondition` falla, `NRestarts=0`.
* [ ] **CA-STORM:** en cualquier despliegue de test, ninguna unidad `@ESC` supera `NRestarts=5` en la ventana `StartLimitIntervalSec`.
* [ ] **CA-TEMPLATE-FRESH:** plantilla modificada en bundle → `deploy` sobrescribe la de `~/.config/systemd/user/` (sha distinto) y hace `daemon-reload`.
* [ ] **CA-TEMPLATE-LAST:** `teardown` de la última instancia elimina plantillas `sddia-*@.service`; con otra instancia viva, las conserva.
* [ ] **CA-PROFILE:** instancia `engineering` termina con `.SddIA/active-domain-profile.json` = `codex-software-engineering`, `git_required:true`; `execute-process --process feature` **no** depende de la regla legado D4 (test con perfil explícito).
* [ ] **CA-VERIFY:** fachada `deploy` sin `--no-verify` produce acta `docs/audits/instance-deploy-{ESC}-{ts}.md`; con unidad habilitada inactiva → `NO-APTO`, exit 5.
* [ ] **CA-EVENTS:** `Instance_Deployed` / `Instance_Torn_Down` en `eda-coverage.json` con suscriptor; smoke `route-domain-event` sin `no_subscriber`.
* [ ] **CA-REGISTRY:** `instances.json` refleja alta en deploy y baja en teardown; nunca contiene valores de claves de bóveda.
* [ ] **CA-CEGUERA:** motor `sddia-installer.sh` sigue sin `curl`/`ss`/LLM (`rg` = 0); HTTP solo en `instance-health-verify`.
* [ ] **CA-NORM:** `sddia-installer-contract` 1.1.0 con hash regenerado vía cadena autorizada; códigos 4 y 5 documentados.
* [ ] **CA-REGRESION:** `test-sddia-installer.sh` y job CI `sddia-installer-smoke` verdes; I-DEP-*/I-TEAR-* 1.0.0 intactos (exit 1/2/3, abort forja).
* [ ] **CA-REDEPLOY-REAL (post-merge, laudo):** redeploy de `/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA` → acta `APTO`, WUI en puerto derivado respondiendo `/api/status`, `NRestarts=0` en todas las unidades habilitadas, `email-watcher@` **no** habilitado sin `--vault` con IMAP propio.

## 5. Fuera de alcance

- Rollback automático tras verify `NO-APTO` (queda instancia desplegada + acta).
- Wizard interactivo de onboarding (`DT-CONFIG-UX-ONBOARDING`, Paciente 0).
- Forja de `paciente0-deploy` / `paciente0-undeploy` (deudas Paciente 0 siguen en `pending/`).
- Cambiar perfil `consumer` de SddIA_AP o su bóveda.
- Reserva dinámica de puertos por sondeo de red en el motor.
- Multi-usuario systemd (`--system`).

## 6. Riesgos

| Riesgo | Mitigación |
|--------|-----------|
| Romper redeploy de SddIA_AP (canal Paciente 0, `runtime_profile:consumer`). | R-PROF-1 preserva rama consumer; CA-REGRESION incluye dry-run con `--vault` estilo Paciente 0. |
| `ExecCondition` oculta fallos reales de configuración. | Log de `skip` con nombres de clave + acta de verify listando unidades no habilitadas. |
| Registro host desincronizado si se borra ROOT a mano. | `deploy` reconcilia: entrada cuyo `root` no existe → se elimina antes de derivar puerto. |
| Sobrescribir plantillas afecta a instancias vivas que las comparten. | `daemon-reload` no reinicia servicios; cambio efectivo en siguiente restart. Documentar en contrato. |

## 7. Dependencias

- Bloquea F5 de `HU-KALMA2-PROJECT-WORKSPACE-SERVER-1xN` (instancia forjadora saneada).
- Requiere laudo del Vértice Biológico para la acción operativa §3.9 sobre la instancia actual en bucle.
