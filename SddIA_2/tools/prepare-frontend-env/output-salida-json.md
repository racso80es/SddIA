---
toolId: prepare-frontend-env
contract_ref: SddIA/tools/tools-contract.json
spec_ref: SddIA/tools/prepare-frontend-env/spec.md
---

# Salida JSON — prepare-frontend-env

## Tabla codificada de salidas (tools-contract.output.output_codes_table)

| exitCode | success | message_resumen | data_presente | descripción |
|----------|---------|-----------------|---------------|-------------|
| 0 | true | "Entorno frontend preparado" | Sí: env_local_exists, env_created | Éxito: npm install OK, .env verificado. |
| 1 | false | "npm install fallo" | Sí: duration_ms | npm install falló. |
| 2 | false | "Error: &lt;excepción&gt;" | Sí (vacío) | Excepción no controlada. |
| 3 | false | "Directorio frontend no encontrado" | Sí: path | src/ no existe. |
