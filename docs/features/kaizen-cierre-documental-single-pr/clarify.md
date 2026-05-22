---
feature_name: kaizen-cierre-documental-single-pr
created: "2026-05-22"
process: feature
version_clarify: "1.0.0"
---

# Clarificación — Un PR, sin datos post-merge

## 1. Problema confirmado

| Paso actual | Fricción |
|-------------|----------|
| Merge PR código (#32) | OK |
| Actualizar `validacion.md` con `merged_pr`, `merge_commit` | Solo conocido **después** del merge |
| Mover PBI a `done/` | Mismo commit documental |
| Push a `main` | **Bloqueado** por `pre-push` (soberanía) |
| Segundo PR `docs/*` (#33) | Obligatorio |

## 2. Laudo de diseño

### 2.1 `validacion.md` — fase única (pre-merge)

| Campo | Obligatorio pre-merge | Notas |
|-------|----------------------|-------|
| `global` | Sí | `APTO` / `NO_APTO` |
| `branch` | Sí | Rama del PR |
| `checks` | Sí | Criterios Argos |
| `git_changes` | Sí | Paths del diff |
| `pr_url` | Recomendado | Inyectado por `delivery-close-cycle` antes del merge |
| `pbi_archived` | Sí | `true` si PBI ya está en `docs/todos/done/` **en esta rama** |
| `merged_pr` | **No** | Opcional auditoría; prohibido como gate de Done |
| `merge_commit` | **No** | Inferible: `git log main -1` / API GitHub |
| `closed` | **No** | Fecha de merge no necesaria en artefacto |

### 2.2 PBI — archivado en la rama del PR

1. Antes de merge: mover `docs/todos/pending/…` → `docs/todos/done/…` en **`feat/*` o `fix/*`**.
2. Frontmatter PBI: `status: listo_para_merge` → tras merge humano puede quedar `cerrado` sin segundo commit (o operador marca `cerrado` en el mismo PR si aún abierto).

### 2.3 Definición de Done (nueva)

```text
Done = un PR mergeado en main
     + validacion.md APTO en el diff de ese PR (pbi_archived: true)
     + PBI en docs/todos/done/ incluido en ese mismo PR
```

**Prohibido:** exigir commit documental adicional a `main` post-merge.

## 3. Opciones evaluadas

| Opción | Descripción | Decisión |
|--------|-------------|----------|
| A | Revocar Fase B en norma + Cursor | **Elegida** (Hito 1) |
| B | Hook `post-merge` que parchea `validacion.md` en `main` con `SDDIA_SKIP_HOOKS` | Rechazada — sigue mutando `main` fuera del PR |
| C | `archive-task-pbi` en `accept-pr` | Fase posterior (O4 opcional) |

## 4. Impacto en operador IA

- Al cerrar tarea: mover PBI y completar `validacion.md` **antes** de `delivery-close-cycle` / merge, en la rama de trabajo.
- No abrir PR `docs/cerrar-pbi-*` salvo deuda histórica.
