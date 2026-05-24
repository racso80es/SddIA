---
feature_name: e1-iota-ci
created: "2026-05-24"
process: feature
branch_name: feat/e1-iota-ci
persist_ref: docs/features/e1-iota-ci
tracks:
  - E.1
agent_planificador: dedalo
---

# Plan de implementación — E.1 IOTA CI

## 0. Estado de la entrega

| Bloque | Estado |
|--------|--------|
| Objetivos + clarify | ✅ |
| Especificación | ✅ `spec.md` |
| H1 — Script smoke | ✅ |
| H2 — Route digest | ✅ |
| H3 — Workflow CI | ✅ |
| H4 — Validación + PR | ⏳ |

---

## 1. Hito 1 — Script `run-iota-ci-smoke.py`

- [x] Fixture `_smoke-iota-ci-merged.json`
- [x] Modos `--simulate` / `--require-physical`
- [x] Limpieza artefactos bus post-smoke

**Criterio:** E1-CA1, E1-CA2, E1-CA3.

---

## 2. Hito 2 — Observabilidad digest en router

- [x] `_invoke_iota_publisher` retorna digest
- [x] `route_domain_event` expone `data.transaction_digest`

**Criterio:** E1-CA4.

---

## 3. Hito 3 — Workflow GitHub Actions

- [x] Job `eda-iota-smoke-simulate`
- [x] Job `eda-iota-physical` con guard fork + secret
- [x] `npm ci` en `iota-immutable-publisher`

**Criterio:** E1-CA1, E1-CA6.

---

## 4. Hito 4 — Validación y cierre

- [ ] Smoke local `--simulate`
- [ ] `execution.md` + `implementation.md`
- [ ] `validacion.md` APTO
- [ ] Actualizar PBI § E.1
- [ ] PR + merge

---

## 5. Commits sugeridos

| # | Contenido |
|---|-----------|
| 1 | docs inicio + spec + plan |
| 2 | script + route digest + workflow |
| 3 | validación + sync PBI |

---

## 6. Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Secret IOTA ausente en GitHub | Job físico skip con log; simulate siempre verde |
| Testnet inestable | Timeout 45s; reintentos en `dlt_bus_materializer` (futuro) |
| `npx tsx` lento en CI | `npm ci` + cache node opcional Kaizen |
