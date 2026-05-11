# start-api

Arrancar API **Admin** con política de configuración **centralizada en JSON de cápsula** (`start-api-config.json` o nombre acordado): directorio de trabajo, puerto, health URL, `portBlocked` fail|kill, timeouts, perfil ASP.NET.

## CÃ³mo ejecutar

La implementaciÃ³n debe residir en la carpeta `scripts/tools/start-api/` en la raÃ­z de este workspace. Ejecute el launcher o binario que el equipo deposite allÃ­ (por ejemplo `.bat` o `.exe` en Windows). Los detalles de argumentos dependen de la cÃ¡psula desplegada.

## Salida

Salida coherente con el contrato de herramientas del motor (JSON en consola con `success`, `exitCode`, `message`, `result` / feedback segÃºn aplique).
