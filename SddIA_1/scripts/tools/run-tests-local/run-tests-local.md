# run-tests-local

Ejecutar tests (unit / integration / e2e / all) invocando `dotnet test`, con orquestación opcional de entorno (`prepare-full-env`, `invoke-mysql-seeds`).

## CÃ³mo ejecutar

La implementaciÃ³n debe residir en la carpeta `scripts/tools/run-tests-local/` en la raÃ­z de este workspace. Ejecute el launcher o binario que el equipo deposite allÃ­ (por ejemplo `.bat` o `.exe` en Windows). Los detalles de argumentos dependen de la cÃ¡psula desplegada.

## Salida

Salida coherente con el contrato de herramientas del motor (JSON en consola con `success`, `exitCode`, `message`, `result` / feedback segÃºn aplique).
