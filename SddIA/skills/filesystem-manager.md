---
uuid: "f4a5b6c7-d8e9-4f0a-1b2c-3d4e5f6a7b8c"
name: "filesystem-manager"
version: "1.0.0"
contract: "skills-contract v1.1.0"
context: "filesystem-ops"
capabilities:
  - "file-read"
  - "file-write"
  - "list-directory"
  - "delete-file"
  - "create-directory"
  - "move-file"
inputs:
  - "operation": "Enum estricto: [READ_FILE, WRITE_FILE, LIST_DIR, DELETE_FILE, CREATE_DIR, MOVE_FILE]"
  - "target_path": "Ruta relativa al directorio raíz del proyecto"
  - "content": "(Opcional) Cadena de texto o binario requerido para operaciones WRITE_FILE"
  - "destination_path": "(Opcional) Ruta de destino para operaciones MOVE_FILE"
outputs:
  - "exitCode": "0 para éxito, 1 para error"
  - "data": "Contenido del archivo (READ_FILE) o array de strings con el listado (LIST_DIR)"
  - "error_log": "Descripción detallada si exitCode es 1 (ej. 'File not found', 'Permission denied')"
---

# Skill: Filesystem Manager (Operaciones Core)

## 1. Propósito y Naturaleza
El `filesystem-manager` es la interfaz física principal con el disco duro del Vértice Biológico. Su misión es ejecutar la lectura, mutación y borrado de artefactos físicos dentro del perímetro autorizado, eliminando la necesidad de que el agente alucine comandos crudos de terminal como `cat`, `echo` o `mkdir`.

## 2. Motor de Ejecución (Modalidad LLM-Native)
Esta skill opera en modalidad *LLM-Native*. No posee una cápsula binaria externa asociada. La propia IA en el entorno de desarrollo actúa como el runtime de ejecución, debiendo traducir el JSON de `inputs` en la acción física nativa del IDE para interactuar con los archivos.

## 3. Lógica Operativa y Límites Termodinámicos
Toda ejecución física que pase por esta skill (tras ser autorizada por Cerbero) debe cumplir los siguientes cortafuegos:
* **Confinamiento de Espacio:** Queda estrictamente prohibida cualquier operación (lectura o escritura) sobre rutas absolutas del sistema operativo (ej. `/etc/`, `C:\Windows`) o intentos de *path traversal* (ej. `../../`). Toda ruta debe resolverse dentro del *workspace* del proyecto.
* **Seguridad Antientrópica (Escritura):** Para operaciones `WRITE_FILE` sobre un archivo ya existente, el agente ejecutor debe haber realizado obligatoriamente un `READ_FILE` previo para tener el contexto de lo que va a sobrescribir. La sobrescritura ciega es un fallo letal.

## 4. Respuesta Paramétrica
Una vez que el motor nativo del IDE completa o falla la acción física, debe devolver el control al hilo de pensamiento emitiendo estrictamente el esquema JSON definido en los `outputs`. La verbosidad humana está desactivada durante la emisión de resultados.