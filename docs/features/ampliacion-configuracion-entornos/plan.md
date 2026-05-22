---
feature_name: ampliacion-configuracion-entornos
created: "2026-05-22"
process: feature
phases: 8
agent_planificador: dedalo
updated: "2026-05-22"
priority: ola-a-hito-0
---

# Plan de implementación — Jerarquía de Bóvedas (configuración de entorno)

Blueprint para Tekton. Entradas: `objectives.md`, `clarify.md`, `spec.md`.

## 0. Mandato estratégico (Ola A)

| Regla | Contenido |
|-------|-----------|
| **Nombre operativo** | Jerarquía de Bóvedas |
| **Posición en Ola A** | **Hito 0** — ejecutar **prioritariamente** antes de cualquier resolución de pasivo técnico |
| **Bloqueo** | Ningún TODO de deuda Ola A (hooks, CLI legacy residual, laboratorio) debe avanzar a Tekton hasta cerrar Hitos 0.1–0.3 con Argos APTO |
| **Rama** | `feat/ampliacion-configuracion-entornos` ✅ |

### 0. Estado de la entrega

| Bloque | Estado | Evidencia |
|--------|--------|-----------|
| Rama de trabajo | ✅ | `feat/ampliacion-configuracion-entornos` |
| Clarificación (Mayeuta) | ✅ | `clarify.md` D1–D12 |
| Especificación (Dedalo) | ✅ | `spec.md` v2 |
| Planificación (Dedalo) | ✅ | este documento v2 |
| **Hito 0.1** — Cargador jerárquico | ✅ | `env_loader.py` |
| **Hito 0.2** — Entrypoints + IOTA | ✅ | commits `42b1a44`, `55f5f82` |
| **Hito 0.3** — Sanitización | ✅ | `.gitignore`, Cúmulo, starter-kit |
| Implementación documental | ⏳ | `implementation.md`, `execution.md` |
| Verificación (Argos) | ⏳ | `validacion.md` |

---

## Hito 0.1 — Implementación del cargador jerárquico

**Touchpoint:** `SddIA/scripts/qa/env_loader.py` (nuevo)

| Paso | Acción | DoD |
|------|--------|-----|
| 0.1.1 | `parse_dotenv_file(path) → dict[str, str]` — líneas `KEY=VALUE`, `#`, `export` opcional | Parseo fail-fast con ruta + línea |
| 0.1.2 | `load_hierarchical_env(repo_root: Path) → dict[str, str]` | Carga `./.dev/.env` (global) primero |
| 0.1.3 | Merge local sobre global | `./.SddIA/.dev/.env` **sobrescribe** claves del dict intermedio |
| 0.1.4 | `apply_env(merged)` vía `os.environ.setdefault` | SO > ficheros (D3) |
| 0.1.5 | Log D4 si **ambos** ficheros existen | Mensaje exacto en stderr |
| 0.1.6 | Smoke en `tmp/` (dict merge + log) | Sin dependencia `python-dotenv` |

**API congelada:**

```python
def load_hierarchical_env(repo_root: Path) -> dict[str, str]:
    """Jerarquía de Bóvedas: global (.dev/.env) → local (.SddIA/.dev/.env) → os.environ."""
```

---

## Hito 0.2 — Refactorización de entrypoints

**Principio:** la jerarquía se aplica **antes de cualquier inicialización de cápsula**. Doble ancla: CLI externo + núcleo de orquestación.

### 0.2.A — Puerta CLI (`execute-process.py`)

| Paso | Acción |
|------|--------|
| 0.2.A.1 | Tras `repo = repo_root()`, invocar `load_hierarchical_env(repo)` |
| 0.2.A.2 | **Antes** de `run_process()` / `shim_execute_action()` |

### 0.2.B — Núcleo de cápsulas (`execute_process_capsules.py`) — **obligatorio**

| Paso | Acción |
|------|--------|
| 0.2.B.1 | Al inicio de `run_process(repo, …)` invocar `load_hierarchical_env(repo)` |
| 0.2.B.2 | Idempotente (D3): segunda llamada no altera env |
| 0.2.B.3 | Garantiza env aunque `run_process` se importe sin pasar por CLI |
| 0.2.B.4 | Subprocesos (`shim_execute_action`, lanzadores tool/skill) heredan `os.environ` — **sin** re-cargar |

### 0.2.C — Entrypoints autónomos (complementarios)

| Entrypoint | Acción |
|------------|--------|
| `execute-action.py` | `load_hierarchical_env(repo)` al inicio de `main()` |
| `event-watcher.py` | Idem tras resolver `REPO_ROOT` |

> La Tarea estratégica cita explícitamente `execute-process.py` y `execute_process_capsules.py`. Los dos entrypoints autónomos permanecen por simetría operativa (D5).

### 0.2.D — Cápsula IOTA (`iota-immutable-publisher/index.ts`)

