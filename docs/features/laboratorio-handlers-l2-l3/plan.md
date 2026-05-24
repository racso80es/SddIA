---
feature_name: laboratorio-handlers-l2-l3
created: "2026-05-24"
process: feature
branch_name: feat/laboratorio-handlers-l2-l3
persist_ref: docs/features/laboratorio-handlers-l2-l3
tracks:
  - L.2
  - L.3
agent_planificador: dedalo
---

# Plan de implementación — Laboratorio handlers L.2 + L.3

Blueprint dual-track alineado a `clarify.md` D1–D8 y `spec.md` §3–§4.

## 0. Estado de la entrega

| Bloque | Estado | Evidencia |
|--------|--------|-----------|
| Rama de trabajo | ✅ | `feat/laboratorio-handlers-l2-l3` |
| Objetivos | ✅ | `objectives.md` |
| Clarificación | ✅ | `clarify.md` |
| Especificación | ✅ | `spec.md` |
| Plan | ✅ | Este documento |
| **Hito 1 — Track L.2** | ⏳ | Gate Impacto SddIA |
| **Hito 2 — Track L.3** | ⏳ | Fases 6–7 feature |
| **Hito 3 — Smoke + validación** | ⏳ | `validacion.md` |

---

## 1. Hito 1 — Track L.2 (`delivery-close-cycle` fase 2)

### 1.1 Cápsula impacto

- [ ] Implementar `capsule_delivery_impact_assessment(repo, inputs, state)`.
- [ ] Diff name-only `SddIA/` vía `git-manager` o subprocess acotado.
- [ ] Respetar `SDDIA_LAB_SKIP_IMPACT_ASSESSMENT`.

### 1.2 Routing

- [ ] Extender `execute_delivery_close_phase` → fase «Impacto SddIA condicional».
- [ ] Poblar `state["sddia_impact"]`; propagar a envelope `run_process` si presente.

### 1.3 Genoma

- [ ] Actualizar `SddIA/process/delivery-close-cycle.md` § Perfil laboratorio.
- [ ] Recalcular `hash_signature` si el genoma cambia materialmente.

**Criterio de salida:** L2-CA1–L2-CA4.

**Estimación:** 1 commit atómico.

---

## 2. Hito 2 — Track L.3 (`feature` fases 6–7)

### 2.1 Cierre documental

- [ ] Implementar `capsule_feature_pbi_archive`.
- [ ] Parser frontmatter mínimo de `validacion.md` (PyYAML existente).
- [ ] Move PBI `pending/` → `done/` vía operación filesystem acotada.

### 2.2 Cierre entrega

- [ ] Implementar `capsule_feature_invoke_delivery_close`.
- [ ] Routing en `execute_process_phase` para `process_def.name == "feature"`.
- [ ] Construir `child_inputs` desde `inputs` + `state["workspace"]`.

### 2.3 Honestidad fases 2–5

- [ ] Verificar que `delegates_are_only_agents` sigue marcando `simulated` (sin regresión).
- [ ] Unificar nota canónica en `execution_report` si diverge.

### 2.4 Genoma

- [ ] Actualizar `SddIA/process/feature.md` § Perfil laboratorio (matriz fase × handler).
- [ ] Recalcular `hash_signature` si aplica.

**Criterio de salida:** L3-CA1–L3-CA5.

**Estimación:** 1–2 commits atómicos.

---

## 3. Hito 3 — Smoke, documentación feature, validación

### 3.1 Fixtures

- [ ] `_smoke-delivery-close-impact-none.json` — bug-fix / sin diff SddIA.
- [ ] `_smoke-delivery-close-impact-mutation.json` — feature con diff SddIA simulado o rama real.
- [ ] `_smoke-feature-pbi-archive.json` — fase 6 con validacion stub.
- [ ] `_smoke-feature-delivery-close.json` — fase 7 con skips parciales lab.

### 3.2 Ejecución lab

- [ ] Documentar comandos en `execution.md`.
- [ ] Regresión smoke `pr-presented-orchestration/_smoke-close-cycle-presented.json`.

### 3.3 Validación Argos

- [ ] `validacion.md` — tablas L2-CA* y L3-CA*, `global: APTO`.
- [ ] Actualizar PBI post-PR11 § L.2–L.3 en rama PR.

### 3.4 Cierre feature

- [ ] `delivery-close-cycle` → PR único L.2 + L.3.

**Criterio de salida:** `validacion.md` APTO; smokes reproducibles.

---

## 4. Orden de ejecución recomendado

```text
H1 (L.2 fase 2) ──┐
H2 (L.3 fases 6–7) ──┴── H3 (smoke + validación + PR)
```

Tracks H1 y H2 pueden desarrollarse en paralelo.

---

## 5. Commits atómicos sugeridos

| # | Contenido |
|---|-----------|
| 1 | `objectives` + `clarify` + `spec` + `plan` (inicio feature) |
| 2 | H1 — gate Impacto SddIA + genoma `delivery-close-cycle` |
| 3 | H2 — cápsulas feature fases 6–7 + genoma `feature` |
| 4 | H3 — smokes + `execution.md` + `validacion.md` + sync PBI |

---

## 6. Dependencias

| Upstream | Relación |
|----------|----------|
| `pr-presented-orchestration` | Handlers L.2 fases 4–7 base |
| `vanguardia-soberania-local` | P1 cerrado; precedencia P2 |
| `refactor-execute-process-engine` | `workspace-init` fase 1 feature |
| Backlog post-PR11 | Manifiesto P2 L.2–L.3 |

---

## 7. Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Diff vacío en rama sin commits | Gate retorna `impact: none`; smoke con rama con cambios reales |
| Move PBI sin validacion APTO | Fase 6 `skipped`; no side-effect |
| Recursión feature → DC → feature | Subproceso solo invoca `delivery-close-cycle`, no `feature` |
| Hash genoma desincronizado | `verify-process-integrity.py` en H3 |

---

## 8. Definition of Done (feature)

- [ ] L2-CA1–L2-CA4 y L3-CA1–L3-CA5 verificados.
- [ ] `validacion.md` con `global: APTO`, `pbi_archived: true` según cierre documental.
- [ ] PBI § L.2–L.3 marcado ✅ en rama PR.
- [ ] Un PR mergeado.
