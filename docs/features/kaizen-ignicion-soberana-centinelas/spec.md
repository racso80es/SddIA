---
feature_name: kaizen-ignicion-soberana-centinelas
created: "2026-08-25"
process: feature
base: main
scope: ignition-systemd
---

# Spec — kaizen-ignicion-soberana-centinelas

## S1 — Fábrica de unidades

`SddIA/templates/systemd/sddia-daemon@.service.template`:

- `WorkingDirectory=%f`
- `EnvironmentFile=-%f/.SddIA/.dev/.env`
- `ExecStart=@@SDDIA_CORE_ROOT@@/SddIA/scripts/daemons/@@DAEMON_NAME@@.sh`
- `Restart=always`

`install_systemd_templates`:

1. No copiar la fábrica como unidad usable (quedaría `@@DAEMON_NAME@@` sin sustituir).
2. Renderizar, para cada nombre en lista canónica, `{instance}/.SddIA/systemd/sddia-{name}@.service`.
3. Seguir copiando el resto de `.template` (email-watcher) con solo `@@SDDIA_CORE_ROOT@@`.

Lista canónica: `event-watcher`, `event-sweeper`, `kalma2-bridge`, `telegram-watcher`, `github-bridge-watcher`.

## S2 — Lanzador bridge

Nuevo `SddIA/scripts/daemons/kalma2-bridge.sh`: carga bóveda, resuelve ELF nativo (`SDDIA_KALMA2_BRIDGE_BIN` o `SddIA/target/{debug,release}/kalma2-bridge`), `exec` en foreground. Cwd = raíz de instancia (la del script → `REPO_ROOT`).

## S3 — Ignición

`SDDIA_DAEMON_JURISDICTION`:

- `systemd` — default si `systemctl --user show-environment` ok.
- `script` — default si no; override explícito siempre gana.

Rama systemd:

1. WARN si `XDG_RUNTIME_DIR` vacío o Linger ≠ yes.
2. `cp` unidades de `{repo}/.SddIA/systemd/` → `${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user/`.
3. `daemon-reload`.
4. `enable --now sddia-{name}@${ESC}.service` con `ESC=$(systemd-escape -p "$REPO_ROOT")` para: event-watcher, event-sweeper, kalma2-bridge (obligatorios). Email si IMAP o R-07. Telegram si `TELEGRAM_BOT_TOKEN`. GitHub si perfil ≠ consumer.
5. Health `GET $KALMA_URL/`. Heartbeats obligatorios.
6. **exit 0** — no `wait`, no jobs `&` de esos centinelas.

Rama script: comportamiento previo (spawn `&` + `wait`).

Cleanup SIGINT: en jurisdicción systemd no `pkill` de centinelas (systemd es el supervisor).

## S4 — Docs

- `start-sddia.md` v1.4.0.
- Bundle ONBOARDING: systemd de núcleo EDA + WUI.
- Teardown Paciente 0: unidades nombradas de instancia.

## Criterios

| ID | Criterio |
|----|----------|
| AC-IC | Creator emite `sddia-event-watcher@.service`, `sddia-event-sweeper@.service`, `sddia-kalma2-bridge@.service` con CORE_ROOT = instance_root |
| AC-SH | Jurisdicción systemd: `start-sddia.sh` no deja esos procesos como hijos de la terminal |
| AC-PORT | Health en `SDDIA_CLIENT_PORT` (Paciente 0: 8766 vía bóveda) |
| AC-IND | Unidades independientes (nombres distintos) |
| AC-TD | Prompt teardown lista stop/disable de las unidades de instancia |
