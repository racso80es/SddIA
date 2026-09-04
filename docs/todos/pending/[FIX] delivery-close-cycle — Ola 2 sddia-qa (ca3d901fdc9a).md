---
document_id: PBI-FIX-FRACTURE-ca3d901fdc9a-OLA2
uuid: "cab81579-7254-40e1-b5ed-8ce5e95034b7"
title: "[FIX] delivery-close-cycle — Ola 2 sddia-qa"
format: markdown
version: "1.0.0"
created: "2026-09-04"
updated: "2026-09-04"
status: "abierto"
refinement_status: clarified
priority: alta
process: bug-fix
type: fix
dispatch: false
parent_pbi: PBI-FIX-FRACTURE-ca3d901fdc9a
parent_ref: docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (ca3d901fdc9a).md
depends_on:
  - PBI-FIX-FRACTURE-ca3d901fdc9a-OLA1
suggested_branch: fix/dcc-sddia-qa-lab
persist_ref_suggested: docs/fixes/dcc-sddia-qa-lab
fracture_hash: ca3d901fdc9a
fracture_process: delivery-close-cycle
incident_ref: "System_Fracture_Detected — ca3d901fdc9a"
friction_ids:
  - F-DCC-EVOLUTION-GATE
  - F-DCC-INDEX-INTEGRITY
architectural_constraints:
  - A-QA-BIN-SSOT-TARGET
  - A-IGNITION-PRODUCE-QA
  - A-HOOK-PARITY-DCC
execution_file_lock:
  - start-sddia.sh
  - SddIA/scripts/qa/git-hooks/hook_common.sh
gates_this_wave:
  - DCC-QA-CA1
  - DCC-QA-CA2
  - DCC-QA-CA3
related:
  - SddIA/tools/sddia-qa/src/main.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/scripts/qa/git-hooks/hook_common.sh
  - SddIA/scripts/qa/git-hooks/pre_push_gate.sh
  - SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
  - docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (ca3d901fdc9a).md
  - docs/todos/pending/[FIX] delivery-close-cycle — Ola 1 cápsula git-manager (ca3d901fdc9a).md
source_audit: "2026-09-04. resolve_sddia_qa_bin y hook_common.resolve_sddia_qa solo miran SddIA/target/{debug,release}/sddia-qa. Crate existe (SddIA/tools/sddia-qa). Ignición no hace cargo build -p sddia-qa. Pre-push evolution gate y DCC aduanas colapsan con el mismo literal."
review_notes: "Ola 2 depende de Ola 1 para un DCC extremo a extremo; el binario QA es independiente y puede construirse antes. No mutar genoma tools/ (DA-2)."
---

# [FIX] delivery-close-cycle — Ola 2 sddia-qa

Absorbe `F-DCC-EVOLUTION-GATE` y `F-DCC-INDEX-INTEGRITY` del padre `PBI-FIX-FRACTURE-ca3d901fdc9a`. Mismo `fracture_hash`.

## Linaje

| Campo | Valor |
|-------|--------|
| Padre | `PBI-FIX-FRACTURE-ca3d901fdc9a` |
| Predecesora | Ola 1 (`git-manager`) — DCC no llega a aduanas si Snapshot muere antes |
| Eventos | `1760faf4-1d13-486a-b8ca-c77c082d1b98`, `65507278-56a4-4328-955b-fd44d2f13569` |
| Hueco | Aduanas DCC y hook pre-push exigen `sddia-qa`; ignición no lo materializa |

## 0. Laudo Ola 2

**Intención:** existe `SddIA/target/{debug,release}/sddia-qa` ejecutable tras ignición. `resolve_sddia_qa_bin` (motor) y `resolve_sddia_qa` (hook) dejan de devolver el literal de compile. Aduana evolution e integridad índices fallan solo por veredicto de gate, no por binario ausente.

### 0.1. Hechos (Filtro A)

| Tesis | Veredicto | Base |
|-------|-----------|------|
| Literal `sddia-qa no encontrado (compilar: cd SddIA && cargo build -p sddia-qa)` | **Hecho** | `phase_capsules.rs` `resolve_sddia_qa_bin`; `hook_common.sh` `resolve_sddia_qa` |
| Crate presente, ELF no | **Hecho** | `SddIA/tools/sddia-qa`; glob `SddIA/target/**/sddia-qa` vacío en el incidente |
| Ignición no construye QA | **Hecho** | `release_pkgs` sin `sddia-qa` |
| Mismo hueco en pre-push | **Hecho** | auditoría previa: push a `main` caía en evolution gate por binario ausente (parser aparte, ya en `cf323cd`) |
| F4b suprime fractura de evolution **blocked** | **Hecho** | `dcc_gate_block_suppresses_fracture`: solo `status == blocked` en Aduana evolution / EDA. Aquí `status: failed` por binario → sí fractura |

### 0.2. Decisiones

1. `cargo build -p sddia-qa` en `_ensure_orchestrator` (release; debug si el resolver del motor/hook lo consulta primero).
2. No cambiar la receta de paths SSOT (`debug` luego `release`). Completar el ELF, no relajar la búsqueda.
3. Genoma `SddIA/tools/sddia-qa.md`: **no** en esta ola (DA-2) salvo que `--seal-capsules` exija testigo; entonces sello vía orquestador, no Write IDE del `{name}.md`.
4. Ola 3 cubre el sobre-escalado `failed` vs `blocked`; esta ola no toca `emit_dcc_phase_fractures`.

## 1. Alcance

| Archivo | Cambio |
|---------|--------|
| `start-sddia.sh` | `-p sddia-qa` en el lote de ignición |

### Fuera de esta ola

- Cápsula `git-manager` (Ola 1).
- Suprimir `System_Fracture_Detected` por receta de compile (Ola 3).
- Relajar `--if-touched` / predicado evolution.

## 2. Criterios de aceptación (Ola 2)

| ID | Criterio | Verificación |
|----|----------|--------------|
| DCC-QA-CA1 | Tras ignición, `SddIA/target/release/sddia-qa` o `debug/` es ELF ejecutable. | `test -x` + mime ELF |
| DCC-QA-CA2 | `sddia-qa gate-evolution --json --range --if-touched --sync-base` arranca (exit ≠ receta «no encontrado»). Veredicto EVOL_* es otro CA. | CLI directo |
| DCC-QA-CA3 | DCC no falla Aduana evolution ni integridad índices con traza `sddia-qa no encontrado`. | acuse JSON |

## Criterio de cierre

- [ ] DCC-QA-CA1…CA3
- [ ] Argos APTO en `validacion.md` del fix
- [ ] Este TODO en `docs/todos/done/` en la rama del PR
