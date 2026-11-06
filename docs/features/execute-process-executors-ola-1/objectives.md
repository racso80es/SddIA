---
feature_name: execute-process-executors-ola-1
created: "2026-11-05"
process: refactorization
branch_name: refactor/execute-process
persist_ref: docs/features/execute-process-executors-ola-1
pbi_ref: AUT-77
document_id: PBI-9e2df289
historia_ref: MAN-30
execution_id: "f41c572c-b6d1-4599-b76b-bf5cba84b7c0"
related:
  - docs/features/migracion-execute-process-rust/objectives.md
  - docs/features/migracion-execute-process-rust/implementation.md
  - SddIA/norms/capsule-json-io.md
  - SddIA/actions/actions-contract.md
  - SddIA/process/process-contract.md
---

# Objetivos — execute-process-executors-ola-1

## Misión

Desconcentrar del crate `SddIA/engine/execute-process` el **despacho físico** de invocación por clase (`action`, `skill`, `tool`) mediante ejecutores especializados, preservando el orquestador Rust nativo como **aduana universal** (resolución `./sddia-run.sh`, Peaje Cerbero/RBAC, telemetría, contrato JSON stdin/stdout, grafo de fases, `phase_invocations`, `execution_report`).

Esta ola es el primer corte extraído de **MAN-30**; el alcance operativo vive en **AUT-77** / `PBI-9e2df289`.

## Contexto operativo

| Hecho | Implicación |
|-------|-------------|
| Orquestador ya es binario Rust soberano (`migracion-execute-process-rust`) | Esta ola no porta sustrato; **extrae** responsabilidades de invocación |
| Despacho concentrado en `engine/capsules.rs`, `phase_capsules.rs`, `actions.rs`, CLI `--tool`/`--action` | Inventario Ola 1 debe etiquetar estos puntos vs diferidos |
| Bridge `execute-action.py` residual (D-P5.1-R / P17) | Inventariar y etiquetar; poda total **fuera** salvo sub-corte con inventario vacío + gates verdes |
| Event ≠ clase ejecutable | Emisión/enrutado EDA fuera de alcance de esta ola |
| `tracker.provider: linear` | Cierre documental sin ficheros nuevos en `docs/todos/{pending,done}/` |

## Objetivos medibles

| ID | Objetivo | Criterio de verificación |
|----|----------|--------------------------|
| **O1** | **Inventario por clase** | Expediente `refactorization` documenta puntos de invocación Ola 1 (actions/skills/tools) vs diferidos |
| **O2** | **Frontera orquestador ↔ ejecutores** | `execute-process` no concentra el despacho físico de las tres familias; entrypoints extraídos pasan por la misma aduana (Cerbero + telemetría + JSON I/O) |
| **O3** | **Paridad action** | Golden/smoke de al menos un camino `action:` representativo vs pre-extracción |
| **O4** | **Paridad skill** | Golden/smoke de al menos un camino `skill:` representativo vs pre-extracción |
| **O5** | **Paridad tool** | Golden/smoke de al menos un camino `tool:` representativo vs pre-extracción |
| **O6** | **Sin duplicar negocio** | Lógica de dominio permanece en cápsulas; activos nuevos (si los hay) con genoma/índice/cápsula coherentes |
| **O7** | **Compatibilidad CLI** | `./sddia-run.sh` / resolución orquestador / contratos documentados **sin** reescritura de consumidores por esta ola |
| **O8** | **Trazabilidad padre** | MAN-30 ↔ AUT-77 / `PBI-9e2df289` enlazados bidireccionalmente |
| **O9** | **Evolution** | Registro en `SddIA/evolution/` si el cambio es material bajo `SddIA/` |
| **O10** | **Cierre documental** | Un PR + `validacion.md` APTO + issue Linear en `state_map.done` |

## Alcance (Ola 1)

1. Inventariar recorridos concentrados: `invoke_action`, fan-out `skill:`/`tool:`, bridge Python residual de acciones, resolución binarios/cápsulas.
2. Definir frontera donde el orquestador conserva SSOT de proceso, grafo de fases, peaje y reporte; los ejecutores asumen despacho físico por familia.
3. Modelar ejecutores como **`action` de despacho por clase** + capa compartida de invocación de cápsulas (skills/tools = tipos de cápsula).
4. Materializar de forma **incremental** (por familia/sub-familia) con paridad antes de retirar el camino monolítico equivalente.
5. Entrypoints de dominio nuevos/promovidos → genoma vía vías autorizadas; reubicación interna sin superficie pública no exige genoma hasta promoción.

## Fuera de alcance

- Rediseño del grafo de fases del códice Software Engineering.
- Agents como ejecutor; nuevos entrypoints process-as-subprocess más allá de la recursión existente.
- Extracción del enrutamiento EDA / creación de `execute-event`.
- Retirada forzada de Python en todo el ecosistema; poda total del bridge de acciones salvo sub-corte P17 con inventario vacío + gates verdes.
- Cambios de UX tracker o ciclo `refine-item` / `forge-*`.
- Big-bang de migración.

## Ley aplicada

- **Jerarquía:** Acción → Agente → Skill → Tools; ejecución en cápsulas; JSON stdin/stdout (`capsule-json-io`).
- **Aduana:** Cerbero + telemetría obligatorios en entrypoints extraídos.
- **Soberanía de rutas:** `SddIA/core/cumulo.paths.json`.
- **Genoma (DA-2):** mutaciones de familia vía `entity-manager` / proceso autorizado.
- **Git:** exclusivamente `skill:git-manager`.
- **Backlog Core-self:** `provider: linear` — sin materializar PBI en `docs/todos/`.
- **Documentación:** `features-documentation-pattern` v1.3.1 bajo `persist_ref`.
