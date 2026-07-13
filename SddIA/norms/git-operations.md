---
uuid: "e5f6a7b8-c9d0-4123-e456-7890abcdef01"
name: "git-operations"
version: "1.2.0"
entity_type: "norm"
jurisdiction: "cerbero"
---

# Política de operaciones Git (norma, no ejecución)

## 1. Separación norma / ejecución

La skill **`git-manager`** es un peón determinista: ejecuta subcomandos del binario `git` nativo según el esquema congelado. **No** interpreta convenciones de negocio del proyecto.

Las reglas de negocio (p. ej. prefijos de rama `feat/`, `fix/`, mensajes de commit, ramas prohibidas) viven **aquí** y son aplicadas **antes** de invocar `git-manager`, por Cerbero y Argos (u otros agentes de auditoría autorizados).

## 2. Contenido normativo (extensible)

Esta versión inicial establece el marco; Dédalo y Cúmulo completarán los detalles según el repositorio activo.

- **Ramas de trabajo:** convención de nombres y troncal de integración (definir por proyecto).
- **Commits:** alineación con Conventional Commits u otra política acordada.
- **Force push:** cuándo está permitido y bajo qué token/contexto.

## 3. Artefactos efímeros y fixtures

| Clase | Ubicación | Versionado | Ciclo de vida |
|-------|-----------|------------|---------------|
| **Input efímero** | `.tmp/<contexto>-<uuid>.json` | No (`.gitignore`) | Crear → consumir → borrar en `finally` |
| **Fixture plantilla** | `docs/features/<feat>/_smoke-<escenario>.json` | Sí (PR de la feature) | Reutilizable; copiar a `.tmp/` para one-shot |
| **Forge lab E2E** | `.SddIA/<dominio>/` con `scope: local` | No | Teardown post-smoke salvo depuración |

**Reglas:**

- Prohibido escribir inputs JSON one-shot (`_close-cycle-*`, `_delivery-close-*` ad hoc) bajo `docs/features/<persist_ref>/`.
- Prohibido forja de laboratorio en catálogo Core (`SddIA/tools/`, etc.) sin cadena productiva y evento ECST.
- Helper canónico: `SddIA/scripts/qa/git-hooks/hook_common.sh` (`_write_ephemeral_json`, `_cleanup_path`).
- Depuración local: `SDDIA_KEEP_TMP=1` conserva payloads y forges lab.

## 3.1 Cobertura EDA genómica (pre-commit / aduana)

- Forja productiva bajo `SddIA/` (skills, events, process, agents, tools, actions, norms, codexes) **solo** vía cadena `entity-manager` → `emit-domain-mutation` → bus.
- Tras backfill Fase C (`--emit --skip-dlt` + `--anchor-merkle`), ejecutar `event-watcher --once` y validar `--scan` **antes** de commits que toquen genoma con hook activo.
- Prohibido inferir cobertura EDA con eventos únicamente en `pending/`; el audit correlaciona cabeceras `processing/` y `processed/` (mitigación transitoria — ver PBI correlación durable sin bus).
- Orden Tekton recomendado: fix correlación → backfill si aplica → watcher + scan → commit genoma.

La ruta legacy `tmp/` (sin punto) queda deprecada; usar **`.tmp/`** en código nuevo. Ambas permanecen en `.gitignore` durante la migración.

## 4. Referencias

- Entrada congelada de la skill: `SddIA/norms/skill-io-git-manager-frozen.md`
- Mapa de rutas Core: `SddIA/core/cumulo.paths.json` → `directories.norms`
- Orquestación PR y SSOT merge: `SddIA/norms/pull-request-orchestration.md` §4
- Runbook operativo fusión local: `docs/features/l1-o5-runbooks-paridad/runbook-accept-pr.md`
- Patrón documental: `SddIA/library/norms/features-documentation-pattern.md` § Artefactos efímeros
