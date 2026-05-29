---
feature_name: kaizen-event-creator-event-family-explicit
created: "2026-05-29"
process: refactorization
items_applied:
  - "H1 spec.md"
  - "H2 event-creator v1.2.0"
  - "H3 run_event_forge fractal + creator_inputs_from_entity"
  - "H4 seeds lab/smoke"
  - "H5 docs Fase 1 D1.9 cerrado"
  - "H6 test_event_forge_fractal"
---

# Ejecución — Kaizen event-creator event_family explícito

## Smokes

```powershell
cd c:\Proyectos\SddIA
python -m unittest SddIA.scripts.qa.test_event_forge_fractal -v
python SddIA/scripts/qa/test_eda_bus_v3plus.py
python SddIA/scripts/qa/recalc-process-hash-signatures.py SddIA/process/event-creator.md --write
python SddIA/scripts/qa/verify-process-integrity.py
```

## Forja sin familia (debe fallar)

```powershell
python SddIA/scripts/qa/execute-process.py --process entity-manager --inputs "{\"entity_class\":\"event\",\"entity_name\":\"neg-test\",\"lifecycle_operation\":\"create\",\"semantic_seed\":{\"event_name\":\"neg-test\",\"event_type\":\"Neg_Test\",\"payload_required\":[]}}"
```

Esperado: error `event_family es obligatorio`.

## Forja con familia explícita (lab local)

Usar `scope: local` y teardown; incluir `"event_family": "domain"` en `semantic_seed`.
