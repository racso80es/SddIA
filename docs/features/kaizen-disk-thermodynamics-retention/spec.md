---
feature_name: kaizen-disk-thermodynamics-retention
created: "2026-11-01"
base: SddIA/tools/ephemeral-cache-purger
scope: "Retención regenerable del repo (target, residuo .events FS, scratch, dist)"
process: feature
tracker_ref: AUT-49
---

# Spec — repo-retention-purger

## Entidades

| Entidad | Rol |
|---------|-----|
| Tool `repo-retention-purger` | Enumera candidatos y ejecuta borrado tras gate |
| Action `purge-repo-retention` (nombre provisional) | Handler en `execute-process`: simulate → gate digest → apply |
| Process opcional `repo-retention-tick` | Invoca action con `simulate_only` o apply acusado (fase posterior) |

Alta de ficheros genoma: `./sddia-run.sh` + `entity-manager` (DA-2). Este spec no autoriza `Write` manual bajo `SddIA/tools/`.

## Modo de store

`SDDIA_EVENT_STORE` → `sddia_event_store::store_mode()` (default `redb`).

| Modo | Slice A | Slice B | C | D |
|------|---------|---------|---|---|
| `redb` | Sí | Sí (residuo FS) | Sí | Sí |
| `filesystem` | Sí | **No lista** | Sí | Sí |
| `dual` | Sí | **No lista** | Sí | Sí |

## Jail (repo-root)

- Raíz: directorio git toplevel del Core (argumento `--repo` o `CARGO_MANIFEST_DIR` ascendente en tests).
- **Permitido:** paths canónicos bajo repo resueltos desde allowlist de slices.
- **Prohibido:** componente `..`, path absoluto fuera del repo, `.git/**`, `.SddIA/runtime/events.redb`, `.SddIA/vector_store/**`, `SddIA/target/native-debug-bins/**`.

## Slice A — `SddIA/target`

- Prefijos: `SddIA/target/debug/deps`, `release/deps`, `debug/incremental`, binarios de test bajo `debug/` (excluir `native-debug-bins`).
- Política deps: agrupar por nombre lógico de artefacto (stem sin hash Cargo); conservar mtime más reciente por grupo.
- Política incremental: por crate (directorio bajo `debug/incremental/`), conservar las 2 sesiones más recientes.
- Abstención apply: proceso `cargo` o `rustc` con `CARGO_TARGET_DIR` apuntando al target del repo (best-effort `/proc` o equivalente portable en tests: flag `skip_live_check` solo en tests).

## Slice B — residuo `.events` (solo `redb`)

Directorios enumerados desde topología runtime (defaults si cumulo no override FS):

- Dominio bus: `pending`, `processed`, `processed/subscribers`, `dead-letter`, `dead-letter/subscribers` (`EdBusPaths`).
- Fractal: `telemetry`, `domain`, `orchestration`, `progress` (`BusTopology`).

**Excluidos:** `processing`, `processing/subscribers`.

Candidato: fichero `*.json` bajo esos dirs. Si `event_id` (stem) existe en `dead_letter_index` → entrada `kept: dead-letter-index`. No borrar side-channel bajo `.SddIA/daemons/state/heartbeats/`.

Abstención apply:

1. `EventStore::open(runtime_db_path)` falla → slice B vacío en plan; apply error `STORE_UNREADABLE`.
2. Proceso `event-watcher` en ejecución con cwd/repo coherente → error `WATCHER_ACTIVE`.

Nota operativa: el watcher en `redb` sigue emitiendo `F-EDA-RESIDUAL-*` ante JSON nuevo; la purga one-shot reduce histórico; no sustituye el contrato del watcher.

## Slice C — scratch

Allowlist prefijos repo-relativos: `.tmp/`, `tmp/`, `docs/tmp/`. TTL configurable (`older_than_hours`, default 168). Entradas con `mtime` anterior a `2000-01-01T00:00:00Z` → policy `hold-suspicious-mtime` (no apply salvo `force_suspicious: true` en plan acusado).

## Slice D — `dist/`

Bundles bajo `dist/` con patrón lab/smoke; retener el más reciente por basename; borrar más viejos que `dist_retention_days` (default 30).

## Plan JSON (salida simulate)

```json
{
  "plan_digest": "sha256:…",
  "repo_root": "/abs/repo",
  "store_mode": "redb",
  "slices": {
    "target": { "candidates": [], "bytes_reclaimable": 0 },
    "events_fs": { "candidates": [], "bytes_reclaimable": 0 },
    "scratch": { "candidates": [], "bytes_reclaimable": 0 },
    "dist": { "candidates": [], "bytes_reclaimable": 0 }
  },
  "candidates": [
    { "path": "rel/path", "bytes": 0, "policy": "keep-latest-dep", "kept": false }
  ]
}
```

Apply stdin exige `plan_digest` idéntico y lista `candidates` subset (prohibido apply con paths no listados en plan acusado).

## Handler action (paridad sandbox)

1. Invocar tool `simulate: true`.
2. Verificar jail sobre cada `path`.
3. Si `simulate_only` → retorno.
4. Invocar tool `simulate: false` + `plan_digest`.

Lock: fichero `.SddIA/runtime/repo-retention-purger.lock` (create exclusivo); liberar al terminar.

## Criterios de aceptación

Mapeo 1:1 a KA-RET-1…KA-RET-8 en `objectives.md`. Tests:

- Crate `repo-retention-purger`: jail, agrupación deps, hold mtime, slice B vacío en `filesystem`.
- Integración handler: digest mismatch → error; simulate_only sin borrado.

## No regresión

- `ephemeral-cache-purger` tests verdes sin cambio de `ALLOW_RE`.
- `SDDIA_EVENT_STORE=filesystem`: suites EDA que cuentan JSON en `.events/` no pierden ficheros vivos en `pending/`/`processing/`.
- Tras apply slice A en fixture, binario staged en `native-debug-bins` sigue presente.
