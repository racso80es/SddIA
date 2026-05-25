---
feature_name: kaizen-higiene-ficheros-temporales
created: "2026-05-25"
process: feature
version_clarify: "1.0.0"
---

# Clarificación — Efímero vs fixture versionado

## 1. Problema confirmado

| Origen | Artefacto | Por qué genera ruido |
|--------|-----------|----------------------|
| `run-eda-e2e-lab.py` | `SddIA/tools/eda-e2e-tool-<hash>.md` | Forja en catálogo **Core** (`scope` por defecto); sin teardown |
| Cierre operativo manual | `docs/features/<feat>/_close-cycle-accept-pr.json` | Input one-shot para `accept-pr`; no es documentación de feature |
| Hooks pre-push / post-merge | `tmp/hook-*.json` | Persisten tras invocación (no hay `unlink`) |
| Runbooks mixtos | Referencias a `tmp/` y `.tmp/` | Operador no sabe cuál es canónico |

## 2. Laudo de diseño

### 2.1 Taxonomía de artefactos

| Clase | Ubicación | Versionado | Ciclo de vida |
|-------|-----------|------------|---------------|
| **Fixture plantilla** | `docs/features/<feat>/_smoke-<escenario>.json` | Sí (PR de la feature) | Reutilizable; valores sustituibles en runtime |
| **Input efímero** | `.tmp/<proceso>-<uuid>.json` | No (gitignored) | Crear → `execute-process` → borrar en `finally` |
| **Forge lab E2E** | `.SddIA/<dominio>/eda-e2e-*` | No | Crear con `scope: local` → borrar en `finally` |
| **Forge productivo** | `SddIA/<dominio>/` vía `entity-manager` | Sí | Solo cadena autorizada + evento ECST |

### 2.2 Carpeta canónica

| Decisión | Motivo |
|----------|--------|
| **`.tmp/`** en raíz del repo | Ya en `.gitignore`; nombre explícito «temporal»; distinto de `/tmp` del SO |
| Deprecar `tmp/` sin punto en código nuevo | Alinear `hook_common` → `.tmp/`; mantener `/tmp` en gitignore una release por compatibilidad |

### 2.3 Flag de depuración

| Variable | Efecto |
|----------|--------|
| `SDDIA_KEEP_TMP=1` | Conservar payloads y forges lab tras smoke (solo depuración local) |
| (default) | Limpieza agresiva en `finally` |

## 3. Opciones evaluadas

| Opción | Descripción | Decisión |
|--------|-------------|----------|
| A | Solo ampliar `.gitignore` con patrones `eda-e2e-*` | Rechazada — enmascara deuda, no corrige causa |
| B | `.tmp/` SSOT + teardown en scripts + norma | **Elegida** |
| C | Sweeper cron que borra untracked | Rechazada — side effects peligrosos; preferir limpieza en origen |

## 4. Impacto en operador IA

- Comandos one-shot (`accept-pr`, `delivery-close-cycle` ad hoc): copiar plantilla a `.tmp/` o usar helper; **no** crear `_close-cycle-*.json` bajo `persist_ref`.
- Smokes E2E: preferir `run-eda-e2e-lab.py` con teardown; no invocar `entity-manager` suelto sin `scope: local` en lab.
- Tras validación manual, comprobar `git status` limpio de paths efímeros.

## 5. Relación con Ruido de Sistema EDA

Los `eda-e2e-tool-*.md` en Core incrementan `orphan_count` en aduana genómica si persisten. El teardown lab **reduce** falsos positivos en smokes locales; no sustituye backfill Fase C de entidades productivas.
