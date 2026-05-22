---
feature_name: ia-obrera-blindaje
created: "2026-05-22"
process: feature
branch_name: feat/ia-obrera-blindaje
persist_ref: docs/features/ia-obrera-blindaje
phases: 5
agent_planificador: dedalo
---

# Plan de implementación — Blindaje IA Obrera

Blueprint Tekton alineado a `objectives.md`, `clarify.md` D1–D9 y `spec.md` v1.0.0.

## 0. Estado de la entrega

| Bloque | Estado | Evidencia |
|--------|--------|-----------|
| Rama de trabajo | ✅ | `feat/ia-obrera-blindaje` |
| Clarificación (Mayeuta) | ✅ | `clarify.md` D1–D9 |
| Especificación (Dedalo) | ✅ | `spec.md` |
| Planificación (Dedalo) | ✅ | este documento |
| **Hito 1 — Norma motor** | ✅ | `external-ai-constraints.md` |
| **Hito 2 — Touchpoints** | ✅ | `.cursorrules` + `touchpoints-ia.md` |
| **Hito 3 — Creators wrap** | ✅ | 8 × `*-creator.md` |
| **Hito 4 — Evolución + EDA** | ✅ | `evolution/ef684063-…` + scan OK |
| **Hito 5 — Argos** | ✅ | `validacion.md` |

---

## 1. Hito 1 — Forja norma motor

**Objetivo:** Materializar `SddIA/norms/external-ai-constraints.md` v1.0.0.

### Tareas

- [ ] Generar UUID v4 (`crypto-broker` o equivalente manual documentado).
- [ ] Redactar cuerpo según `spec.md` §3.2 (DA-1..3, directorios, comandos, prefijo creator).
- [ ] Incluir frontmatter mínimo coherente con normas motor (`UUID`, `Versión`, `Tipo`).
- [ ] Verificar coherencia con `obediencia-procesos.md` y `paths-via-cumulo.md`.

**Criterio de salida:** archivo legible; Argos puede auditar DA-1..3 sin ambigüedad.

---

## 2. Hito 2 — Inyección touchpoint

**Objetivo:** Difundir norma hacia IDE sin violar SSOT.

### Tareas

- [ ] Añadir §8 a `.cursorrules` (`spec.md` §4.1).
- [ ] Actualizar `SddIA/norms/touchpoints-ia.md` — tabla touchpoints + referencia Jules/Windsurf (`spec.md` §4.2).
- [ ] Opcional: mención en `README.md` raíz bajo «parámetros de restricción» si ya lista `.cursorrules`.

**Criterio de salida:** `.cursorrules` enlaza norma; no contiene texto contradictorio ni copia íntegra de la norma.

---

## 3. Hito 3 — Prefijo en procesos creator (Fase B)

**Objetivo:** Envolver contexto Tekton con directriz letal en los 8 creators.

### Tareas

- [ ] Insertar sección `## Directriz de ejecución obrera` en cada archivo (`spec.md` §5.1).
- [ ] Recalcular `hash_signature` en frontmatter YAML de cada proceso modificado.
- [ ] Ejecutar `python SddIA/scripts/qa/verify-process-integrity.py` — exit 0.

| # | Archivo | Orden sugerido |
|---|---------|----------------|
| 1 | `tool-creator.md` | Piloto — validar patrón |
| 2 | `action-creator.md` | |
| 3 | `skill-creator.md` | |
| 4 | `agent-creator.md` | |
| 5 | `process-creator.md` | |
| 6 | `norm-creator.md` | |
| 7 | `codex-creator.md` | |
| 8 | `event-creator.md` | |

**Criterio de salida:** 8/8 creators con sección; integridad de procesos OK.

---

## 4. Hito 4 — Evolución y sello EDA

**Objetivo:** Trazabilidad federal + bus sin huérfanas.

### Tareas

- [ ] Crear `SddIA/evolution/{uuid}.md` con impacto y artefactos (`spec.md` §6.1).
- [ ] Invocar `entity-manager` o backfill EDA según contrato vigente (`spec.md` §6.2).
- [ ] Ejecutar `audit-entity-eda-coverage.py --scan --json` — documentar resultado en `execution.md`.

**Criterio de salida:** evolution log presente; scan EDA sin nuevas huérfanas atribuibles a esta entrega.

---

## 5. Hito 5 — Verificación Argos y cierre

**Objetivo:** Cerrar feature con `validacion.md` y PR.

### Tareas

- [ ] Generar `implementation.md` (touchpoints tocados).
- [ ] Generar `execution.md` (comandos ejecutados, hashes).
- [ ] Generar `validacion.md` con checks CA-1..CA-7 (`spec.md` §7).
- [ ] Invocar `delivery-close-cycle` con `source_process: feature`, `persist_ref`, `branch_name`.

**Criterio de salida:** PR abierto; `PullRequest_Presented` en bus; aduana `pull-request-review` pasa (si activa).

---

## 6. Orden de ejecución recomendado (Tekton)

```
Hito 1 → Hito 2 → Hito 3 → verify-process-integrity → Hito 4 → Hito 5
```

**Dependencia:** Hito 2 y 3 pueden paralelizarse tras Hito 1; Hito 4 requiere Hito 1 completo.

---

## 7. Riesgos y mitigaciones

| Riesgo | Mitigación |
|--------|------------|
| Drift `hash_signature` en creators | Recalcular con JSON canónico de `phases`; gate `verify-process-integrity` |
| `entity-manager` sin clase `norm` | Backfill `--emit` documentado en feature Ola C |
| IA ignora `.cursorrules` | Fase C hooks ya bloquean commits sin EDA; refuerzo normativo es capa adicional |
| Duplicación norma en touchpoint | Spec §4.1 limita a resumen + enlace |

---

## 8. Referencias

- TODO: `docs/todos/TODO-BLINDAJE-IA-OBRERA.md`
- Fase C cerrada: `docs/features/pbi-005-hito3-git-hooks/`
- Touchpoints SSOT: `SddIA/norms/touchpoints-ia.md`
- Proceso feature: `SddIA/process/feature.md` v1.2.0
