#!/bin/bash

# Abortar en caso de error
set -e

# Nombre del entorno virtual
VENV_DIR=".venv"

# Verificar si el entorno virtual no existe para crearlo
if [ ! -d "$VENV_DIR" ]; then
    python3 -m venv "$VENV_DIR"
fi

# Activar el entorno virtual e instalar/actualizar dependencias de forma silenciosa
source "$VENV_DIR/bin/activate"
pip install -r requirements.txt -q

# Ejecutar el script principal de SddIA con los parámetros pasados al wrapper
python SddIA/scripts/qa/execute-process.py "$@"
