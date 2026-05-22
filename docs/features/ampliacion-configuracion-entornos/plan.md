---
feature_name: ampliacion-configuracion-entornos
created: "2026-05-22"
process: feature
phases: 5
agent_planificador: dedalo
---

# Plan de implementación — Ampliación configuración de entornos

Blueprint para Tekton. Entradas: `objectives.md`, `clarify.md`, `spec.md`.

## 0. Estado de la entrega

| Bloque | Estado | Evidencia |
|--------|--------|-----------|
| Rama de trabajo | ⏳ | `feat/ampliacion-configuracion-entornos` (checkout pendiente) |
| Clarificación (Mayeuta) | ✅ | `clarify.md` D1–D10 |
| Especificación (Dedalo) | ✅ | `spec.md` |
| Planificación (Dedalo) | ✅ | este documento |
| Implementación (Tekton) | ⏳ | — |
| Verificación (Argos) | ⏳ | — |

---

## Fase 1 — Módulo cargador (Python)

**Touchpoints:** `SddIA/scripts/qa/env_loader.py` (nuevo)

| Paso | Acción |
|------|--------|
| 1.1 | Implementar `parse_dotenv_file` con tests inline o script smoke en `tmp/` |
| 1.2 | Implementar `load_hierarchical_env` + log D4 |
| 1.3 | `apply_env` con `setdefault` |

**DoD:** importable desde los tres entrypoints; sin dependencias pip nuevas.

---

## Fase 2 — Cableado entrypoints

**Touchpoints:**

- `SddIA/scripts/qa/execute-process.py`
- `SddIA/scripts/qa/execute-action.py`
- `SddIA/scripts/daemons/event-watcher.py`

| Paso | Acción |
|------|--------|
| 2.1 | Import + `load_hierarchical_env(repo)` en punto pre-cápsula de cada CLI |
| 2.2 | Verificar que subprocesos heredan env (no duplicar llamada en `shim_execute_action`) |

**DoD:** arranque sin ficheros `.env` no falla; con ficheros de prueba en `tmp/` el merge es correcto.

---

## Fase 3 — Migración IOTA + infraestructura

**Touchpoints:**

- `SddIA/scripts/tools/iota-immutable-publisher/index.ts`
- `.gitignore`
- `SddIA/core/cumulo.paths.json`
- `SddIA/tools/iota-immutable-publisher.md`
- `SddIA/scripts/starter-kit/.SddIA/.dev/.env.example`

| Paso | Acción |
|------|--------|
| 3.1 | Retirar `dotenv.config` de `index.ts`; ajustar mensajes |
| 3.2 | Actualizar `.gitignore` (`.dev/`, `.SddIA/.dev/`) |
| 3.3 | Registrar `env_hierarchy` en Cúmulo |
| 3.4 | Plantilla `.env.example` en starter-kit |
| 3.5 | Actualizar tool.md Security |

**DoD:** grep sin `path.join(__dirname, ".env")` en IOTA; Cúmulo válido JSON.

---

## Fase 4 — Documentación de ejecución

**Touchpoints:** `implementation.md`, `execution.md` bajo `persist_ref`

| Paso | Acción |
|------|--------|
| 4.1 | `implementation.md` — touchpoints y guía migración operador |
| 4.2 | `execution.md` — registro de smoke manual |

**DoD:** frontmatter válido según `features-documentation-pattern`.

---

## Fase 5 — Verificación Argos

| Check | Comando / evidencia |
|-------|---------------------|
| Smoke loader | Script Python temporal: crear `.dev/.env` + `.SddIA/.dev/.env` en tmp, assert merge + log |
| CLI arranque | `execute-process.py --process feature --inputs '{"feature_name":"smoke"}'` (fase simulated OK) |
| IOTA lab | `SDDIA_LAB_SIMULATE_IOTA=1` sin `.env` local en cápsula |
| Integridad JSON | `python -m json.tool SddIA/core/cumulo.paths.json` |
| EDA orphans | Solo si se muta genoma indexado — esta feature toca Cúmulo JSON, **no** entidades `.md` nuevas |

Salida: `validacion.md` con veredicto APTO/NO APTO.

---

## Fase 6 — Cierre (post-Argos)

Delegar `delivery-close-cycle` con:

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

1. `feat(config): add hierarchical env_loader module`
2. `feat(config): wire env hierarchy in QA entrypoints and watcher`
3. `refactor(iota): remove local dotenv; cumulo env_hierarchy + gitignore`
4. `docs(feature): ampliacion-configuracion-entornos implementation and validation`

---

## Dependencias y paralelismo

| Feature relacionada | Relación |
|--------------------|----------|
| `pbi-005-hito3-git-hooks` | Independiente |
| `remove-cli-legacy-compat` | Sin conflicto |
| Laboratorios Vía C | Beneficiados por starter-kit |

---

## Handoff Tekton

**Entradas listas:** `objectives.md`, `clarify.md`, `spec.md`, `plan.md`.

**Próximo paso:** checkout `feat/ampliacion-configuracion-entornos` desde `main` → Fase 1.
