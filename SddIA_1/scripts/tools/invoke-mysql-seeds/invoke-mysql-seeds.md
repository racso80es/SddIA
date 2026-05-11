# invoke-mysql-seeds

Comprobar MySQL, aplicar migraciones EF Core y ejecutar seeds Admin (p. ej. `RUN_SEEDS_ONLY=1`), según política del proyecto.

## CÃ³mo ejecutar

La implementaciÃ³n debe residir en la carpeta `scripts/tools/invoke-mysql-seeds/` en la raÃ­z de este workspace. Ejecute el launcher o binario que el equipo deposite allÃ­ (por ejemplo `.bat` o `.exe` en Windows). Los detalles de argumentos dependen de la cÃ¡psula desplegada.

## Salida

Salida coherente con el contrato de herramientas del motor (JSON en consola con `success`, `exitCode`, `message`, `result` / feedback segÃºn aplique).
