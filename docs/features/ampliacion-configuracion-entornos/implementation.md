---
feature_name: ampliacion-configuracion-entornos
created: "2026-05-22"
process: feature
items:
  - env_loader.py
  - execute-process.py
  - execute_process_capsules.py
  - execute-action.py
  - event-watcher.py
  - iota-immutable-publisher/index.ts
  - cumulo.paths.json
  - gitignore
---

# Implementación — Jerarquía de Bóvedas

## Hito 0.1 — Cargador

| Touchpoint | Cambio |
|------------|--------|
| `SddIA/scripts/qa/env_loader.py` | **Nuevo** — `parse_dotenv_file`, `apply_env`, `load_hierarchical_env` |

## Hito 0.2 — Entrypoints

| Touchpoint | Cambio |
|------------|--------|
| `execute-process.py` | `load_hierarchical_env(repo)` pre-`run_process` |
| `execute_process_capsules.py` | `load_hierarchical_env(repo)` inicio `run_process` |
| `execute-action.py` | Idem en `main()` |
| `event-watcher.py` | Idem en `main()`; `_iota_timeout_seconds()` lazy post-carga |
| `iota-immutable-publisher/index.ts` | Sin `dotenv`; mensajes → bóveda instancia |

## Hito 0.3 — Sanitización

| Touchpoint | Cambio |
|------------|--------|
| `.gitignore` | `.dev/`, `.SddIA/.dev/`; retirada regla puntual IOTA `.env` |
| `cumulo.paths.json` | Clave `env_hierarchy` |
| `starter-kit/.SddIA/.dev/.env.example` | Plantilla operador |
| `SddIA/tools/iota-immutable-publisher.md` | Security → Jerarquía de Bóvedas |

## Migración operador

1. Copiar secretos de `SddIA/scripts/tools/iota-immutable-publisher/.env` → `.SddIA/.dev/.env`
2. Borrar `.env` legacy local en la cápsula
3. Opcional: variables compartidas del equipo en `.dev/.env` (raíz repo)
