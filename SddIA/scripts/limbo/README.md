# Limbo — Purgatorio de scripts (transitorio)

Directorio normativo para artefactos ejecutables **retirados del catálogo de skills del Core** que aún no han sido sustituidos por un Proceso forjado por Dédalo.

## Reglas

1. **Ubicación:** solo bajo `SddIA/scripts/limbo/` (referencia en `SddIA/core/cumulo.paths.json` → `directories.scripts_limbo`).
2. **Anti-ejecución accidental:** todo fichero colocado aquí debe usar una extensión **`.limbo`** antes de la extensión real del lenguaje, p. ej. `sddia-evolution.limbo.py`, `legacy-task.limbo.ps1`.
3. **Salida:** al completarse el Proceso sustituto, el fichero se elimina de Limbo o se archiva en evolución según política de `SddIA/evolution/`.

No ejecutar binarios desde este directorio en flujos de agentes sin revisión explícita y migración a Proceso + skills autorizadas.
