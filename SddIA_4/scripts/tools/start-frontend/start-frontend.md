# start-frontend

Levantar **GesFer.Product.Front** con `npm run dev`. Puerto por defecto **3000**. Health recomendado **`/api/health`** (evita rewrites i18n). Política **`portBlocked`**: `false` (fallar si ocupado) o `kill` (solo si la implementación identifica el proceso como dev server seguro).

## CÃ³mo ejecutar

La implementaciÃ³n debe residir en la carpeta `scripts/tools/start-frontend/` en la raÃ­z de este workspace. Ejecute el launcher o binario que el equipo deposite allÃ­ (por ejemplo `.bat` o `.exe` en Windows). Los detalles de argumentos dependen de la cÃ¡psula desplegada.

## Salida

Salida coherente con el contrato de herramientas del motor (JSON en consola con `success`, `exitCode`, `message`, `result` / feedback segÃºn aplique).
