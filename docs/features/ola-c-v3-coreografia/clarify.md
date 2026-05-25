---
feature_name: ola-c-v3-coreografia
created: "2026-05-25"
process: feature
purpose: Cierre documental y consolidación Ola C V3 — coreografía asíncrona (Estado de Suscriptores)
updated: "2026-05-25"
---

# Clarificación — Ola C V3: Coreografía Asíncrona

Transcript de decisiones (2026-05-25). Reinicio formal del track **P4** del manifiesto operativo post-PR11 tras triaje de código en `main`.

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso de inicio? | **`feature`** v1.3.0 — **cierre y consolidación**, no greenfield |
| Rama | `feat/ola-c-v3-coreografia-cierre` (nueva iteración; PR #24 ya mergeó `feat/ola-c-v3-coreografia`) |
| `persist_ref` | `docs/features/ola-c-v3-coreografia` |
| Manifiesto operativo | `docs/todos/pending/[OPERATIVO] Backlog pendiente post-PR11 — Hito 3, Ola C y laboratorio.md` § Prioridad 4 |
| Manifiesto arquitectura | `docs/todos/done/[ARQUITECTURA] Especificación Técnica Avanzada_ El Genoma de Eventos y Coreografía Asíncrona (Ola C) V3.md` |
| Upstream entregado | PR #24 (coreografía base), #25 (topología simétrica V3+), #27 (retirada acción route), #29 + fixes Kaizen (sweep inline + terminalización DL) |

---

## D2 — Triaje: directivas tácticas vs estado real en `main`

| Directiva (contexto inyectado) | Estado | Evidencia |
|--------------------------------|:------:|-----------|
| **1. Inmutabilidad del padre** — ECST en `pending/` solo lectura | ✅ | `route_domain_event_core.py` no escribe en `pending/`; cabeceras replicadas en `processing/`/`processed/`/`dead-letter/` vía `ensure_state_header` |
| **2. Testigos atómicos** `[UUID].[SUSCRIPTOR].json` | ✅ | `eda_bus_utils.write_processing_witness` / `promote_witness`; rutas `*/subscribers/` (V3+) |
| **3. Fallo asimétrico** — DL no colapsa padre | ✅ | `promote_witness(..., to_state="dead-letter")`; padre permanece hasta sweep o Kaizen terminal |
| **4. Recolección diferida** — `event-sweeper.py` | ✅ | `SddIA/scripts/daemons/event-sweeper.py` + `try_sweep_event`; alerta Kaizen en stderr |
| Middleware mueve testigos processing → processed/dead-letter | ✅ | `promote_witness` en `_handle_subscriber` |
| Prohibición eliminación en caliente del padre | ⚠️ Parcial | Sweeper **y** route invocan `try_sweep_event` al cierre (PR #29) — doble vía operativa, no violación de inmutabilidad durante procesamiento |
| Subcarpetas `receipts/` + sello `.procesado`/`.error` | ❌ Obsoleto | Sustituido por testigos JSON (decisión V3 §3.3 spec) — **no reimplementar** |
| Recibos `[UUID].[PURPOSE].notificado` | ❌ Obsoleto | Sustituido por `[UUID].[subscriber_id].json` |
| `delivery_state` en JSON padre | ⚠️ Residual | Campo legacy en plantillas ECST (`{}` al emitir); lectura **in-memory** para digest IOTA en route; **prohibido mutar padre en disco** (`events-contract.md`) |
| Fan-out asíncrono | ✅ | `ThreadPoolExecutor` en route; `SDDIA_LAB_ROUTE_SYNC=1` para regresión |
| Watcher ciego → proceso | ✅ | `event-watcher.py` → `execute-process route-domain-event` |
| CI / tests automatizados | ⚠️ Parcial | `test_eda_bus_v3plus.py` + `run-eda-e2e-lab.py`; **sin** job CI dedicado al sweeper daemon |

---

## D3 — Desviación topológica aceptada (V3 → V3+)

La spec original de esta feature (`spec.md` §3.2) describe testigos bajo `.events/subscribers/{processing,processed,dead-letter}/` planos.

**En producción (PR #25)** la topología evolucionó a **simétrica por estado**:

```
.events/
  pending/[UUID].json
  processing/[UUID].json + processing/subscribers/[UUID].[sid].json
  processed/[UUID].json + processed/subscribers/...
  dead-letter/[UUID].json + dead-letter/subscribers/...
```

| Pregunta | Decisión |
|----------|----------|
| ¿Revertir a topología plana V3? | **No** — V3+ es SSOT; actualizar `spec.md` en fase Dedalo |
| Feature de referencia topológica | `docs/features/refactor-topologia-eventos-ola-c-v3/` (APTO, PR #25) |
| Relación | Esta feature **consolida** la visión coreográfica; el refactor documenta el delta estructural |

---

## D4 — Alcance de esta iteración (cierre)

| Incluido | Excluido |
|----------|----------|
| Actualizar `objectives.md`, `spec.md`, `validacion.md` con estado real | Nuevo genoma ECST / suscriptores de negocio |
| Sincronizar manifiesto backlog P4 → ✅ con brechas explícitas | Reintroducir `receipts/` o sufijos `.procesado` |
| Documentar doble vía sweep (inline route + daemon) | Migración histórico bus |
| Opcional: smoke CI sweeper (`--once`) en workflow existente | Daemon sweeper permanente en producción |
| Cierre documental pre-merge (`validacion.md` APTO, PBI archivado si aplica) | L1-O5 runbooks (P1 residual aparte) |

---

## D5 — Brechas residuales (objetivos medibles)

| ID | Brecha | Prioridad |
|----|--------|-----------|
| **C3-R1** | Backlog operativo § P4 desactualizado (marca ⏳ componentes ya en `main`) | Alta |
| **C3-R2** | Manifiesto arquitectura §5 pendiente (fecha 2026-05-19) | Media |
| **C3-R3** | `validacion.md` sin frontmatter APTO ni `delivery-close-cycle` | Alta |
| **C3-R4** | Referencias normativas a `delivery_state` como criterio de éxito del bus (p. ej. E1-O2) vs testigos | Media |
| **C3-R5** | Job CI opcional: `event-sweeper.py --once` post E2E lab | Baja |
| **C3-R6** | `clarify.md` ausente (este documento) | ✅ Resuelto |

---

## D6 — Criterio de Done (feature)

```text
Done = PR único mergeado en main
  + validacion.md global: APTO
  + backlog P4 actualizado (Ola C V3 coreografía ✅ o brecha residual acotada)
  + spec alineada a topología V3+ simétrica
```

No reabrir PR #24/#25/#27/#29 salvo regresión demostrada.
