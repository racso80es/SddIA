---
feature_name: execute-process-executors-ola-1
created: "2026-11-05"
purpose: Estabilización de alcance Mayeuta — AUT-77 / Ola 1 ejecutores por clase desde execute-process
process: refactorization
branch_name: refactor/execute-process
persist_ref: docs/features/execute-process-executors-ola-1
pbi_ref: AUT-77
document_id: PBI-9e2df289
historia_ref: MAN-30
execution_id: "f41c572c-b6d1-4599-b76b-bf5cba84b7c0"
---

# Clarificación — execute-process-executors-ola-1

Transcript Mayeuta (2026-11-05). Fase **Estabilización de alcance** del proceso `refactorization`. Fuente primaria: PBI `AUT-77` (`PBI-9e2df289`) refinado (`verdict: refined`, workspace `refine-item/092915c7-…`); semilla operativa del runtime; laudos previos del Vértice Biológico sobre la no-existencia de `execute-event`.

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `refactorization` v1.2.4 |
| `feature_name` | `execute-process-executors-ola-1` |
| Rama | `refactor/execute-process` |
| `persist_ref` | `docs/features/execute-process-executors-ola-1` |
| `pbi_ref` / `document_id` | `AUT-77` / `PBI-9e2df289` |
| Padre | `MAN-30` (alcance origen; ola extraída) |
| `execution_id` | `f41c572c-b6d1-4599-b76b-bf5cba84b7c0` |
| Fase | Estabilización Mayeuta → handoff Dedalo vía `objectives.md` |
| Tracker | `provider: linear` — cuerpo SSOT en issue; caché local de refino en workspace `refine-item` |

---

## D1 — Naturaleza del corte

| Pregunta | Decisión |
|----------|----------|
| ¿Qué se liquida? | Concentración del **despacho físico** de invocación por clase (`action` / `skill` / `tool`) dentro del crate orquestador `SddIA/engine/execute-process`. |
| ¿Qué se preserva? | Orquestador Rust nativo como **aduana universal**: resolución `./sddia-run.sh`, Peaje Cerbero/RBAC, telemetría, envelope JSON, grafo de fases, `phase_invocations`, `execution_report`. |
| ¿Rediseño funcional? | **No.** Extracción incremental con paridad; no big-bang ni reescritura de consumidores CLI. |

**Toll:** el síntoma es monolito de invocación en el host; no es rediseño del códice SE ni del bus EDA.

---

## D2 — Ontología de ejecutores (laudo 1)

| Opción | Decisión |
|--------|----------|
| Ejecutores = `process` multi-fase nuevos | **Rechazada** como modelo por defecto de esta ola. |
| Ejecutores = `action` de despacho por clase + capa compartida de invocación de cápsulas | **Adoptada.** |
| Skills / tools | Siguen siendo **tipos de cápsula**, no procesos. |
| `execute-process` | Retiene orquestación + aduana; deja de concentrar el despacho físico de las tres familias. |

Handoff Dedalo: materializar frontera módulos/crates/entrypoints sin violar este laudo.

---

## D3 — Inventario cerrado Ola 1 vs diferidos (laudo 3)

| Familia | Ola 1 | Notas |
|---------|-------|-------|
| `action` | **Dentro** | Incluye bridge Python residual de acciones como inventario/etiqueta; poda total diferida salvo sub-corte con inventario vacío + gates verdes (P17). |
| `skill` | **Dentro** | Fan-out `skill:` / resolución binario·cápsula. |
| `tool` | **Dentro** | Fan-out `tool:` / resolución binario·cápsula. |
| `agent` como ejecutor | **Diferido** | Fuera de ola. |
| `process` como subprocess nuevo | **Diferido** | Más allá de la recursión ya existente. |
| Event / `execute-event` | **No aplica** | Un Event es contrato/instancia ECST; se emite vía `action:emit-*` y se enruta vía procesos (`route-domain-event`). No es clase ejecutable homóloga. Extraer EDA sería otra ola. |

---

## D4 — Genoma vs módulo interno (laudo 2)

| Caso | Decisión |
|------|----------|
| Entrypoint de dominio nuevo o promovido a superficie pública | Genoma + índice + cápsula vía vías autorizadas (`entity-manager` / proceso). |
| Reubicación interna sin superficie pública nueva | No exige genoma hasta promoción. |
| Lógica de negocio | Prohibido duplicarla fuera de cápsulas. |

---

## D5 — Contrato preservado (laudo 4)

Obligatorio mantener:

- `capsule-json-io` (stdin/stdout JSON).
- Semántica `exitCode` / errores tipados.
- Peaje Cerbero / contextos de ejecución.
- Telemetría / peaje termodinámico fail-soft.
- `./sddia-run.sh` + resolución del orquestador; **sin** reescritura de consumidores por esta ola.

---

## D6 — Estrategia de migración (laudo 5)

| Regla | Decisión |
|-------|----------|
| Big-bang | **Prohibido.** |
| Unidad de migración | Por familia / sub-familia. |
| Gate de retirada del camino monolítico | Paridad verificable (golden o smoke) de al menos un camino representativo por familia **antes** de retirar el equivalente concentrado. |
| Inventario documental | Obligatorio en el expediente `refactorization` (Ola 1 vs diferidos). |

---

## D7 — Fuera de alcance (ratificado)

- Rediseño del grafo de fases del códice Software Engineering.
- Retirada forzada del intérprete Python de todo el ecosistema.
- Cambios de UX tracker o ciclo `refine-item` / `forge-*`.
- Extracción del enrutamiento EDA / bus (`route-domain-event`).

---

## D8 — Trazabilidad

| Enlace | Exigencia |
|--------|-----------|
| `MAN-30` ↔ `AUT-77` / `PBI-9e2df289` | Bidireccional: padre conserva referencia a ola extraída; hijo cita padre. |
| Evolution | Actualizar si el cambio es material bajo `SddIA/`. |
| Git | Solo `skill:git-manager` / `./sddia-run.sh --tool git-manager`. |

---

## D9 — Criterios de Done (documentales + CA)

Heredados del PBI refinado; Argos los consumirá vía `objectives.md` + `spec.md`:

1. Inventario documentado de puntos de invocación por clase (Ola 1 vs diferidos).
2. Frontera orquestador ↔ ejecutores materializada; despacho físico desconcentrado; aduana intacta.
3. Paridad `action:` / `skill:` / `tool:` (golden/smoke ≥1 camino/familia).
4. Sin lógica de negocio duplicada fuera de cápsulas; genoma coherente si hay activos nuevos.
5. Consumidores CLI / `./sddia-run.sh` sin reescritura por esta ola.
6. Trazabilidad MAN-30 ↔ AUT-77.
7. Evolution si mutación material Core.
8. Cierre: un PR + `validacion.md` APTO + issue Linear en `state_map.done` (`provider: linear`).

---

## Estado de estabilización

| Ítem | Estado |
|------|--------|
| Preguntas ontológicas D1–D5 (refine-item) | **Cerradas** (laudos) |
| Preguntas abiertas bloqueantes | **Ninguna** |
| Handoff | `objectives.md` = `refined_requirements` → Dedalo (`spec.md` / `plan.md`) |
| Evidencia Linear live en esta sesión | Shell Auto-review bloqueó `linear-tracker-adapter`; cuerpo tomado de caché `item.refined.md` + veredicto `refined` (coherente con semilla y laudos previos) |
