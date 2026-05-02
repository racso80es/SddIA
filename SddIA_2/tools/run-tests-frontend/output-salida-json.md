---
toolId: run-tests-frontend
contract_ref: SddIA/tools/tools-contract.json
spec_ref: SddIA/tools/run-tests-frontend/spec.md
---

# Salida JSON — run-tests-frontend

## Tabla codificada de salidas (tools-contract.output.output_codes_table)

| exitCode | success | message_resumen | data_presente | descripción |
|----------|---------|-----------------|---------------|-------------|
| 0 | true | "Tests completados correctamente" | Sí: scope, [lint_exit], [build_exit], [unit_exit], [e2e_exit] | Éxito: todos los tests del scope pasaron. |
| 1 | false | "npm install fallo" | Sí: scope | npm install falló antes de ejecutar tests. |
| 1 | false | "Tests con fallos" | Sí: scope, lint_exit/build_exit/unit_exit/e2e_exit según fallos | Algún test del scope falló. |
| 1 | false | "Error: &lt;excepción&gt;" | Sí: scope | Excepción no controlada durante la ejecución. |

*Nota:* Los códigos de salida de npm (lint, build, unit, e2e) se propagan cuando fallan; el exitCode final puede ser distinto de 1 si npm devuelve otro valor.
