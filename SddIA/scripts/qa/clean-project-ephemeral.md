---
tracker_ref: AUT-79
scope: operador clean-project-ephemeral — slice cargo-deps
---

# Limpieza de ficheros temporales

Script: `SddIA/scripts/qa/clean-project-ephemeral.sh`

Acceso directo en la raíz: `./sddia-clean-ephemeral.sh` fija `--repo`, **`--apply`** y **`--with-cargo-deps`** (purga scratch, logs de runtime, workspaces vacíos y hashes Cargo viejos en `debug/deps` + `release/deps`). Argumentos extra se reenvían (p. ej. `--with-daemon-logs`, `--no-wait`). Inventario en seco sin borrar: invocar `SddIA/scripts/qa/clean-project-ephemeral.sh` sin `--apply`.

Operador de disco para la instancia local del Core-self. Inventaría por defecto. El borrado solo ocurre con `--apply`.

Las rutas salen de `SddIA/core/cumulo.paths.json` (v1.11.1): `paths.workspacesRoot`, `eda_bus.store`, `daemons_instance`, bus filesystem `/.events/`.

Auxiliar AUT-79: `SddIA/scripts/qa/clean-project-ephemeral-cargo-deps.py`. Test: `SddIA/scripts/qa/test-clean-project-ephemeral-cargo-deps.sh`.

No sustituye la cápsula `repo-retention-purger` (AUT-49). No genera entidades de genoma.

## Qué limpia

| Slice | Ruta | Cuándo entra | Acción |
|-------|------|--------------|--------|
| `workspaces-empty` | `.SddIA/workspaces/{proceso}/{execution_id}/` vacío | Siempre | `sddia-qa workspace-prune --empty` si el binario existe; si no, `rmdir` de directorios de ejecución vacíos |
| `runtime-logs` | `.SddIA/runtime/*.log` | Siempre | Borra logs de runtime |
| `scratch` | `.tmp/`, `tmp/`, `docs/tmp/` | Siempre | Ficheros con mtime mayor que `--scratch-hours` (defecto 168) |
| `daemon-logs` | `.SddIA/daemons/logs/` | `--with-daemon-logs` | Ficheros más viejos que `--daemon-log-days` (defecto 14) |
| `events-fs` | `.events/{pending,processed,telemetry,domain,orchestration,progress}/**/*.json` | `--with-events-fs` | Solo si `SDDIA_EVENT_STORE=redb` y no hay proceso `event-watcher` |
| `cargo-deps` | `SddIA/target/debug/deps`, `SddIA/target/release/deps` | `--with-cargo-deps` | Generaciones Cargo obsoletas por hash (AUT-79); ver abajo |

`workspaces-empty` delega en `sddia-qa` (`SDDIA_QA_BIN`, luego `SddIA/target/release/sddia-qa`, luego `debug`). El binario resuelve el repo desde el directorio de trabajo; el script lo invoca con cwd en la raíz.

## Qué no toca

- `.git/**`
- `.SddIA/runtime/events.redb` y sus sidecars
- `.SddIA/vector_store/**`
- `.SddIA/proofs/**`
- `.SddIA/services/**`
- `.SddIA/daemons/state/**` (latidos; no son cola ECST)
- `.events/processing/**`
- `.events/dead-letter/**` (el shell no consulta `dead_letter_index`)
- `SddIA/target/native-debug-bins/**`, binarios sueltos en `debug/`/`release/`, `wasm32-wasip1/**`, `.fingerprint/**`, `incremental/**`

El modo de store se lee de `SDDIA_EVENT_STORE`, si no de `.SddIA/.dev/.env`, si no de `.dev/.env`. Sin clave, el contrato fija `redb`. En `filesystem` o `dual` el slice `events-fs` no borra nada.

## Uso

Inventario:

```bash
./sddia-clean-ephemeral.sh
./sddia-clean-ephemeral.sh --json
```

Purga vía acceso directo (ya incluye `--apply` y `--with-cargo-deps`):

```bash
./sddia-clean-ephemeral.sh
```

Purga manual con el operador (mismas banderas explícitas):

```bash
SddIA/scripts/qa/clean-project-ephemeral.sh --repo "$(pwd)" --apply --with-cargo-deps
```

Logs de daemon y residuo ECST en disco (store `redb`, watcher parado):

```bash
./sddia-clean-ephemeral.sh --apply \
  --with-daemon-logs --daemon-log-days 14 \
  --with-events-fs
```

Test AUT-79:

```bash
SddIA/scripts/qa/test-clean-project-ephemeral-cargo-deps.sh
```

Salida texto: título de resultado (inventario o purga), repositorio y una línea por slice con contador y acción en español. `deps Cargo` incluye tamaño en GiB/MiB. Después, si hay consola y no se pasó `--json` ni `--no-wait`: `Pulsa una tecla para cerrar.`

Salida `--json`: incluye `cargo_deps`, `cargo_deps_bytes`, `cargo_deps_action`. No espera tecla.

## Slice `cargo-deps` (AUT-79)

Implementado en el operador shell (AUT-79). La cápsula **`repo-retention-purger`** (AUT-49) usa la misma regla de hash desde el cierre L2; vía `./sddia-run.sh --action purge-repo-retention` con simulate → digest → apply.

Regla:

1. Solo ficheros inmediatos en `debug/deps` y `release/deps`.
2. Por nombre lógico de crate, conservar todos los ficheros del hash con mtime más reciente.
3. Borrar ficheros de hashes anteriores.
4. Si el hash más reciente no tiene `.rlib`/`.so`/`.a`/`.dylib` y un hash viejo sí, no borrar nada en ese grupo (`hold-incomplete`).

Abstención: `cargo`/`rustc` activo sobre `SddIA/target`, o `/proc` ilegible → cero borrados en este slice.

## Límites

- No abre `events.redb`. Por eso no purga `dead-letter/`.
- No borra workspaces con contenido ni `execution_report.json`.
- Un path que resuelva fuera de la raíz del repo aborta el proceso.
- No rota bundles `dist/` (slice D de AUT-49).
