---
document_id: AUDIT-PACIENTE0-CENTINELAS-EMAIL-SORDO-20260826
uuid: "d1631e7c-0a48-4628-91fd-d691e6c72113"
title: "Auditoría — centinelas Paciente 0 sordos al correo; solo responde la forja"
created: "2026-08-26"
instance_path: /home/racso/Proyectos/SddIA_AP
forge_path: /home/racso/Proyectos/SddIA
wui_ap: "http://127.0.0.1:8766/"
wui_forge: "http://127.0.0.1:8765/"
verdict: AP_UNITS_HIJACKED_BY_FORGE_LAUNCHERS
friction_ids:
  - F-SYS-02
  - F-DEP-10
  - F-CEN-PKILL
  - R-07
related:
  - PBI-DT-PACIENTE0-DEPLOY-PROCESS
  - PBI-KAIZEN-IGNICION-SOBERANA
  - docs/audits/paciente0-lock-session-centinelas-20260825.md
---

# Auditoría — correo Paciente 0 sin reacción (2026-08-26)

Estímulo: los centinelas de `SddIA_AP` no reaccionan a un correo en la cuenta de Paciente 0; solo responde el sistema de la forja `SddIA`. Cero secretos (IMAP: nombres de clave y huella SHA-12, no valores).

## 0. Veredicto

**Paciente 0 no tiene un bus ni un IMAP propios en runtime.** Las unidades `@%f` de `SddIA_AP` están `enabled`, pero `~/.config/systemd/user/sddia-*.service` (un solo archivo por plantilla) apunta `ExecStart` a la **forja**. Los lanzadores resuelven `REPO_ROOT` desde `SCRIPT_DIR`, ignoran `WorkingDirectory=%f`, hacen `cd` a `/home/racso/Proyectos/SddIA` y toman ELF `target/debug`. El correo de Paciente 0 no entra en el bus de `SddIA_AP`. `:8766` y `:8765` sirven el **mismo** JSON de inbox (bytes idénticos).

No es un fallo IMAP de la cuenta Paciente 0 como primera causa. Es colisión de plantilla user + ceguera espacial rota en wrappers + R-07 violado (lab `email-watcher@…SddIA` **active**).

## 1. Matriz viva (snapshot 07:41 CEST)

| Unidad | AP `@…SddIA_AP` | Forja `@…SddIA` |
|--------|-----------------|-----------------|
| email-watcher | active; MainPID 2883; NRestarts=1 | active; MainPID 1858; AUTH FAILURE |
| event-watcher | crash-loop NRestarts≈96; log `Repo: …/SddIA` | active (también NRestarts altos) |
| event-sweeper | active | activating/auto-restart |
| kalma2-bridge | active PID 1888 puerto **8766** | active PID 1883 puerto **8765** |
| telegram-watcher | active | active + `sddia-daemon@telegram-watcher` legado |

HTTP 200 en ambos puertos. `cwd` de 1858/1883/1888/2883 = **forja**, no `SddIA_AP`.

## 2. Causalidad

### F-SYS-02 — plantilla user única pisa `CORE_ROOT`

`start-sddia.sh` copia `{repo}/.SddIA/systemd/*.service` → `~/.config/systemd/user/` **sin disambiguar instancia**. `FragmentPath` de `@AP` y `@forja` es el mismo archivo.

Tras reboot (linger), las plantillas en home tienen:

```text
ExecStart=/home/racso/Proyectos/SddIA/SddIA/daemons/email-watcher.sh
```

La copia **renderizada** en `{SddIA_AP}/.SddIA/systemd/` sí apunta a `…/SddIA_AP/SddIA/…`. **No está instalada** en el user manager. `diff` home vs instancia = distintos.

### F-DEP-10 — lanzador `SCRIPT_DIR` > `%f`

`email-watcher.sh`: `REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"` + `cd "$REPO_ROOT"` + preferencia **debug** sobre release.

`scripts/daemons/_run_daemon.sh`: igual; `LOCK_FILE="$REPO_ROOT/.SddIA/daemons/status/${DAEMON}.lock"`.

