# run-test-e2e-local

Automatizar pruebas E2E **HTTP** contra la **API Admin** en local: smoke (health, swagger, login), lectura de empresas, CRUD empresa y CRUD usuario. No confundir con la variante **SddIA_3** (`dotnet test` sobre `GesFer.Product.Back.E2ETests`).

## CÃ³mo ejecutar

La implementaciÃ³n debe residir en la carpeta `scripts/tools/run-test-e2e-local/` en la raÃ­z de este workspace. Ejecute el launcher o binario que el equipo deposite allÃ­ (por ejemplo `.bat` o `.exe` en Windows). Los detalles de argumentos dependen de la cÃ¡psula desplegada.

## Salida

Salida coherente con el contrato de herramientas del motor (JSON en consola con `success`, `exitCode`, `message`, `result` / feedback segÃºn aplique).
