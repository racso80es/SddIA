# run-tests-frontend

Ejecutar en `src/` los scripts npm de lint, build, unit y e2e según `testScope`, sin invocar npm “a mano” desde el agente.

## CÃ³mo ejecutar

La implementaciÃ³n debe residir en la carpeta `scripts/tools/run-tests-frontend/` en la raÃ­z de este workspace. Ejecute el launcher o binario que el equipo deposite allÃ­ (por ejemplo `.bat` o `.exe` en Windows). Los detalles de argumentos dependen de la cÃ¡psula desplegada.

## Salida

Salida coherente con el contrato de herramientas del motor (JSON en consola con `success`, `exitCode`, `message`, `result` / feedback segÃºn aplique).
