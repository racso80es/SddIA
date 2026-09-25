---
feature_name: sddia-deterministic-installer
created: "2026-09-25"
process: feature
base: main
scope: scripts
branch_name: feat/sddia-deterministic-installer
persist_ref: docs/features/sddia-deterministic-installer
execution_id: "7b22f932-c162-4104-b38d-b1c9c6068414"
document_id: PBI-ARQUITECTURA-DEPLOY-DETERMINISTA
---

# Spec — sddia-deterministic-installer

## 1. Superficie CLI

| Artefacto | Rol |
|-----------|-----|
| `./sddia-installer.sh` | Wrapper raíz: `exec SddIA/scripts/sddia-installer.sh "$@"` |
| `SddIA/scripts/sddia-installer.sh` | Motor. `set -euo pipefail`. Source `sddia_shell_lib.sh`. |

```text
./sddia-installer.sh deploy   [--root PATH] [--vault PATH] [--force] [--skip-build] [--dry-run]
./sddia-installer.sh teardown [--root PATH] [--force] [--dry-run]
```

`--force` en teardown = confirmar wipe (sigue sin prompt; el flag es el consentimiento). Deploy `--force` = teardown previo + deploy.

`--dry-run`: imprime plan JSON (root, esc, units, bundle_profile, vault_set) y exit 0. No muta FS ni systemd.

## 2. Resolución de ROOT (L-PATH)

1. `--root`
2. `SDDIA_INSTALL_ROOT` (env ya cargado)
3. `_sddia_load_vault` forja → `SDDIA_INSTALL_ROOT`
4. Last-resort: `/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA`

Abort si ROOT resuelto = `FORGE_ROOT` (realpath).

## 3. Perfil `full-node` (`build-release-bundle.sh`)

Tras semilla `CONSUMER_BINS` + escáner códice:

```text
si PROFILE ∈ {full-node, full}:
  para cada Cargo.toml en SddIA/{tools,skills,daemons,engine,interfaces}/*:
    CAPSULE_SET[basename(dir)] = 1
  CAPSULE_SET[execute-process] = 1
  CAPSULE_SET[kalma2-bridge] = 1
```

Filtro C (poda `codex-software-engineering` + drop `github-bridge-watcher`) **solo** `consumer`. `full-node` no poda library/codexes. ELF ausente en skip-build = WARN (paridad actual); build real fail-closed en núcleo obligatorio.

## 4. Deploy (Ceguera)

1. **Host:** `systemctl --user` invocable. Si no `--skip-build`: `command -v cargo`.
2. **Live-gate:** si `is_live(ROOT)` y no `--force` → stderr + exit 2.
3. **Force:** si vivo → teardown (mismo ROOT) y continuar.
4. **Bundle:** `./SddIA/scripts/build-release-bundle.sh --out "$ROOT" --profile full-node` (+ `--skip-build` si flag).
5. **Creator:** `./sddia-run.sh --process instance-creator --inputs` JSON:
   - `instance_root` = ROOT
   - `runtime_profile` = `engineering`
   - `vault_source` = `--vault` o `{FORGE}/.dev/.env` si existe
   - `skip_ignition` = true
6. **Systemd:** para cada unidad en `{ROOT}/.SddIA/systemd/sddia-*@.service` (y `sddia-email-watcher@.service`):
   - copiar a `~/.config/systemd/user/` si no existe plantilla factory
   - `daemon-reload`
   - `enable --now "sddia-<name>@${ESC}.service"` donde `ESC=$(systemd-escape -p "$ROOT")`
   - Set mínimo: `event-watcher`, `event-sweeper`, `kalma2-bridge`, `email-watcher`. Si el launcher `.sh` del daemon existe en el bundle: `telegram-watcher`, `github-bridge-watcher`.

Cero secretos en logs. Fallo de cualquier paso = abort (no leave-half salvo que el propio creator ya haya escrito; `--force` recupera).

## 5. Teardown

1. Resolver ROOT. Abort si ROOT = forja.
2. `ESC=$(systemd-escape -p "$ROOT")`.
3. `systemctl --user list-units --all --plain --no-legend "sddia-*@${ESC}.service"` + unit-files homólogos → `stop` + `disable`.
4. No `rm` de `~/.config/systemd/user/sddia-*@.service` (factory compartida).
5. `daemon-reload`.
6. Procesos residuales: `pgrep -a` cuyo argv/cwd contiene ROOT → SIGTERM; timeout corto; SIGKILL si persisten.
7. `rm -rf "$ROOT"`.
8. Veredicto: directorio ausente + cero unidades active `@${ESC}`.

## 6. Tests (smoke)

`SddIA/scripts/qa/test-sddia-installer.sh`:

| Caso | Esperado |
|------|----------|
| `deploy --dry-run` sin flags | JSON con last-resort o env; `bundle_profile=full-node`; exit 0 |
| `deploy --root /tmp/sddia-inst-smoke --dry-run` | root = path dado |
| `deploy --root $FORGE --dry-run` | abort ≠ 0 |
| `teardown --root /tmp/… --dry-run` | `esc` coherente con `systemd-escape -p` |
| `full-node` discovery | script bundle lista cápsulas que incluyen CONSUMER_BINS + al menos un crate extra (`sddia-qa` o `github-bridge-watcher`) |
| destino con `.SddIA` stub + `deploy` sin `--force` (dry-run live-gate via función o fixture) | abort 2 |

Prohibido `systemctl enable` real en smoke CI.

## 7. Fuera

Handler `instance_creator.rs`. Genoma process/norm. Deploy a ruta PBI real.
