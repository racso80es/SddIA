---
feature_name: l1-o5-runbooks-paridad
created: "2026-05-25"
process: feature
branch_name: feat/l1-o5-runbooks-paridad
persist_ref: docs/features/l1-o5-runbooks-paridad
tracks:
  - L1-O5
  - D.2
agent_planificador: dedalo
---

# Plan de implementación — L1-O5 Runbooks paridad

Blueprint alineado a `clarify.md` D1–D9 y `spec.md` §3–§8.

## 0. Estado de la entrega

| Bloque | Estado | Evidencia |
|--------|--------|-----------|
| Rama de trabajo | ✅ | `feat/l1-o5-runbooks-paridad` |
| Objetivos | ✅ | `objectives.md` |
| Clarificación | ✅ | `clarify.md` |
| Especificación | ✅ | `spec.md` |
| Plan | ✅ | Este documento |
| **Hito 1 — Runbook SSOT** | ✅ | `runbook-accept-pr.md` |
| **Hito 2 — Legacy + norma** | ✅ | Banners + enlace normativo |
| **Hito 3 — Gate QA** | ✅ | `verify-runbook-paridad.py` |
| **Hito 4 — Smoke + validación** | ✅ | `execution.md`, `validacion.md` |
| **Hito 5 — Cierre documental** | ✅ | FIX + PBI → `done/` |

---

## 1. Hito 1 — Runbook canónico `runbook-accept-pr.md`

### Tareas

- [ ] Redactar runbook según `spec.md` §3.1 (8 secciones obligatorias).
- [ ] Incluir plantillas JSON con rutas a fixtures existentes (no duplicar JSON salvo fixture mínimo local).
- [ ] Tabla anti-patrones vs vía canónica.
- [ ] Diagrama mermaid cadena `review` → `accept-pr` → `watcher` (opcional, recomendado).

### Criterio de salida

L1O5-CA1; revisión manual: operador puede fusionar PR local sin consultar `git-manager.py`.

**Estimación:** 1 commit atómico (`docs: runbook accept-pr SSOT`).

---

## 2. Hito 2 — Banners legacy + enlace normativo

### 2.1 Banners execution.md históricos

- [ ] `docs/features/pbi-005-hito2-action-engine/execution.md` — banner + wrapper `runbook-historical`.
- [ ] `docs/features/pbi-005-debt-liquidation/execution.md` — idem.
- [ ] `docs/features/pbi-005-hito3-git-hooks/execution.md` — idem.

### 2.2 Norma

- [ ] Añadir referencia a `runbook-accept-pr.md` en `pull-request-orchestration.md` §6.
- [ ] Nota breve en `git-operations.md` §3 si aporta paridad.

### Criterio de salida

L1O5-CA2, L1O5-CA3.

**Estimación:** 1 commit atómico (`docs: banners runbook legacy + norma`).

---

## 3. Hito 3 — Gate `verify-runbook-paridad.py`

### Tareas

- [ ] Implementar script según `spec.md` §6.
- [ ] Exentar bloques `<!-- runbook-historical -->`.
- [ ] Exentar `docs/todos/done/**`.
- [ ] Probar contra repo pre/post cambios (debe fallar antes, verde después).
- [ ] Documentar invocación en runbook § «Verificación».

### Criterio de salida

L1O5-CA4.

**Estimación:** 1 commit atómico (`feat(qa): verify-runbook-paridad gate`).

---

## 4. Hito 4 — Smoke, implementación y validación Argos

### 4.1 Smoke documentado

- [ ] Ejecutar smoke con fixture vanguardia o ola-b.
- [ ] Registrar en `execution.md` (stdout JSON, `closed_branch` / `hygiene_failure`).
- [ ] Redactar `implementation.md` con touchpoints tocados.

### 4.2 Validación

- [ ] `validacion.md` — checks V-L1O5-1..6, `global: APTO`.
- [ ] Ejecutar `verify-process-integrity.py` + `verify-runbook-paridad.py`.
- [ ] Actualizar `objectives.md` § Estado → implementación ✅.

### Criterio de salida

L1O5-CA5, V-L1O5-6.

**Estimación:** 1 commit (`docs: execution + validacion L1-O5`).

---

## 5. Hito 5 — Cierre documental en rama (fase 6 feature)

### Tareas

- [ ] Mover FIX `delete_branch` → `docs/todos/done/`.
- [ ] Mover manifiesto post-PR11 → `docs/todos/done/`; `status: cerrado`, versión bump.
- [ ] `validacion.md`: `pbi_archived: true`.
- [ ] Enlace bidireccional FIX ↔ vanguardia ↔ esta feature.

### Criterio de salida

L1O5-CA6, L1O5-CA7, PBI-O1..O3.

**Estimación:** Incluido en commit cierre o commit dedicado pre-PR.

---

## 6. Hito 6 — Cierre de entrega (fase 7 feature)

- [ ] `delivery-close-cycle` → PR único.
- [ ] `PullRequest_Presented` vía subproceso estándar.
- [ ] Post-merge: `accept-pr` sobre rama feature (meta — comer el propio dogfood).

**Fuera de planificación actual** — ejecutar en fase implementación.

---

## 7. Orden de ejecución

```text
H1 (runbook SSOT)
    └── H2 (banners + norma)
            └── H3 (gate QA)
                    └── H4 (smoke + validacion)
                            └── H5 (PBI/FIX done/)
                                    └── H6 (PR)
```

Secuencia estrictamente lineal — H3 depende de banners para gate verde.

---

## 8. Commits atómicos sugeridos

| # | Contenido |
|---|-----------|
| 1 | `objectives` + `clarify` + `spec` + `plan` (planificación) |
| 2 | `runbook-accept-pr.md` |
| 3 | banners legacy + norma |
| 4 | `verify-runbook-paridad.py` |
| 5 | `implementation.md` + `execution.md` + `validacion.md` |
| 6 | FIX + PBI → `done/` + sync manifiesto |

---

## 9. Dependencias

| Upstream | Relación |
|----------|----------|
| `vanguardia-soberania-local` | L1-O1–O4 código + genoma |
| `pbi-005-hito3-ola-b` | Fixtures smoke + nota hooks |
| `pull-request-orchestration.md` | SSOT normativo §4 |
| Manifiesto post-PR11 | PBI a archivar |

---

## 10. Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Gate demasiado agresivo (falsos positivos en genomas proceso) | Excluir `SddIA/process/*.md` delegaciones; solo `docs/features` activos |
| Operador ignora runbook | Gate CI + norma enlazada |
| Recalcular `hash_signature` normas | Solo si diff material en frontmatter genoma |
| PBI movido antes de L1-O5 ✅ | Gate: `pbi_archived` solo con V-L1O5-* verdes |

---

## 11. Definición de done (esta feature)

```text
Done = PR mergeado en main
     + runbook SSOT + gate verde
     + validacion.md APTO (pbi_archived: true)
     + PBI post-PR11 en docs/todos/done/
     + FIX delete_branch en docs/todos/done/
```
