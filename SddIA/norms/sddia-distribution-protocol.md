---
uuid: "distribution-protocol-via-c"
name: "sddia-distribution-protocol"
version: "1.0.0"
contract: "knowledge-contract v1.0.0"
---

## Protocolo de Distribución SddIA — Patrón “Vía C”

### Principio Rector
SddIA se distribuye como **dependencia inyectable** bajo el patrón **“Vía C”**: el proyecto consumidor **inyecta** una copia fresca de `SddIA/` desde un repositorio remoto (o fuente autorizada) mediante un bootstrapper, y la trata como **librería**.

### Prohibición Absoluta (Inmutabilidad de la Carpeta Inyectada)
Queda **terminantemente prohibido** mutar, editar, parchear o “arreglar localmente” cualquier archivo dentro de la carpeta inyectada `SddIA/` desde un proyecto consumidor.

- Las modificaciones al Core **solo** se realizan en el repositorio fuente de SddIA (upstream).
- El proyecto consumidor debe considerar `SddIA/` como **artefacto regenerable**: si algo cambia, se vuelve a inyectar.

### Vía C (Consumer Space) — Extensión Permitida
La extensión y personalización del entorno se realiza en el **espacio del consumidor**, fuera de `SddIA/`, típicamente bajo:

- `.SddIA/local.paths.json` (rutas y topología local; fusión con `SddIA/core/cumulo.paths.json`)
- `.SddIA/tools/` (herramientas locales)
- `.SddIA/library/codexes/` (manifiesto del **Códice de Dominio** importado; simetría fractal con `SddIA/library/codexes/`)
- `.SddIA/library/norms/` (normas atómicas indexadas por el Códice; simetría fractal con `SddIA/library/norms/`)

La ruta legada `.SddIA/norms/` queda **obsoleta** para paquetes importados; solo puede usarse para extensiones locales futuras no cubiertas por `codex_sync`. Tras sincronizar un Códice, purgar `.md` residuales en `.SddIA/norms/`.

### Razón Operativa
Este protocolo elimina el *drift* (divergencia silenciosa) y garantiza:

- Reproducibilidad de entorno (inyección determinista)
- Actualización limpia (reinyección por versión/branch)
- Trazabilidad (cambios upstream, no parches locales)
