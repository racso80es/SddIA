---
feature_name: ampliacion-configuracion-entornos
created: "2026-05-22"
process: feature
purpose: Jerarquía de Bóvedas — .dev/.env global vs .SddIA/.dev/.env local
updated: "2026-05-22"
---

# Clarificación — Jerarquía de Bóvedas

Transcript de decisiones (2026-05-22), ampliado con mandato estratégico Ola A.

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso de inicio? | **`feature`** v1.2.0 |
| Nombre operativo | **Jerarquía de Bóvedas** |
| Rama | `feat/ampliacion-configuracion-entornos` ✅ |
| `persist_ref` | `docs/features/ampliacion-configuracion-entornos` |
| Manifiesto | `docs/todos/pending/AmpliacionConfiguracionEntornos.md` |

---

## D2 — Resolución de rutas (bóvedas)

| Pregunta | Decisión |
|----------|----------|
| ¿Ancla de `./`? | `repo_root()` — marcador `SddIA/core/cumulo.paths.json` |
| Bóveda global | `{repo}/.dev/.env` |
| Bóveda instancia | `{repo}/.SddIA/.dev/.env` |
| ¿Crear directorios? | **No** — lectura condicional; ausencia = skip |

---

## D3 — Precedencia completa (stack)

1. **SO** (`os.environ` heredado) — intocable por ficheros.
2. **`./.dev/.env`** — global; rellena dict intermedio.
3. **`./.SddIA/.dev/.env`** — local; **sobrescribe** claves del merge (2) en dict intermedio.
4. Volcado a `os.environ` vía `setdefault` — no pisa SO.

---

## D4 — Log de gobernanza

| Condición | Mensaje (stderr) |
|-----------|------------------|
| Ambos ficheros existen | `[CONFIG] Jerarquía detectada: Aplicando SddIA/.dev/.env sobre .dev/.env` |
| Solo uno o ninguno | Sin log obligatorio |

---

## D5 — Alcance de entrypoints (refinado)

| Punto de carga | ¿Obligatorio? | Motivo |
|----------------|---------------|--------|
| `execute-process.py` | **Sí** | Puerta CLI citada en manifiesto estratégico |
| `execute_process_capsules.run_process()` | **Sí** | Núcleo pre-cápsula; cubre imports directos del intérprete |
| `execute-action.py` | Sí (complemento) | Watcher y shims autónomos |
| `event-watcher.py` | Sí (complemento) | Daemon sin pasar por execute-process |
| Subprocesos / cápsulas Node | Heredan env | Sin re-cargar; idempotencia D3 |
| `iota-immutable-publisher` | Sin dotenv local | Env ya inyectado por padre |

**Laudo:** la Tarea estratégica exige `execute-process.py` + `execute_process_capsules.py`. Los entrypoints autónomos se mantienen para cobertura operativa completa.

---

## D6 — Módulo cargador

| Decisión | Valor |
|----------|-------|
| Ubicación | `SddIA/scripts/qa/env_loader.py` |
| API | `load_hierarchical_env(repo_root: Path) -> dict[str, str]` |
| Parser | Propio; sin `python-dotenv` |

---

## D7 — Migración IOTA

| Antes | Después |
|-------|---------|
| `dotenv.config(__dirname/.env)` | **Eliminado** |
| Secretos en `scripts/tools/iota-immutable-publisher/.env` | Migrar a `.SddIA/.dev/.env` |

---

## D8 — Cúmulo SSOT

```json
"env_hierarchy": {
  "global": ".dev/.env",
  "instance": ".SddIA/.dev/.env"
}
```

---

## D9 — Plantillas y genoma

| Artefacto | Acción |
|-----------|--------|
| `starter-kit/.SddIA/.dev/.env.example` | Crear |
| `SddIA/tools/iota-immutable-publisher.md` | Security → Jerarquía de Bóvedas |
| `SddIA/evolution/` | Entrada al merge |

---

## D10 — Git e integración

| Tema | Decisión |
|------|----------|
| Merge | `accept-pr` post-Argos APTO |
| Commits | Uno por hito 0.1 / 0.2 / 0.3 + docs |

---

## D11 — Prioridad estratégica Ola A (nuevo)

| Pregunta | Decisión |
|----------|----------|
| ¿Posición en backlog Ola A? | **Hito 0** — Jerarquía de Bóvedas |
| ¿Relación con pasivos técnicos? | Ejecutar **antes** de cualquier resolución de pasivo restante |
| ¿Gate? | Argos APTO en 0.3 desbloquea hooks, deuda CLI residual y faenas laboratorio dependientes de env |

---

## D12 — Sanitización de sistema (Hito 0.3)

| Acción | Detalle |
|--------|---------|
| Eliminar `.env` dispersos | Auditar `SddIA/scripts/tools/**`; retirar SSOT local IOTA |
| `.gitignore` | Verificar `.dev/` y `.SddIA/.dev/`; retirar regla puntual `iota-immutable-publisher/.env` |
| Verificación | Grep: cero `dotenv.config` en cápsulas tools post-merge |

---

## Preguntas abiertas

Ninguna bloqueante. Tekton puede iniciar Hito 0.1.