| Paso | Acción |
|------|--------|
| 0.2.D.1 | Eliminar `import * as dotenv` y `dotenv.config({ path: …/.env })` |
| 0.2.D.2 | Consumir `process.env.*` ya inyectado por padre Python |
| 0.2.D.3 | Mensajes de error → `.SddIA/.dev/.env` (no `.env` local de cápsula) |
| 0.2.D.4 | Retirar `dotenv` de `package.json` si queda huérfana |

**DoD Hito 0.2:** `execute-process.py --process …` y `run_process()` directo ven mismas variables; IOTA sin dotenv local.

---

## Hito 0.3 — Sanitización de sistema

| Paso | Acción | Evidencia |
|------|--------|-----------|
| 0.3.1 | **Eliminar** referencias SSOT a `.env` en subdirectorios de tools | Grep cero: `path.join(__dirname, ".env")`, `dotenv.config` en `SddIA/scripts/tools/` |
| 0.3.2 | Retirar entrada puntual `SddIA/scripts/tools/iota-immutable-publisher/.env` de `.gitignore` | Sustituida por reglas de bóveda |
| 0.3.3 | Añadir/verificar en `.gitignore`: `.dev/` y `.SddIA/.dev/` | Ambas rutas ignoradas |
| 0.3.4 | Auditar repo: `rg '\.env' SddIA/scripts/tools/` — solo `.env.example` permitidos | Informe en `implementation.md` |
| 0.3.5 | Registrar `env_hierarchy` en `SddIA/core/cumulo.paths.json` | JSON válido |
| 0.3.6 | Plantilla `SddIA/scripts/starter-kit/.SddIA/.dev/.env.example` | Starter-kit alineado |
| 0.3.7 | Actualizar `SddIA/tools/iota-immutable-publisher.md` — Security / Jerarquía de Bóvedas | Genoma tool |
| 0.3.8 | Entrada en `SddIA/evolution/` | Trazabilidad federal |

**DoD Hito 0.3:** ningún `.env` operativo disperso bajo `scripts/tools/`; bóvedas en gitignore; operador migra secretos IOTA legacy → `.SddIA/.dev/.env`.

---

## Fase 4 — Documentación de ejecución

| Paso | Artefacto |
|------|-----------|
| 4.1 | `implementation.md` — touchpoints 0.1–0.3, guía migración operador |
| 4.2 | `execution.md` — smoke por hito |

---

## Fase 5 — Verificación Argos

| ID | Check | Hito |
|----|-------|------|
| V-0.1 | Merge local > global en dict; SO intacto | 0.1 |
| V-0.2 | Log `[CONFIG] Jerarquía detectada:…` con ambos ficheros | 0.1 |
| V-0.3 | `execute-process.py` arranca sin bóvedas | 0.2 |
| V-0.4 | `run_process()` vía import carga env (capsules) | 0.2 |
| V-0.5 | IOTA lab con secretos solo en `.SddIA/.dev/.env` | 0.2 |
| V-0.6 | Grep sanitización — cero dotenv local en tools | 0.3 |
| V-0.7 | `.gitignore` contiene `.dev/` y `.SddIA/.dev/` | 0.3 |
| V-0.8 | `python -m json.tool cumulo.paths.json` | 0.3 |

Salida: `validacion.md` — veredicto APTO/NO APTO.

---

## Fase 6 — Cierre (post-Argos)

```json
{
  "feature_name": "ampliacion-configuracion-entornos",
  "branch_name": "feat/ampliacion-configuracion-entornos",
  "persist_ref": "docs/features/ampliacion-configuracion-entornos",
  "source_process": "feature"
}
```

Merge hacia `main` vía **`accept-pr`**.

---

## Orden de commits sugerido

1. `feat(config): Hito 0.1 — env_loader jerarquía de bóvedas`
2. `feat(config): Hito 0.2 — wire execute-process + execute_process_capsules + entrypoints autónomos`
3. `refactor(iota): Hito 0.2 — eliminar dotenv local en iota-immutable-publisher`
4. `chore(config): Hito 0.3 — sanitización .env dispersos, gitignore y cumulo`
5. `docs(feature): implementation, execution y validacion ampliacion-configuracion-entornos`

---

## Matriz de precedencia Ola A (actualizada)

| Faena Ola A | Estado previo | **Gate Hito 0** |
|-------------|---------------|-----------------|
| Jerarquía de Bóvedas (esta feature) | ⏳ | — |
| Hooks Git Hito 3 | Pendiente | **Bloqueado** hasta APTO 0.3 |
| Deuda CLI / laboratorio residual | Parcial | **Bloqueado** hasta APTO 0.3 |
| Pasivos EDA / entidades | En curso | Puede continuar si no muta entrypoints |

---

## Handoff Tekton

**Entradas:** `objectives.md`, `clarify.md`, `spec.md`, `plan.md` (v2).

**Próximo paso:** Hito 0.1 → `env_loader.py`.
