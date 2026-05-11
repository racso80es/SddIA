# Definiciones normativas de tools (workspace local)

Esta carpeta es el **catálogo de especificaciones** de las herramientas de este proyecto: un fichero `{name}.md` por tool e `index.md`.

**Ruta lógica (SSOT):** `.sddia/tools/` — misma ubicación que esta carpeta.

**Windows:** el sistema de ficheros NTFS **no distingue mayúsculas** en nombres de ruta. Por tanto `.sddia`, `.SddIA`, `Tools` y `tools` pueden **resolver al mismo directorio** que ves aquí como `.SddIA/Tools/`. No es una duplicación: es una sola carpeta.

La **implementación** (binarios, scripts, `spec.json` de cápsula) vive en **`scripts/tools/{name}/`** en la raíz de este workspace.
