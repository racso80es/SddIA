---
feature_name: ampliacion-configuracion-entornos
created: "2026-05-22"
process: feature
purpose: Jerarquía .dev/.env global vs .SddIA/.dev/.env local
---

# Clarificación — Ampliación configuración de entornos

Transcript de decisiones (2026-05-22) para cerrar ambigüedades del manifiesto `AmpliacionConfiguracionEntornos.md`.

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso de inicio? | **`feature`** v1.2.0 |
| Rama propuesta | `feat/ampliacion-configuracion-entornos` |
| `persist_ref` | `docs/features/ampliacion-configuracion-entornos` |
| Entorno | **Producción (IDE)** — fases Mayeuta/Dedalo en Cursor; Tekton diferido |
| Manifiesto | `docs/todos/pending/AmpliacionConfiguracionEntornos.md` |

---

## D2 — Resolución de rutas

| Pregunta | Decisión |
|----------|----------|
| ¿Ancla de `./`? | Raíz del workspace vía `repo_root()` (`SddIA/core/cumulo.paths.json` como marcador) |
| Rutas canónicas | `{repo}/.dev/.env` y `{repo}/.SddIA/.dev/.env` |
| ¿Crear directorios en runtime? | **No** — solo leer si existen; ausencia = skip silencioso (salvo log D4) |

---

## D3 — Precedencia completa (stack)

Orden de aplicación al arrancar un entrypoint:

1. **Entorno del SO** (`os.environ` heredado) — máxima precedencia; **intocable** por ficheros.
2. **`./.dev/.env`** — rellena claves ausentes en SO.
3. **`./.SddIA/.dev/.env`** — rellena/sobrescribe claves del merge (1+2) en el diccionario intermedio; al volcar a `os.environ`, respeta regla dotenv: no pisa claves ya definidas en SO.

> El manifiesto exige que (b) prevalezca sobre (a) **entre ficheros**. No altera la precedencia estándar SO > dotenv.

---

## D4 — Log de gobernanza

| Condición | Mensaje exacto (stderr) |
|-----------|-------------------------|
| Existen **ambos** `.dev/.env` y `.SddIA/.dev/.env` | `[CONFIG] Jerarquía detectada: Aplicando SddIA/.dev/.env sobre .dev/.env` |
| Solo global | Sin log obligatorio |
| Solo local | Sin log obligatorio |
| Ninguno | Sin log |

Formato: una línea; prefijo `[CONFIG]`; canal **stderr** (no rompe envelope JSON en stdout).

---

## D5 — Alcance de entrypoints

| Entrypoint | ¿Carga jerarquía? | Motivo |
|------------|-------------------|--------|
| `execute-process.py` | **Sí** | Citado en manifiesto; padre de cápsulas |
| `execute-action.py` | **Sí** | Invocado por watcher y shims; debe ver mismos secretos |
| `event-watcher.py` | **Sí** | Daemon autónomo; dispara IOTA vía acciones |
| Subprocesos Python hijos | **Heredan** `os.environ` ya cargado | Evita doble parseo |
| `iota-immutable-publisher` (Node) | **No dotenv local** | Confía en `env` del padre |

Descartado: cargar solo en `execute-process.py` — dejaría `event-watcher` e `execute-action` sin secretos IOTA en producción.

---

## D6 — Ubicación del módulo

| Opción | Decisión |
|--------|----------|
| Inline en `execute-process.py` | ❌ |
| Función en `execute_process_core.py` | ❌ mezcla responsabilidades |
| **`SddIA/scripts/qa/env_loader.py`** | ✅ importable por los tres entrypoints QA/daemon |

API mínima:

```python
def load_hierarchical_env(repo: Path) -> dict[str, str]:
    """Merge global → local; aplica a os.environ sin pisar SO; retorna claves cargadas desde ficheros."""
```

Parser: implementación propia mínima (`KEY=VALUE`, strip comillas, ignorar líneas vacías/`#`) — **sin** dependencia `python-dotenv` nueva.

---

## D7 — Migración iota-immutable-publisher

| Antes | Después |
|-------|---------|
| `dotenv.config({ path: path.join(__dirname, ".env") })` | **Eliminar** carga local |
| `.env` en `scripts/tools/iota-immutable-publisher/` | **Deprecado** — migrar secretos a `.SddIA/.dev/.env` |
| `.gitignore` línea 14 | Sustituir por `.SddIA/.dev/` y `.dev/` |

Mantener dependencia `dotenv` en `package.json` solo si otra cápsula la requiere; en `index.ts` retirar import y llamada.

Actualizar feedback de error para citar `.SddIA/.dev/.env` en lugar de `.env` local.

---

## D8 — Cúmulo SSOT

Registrar en `cumulo.paths.json`:

```json
"env_hierarchy": {
  "global": ".dev/.env",
  "instance": ".SddIA/.dev/.env"
}
```

Coherente con `eda_instance.customization` bajo `.SddIA/`.

---

## D9 — Plantillas y documentación

| Artefacto | Acción |
|-----------|--------|
| `SddIA/scripts/starter-kit/.SddIA/.dev/.env.example` | Crear con `IOTA_WALLET_SECRET`, `IOTA_ANCHOR_PACKAGE_ID`, `SDDIA_IOTA_TIMEOUT_SECONDS` |
| `.dev/.env.example` (repo root starter-kit) | Opcional — comentario de variables compartidas |
| `SddIA/tools/iota-immutable-publisher.md` | Actualizar sección Security → rutas jerárquicas |
| `SddIA/evolution/` | Entrada de evolución al merge |

---

## D10 — Git e integración

| Tema | Decisión |
|------|----------|
| Rama desde | `main` (producción) |
| Commits | Atómicos: (1) módulo + entrypoints, (2) IOTA + gitignore + Cúmulo, (3) docs feature Tekton |
| Merge | Vía `accept-pr` post-Argos APTO |

**Nota operativa:** al iniciar en IDE con rama activa distinta (`feat/pr-presented-orchestration`), el checkout a la rama feature queda **pendiente** hasta ventana limpia de git — no bloquea planificación.

---

## Preguntas abiertas

Ninguna bloqueante para Tekton. La planificación puede continuar.
