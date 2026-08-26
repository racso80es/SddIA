---
document_id: PBI-KAIZEN-AISLAMIENTO-MULTI-INSTANCIA
uuid: "b5d19318-a0fd-440b-9aac-8c6d93f775ed"
title: "[KAIZEN] Aislamiento multi-instancia de centinelas (forja + clientes coexistentes)"
format: markdown
version: "1.0.0"
status: done
type: kaizen
priority: alta
process: feature
persist_ref: docs/features/kaizen-aislamiento-multi-instancia
created: "2026-08-26"
updated: "2026-08-26"
pbi_archived: true
derived_from:
  - PBI-KAIZEN-IGNICION-SOBERANA
  - PBI-DT-PACIENTE0-DEPLOY-PROCESS
  - AUDIT-PACIENTE0-CENTINELAS-EMAIL-SORDO-20260826
antecesor_persist_ref: docs/features/kaizen-ignicion-soberana-centinelas
antecesor_audit_ref: docs/audits/paciente0-centinelas-email-sordo-20260826.md
lock_session_audit_ref: docs/audits/paciente0-lock-session-centinelas-20260825.md
instance_path: /home/racso/Proyectos/SddIA_AP
forge_path: /home/racso/Proyectos/SddIA
wui_ap: 8766
wui_forge: 8765
friction_ids:
  - F-SYS-02
  - F-DEP-10
  - F-CEN-PKILL
  - R-07
tech_debt_ids:
  - DT-SYSTEMD-USER-TEMPLATE-UNIVERSAL
  - DT-LAUNCHER-INSTANCE-ROOT
  - DT-CEN-NO-GLOBAL-PKILL
blocks_on: []
---

# [KAIZEN] Aislamiento multi-instancia de centinelas

## 0. Contexto

Empiría **2026-08-26** (host con forja `SddIA` + Paciente 0 `SddIA_AP` + linger): unidades `@%f` de AP `enabled`, pero el user manager ejecuta lanzadores/ELF de la **forja**. Inbox WUI `:8766` ≡ `:8765` (mismos bytes). Correo a la cuenta Paciente 0 no reacciona en el bus de AP. SSOT: `docs/audits/paciente0-centinelas-email-sordo-20260826.md`.

