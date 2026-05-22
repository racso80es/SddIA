---
feature_name: ampliacion-configuracion-entornos
created: "2026-05-22"
process: feature
branch_name: feat/ampliacion-configuracion-entornos
persist_ref: docs/features/ampliacion-configuracion-entornos
pbi_ref: docs/todos/pending/AmpliacionConfiguracionEntornos.md
priority: ola-a-hito-0
updated: "2026-05-22"
---

# Objetivos — Jerarquía de Bóvedas (configuración de entorno)

## Misión

Establecer la **Jerarquía de Bóvedas** como infraestructura de configuración federal de SddIA: cargador jerárquico genérico, cableado en el núcleo de cápsulas, sanitización de `.env` dispersos y migración IOTA a la nueva topología.

## Mandato estratégico Ola A

Esta fase es **Hito 0** de la Ola A ampliada. Debe ejecutarse **prioritariamente** antes de cualquier resolución de pasivo técnico restante (hooks, deuda CLI, laboratorio).

## Jerarquía de Bóvedas

| Nivel | Bóveda | Ruta | Rol |
|-------|--------|------|-----|
| Global | Bóveda repo | `./.dev/.env` | Valores compartidos del workspace |
| Local | Bóveda instancia | `./.SddIA/.dev/.env` | Overrides soberanos; **prevalece** sobre global |

Precedencia entre ficheros: **local > global**. Precedencia global del stack: **SO > bóvedas** (`setdefault`).

El motor se aplica **antes de cualquier inicialización de cápsula** (CLI + `execute_process_capsules.run_process`).

## Hitos de configuración

| Hito | Contenido | Entregable |
|------|-----------|------------|
| **0.1** | Cargador jerárquico | `SddIA/scripts/qa/env_loader.py` — `load_hierarchical_env(repo_root)` |
| **0.2** | Refactor entrypoints | `execute-process.py`, `execute_process_capsules.py`, IOTA `index.ts`; complemento: `execute-action.py`, `event-watcher.py` |
| **0.3** | Sanitización | Eliminar `.env` dispersos en tools; `.dev/` y `.SddIA/.dev/` en `.gitignore` |

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| O0 | **Prioridad Ola A** | Hitos 0.1–0.3 cerrados antes de retomar pasivos técnicos Ola A |
| O1 | **Módulo reutilizable** | `load_hierarchical_env(repo_root) → dict` aplicado a `os.environ` |
| O2 | **Doble ancla runtime** | `execute-process.py` **y** `run_process()` en `execute_process_capsules.py` invocan loader pre-cápsula |
| O3 | **Log de gobernanza** | Ambos ficheros → `[CONFIG] Jerarquía detectada: Aplicando SddIA/.dev/.env sobre .dev/.env` |
| O4 | **Agnosticismo de claves** | Sin hardcode de variables; merge genérico `KEY=VALUE` |
| O5 | **IOTA sin dotenv local** | `iota-immutable-publisher` consume env inyectado; cero `dotenv.config` en cápsula |
| O6 | **SSOT bóvedas** | `env_hierarchy` en Cúmulo; plantillas `.env.example` en starter-kit; **README.md** actualizado |
| O7 | **Precedencia OS** | Variables del SO no sobrescritas por ficheros |
| O8 | **Sanitización** | Cero `.env` operativos bajo `SddIA/scripts/tools/`; gitignore verificado |

## Fuera de alcance

- Gestores de secretos externos (Vault, 1Password).
- `.env.local` de frontends GesFer (Vía C).
- Retirada de shims CLI Ola C (feature aparte).

## Manifiesto operativo

Origen: `docs/todos/pending/AmpliacionConfiguracionEntornos.md`

## Estado

| Fase feature | Estado |
|--------------|--------|
| Clarificación | ✅ `clarify.md` (D1–D12) |
| Objetivos | ✅ este documento v2 |
| Especificación | ✅ `spec.md` v2 |
| Planificación | ✅ `plan.md` v2 |
| Hito 0.1–0.3 (Tekton) | ✅ implementado |
| Verificación (Argos) | ✅ `validacion.md` APTO |
