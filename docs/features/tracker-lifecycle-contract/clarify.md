---
feature_name: tracker-lifecycle-contract
created: "2026-10-22"
process: feature
purpose: Laudos de id, cuerpo auditado y fila única antes de forjar el contrato
execution_id: "3e0a8c0f-1b97-4b9c-a467-b149494b3e28"
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-01
---

# Clarificación — tracker-lifecycle-contract

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id` `3e0a8c0f-1b97-4b9c-a467-b149494b3e28`. Rama `feat/tracker-lifecycle-contract`. Mayeuta…Argos: simulated / phase-barrier; relevo IDE.

## Decisiones

| ID | Laudo |
|----|-------|
| L-ID | El id es `tracker:lifecycle`. El contrato y el fichero son `tracker.lifecycle`. |
| L-BODY | `validate_output_payload` recibe `cap.body`. `emit_v2` siempre escribe `success`, `exitCode` y `message`. `result` solo en éxito. `meta` y `error` viajan en el mismo objeto. |
| L-SCHEMA | Required = `success` + `exitCode`. `additionalProperties: true`. Copiar `doc.closure` (`exitCode` + `data`) rechazaría la cápsula. |
| L-ROW | Una fila. Provider `tool:linear-tracker-adapter`. Este PBI no condiciona el resolver: eso es el PBI 14. Con `file` no hay `di_binding`. |
| L-FORGE | `norm-creator` update no preserva `catalog`. Write atómico de la taxonomía + `emit-domain-mutation`. El schema JSON no es entidad `{name}.md`. |
| L-PROVIDES | La tool aún no declara `provides`. El filtro A (id en catálogo) pasa. El gate siguiente responde `CAPABILITY_PROVIDER_MISMATCH` hasta el PBI 02. |
