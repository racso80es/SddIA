# Limbo — Purgatorio de scripts (transitorio)

Directorio normativo para artefactos ejecutables **retirados del catálogo de skills del Core** que aún no han sido sustituidos por un Proceso forjado por Dédalo.

**Ruta en Cúmulo:** `SddIA/core/cumulo.paths.json` → `directories.scripts_limbo` → `SddIA/scripts/limbo/`.

## Reglas de uso

1. **Ubicación:** solo bajo `SddIA/scripts/limbo/`.
2. **Anti-ejecución accidental:** todo fichero colocado aquí debe usar una extensión **`.limbo`** antes de la extensión real del lenguaje, p. ej. `sddia-evolution.limbo.py`, `legacy-task.limbo.ps1`.
3. **Salida:** al completarse el Proceso sustituto, el fichero se elimina de Limbo o se archiva en evolución según política de `SddIA/evolution/`.

No ejecutar binarios desde este directorio en flujos de agentes sin revisión explícita y migración a Proceso + skills autorizadas.

## Carpeta `archetypes/`

Aquí se conservan **instantáneas por generación** (`*.SddIA_N.archetype.*`) de agentes, herramientas, procesos, tokens, constitution, etc. Sirven como referencia histórica y para triage; **no sustituyen** el SSOT del motor.

| Ámbito | Canónico en Core (fuera de Limbo) |
|--------|-------------------------------------|
| Tokens | `SddIA/tokens/` — contrato de familia: [`tokens-contract.md`](../../tokens/tokens-contract.md); catálogo: [`index.md`](../../tokens/index.md); cada token: `paths.tokensPath/<token-id>.md` (p. ej. [`karma2-token.md`](../../tokens/karma2-token.md)). |
| Agentes actuales | `SddIA/agents/` (V5); los arquetipos bajo `archetypes/agents/` documentan versiones heredadas u organización por rol (p. ej. auditor). |

Al actualizar normativa del dominio tokens en el Core, los README de arquetipo bajo `archetypes/tokens/` pueden quedar desfasados; prevalece siempre **`SddIA/tokens/tokens-contract.md`** y el **`index.md`** del dominio.
