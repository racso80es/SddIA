---
uuid: "c17189c7-75ff-46cc-995c-d8b9a8af12e4"
name: "sddia-distribution-protocol"
version: "1.1.0"
contract: "knowledge-contract v1.0.0"
hash_signature: "sha256:pending-evolution-kaizen-consumer-ignition"
evolved_from: "1.0.0"
laudo_locus: "norm-creator solo materializa directories.library_norms; esta norma Core vive en directories.norms — mutación bajo feature activa kaizen-consumer-ignition-filtro-c"
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

---

## Release Bundle hermético (v1.1.0)

### Generador
Script canónico: `SddIA/scripts/build-release-bundle.sh`.

```bash
./SddIA/scripts/build-release-bundle.sh \
  --out dist/sddia-release-consumer \
  --codex codex-kalma2-assistant \
  --profile consumer
```

### Contenido obligatorio
- Binarios runtime ELF (orquestador, centinelas, bridge, cápsulas eferentes del grafo).
- Contratos / índices / procesos necesarios para ejecución.
- Códice anclado (`--codex`) cuando se suministra.
- `MANIFEST.json` + **`ONBOARDING.md` autogenerado** (paridad absoluta con el artefacto).
- **Cero** fuentes de ingeniería (`.rs`, `Cargo.toml`, árboles `src/`).

### Filtro C en el paquete
Con `--profile consumer`: no incluir lanzador/binario `github-bridge-watcher`. Runtime: `SDDIA_RUNTIME_PROFILE=consumer`.

### Resolución de cápsulas (F-06)
El generador lee el códice inyectado (y suscripciones eferentes) y empaqueta dependencias del grafo. Gate mínimo: `send-telegram-notification` (`.md` + binario) verificable.

---

## Proceso `instance-creator`

Motor ejecutable de despliegue (forjado vía `entity-manager` → `process-creator`):

1. Instanciar topología `.SddIA/` en carpeta objetivo.
2. Inyectar secretos desde vault/plantilla (nunca loguear secretos).
3. Registrar unidades systemd herméticas (`WorkingDirectory=%f`, `EnvironmentFile=%f/.SddIA/.dev/.env`).
4. Ignitar daemons según perfil y jurisdicción sensorial (R-07).
5. Orquestar smoke post-ignición: `eda-local-topology-test` / estímulo `Local_QA_Requested` → `success: true`.

Invocación canónica:

```bash
./sddia-run.sh --process instance-creator --inputs '{...}'
```

Prohibido inventar binario CLI `sddia` como fachada.

`sync-client-assets` **precede o complementa** la inyección de códice; no lo sustituye este proceso.

---

## Multi-cliente hermético (`%f`)

- Prohibidos centinelas globales compartidos entre instancias.
- Toda unidad de instancia: `WorkingDirectory=%f` y bóveda `%f/.SddIA/.dev/.env`.
- Plantilla de referencia: `SddIA/templates/systemd/sddia-email-watcher@.service.template`.
- `start-sddia.sh` / `instance-creator` operan solo sobre la carpeta objetivo.

---

## ONBOARDING.md (proyección consumidor)

Artefacto **generado** por el bundle (y/o fase final de `instance-creator`). Reduce dependencia de manuales estáticos. **No** sustituye el wizard UX (`DT-CONFIG-UX-ONBOARDING`).
