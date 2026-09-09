---
feature_name: kalma2-bridge-aiua-interact-stale-elf
created: "2026-09-09"
process: bug-fix
branch: fix/kalma2-bridge-aiua-interact-stale-elf
execution_id: "4ccdf721-f1d4-40e1-b7ca-40002e9d5da5"
items_applied:
  - verify-ca1-ca5-live
  - archive-pbi-done
  - evolution-register
  - genome-untouched
---

# Execution — kalma2-bridge-aiua-interact-stale-elf

## Init

```bash
SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_ALLOW_DIRTY=1 SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 \
  ./sddia-run.sh --process bug-fix --inputs-file .tmp/bug-fix-kalma2-bridge-aiua-interact-stale-elf-init.json
```

`execution_id`: `4ccdf721-f1d4-40e1-b7ca-40002e9d5da5`. workspace-init **executed**. Diseño `simulated`. Planificación commit `4e4f58d`.

## Órgano vivo (Fase 1, sin rebuild)

| Check | Resultado |
|-------|-----------|
| Unidad | `sddia-kalma2-bridge@home-racso-Proyectos-SddIA.service` `active`/`running` |
| MainPID | `1244544` (start 2026-09-09 15:18:25 CEST) |
| exe | `/home/racso/Proyectos/SddIA/SddIA/target/release/kalma2-bridge` |
| ELF mtime | 2026-09-09 15:17:48 CEST |
| Bind | `127.0.0.1:8765` |
| Journal | `lock huérfano pid=6151; recuperando` |
| `strings` | `_ZN13kalma2_bridge20handle_aiua_interact17hef1b975fdf33b161E`; `.rodata` `.../api/aiua/interafile` |
| Resolvedor | path release, exit 0 |

## Probes HTTP (2026-09-09 ~16:11 CEST)

| Stimulus | Resultado | Lectura |
|----------|-----------|---------|
| `GET /` | HTTP 200, 0.6 ms, HTML con `#aiua-pulse` | CA-3 superficie |
| `POST /api/no-existe` | HTTP 404, `ruta desconocida`, 0.4 ms | Dispatcher vivo |
| `POST /api/aiua/interact` `-m 3` / `-m 120` | timeout, 0 bytes, **no** 404 | CA-2: ruta despachada; upstream retiene |
| `POST /api/chat` `-m 3` | timeout, 0 bytes, **no** 404 | CA-4 enrutamiento; combustión = PBI hermano |

Sin segundo latido Gemini (DA-5). Sin rebuild (`A-NO-AUTO-COMPILE-HOT-PATH`).

## Archivo

PBI `pending/` → `done/`. `status: cerrado`. `fix_ref: docs/fixes/kalma2-bridge-aiua-interact-stale-elf`. `document_id` conservado.

## Evolution

`sddia-qa evolution-register` → `51a53297-7e8d-469e-8bb2-8b413b28a366`.

## Fuera del diff

`SddIA/interfaces/kalma2-bridge/src/main.rs`, `interfaces/kalma2/app.js`, `.SddIA/observability/ecosystem-health.json` (sucios locales).
