---
feature_name: kaizen-pec-subscribers-circuit-audit
created: "2026-08-17"
process: feature
branch_name: feat/kaizen-pec-subscribers-circuit-audit
persist_ref: docs/features/kaizen-pec-subscribers-circuit-audit
pbi_ref: docs/todos/done/[Kaizen] Process_Execution_Completed — suscriptores y auditoría de circuito EDA.md
document_id: PBI-KAIZEN-PEC-SUBSCRIBERS-CIRCUIT-AUDIT
uuid: fe8d3d21-ebeb-4a83-8b53-f2d7f0c19b16
source_incident: e273713c-dd91-487b-8716-1bdc8c5da741
correlation_id: ""
status: stabilized
verdict: ok
---

# Objetivos — kaizen-pec-subscribers-circuit-audit

## Misión

Cerrar el circuito ciego de `Process_Execution_Completed`: fan-out real (Telegram + testigo durable **XOR** política de purge/archivo) de modo que `GET /api/status` deje de 404 post-route; y automatizar en `event-bus-audit` la detección de `EMPTY_SUBSCRIBERS`, `FAMILY_MISMATCH`, `ORPHAN_REGISTRY_KEY` y `PURGE_BLACKHOLE`.

## Problema (ley aplicada)

Registro orquestación con PEC `[]` + `purge_after=true` → consenso vacuoso → borrado del JSON. Dual-signal Kalma2 asume PEC durable por `correlation_id`; el bridge proyecta 404 → timeout UI 120s. Filtro C: el bridge lee; el nervio entrega — no suscribir el bridge.

## Objetivos

| ID | Objetivo | Criterio de cierre |
|----|----------|-------------------|
| **O1** | Suscriptores de interés en PEC | Registro no vacío; fan-out real; circuito humano (Telegram) + **exactamente una** de: testigo durable por `correlation_id` **o** política no-unlink/archivo PEC |
| **O2** | Auditoría cobertura catálogo ↔ registro | `event-bus-audit` (o wrapper laudo) detecta los cuatro códigos; `PURGE_BLACKHOLE` / empty orchestration → `Kaizen_Alert_Required` |

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| **AC-O1-FANOUT** | `Process_Execution_Completed` ≥1 suscriptor indexado; fan-out observado en testigos |
| **AC-O1-STATUS** | Tras route+purge del padre, `GET /api/status?event_id=<cid>` proyecta `completed`\|`failed`\|`initialized`\|`awaiting_agents` — no 404 persistente ni timeout 120s ciego |
| **AC-O1-TG** | Telegram (o exención Dedalo documentada) notifica cierre con `process_name` + `correlation_id` |
| **AC-O1-XOR** | S2 testigo durable **XOR** S3 política purge/archivo — no ambas |
| **AC-O2-CODES** | Audit emite `EMPTY_SUBSCRIBERS`, `FAMILY_MISMATCH`, `ORPHAN_REGISTRY_KEY`, `PURGE_BLACKHOLE`; umbral orchestration alerta Kaizen |
| **AC-DOC** | `validacion.md` APTO + PBI en `docs/todos/done/` en el mismo PR |

## Invariantes

- `correlation_id ≡ event_id` del `Kalma2_Process_Requested` (L3 vigente).
- Un solo evento de cierre: `Process_Execution_Completed` (prohibido inventar segundo).
- Mutación genoma / registros Cúmulo solo vía cadena autorizada (`entity-manager` / proceso `feature`).
- Git solo vía `skill:git-manager`.

## Fuera de alcance

- SSE / canal telemetría de progreso Kalma2.
- PPR #174+#177 / umbrales Radamanto.
- Reabrir PBI-044 / dual-signal.
- Bridge como emisor o suscriptor; IOTA o Radamanto-on-PEC; Telegram en `Kalma2_Process_Requested`.
- Cableado obligatorio de H2–H5 (solo hallazgos O2 en v1).

## Handoff

Dedalo consume este cuerpo como `refined_requirements` y resuelve L-O1-XOR (S2 vs S3), touchpoints y `spec.md`/`plan.md`. Ver `clarify.md` D3–D6.
