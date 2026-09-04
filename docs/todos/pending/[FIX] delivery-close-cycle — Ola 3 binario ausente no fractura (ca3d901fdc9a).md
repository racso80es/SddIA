---
document_id: PBI-FIX-FRACTURE-ca3d901fdc9a-OLA3
uuid: "d68066cf-5165-4602-9707-15024b951bdb"
title: "[FIX] delivery-close-cycle — Ola 3 binario ausente no fractura"
format: markdown
version: "1.0.0"
created: "2026-09-04"
updated: "2026-09-04"
status: "abierto"
refinement_status: clarified
priority: media
process: bug-fix
type: fix
dispatch: false
parent_pbi: PBI-FIX-FRACTURE-ca3d901fdc9a
parent_ref: docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (ca3d901fdc9a).md
suggested_branch: fix/dcc-lab-missing-binary-no-fracture
persist_ref_suggested: docs/fixes/dcc-lab-missing-binary-no-fracture
fracture_hash: ca3d901fdc9a
fracture_process: delivery-close-cycle
incident_ref: "System_Fracture_Detected — ca3d901fdc9a"
friction_ids:
  - F-DCC-SNAPSHOT-FINAL
  - F-DCC-PUBLICACIN-REMOTA
  - F-DCC-EVOLUTION-GATE
  - F-DCC-INDEX-INTEGRITY
architectural_constraints:
  - A-F4B-BINARIO-LAB
  - A-FRACTURE-SOLO-ONTOLOGIA
  - A-COMPILE-RECIPE-NO-KINTSUGI
execution_file_lock:
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
gates_this_wave:
  - DCC-NF-CA1
  - DCC-NF-CA2
  - DCC-NF-CA3
related:
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/norms/obediencia-procesos.md
  - docs/fixes/dcc-simulated-barrier-c51acf014c0f/
  - docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (ca3d901fdc9a).md
source_audit: "2026-09-04. Cuatro System_Fracture_Detected en un ciclo cuyo error es receta de compile. dcc_gate_block_suppresses_fracture solo status=blocked en Aduana evolution/EDA. status=failed por binario ausente no está en F4b/red/hook-evol/workflow-scope. Cúmulo materializó PBI; Mayeuta no clasificó causa (process_fix genérico). Lab incompleto ≠ colapso de proceso oficial."
review_notes: "Ola 3 no sustituye construir los ELF (Ola 1–2). Evita Kintsugi por higiene de target/. Engine execute-process no es genoma DA-2."
---

# [FIX] delivery-close-cycle — Ola 3 binario ausente no fractura

Absorbe el **sobre-escalado** de las cuatro fricciones del padre: la traza es accionable (compilar) y no debe abrir circuito Kintsugi.

## Linaje

| Campo | Valor |
|-------|--------|
| Padre | `PBI-FIX-FRACTURE-ca3d901fdc9a` |
| Precedente normativo | F4b (gate evolution / red transitoria / workflow scope / título) en `delivery_close.rs` |
| Hueco | `failed` + literal de binario ausente → `emit_dcc_phase_fractures` |

## 0. Laudo Ola 3

**Intención:** si `error_trace` es receta de compile de cápsula skill o de `sddia-qa`, DCC permanece `failed`/`blocked` en el acuse JSON y **no** emite `System_Fracture_Detected`. El operador ve el literal; Cúmulo no abre PBI de fractura sistémica.

### 0.1. Hechos (Filtro A)

| Tesis | Veredicto | Base |
|-------|-----------|------|
| Cuatro fracturas, dos literales de compile | **Hecho** | pending `72a41559`, `aedcea75`, `1760faf4`, `65507278` @ 2026-09-04T08:12:14Z |
| F4b no cubre este caso | **Hecho** | `dcc_gate_block_suppresses_fracture` exige `blocked` + nombre de aduana; aquí `failed` |
| Mayeuta: causa no clasificada | **Hecho** | padre v1.0.0 `process_fix` genérico |
| Ola 1–2 eliminan el síntoma en lab completo | **Hecho** | no eliminan la clase de error en otro clon sin `cargo build` |

### 0.2. Decisiones

1. Añadir predicado `dcc_lab_binary_missing_suppresses_fracture(phase, status, trace)` en `delivery_close.rs`, simétrico a F4b.
2. Match de traza (literales ya emitidos, case-insensitive):
   - `cápsula skill '` … `no encontrada bajo SddIA/target`
   - `sddia-qa no encontrado`
3. Fases cubiertas: Snapshot final, Publicación remota, Aduana evolution, Aduana integridad índices (y cualquier fase que reutilice el mismo resolver).
4. **No** `fail_soft`: el ciclo sigue fallando. Solo se omite el evento de dominio.
5. Test unitario del predicado + que `emit_dcc_phase_fractures` no materialice pending ante fixture con esas trazas.

## 1. Alcance

| Archivo | Cambio |
|---------|--------|
| `SddIA/engine/execute-process/src/engine/delivery_close.rs` | predicado + llamada en `emit_dcc_phase_fractures` + tests |

### Fuera de esta ola

- Construir `git-manager` / `sddia-qa` (Ola 1–2).
- Cambiar mensaje de error (los literales son el contrato del match).
- Suprimir fracturas de `git-manager` revocado (RBAC) o de gate EVOL_* real.

## 2. Criterios de aceptación (Ola 3)

| ID | Criterio | Verificación |
|----|----------|--------------|
| DCC-NF-CA1 | Traza `sddia-qa no encontrado` en Aduana evolution/índices → no hay JSON `System_Fracture_Detected` nuevo. Acuse `success: false`. | test `emit_dcc_phase_fractures` |
| DCC-NF-CA2 | Traza `cápsula skill 'git-manager' no encontrada bajo SddIA/target` en Snapshot/Publicación → igual. | test |
| DCC-NF-CA3 | Traza de gate evolution real (`evolution gate (--range --if-touched) failed`) y revocación RBAC **siguen** el comportamiento previo (no ampliar el match). | test negativo |

## Criterio de cierre

- [ ] DCC-NF-CA1…CA3
- [ ] Argos APTO en `validacion.md` del fix
- [ ] Este TODO en `docs/todos/done/` en la rama del PR