El Kaizen `kaizen-ignicion-soberana-centinelas` (PR #191) absorbió **unidades nombradas** `sddia-{daemon}@%f` y `WorkingDirectory=%f`. **No** absorbió: (1) plantilla user compartida con `ExecStart` absoluto de `@@SDDIA_CORE_ROOT@@`; (2) wrappers que abandonan el cwd; (3) `pkill -x` al arrancar. AC-IC del antecesor queda **APTO en disco de instancia** y **NO APTO en runtime user** cuando coexisten dos raíces.

Objetivo de este PBI: **independencia de centinelas entre raíces** (forja, Paciente 0, N clientes) en el mismo `user.slice`. No es «Paciente 0 en solitario». No reabre F-TRIAGE ni G5 IMAP como default.

Init lab (cuando se abra el ciclo): `./sddia-run.sh --process feature` + skips archive/delivery + pin `SDDIA_EXECUTE_PROCESS_BIN` release. `feature_name`: `kaizen-aislamiento-multi-instancia`. Rama: `feat/kaizen-aislamiento-multi-instancia`. `persist_ref` reservado arriba. Mutación de genoma: `entity-manager` donde aplique (plantillas systemd, `instance-creator`, scripts; **no** forjar `paciente0-deploy`).

---

## 0bis. Adecuación del estímulo (anti-alucinación)

Texto del operador vs genoma + auditoría. Lo que **no** se copia literal.

| Afirmación del estímulo | Veredicto | Corrección |
|-------------------------|-----------|------------|
| `%f` «ruta escapada» pasada en `enable` | **Inexacto** | `enable` usa el **especificador de instancia** (`%i` = nombre escapado, p. ej. `home-racso-Proyectos-SddIA_AP`). `%f` es el **path sin escapar** (típicamente absoluto) que systemd reconstruye desde ese `%i` (`systemd-escape -p`). `WorkingDirectory=%f` ya es correcto en plantillas vigentes. |
| Un `ExecStart=%f/SddIA/scripts/daemons/@@DAEMON_NAME@@.sh` para todos | **Inexacto** | Fábrica `sddia-daemon@.service.template`: sí, `scripts/daemons/@@DAEMON_NAME@@.sh` (event-watcher, sweeper, kalma2-bridge, telegram, github). **email-watcher** es `SddIA/daemons/email-watcher.sh` (plantilla aparte). No unificar a `scripts/daemons/email-watcher.sh` sin mover el lanzador. |
| Solo `instance-creator` pisa plantillas user | **Incompleto** | El overwrite empírico es `start-sddia.sh` → `~/.config/systemd/user/` (F-SYS-01/02). `instance-creator` **renderiza** `@@SDDIA_CORE_ROOT@@` a path absoluto en `{instancia}/.SddIA/systemd/`. Hay que cambiar **fábrica + creator + sync de ignición**. Con `ExecStart` basado en `%f`, copiar el mismo molde a home deja de ser destructivo (contenido idéntico). |
| `$PWD` como «verdad absoluta» si systemd lanza | **Peligroso si es el único oráculo** | Tras `WorkingDirectory=%f`, `$PWD` coincide con la instancia **si** el wrapper no hace `cd` previo. Un lanzador invocado a mano desde otro cwd se equivocaría. Jerarquía: `SDDIA_INSTANCE_ROOT` (si absoluto y existe) → cwd systemd (`PWD` solo si el proceso aún no cambió de directorio) → `SCRIPT_DIR` como fallback **lab**. |
| `_run_daemon.sh` es el único `pkill -x` | **Incompleto** | También `start-sddia.sh` `cleanup()` líneas `pkill -x` (event-watcher, kalma2-bridge, email-watcher, iota). La rama `DAEMON_JURIS=systemd` **ya no** pkill al SIGINT; el fratricidio vivo es `_stop_previous` **en cada arranque** del wrapper. |
| Lock CEN-01 es solo `pkill` | **Incompleto** | `sddia-daemon-runtime` aborta duplicado por `.lock` de **esa** `REPO_ROOT`. Con F-DEP-10 sin curar, AP y forja comparten lock de forja (`lock activo pid=1858`). Curar F-DEP-10 separa locks; aun así hay que quitar `pkill -x`. |
| «Independencia absoluta» = no coexistir con lab | **Fuera** | R-07 (lab `email-watcher@…SddIA` no `active` si el host es sensorial del cliente) es **criterio operativo**, no sustituye F-SYS-02/10/PKILL. Lab y cliente **pueden** coexistir si cada uno tiene `REPO_ROOT`, ELF, bus y lock propios. |
| IMAP AUTH de forja = fallo de cuenta Paciente 0 | **Falso** (audit §0) | IMAP_USER forja ≠ AP (huellas distintas). HOST/MAILBOX iguales. Primera causa = identidad de proceso, no secretos AP ausentes. |
| Lock-session / linger | **Fuera de este PBI** | Audit 2026-08-25: lock no para unidades. No mezclar con F-SYS-02. |

---

## 0ter. Snapshot empírico (no repetir la auditoría)

| Hecho | Valor |
|-------|--------|
| `FragmentPath` user | un archivo por stem; `ExecStart` forja |
| Render AP en disco | `{SddIA_AP}/.SddIA/systemd/` con `ExecStart` AP; **no** instalado |
| cwd PIDs 1858/1883/1888/2883 | `/home/racso/Proyectos/SddIA` |
| `event-watcher@AP` | `Repo: …/SddIA`; NRestarts ~96 |
| Inbox GET | `:8766` ≡ `:8765` |
| Lock forja | pid del proceso de la unidad **etiquetada** AP |
| Lock AP | pid fósil 81312 (2026-08-25) |

---

## 1. Fricciones

| ID | Síntoma | Causa raíz | Ad-hoc (no SSOT) | Acción Kaizen |
|----|---------|------------|------------------|---------------|
| **F-SYS-02** | `enable @escape(AP)` usa `ExecStart` de la forja | Plantilla user única; `@@SDDIA_CORE_ROOT@@` horneado al último `cp`/`render` | Re-copiar unidades desde `{AP}/.SddIA/systemd/` | **DT-SYSTEMD-USER-TEMPLATE-UNIVERSAL:** `ExecStart=%f/SddIA/…` (path por familia de lanzador); dejar de sustituir CORE_ROOT absoluto en ExecStart |
| **F-DEP-10** | Log `Repo: forja` con unidad `@AP`; ELF `target/debug` forja | `email-watcher.sh`, `_run_daemon.sh`, `kalma2-bridge.sh`: `REPO_ROOT` desde `SCRIPT_DIR` + `cd` | Ninguno canónico | **DT-LAUNCHER-INSTANCE-ROOT:** resolver raíz de instancia (env / cwd systemd); lock y `.events` bajo esa raíz |
| **F-CEN-PKILL** | SIGTERM cruzado; auto-restart AP y forja | `_stop_previous`: `pkill -x "$DAEMON"` | No | **DT-CEN-NO-GLOBAL-PKILL:** matar solo PID del lock de **esa** raíz; jurisdicción systemd: no pkill (cgroup / `KillMode`) |
| **R-07** | Lab email `active` junto al cliente | Política sensorial; no es la plantilla `%f` | `stop`/`disable` `@…SddIA` | Criterio de **ensayo** Paciente 0; no es la forja de plantillas |

Residual colateral (no bloquea el diseño `%f`, sí el ELF del cliente): `email-watcher.sh` prefiere `target/debug` a `release` (analogía F-DEP-07). Absorber en el mismo ciclo de lanzadores o dejar DT-ORCHESTRATOR-DEBUG-FIRST explícito en wrappers.

### 1.1 Esperado vs observado (2026-08-26)

| Circuito | Esperado post-ignición soberana | Observado |
|----------|---------------------------------|-----------|
| `ExecStart` `@AP` | wrapper bajo `SddIA_AP` o `%f` | wrapper **forja** |
| Bus `@AP` | `{AP}/.events/…` | `{forja}/.events/…` |
| WUI `:8766` | datos AP | bytes = `:8765` |
| Arranque segundo `event-watcher` | no mata al primero de otra raíz | `pkill -x` |

---

## 2. Qué **no** reabre este ciclo

- F-DEP-01…04 (handler CORE_ROOT / overlay) salvo que el render vuelva a hornear absoluto.
- F-DEP-07/08/09 del redeploy 20260825 (orquestador `execute-process`), salvo el analog debug-first de **wrappers** de centinela.
- AC-REBOOT linger / lock-session.
- Wizard `DT-CONFIG-UX-ONBOARDING`.
- Forjar `paciente0-deploy` / `paciente0-undeploy`.

---

## 3. Objetivos

1. **Molde user universal:** `ExecStart` no contiene path de host horneado. `%f` = raíz de instancia. Email y fábrica daemon conservan **rutas relativas distintas** bajo `%f`.
2. **Lanzadores ciegos a la ubicación del script** cuando hay instancia: bus, lock, ELF bajo esa raíz.
3. **Cero `pkill -x` por nombre de binario** en arranque/parada de centinelas. Parada = PID de lock de esa raíz y/o systemd.
4. Ensayo: forja + `SddIA_AP` (o dos raíces) en el mismo uid: dos WUI, dos buses, dos IMAP (si aplica), `pkill` de uno no tumba al otro.

---

## 4. Criterios de cierre

### Motor

- [ ] Plantillas `sddia-daemon@.service.template` y `sddia-email-watcher@.service.template`: `ExecStart=%f/SddIA/scripts/daemons/@@DAEMON_NAME@@.sh` y `ExecStart=%f/SddIA/daemons/email-watcher.sh` respectivamente. `instance-creator` **no** reemplaza ExecStart con `instance_root` absoluto.
- [ ] Tras `cp` a `~/.config/systemd/user/` desde forja **o** desde AP, `systemctl --user show sddia-event-watcher@${ESC_AP} -p ExecStart` resuelve binario/wrapper **bajo AP** (`%f` expandido).
- [ ] Lanzadores: con `WorkingDirectory` AP y `ExecStart` forja (regresión de plantilla), **o** con ambos AP, `REPO_ROOT` efectivo = AP. Test: no `cd` a `SCRIPT_DIR/../..` si `SDDIA_INSTANCE_ROOT` o cwd instancia es válido.
- [ ] `_run_daemon.sh` `_stop_previous`: **0** `pkill -x`. Solo PID leído del lock de esa raíz (si vivo). Jurisdicción systemd: skip stop-by-name.
- [ ] `start-sddia.sh` `cleanup` script: mismo veto de `pkill -x` global; si se conserva parada lab, acotar a PIDs de `$REPO_ROOT`.
- [ ] Ensayo dos raíces: `pgrep -a event-watcher` muestra cwd/exe distintos; NRestarts de AP no se disparan al `restart` de la forja.

### Operador / R-07

- [ ] Ensayo Paciente 0: lab `@…SddIA` email/telegram/github según política R-07 documentada (coexistencia OK si aislados; no `active` lab IMAP si se declara host=cliente).

### Fuera

- G5 reunión IMAP.
- Namespacing de nombres de unidad (`sddia-event-watcher-ap@.service`) — innecesario si `%f` + lanzadores + no-pkill bastan.

---

## 5. Orden de forja

```text
(1) F-SYS-02  plantillas %f + instance-creator sin hornear CORE_ROOT en ExecStart
    tests handler: ExecStart contiene %f, no /home/...
(2) F-DEP-10  resolución REPO_ROOT compartida (sddia_shell_lib o bloque único)
    email-watcher.sh, kalma2-bridge.sh, _run_daemon.sh / _exec_daemon.sh
    opcional: preferir release sobre debug en bundle (F-DEP-07 wrapper)
(3) F-CEN-PKILL  extirpar pkill -x; lock PID-only; start-sddia cleanup
(4) Ensayo forja + SddIA_AP: plantillas user, cwd, inbox distinto, sin fratricidio
(5) validacion.md APTO + PBI → done (cierre documental en rama, un PR)
```

Norma: `sddia-distribution-protocol` (Vía C: no parchear `SddIA/` inyectado en el cliente; el bisturí es **forja**). UUID PBI: `b5d19318-a0fd-440b-9aac-8c6d93f775ed`.

---

## 6. Referencias

| Ref | Uso |
|-----|-----|
| `docs/audits/paciente0-centinelas-email-sordo-20260826.md` | Empiría F-SYS-02 / F-DEP-10 / F-CEN-PKILL / R-07 |
| `docs/features/kaizen-ignicion-soberana-centinelas/` | Antecesor unidades `@%f` |
| `SddIA/templates/systemd/sddia-daemon@.service.template` | `@@SDDIA_CORE_ROOT@@` hoy |
| `SddIA/templates/systemd/sddia-email-watcher@.service.template` | path `daemons/` no `scripts/` |
| `SddIA/engine/execute-process/src/engine/handlers/instance_creator.rs` | `install_systemd_templates` |
| `start-sddia.sh` | `_sync_user_systemd_units`; `cleanup` pkill |
| `SddIA/scripts/daemons/_run_daemon.sh` | `pkill -x`; `SCRIPT_DIR` |
| `SddIA/daemons/email-watcher.sh` | `SCRIPT_DIR`; debug-first |
| `docs/todos/DeudaTecnica/[DEUDA] Paciente 0 — prompt y proceso de despliegue.md` | R-07 lab; F-SYS-01 |

---

## 7. Auditoría al cierre del ciclo feature

Documento nuevo bajo `paths.auditsPath` (`docs/audits/`): dos raíces, `ExecStart` expandido, cwd, SHA inbox **distintos** (o vacío vs no-vacío), 0 `pkill -x` en wrappers, NRestarts estables al `restart` de la otra instancia. No reescribir el audit 20260826; este es la cicatriz de **absorción**.
