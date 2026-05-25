---
feature_name: kaizen-alert-required-eda-v2
process: feature
created: "2026-05-25"
persist_ref: docs/features/kaizen-alert-required-eda-v2
branch_name: feat/kaizen-alert-required-eda-v2
pbi_ref: docs/todos/pending/kaizen-alert-required-eda-v2.md
document_id: PBI-KAIZEN-ALERT-REQUIRED-EDA-V2
status: implementado
updated: "2026-05-25"
related:
  - docs/features/norma-paridad-documental/
  - SddIA/process/pull-request-review.md
  - SddIA/scripts/qa/audit-doc-parity.py
  - SddIA/scripts/qa/execute_process_capsules.py
  - SddIA/agents/cumulo.md
  - SddIA/core/event-subscriptions.json
  - SddIA/events/events-contract.md
  - docs/todos/pending/Argos_Eda_Emision
---

# Objetivos — Kaizen_Alert_Required (EDA v2)

## Meta

Cerrar la **deuda EDA v2** heredada de `norma-paridad-documental`: sustituir el puente síncrono (`kaizen_items` → escritura directa `PENDING_AUDIT_DOC_*` en cápsula Kaizen) por el flujo reactivo acordado — **Aduana deposita evento; Cúmulo persiste**.

```text
Aduana (pull-request-review)  →  eda_bus.pending  →  desentendimiento total
Cúmulo (suscriptor único)     →  Kaizen_Alert_Required  →  docs/todos/pending/
```

## Contexto operativo

| Hecho | Implicación |
|-------|-------------|
| PR #46 mergeado (2026-05-25) | Sensor DIA y puente lab v1 operativos en `main` |
| PBI `PBI-KAIZEN-ALERT-REQUIRED-EDA-V2` en `pending/` | Fuente SSOT de hitos H1–H6 |
| Puente v1 en `capsule_pr_review_kaizen` | Violación desacople EDA — **extirpar** |
| `norma-paridad-documental/spec.md` §3.5 | Contrato evento documentado; implementación diferida a este PBI |

## Objetivos medibles — PBI H1–H6

| ID | Hito PBI | Objetivo | Criterio |
|----|----------|----------|----------|
| **H1-O1** | H1 | **Forja del Chispazo** | `SddIA/events/kaizen-alert-required.md` registrado en `events/index.md`; payload §4 PBI |
| **H2-O1** | H2 | **Suscripción Sistema Nervioso** | `Kaizen_Alert_Required` en `event-subscriptions.json`; **Cúmulo único suscriptor** |
| **H3-O1** | H3 | **Emisión desde Aduana** | Tras `alert_required: true`, cápsula triaje deposita ECST en `eda_bus.pending`; **cero** `kaizen_items` / `dia_audit` hacia Kaizen |
| **H4-O1** | H4 | **Poda puente síncrono** | Eliminados `_dia_audit_hash`, rama DIA en `capsule_pr_review_kaizen`, escritura directa `PENDING_AUDIT_DOC_*` |
| **H5-O1** | H5 | **Despertar Cúmulo** | Genoma + instrucciones declaran mandato: materializar cicatriz en `docs/todos/pending/` |
| **H6-O1** | H6 | **Handler reactivo E2E** | `event-watcher` + `route-domain-event` despachan a Cúmulo; smoke en `execution.md` |

## Restricción arquitectónica — ceguera espacial

| Principio | Obligación |
|-----------|------------|
| **Aduana ciega a persistencia** | Cápsulas PR review **no** escriben en `docs/todos/` post-poda |
| **Sensor inerte** | `audit-doc-parity.py` sin cambios; solo stdout JSON |
| **Payload desnormalizado** | Evento lleva `review_id`, `alert_justification`, `implicated_files` — **prohibido** acoplar a `.tmp/audit-doc-parity-*.json` |
| **Suscriptor único** | Sin fan-out Mayeuta/Argos en v1 de este evento |
| **No bloqueo aduana** | `delivery_state: success` sin esperar materialización Cúmulo |

## Criterios de aceptación (PBI §12)

| ID | Criterio |
|----|----------|
| KA-CA1 | ECST + fila `events/index.md` |
| KA-CA2 | Suscripción única Cúmulo |
| KA-CA3 | Payload ECST cumple §4 |
| KA-CA4 | Aduana deposita evento; cero escritura directa `docs/todos/` por cápsula PR review |
| KA-CA5 | Poda completa puente v1 |
| KA-CA6 | Cúmulo + handler materializan TODO; smoke E2E verde |
| KA-CA7 | `verify-process-integrity` sin regresión |
| KA-CA8 | `validacion.md` APTO + PBI en `done/` (un PR) |

## Fuera de alcance

- Cierre `Argos_Eda_Emision` / `pending_argos_eda_emission` (DLT merge post-accept-pr).
- Suscripción IOTA/DLT para `Kaizen_Alert_Required` v1.
- Cambios al sensor `audit-doc-parity.py` (salvo invocación desde emisor).

## Ley aplicada

- Proceso **`feature`** v1.3.0 + `features-documentation-pattern` v1.2.1.
- Contrato **`events-contract`** + patrón deposit `eda_bus.pending`.
- Cierre documental en rama (un PR): PBI → `done/` + `validacion.md` `pbi_archived: true`.