systemd inyecta `EnvironmentFile=-%f/.SddIA/.dev/.env` (bóveda AP para `@AP`) y `WorkingDirectory=%f`, pero el wrapper **abandona** ese cwd. ELF y bus = forja.

Evidencia AP `event-watcher` (unidad `@SddIA_AP`):

```text
Repo: /home/racso/Proyectos/SddIA
roots= […/SddIA/.events/{telemetry,pending,domain,orchestration}]
```

### F-CEN-PKILL — `pkill -x event-watcher`

`_stop_previous` hace `pkill -x "$DAEMON"`. Mata **todos** los ELF del mismo nombre (forja y AP). Explica `auto-restart` cruzado (AP 96; forja 92) y SIGTERM en el MainPID de AP.

### R-07 + CEN-01 lock

Boot 07:25: unidad AP email: `lock activo pid=1858; abortando duplicado` (lock en **forja**). Restart 07:26: MainPID 2883; lock forja = 2883. Lock AP sigue pid **81312** (fósil 2026-08-25).

Forja email (1858): `imap login AUTHENTICATIONFAILED` en bucle. IMAP_USER forja ≠ Paciente 0 (huellas distintas; HOST y MAILBOX iguales). Cuenta Paciente 0 no es la que autentica el PID 1858.

### WUI

`kalma2-bridge` `@AP`: `SDDIA_CLIENT_PORT=8766` y `PWD=/home/racso/Proyectos/SddIA`. GET `/api/email-inbox` `:8766` ≡ `:8765` (mismo SHA-256). El operador que mira Paciente 0 en 8766 está viendo el almacén de la forja.

## 3. Por qué un correo a la cuenta Paciente 0 «no hace nada»

1. No hay `email-watcher` cuyo `REPO_ROOT` sea `SddIA_AP` (lanzador AP no está en `ExecStart`).
2. El bus que enruta de verdad es el de la forja; el watcher AP, cuando vive, **también** mira `.events` de la forja y se suicida por `pkill`.
3. Credenciales IMAP de la forja fallan; no equivalen a la cuenta Paciente 0.
4. Inbox WUI 8766 no es un SSOT de instancia.

No se re-envió correo en esta auditoría (DA-5 / no stim IMAP).

## 4. Qué no es

- Linger / lock-session (audit 2026-08-25): unidades user siguen loaded; el síntoma de hoy es **identidad de binario/cwd**, no sleep ACPI.
- Ausencia de bóveda AP: claves IMAP presentes en `{AP}/.SddIA/.dev/.env`.
- Filtro C / bundle: el árbol instancia sigue teniendo wrappers con `ExecStart` correcto **en disco de instancia**; el user systemd no los usa.

## 5. Mitigación operativa (no aplicada en este estímulo)

1. R-07: `stop`/`disable` `sddia-*-watcher@…SddIA` y `sddia-kalma2-bridge@…SddIA` (lab), más `sddia-daemon@telegram-watcher`.
2. Reinstalar plantillas **desde** `{SddIA_AP}/.SddIA/systemd/` → `~/.config/systemd/user/` + `daemon-reload` + `restart` solo `@…SddIA_AP`.
3. Hasta absorber F-DEP-10: `ExecStart` debe ser el wrapper **bajo `%f`**, o los wrappers deben honrar `WorkingDirectory` / `SDDIA_INSTANCE_ROOT` en lugar de `SCRIPT_DIR`.
4. `pkill -x` solo sobre PID de lock de **esa** raíz.

Absorción Core: ciclo `feature` (plantillas user namespaced o `CORE_ROOT=%f` real en ExecStart; lanzadores cwd-first). No forjado aquí.

## 6. Kaizen

Fricción **nueva** respecto al Kaizen ignición soberana (AC-IC en instancia; runtime user pisa). PBI: `docs/todos/pending/[KAIZEN] aislamiento multi-instancia centinelas.md` (`PBI-KAIZEN-AISLAMIENTO-MULTI-INSTANCIA`).
