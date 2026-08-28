## Resumen

El init de laboratorio podía colgarse indefinidamente: la bóveda reinyectaba `AGENT_RUNTIME_COMMAND` aunque el relevo lo llevara el IDE, y el motor esperaba al hijo sin techo ni entierro del grupo de procesos.

- **Techo y entierro**: `wait` acotado a 660s, spawn en PGID con kill de grupo al vencer, guarda de reentrada por `SDDIA_AGENT_RUNTIME_DEPTH` y flag `SDDIA_AGENT_RELAY_IDE` para ceder el turno al IDE sin desconfigurar la bóveda.
- **Paridad de bóveda**: `_sddia_load_vault` pasa a setdefault con la misma precedencia que `env.rs`; el entorno del invocador deja de quedar pisado.
- **Trazabilidad**: el `execution_id` del motor viaja a payload, prompt, handoff y stub, con guarda `persist-execution-id-conflict`.
- **Circuito daemon**: clase `daemon` en el piloto de `entity-manager` (v1.0.2 forjado por proceso), fail-soft de `residual_runner` extirpado y censo del índice sincronizado.
- **Higiene**: el init aborta sobre árbol sucio y el snapshot deja de capturar entradas sin trackear ajenas bajo `docs/todos/`.

## Verificación

- Suite: `268 passed; 0 failed; 1 ignored`
- Smokes LAB-CA1…CA11: `9/9 PASS` vía `.tmp/smoke-lab-init.sh`
- `validacion.md`: `global: APTO`, `pbi_archived: true`

Cierra `PBI-KAIZEN-FEATURE-LAB-INIT-FRICTIONS`.