---
feature_name: ampliacion-configuracion-entornos
created: "2026-05-22"
process: feature
branch_name: feat/ampliacion-configuracion-entornos
persist_ref: docs/features/ampliacion-configuracion-entornos
pbi_ref: docs/todos/pending/AmpliacionConfiguracionEntornos.md
---

# Objetivos — Ampliación configuración de entornos

## Misión

Introducir un **cargador jerárquico genérico** de variables de entorno en los entrypoints centrales del runtime SddIA, con precedencia **instancia local sobre global**, y migrar `iota-immutable-publisher` fuera del `.env` ad hoc en su directorio de cápsula.

## Jerarquía objetivo

| Nivel | Ruta | Rol |
|-------|------|-----|
| Global (repo) | `./.dev/.env` | Valores compartidos del workspace |
| Local (instancia) | `./.SddIA/.dev/.env` | Overrides por proyecto; **prevalece** sobre global |

El motor debe aplicarse **antes** de invocar cualquier cápsula de ejecución.

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| O1 | **Módulo reutilizable** | `env_loader.py` (o equivalente) con API `load_hierarchical_env(repo_root) → dict` aplicada a `os.environ` |
| O2 | **Entrypoints cableados** | `execute-process.py`, `execute-action.py` y `event-watcher.py` cargan jerarquía al arranque |
| O3 | **Log de gobernanza** | Si existen ambos archivos: `[CONFIG] Jerarquía detectada: Aplicando SddIA/.dev/.env sobre .dev/.env` en stderr |
| O4 | **Agnosticismo de claves** | Sin hardcode de nombres de variable; solo merge de pares `KEY=VALUE` |
| O5 | **Migración IOTA** | `iota-immutable-publisher` deja de usar `dotenv` local en `__dirname`; secretos vía jerarquía |
| O6 | **SSOT y gitignore** | Rutas registradas en Cúmulo; `.dev/` y `.SddIA/.dev/` ignoradas; plantillas `.env.example` en starter-kit |
| O7 | **Precedencia OS** | Variables ya presentes en el entorno del SO **no** se sobrescriben por los ficheros (comportamiento dotenv estándar) |

## Fuera de alcance

- Gestores de secretos externos (Vault, 1Password, etc.).
- Variables de frontends GesFer (`.env.local` en laboratorios Vía C) — convención aparte.
- Retirada de shims CLI Ola C (`execute-process` / `execute-action`).
- Hooks Git Hito 3 (feature paralela).

## Manifiesto operativo

Origen: `docs/todos/pending/AmpliacionConfiguracionEntornos.md`

## Estado

| Fase feature | Estado |
|--------------|--------|
| Clarificación | ✅ `clarify.md` |
| Objetivos | ✅ este documento |
| Especificación | ✅ `spec.md` |
| Planificación | ✅ `plan.md` |
| Implementación (Tekton) | ⏳ pendiente |
| Verificación (Argos) | ⏳ pendiente |
