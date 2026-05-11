# run-tests-local

Orquestar entorno opcional y ejecutar `dotnet test` por alcance. En esta línea, la URL base E2E por defecto suele apuntar a **API Product** (`5020`), no Admin.

## CÃ³mo ejecutar

La implementaciÃ³n debe residir en la carpeta `scripts/tools/run-tests-local/` en la raÃ­z de este workspace. Ejecute el launcher o binario que el equipo deposite allÃ­ (por ejemplo `.bat` o `.exe` en Windows). Los detalles de argumentos dependen de la cÃ¡psula desplegada.

## Salida

Salida coherente con el contrato de herramientas del motor (JSON en consola con `success`, `exitCode`, `message`, `result` / feedback segÃºn aplique).
