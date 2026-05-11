# postman-mcp-validation

Ejecutar colección Postman/Newman contra la API alcanzable en `baseUrl` (en esta línea, default típico **5020** si apunta a Product).

## CÃ³mo ejecutar

La implementaciÃ³n debe residir en la carpeta `scripts/tools/postman-mcp-validation/` en la raÃ­z de este workspace. Ejecute el launcher o binario que el equipo deposite allÃ­ (por ejemplo `.bat` o `.exe` en Windows). Los detalles de argumentos dependen de la cÃ¡psula desplegada.

## Salida

Salida coherente con el contrato de herramientas del motor (JSON en consola con `success`, `exitCode`, `message`, `result` / feedback segÃºn aplique).
