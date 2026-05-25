---
feature_name: norma-paridad-documental
created: "2026-05-25"
process: feature
purpose: DIA — Declaración de Impacto de Artefactos y sensor audit-doc-parity con desacople EDA
---

# Clarificación — Norma de Paridad Documental (DIA)

Transcript de decisiones (2026-05-25).

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso de inicio? | **`feature`** v1.3.0 |
| Rama | `feat/norma-paridad-documental` |
| `persist_ref` | `docs/features/norma-paridad-documental` |
| PBI | `docs/todos/pending/norma-paridad-documental.md` (`PBI-NORMA-PARIDAD-DOCUMENTAL`) |
| Upstream | `docs/features/pull-request-review-redesign/` (aduana v2), `SddIA/templates/` (catálogo motor) |

---

## D2 — Triaje de bloqueos (pre-planificación)

| Área | Estado | Veredicto |
|------|--------|-----------|
| Genoma `pull-request-review` v2.0.0 | ✅ En `main` | **Sin bloqueo** — extensible en § Triaje técnico |
| Handler `capsule_pr_review_technical` | ✅ Invoca `verify-process-integrity` | **Sin bloqueo** — patrón de encadenar sensor QA |
| Handler `capsule_pr_review_kaizen` | ✅ Persiste TODO desde `kaizen_items` | **Sin bloqueo** — puente lab v1 sin EDA |
| Plantilla `spec-template` en `SddIA/templates/` | ❌ Ausente (solo en labs `SddIA_1`…) | **No bloqueante** — H1 crea plantilla canónica |
| Norma `features-documentation-pattern` vs `spec.json` | ⚠️ Tensión aparente | **Resuelto** — JSON permitido en **plantillas motor** (`templates-contract`); prohibido en artefactos de tarea feature |
| Evento `Kaizen_Alert_Required` en bus | ❌ No existe | **No bloqueante v1** — contrato documentado; lab usa `kaizen_items` |
| `.tmp/` gitignored | ✅ | **Sin bloqueo** — alert file efímero |

**Conclusión:** no hay bloqueos estructurales. Planificación puede continuar.

---

## D3 — Alcance DIA: rutas monitorizadas

| Pregunta | Decisión |
|----------|----------|
| ¿Solo `SddIA/core/`? | **No** — incluir también `SddIA/process/`, `SddIA/scripts/qa/` (mutaciones genoma y gates) |
| ¿`README.md` raíz? | **Sí** — si aparece en diff y `impacts_doc: false`, alerta |
| ¿Cambios solo en `docs/features/<persist_ref>/`? | **Exento** — paridad documental de la propia feature no auto-dispara |
| ¿Cambios en `docs/todos/`? | **Exento** — Kaizen ya es deuda explícita |

Prefijos por defecto (override vía `--monitored-paths`):

```text
SddIA/core/
SddIA/process/
SddIA/scripts/qa/
README.md
```

---

## D4 — Semántica `impacts_doc`

| Valor | Comportamiento sensor |
|-------|----------------------|
| `impacts_doc: true` | Si diff toca rutas monitorizadas **y** sección `### Impacto en Documentación` no vacía → **OK** (exit 0, `alert_required: false`) |
| `impacts_doc: true` + sección vacía | **Alerta** — declaración inconsistente |
| `impacts_doc: false` + diff monitorizado | **Alerta** — posible fuga de conocimiento |
| Clave ausente en frontmatter | Tratar como **`false`** (asunción de omisión) |
| `impacts_doc: false` + diff solo en `persist_ref` docs | **OK** — sin mutación estructural externa |

---

## D5 — Contrato salida sensor (H3 — ceguera espacial)

| Pregunta | Decisión |
|----------|----------|
| ¿Exit 1 en alerta? | **Prohibido** — PBI Filtro B + Fricción Suave |
| Exit codes | `0` = éxito o alerta documental; `2` = error operativo (repo, args, spec ilegible) |
| Formato stdout | JSON obligatorio con `--json` (default en invocación cápsula) |
| Campos mínimos | `success`, `alert_required`, `reason`, `monitored_hits`, `persist_ref`, `impacts_doc` |
| Archivo alerta | Opcional `--alert-file .tmp/audit-doc-parity-{id}.json` — mismo payload |
| ¿Llamar a Cúmulo? | **Prohibido** en el script |

---

## D6 — Integración aduana (H2 + H4 declarativo)

| Pregunta | Decisión |
|----------|----------|
| ¿Fase del sensor? | **Triaje técnico** (junto a `verify-process-integrity`) — no bloquea si integrity falla vs alerta DIA |
| ¿`delivery_state: failed` por DIA? | **Nunca** |
| Puente lab v1 | Cápsula parsea JSON → append `state["kaizen_items"]` con mensaje estructurado; fase Cosecha Kaizen persiste TODO |
| Puente EDA v2 (deuda) | Orquestador deposita `Kaizen_Alert_Required` en bus; Cúmulo suscriptor async |
| Nombre TODO Kaizen | `PENDING_AUDIT_DOC_{hash8}.md` bajo `docs/todos/pending/` (prefijo acordado PBI) |
| Hash | SHA256 truncado de `(persist_ref + monitored_hits sorted)` |

Regla genoma (texto declarativo, no imperativo en sensor):

> Tras invocar `audit-doc-parity.py`, si `alert_required: true`, registrar deuda Kaizen en fase Cosecha sin alterar veredicto de bloqueo.

---

## D7 — Plantilla H1 (`spec-template`)

| Pregunta | Decisión |
|----------|----------|
| Ubicación | `SddIA/templates/spec-template/` (nuevo template_id) |
| `spec.json` | Metadato catálogo motor (`templates-contract`) — **no** SSOT de tarea feature |
| `spec.md` plantilla | Frontmatter ejemplo con `impacts_doc: false` + sección DIA vacía como placeholder |
| Índice | Añadir fila en `SddIA/templates/index.md` |

---

## D8 — Validación empírica (PBI §5)

| Paso | Acción |
|------|--------|
| 1 | Feature smoke: alterar stub bajo `SddIA/core/` o fixture controlado con `impacts_doc: false` |
| 2 | Ejecutar `audit-doc-parity.py` contra diff simulado |
| 3 | Confirmar exit 0 + `alert_required: true` |
| 4 | Ejecutar aduana lab (`pull-request-review`) → TODO Kaizen en `docs/todos/` |
| 5 | Confirmar PR/aduana **no** en estado `failed` por DIA |

---

## D9 — Orquestación implementación (preview plan)

| Hito | Entrega |
|------|---------|
| H1 | Plantilla DIA + índice templates |
| H2 | Regla en `pull-request-review.md` |
| H3 | `audit-doc-parity.py` + tests manuales documentados |
| H4 | Extensión cápsula triaje técnico + contrato EDA en spec (sin suscripción bus v1) |
| H5 | Smoke + `validacion.md` + cierre PBI en rama PR |
