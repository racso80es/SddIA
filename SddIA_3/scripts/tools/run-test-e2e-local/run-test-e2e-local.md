# run-test-e2e-local

Orquestar entorno (opcional), comprobar `/health` de **Admin** y **Product**, compilar y ejecutar:

`dotnet test …/GesFer.Product.Back.E2ETests.csproj --filter "Category=E2E"`

con `E2E_BASE_URL` = URL Product y variables alineadas con Admin según especificación del proyecto.

## CÃ³mo ejecutar

La implementaciÃ³n debe residir en la carpeta `scripts/tools/run-test-e2e-local/` en la raÃ­z de este workspace. Ejecute el launcher o binario que el equipo deposite allÃ­ (por ejemplo `.bat` o `.exe` en Windows). Los detalles de argumentos dependen de la cÃ¡psula desplegada.

## Salida

Salida coherente con el contrato de herramientas del motor (JSON en consola con `success`, `exitCode`, `message`, `result` / feedback segÃºn aplique).
