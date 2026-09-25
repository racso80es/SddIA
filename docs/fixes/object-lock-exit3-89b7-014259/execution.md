---
feature_name: object-lock-exit3-89b7-014259
created: "2026-09-25"
process: bug-fix
branch_name: fix/object-lock-exit3-89b7-014259
persist_ref: docs/fixes/object-lock-exit3-89b7-014259
pbi_ref: docs/todos/pending/[FIX] route-domain-event — fractura sistémica (89b7c8b105ec).md
document_id: PBI-FIX-FRACTURE-89b7c8b105ec
execution_id: "2b45bcaf-60ea-46bc-ae41-cb2102d92925"
items_applied:
  - dlt-object-lock-predicate
  - mayeuta-lock-and-exit3
  - kalma-exit3-suppress
  - genome-enrich-1.5.0
  - evolution-alta
---

# Ejecución — sellos `89b7c8b105ec` y `0142599491fb`

## Init

```bash
SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_ALLOW_DIRTY=1 SDDIA_LAB_SKIP_PBI_ARCHIVE=1 SDDIA_LAB_SKIP_DELIVERY_CLOSE=1 \
  ./sddia-run.sh --process bug-fix --inputs-file /tmp/bug-fix-object-lock-exit3-init.json
```

`execution_id`: `2b45bcaf-60ea-46bc-ae41-cb2102d92925`. Diseño `simulated`. Commit planificación `9be6143`.

## Tests

```text
cd SddIA && cargo test -p execute-process --lib -- dlt_transient emit_dlt_batch_fracture analyze_fracture_kaizen
# 25 passed; 0 failed

cd SddIA && cargo test -p kalma2-bridge -- prosthetic_exit
# prosthetic_exit_3_does_not_emit_fracture ok
```

## Genoma

`entity-manager` update `enrich-fracture-pbi-kaizen` v1.5.0. `hash_signature: sha256:0ab82805973f62844946b7cdef5b6e183703ac83d29290adab942d4f9982818f`. El sello de bus reutilizó `ab2f5558-16ae-462a-8a1f-ac49811893d2` (`idempotent: true`): el resolutor local casa UUID + `update`, no el hash nuevo. El artefacto en disco es 1.5.0.

## Evolution

`sddia-qa evolution-register` → `c9c0206e-aed4-4e48-a17d-b28d1de43d46` (`EVOL_OK`). La aduana DCC marcó `EVOL_MATERIAL_UNREGISTERED` porque `SddIA/actions/index.md` no estaba en `relacionado`. Misma ficha, `modificacion`, con el índice incluido.

## Cierre

`validacion.md` APTO (`pbi_archived: true`) y ambos PBI en `done/` tras el run [36101108875](https://github.com/racso80es/SddIA/actions/runs/36101108875).
