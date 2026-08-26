---
uuid: "c17189c7-75ff-46cc-995c-d8b9a8af12e4"
name: "sddia-distribution-protocol"
version: "1.2.3"
contract: "knowledge-contract v1.0.0"
hash_signature: "sha256:pending-f-bundle-06-telegram-gateway"
evolved_from: "1.2.2"
laudo_locus: "norm-creator solo materializa directories.library_norms; esta norma Core vive en directories.norms — mutación bajo bug-fix activo bundle-consumer-telegram-gateway (F-BUNDLE-06)"
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
El generador lee el códice inyectado (y suscripciones eferentes) y empaqueta dependencias del grafo. Gate mínimo: `send-telegram-notification` (`.md` + binario) verificable. Si `telegram-watcher` ∈ paquete, exigir además `telegram-gateway` (`.md` + binario) — grafo aferente conversacional (F-BUNDLE-06); el escáner eferente no deriva `--process` desde fuentes daemon.

### Gate ELF fresco (L-BUNDLE-STALE v2 / F-DEP-03)
- Tras copiar ELF, `strings` sobre centinelas **no** debe contener `execute-process.py`.
- `--skip-build` solo si existe testigo `SddIA/target/${profile}/<bin>.sha256` cuyo `source_sha256` (SHA-256 del cierre de compilación local: crate + `path =` + `Cargo.lock` workspace) y `elf_sha256` coinciden con el working tree y el ELF. Testigo ausente o divergencia → abort; exigir rebuild **sin** `--skip-build`.
- Prohibido mtime / `find -newer` como oráculo. Prohibido MD5. El testigo lo escribe el propio `build-release-bundle.sh` tras `cargo build`.

---

## Proceso `instance-creator`

Motor ejecutable de despliegue (forjado vía `entity-manager` → `process-creator`):

1. Instanciar topología `.SddIA/` en carpeta objetivo; `local.paths.json` desde starter-kit. Prohibido dejar stub `{}` / objeto JSON vacío (existencia del fichero no implica overlay válido).
2. Inyectar secretos desde vault/plantilla (nunca loguear secretos).
3. Registrar unidades systemd herméticas (`WorkingDirectory=%f`, `ExecStart=%f/SddIA/…`). Prohibido hornear path absoluto de host en ExecStart. `%f` = raíz de instancia en runtime.
4. Ignitar daemons según perfil y jurisdicción sensorial (R-07). En bundle (`MANIFEST.json` o sin `Cargo.toml`): ignorar `SDDIA_EXECUTE_PROCESS_BIN` cuyo path no esté bajo la raíz de instancia.
5. Smoke: preflight topología (overlay no vacío). **No** emitir `Local_QA_Requested` (emisor autorizado = hook pre-push; `payload.branch` REQUIRED). Si `skip_ignition`, no exigir `route-domain*`. Si ignición no skipped: `route-domain-event` `success:true` sobre un evento de laboratorio.

Invocación canónica:

```bash
./sddia-run.sh --process instance-creator --inputs '{...}'
```

Prohibido inventar binario CLI `sddia` como fachada.

`sync-client-assets` **precede o complementa** la inyección de códice; no lo sustituye este proceso.

---

## Multi-cliente hermético (`%f`)

- Prohibidos centinelas globales compartidos entre instancias.
- Toda unidad de instancia: `WorkingDirectory=%f`, `ExecStart=%f/SddIA/…` (sin path de host), bóveda `%f/.SddIA/.dev/.env`.
- Plantilla de referencia: `SddIA/templates/systemd/sddia-email-watcher@.service.template`.
- `start-sddia.sh` / `instance-creator` operan solo sobre la carpeta objetivo.
- `_sddia_resolve_orchestrator`: si existen debug y release, usar debug **solo** si su mtime es estrictamente mayor que release (lab); en caso contrario release (F-DEP-07).

---

## ONBOARDING.md (proyección consumidor)

Artefacto **generado** por el bundle (y/o fase final de `instance-creator`). Reduce dependencia de manuales estáticos. **No** sustituye el wizard UX (`DT-CONFIG-UX-ONBOARDING`).
