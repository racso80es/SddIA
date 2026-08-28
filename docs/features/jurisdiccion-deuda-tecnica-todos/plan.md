---
feature_name: jurisdiccion-deuda-tecnica-todos
created: "2026-08-28"
process: feature
branch_name: feat/jurisdiccion-deuda-tecnica-todos
persist_ref: docs/features/jurisdiccion-deuda-tecnica-todos
pbi_ref: docs/todos/pending/Registro y Resolución de Deuda Técnica (Kintsugi Ontológico).md
document_id: PBI-OPER-DEUDA-TECNICA-KINTSUGI-001
uuid: 4be8aeee-896a-4d2f-b2d3-3ee0d05fbd80
execution_id: "a3050468-df71-4922-bac9-3743bef2e54d"
phases:
  - l0-design
  - l1-norm-creator
  - l2-migrate-deuda
  - l3-tmp-discard
  - l4-tests-ca5
  - l5-evolution
  - l6-closure
---

# Plan — Jurisdicción docs/todos y portador de deuda

Orden: L1 → L2 → L3 → L4 → L5 → L6. Este commit sella Diseño (`clarify.md` + `objectives.md` + `spec.md` + `plan.md`). Ejecución Tekton **después** de esta parada.

## Fase L0 — Diseño (esta parada)

Artefactos bajo `persist_ref`. Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id`: `a3050468-df71-4922-bac9-3743bef2e54d`.

## Fase L1 — Norma (CA1/CA2/CA4)

```text
./sddia-run.sh --process entity-manager --inputs-file .tmp/entity-manager-<uuid>.json
```

Payload: `entity_class: norm`, `lifecycle_operation: create`, `entity_name: todos-jurisdiction`, `semantic_seed` según `spec.md` Hito 1.

Prohibido `Write` directo sobre `SddIA/library/norms/`. Prefijo RAW Kernel antes de forja. Verificar `objectives.md` de esta feature (DA-4).

## Fase L2 — Migración `DeudaTecnica/` (CA3/CA4)

`git mv` (vía `git-manager`) de los tres `.md` a `docs/todos/pending/`. Parche de frontmatter: `dispatch: false`; paths cruzados Paciente 0; no alterar `tech_debt_ids` ni `process_candidate`.

Tras move: directorio `DeudaTecnica/` vacío → eliminar.

## Fase L3 — `docs/todos/tmp/` (F-TODOS-BUCKET-HUERFANO parcial)

Cinco ficheros `consolidado`. Borrar en el mismo PR o sustituir el directorio por un único `.gitkeep` + puntero en la norma. No promover a pending.

kitchen/historias: **cero moves**.

## Fase L4 — Tests CA5

Archivo: `SddIA/engine/execute-process/src/engine/handlers/task_queue_manager.rs` (módulo de tests). Casos del spec Hito 3.

Si el predicado de archivado no es función pública: extraer helper mínimo **en el mismo crate** o testear vía el caso `extract_pbi_path` + comentario de paridad con `phase_capsules.rs:1024`. No tocar lógica de fan-out.

Comando: `cd SddIA && cargo test -p execute-process extract_pbi_path`.

## Fase L5 — Evolution

Alta registro canónico v1.1.2. `gate-evolution --range` antes de push si el diff toca `directories.evolution`.

## Fase L6 — Cierre (fuera de esta parada)

`implementation.md` + `execution.md` → Argos `validacion.md` → PBI a `done/` → `delivery-close-cycle`.

## Touchpoints

| Path | Cambio | Vía |
|------|--------|-----|
| `SddIA/library/norms/todos-jurisdiction.md` | Alta | entity-manager |
| `SddIA/library/norms/index.md` | Fila | creator |
| `docs/todos/pending/*Paciente*` / `*Escaneo*` | Move | git-manager |
| `docs/todos/DeudaTecnica/` | Baja | git-manager |
| `docs/todos/tmp/` | Descarte | git-manager |
| `task_queue_manager.rs` | Tests | IDE (no genoma) |
| `SddIA/evolution/{uuid}.md` | Hito | cápsula register |

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Despacho accidental de semillas | `dispatch: false` + TQM respeta flag si existe; si no, no emitir estímulo Kalma2 |
| Recrear `DeudaTecnica/` por hábito | Norma: destino retirado; Filtro A |
| Solape fan-out | CA6: grep del PR sin `fracture_pbi` |
