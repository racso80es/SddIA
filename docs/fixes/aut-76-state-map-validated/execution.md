---
feature_name: aut-76-state-map-validated
created: "2026-11-06"
process: bug-fix
branch_name: fix/aut-76-state-map-validated
persist_ref: docs/fixes/aut-76-state-map-validated
pbi_ref: AUT-76
document_id: PBI-222b4def
uuid: 222b4def-325a-4645-80d3-fbb15bf7d34b
execution_id: "6c28414f-8a36-4f66-b9e3-4969dc642978"
---

# Ejecución — AUT-76

## T0

Rama `fix/aut-76-state-map-validated`. `bug-fix` denegado por la misma DA. Planificación en relevo IDE.

## T1–T2

```bash
cd SddIA && cargo test -p execute-process --lib contract_accepts_state_map_validated_and_rejects_unknown_key
cd SddIA && cargo test -p execute-process --lib state_rank_orders_cycle
cd SddIA && cargo test -p execute-process --lib discard_when_issue_already_at_or_past_target
cd SddIA && cargo build -p execute-process
```

1/1 OK cada test. Binario `SddIA/target/debug/execute-process` recompilado.

## T6 — Desbloqueo DA

```bash
SDDIA_AGENT_RELAY_IDE=1 ./sddia-run.sh --process refine-item --inputs '{"item_ref":"AUT-76","project_slug":"core"}'
```

Acuse: `success: true`. **No** `clave desconocida 'validated'`. `execution_id` `d8795abe-e6f1-44fb-823e-3dbe9730136f`. `status: awaiting_agents` (Mayeuta simulated; DoR/cuerpo de AUT-76 fuera de este fix). AUT-76 es el PBI de fractura de esta misma DA.

## T5

`sddia-qa evolution-register` → `6f997575-80ab-431f-ae2a-9fa296eb69f6`.

## T7

`validacion.md` APTO. Cierre vía `delivery-close-cycle`. Sin `docs/todos/`.
