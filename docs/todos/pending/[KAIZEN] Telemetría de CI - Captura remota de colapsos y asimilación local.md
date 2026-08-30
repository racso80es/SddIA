---
document_id: PBI-KAIZEN-CI-TELEMETRY-OBSERVABILITY
uuid: "8e7f6a5b-4c3d-2e1f-0a9b-8c7d6e5f4a3b"
title: "[KAIZEN] Telemetría de CI: Captura remota de colapsos y asimilación local"
format: markdown
version: "1.0.0"
created: "2026-08-30"
updated: "2026-08-30"
status: pending
refinement_status: proposed
priority: media
process: feature
executor_vehicle: feature
type: kaizen
dispatch: false
suggested_branch: feat/kaizen-ci-telemetry-observability
persist_ref_suggested: docs/features/kaizen-ci-telemetry-observability
depends_on: []
related:
  - .github/workflows/sddia-index-qa.yml
  - SddIA/daemons/github-bridge-watcher.md
  - SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs
  - SddIA/agents/radamanto.thresholds.json
  - SddIA/events/telemetry/index.md
---

# [KAIZEN] Telemetría de CI: Captura remota de colapsos y asimilación local

## Mandato
Erradicar la fuga entrópica generada por los fallos ciegos en la Integración Continua (CI). Actualmente, si un job de GitHub Actions colapsa[cite: 48], el ecosistema local de SddIA ignora el suceso, forzando al Vértice Biológico a inspeccionar logs remotos manualmente. 

El objetivo es establecer un puente de telemetría desde la trinchera remota hacia el motor local. Un fallo en CI debe cruzar el umbral, convertirse en un evento termodinámico local (`CI_Job_Failed`) y ser acumulado por Radamanto para que, si un test demuestra fragilidad estructural crónica, el sistema lo degrade y fuerce su reparación.

## 1. Superficie de Impacto
- **Workflow Remoto:** `.github/workflows/sddia-index-qa.yml`[cite: 48].
- **Puente Sensorial:** `SddIA/daemons/github-bridge-watcher.md`.
- **Motor de Actuaría:** `SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs` y `SddIA/agents/radamanto.thresholds.json`.

## 2. Estrategia de Refactorización (Línea de Montaje)

### Ola A1: Inyección de Señal en Entorno Remoto
Modificar la definición de los jobs en `.github/workflows/sddia-index-qa.yml` para interceptar el fallo de las aduanas:
- Añadir un paso final con la condición `if: failure()`.
- Este paso extraerá el nombre del *job/step* colapsado y emitirá un comentario estructurado en el *Pull Request* asociado utilizando el CLI de GitHub (`gh pr comment`). 
- El comentario incluirá una firma criptográfica o prefijo determinista (ej. `[SDDIA-CI-TELEMETRY]`) y un payload en JSON con los detalles de la fricción.

### Ola A2: El Puente de Asimilación (Cruce del Umbral)
El entorno remoto no tiene acceso al bus fractal `/.events/telemetry/`. La responsabilidad recae en el centinela local:
- Ampliar el código y el contrato de `github-bridge-watcher` para que, en sus ciclos de barrido, parsee los comentarios de los PRs abiertos.
- Al detectar la firma determinista, extraerá el payload y emitirá internamente el evento `CI_Job_Failed` a la ruta `/.events/telemetry/`.
- El centinela debe marcar el comentario (vía reacción o un archivo de estado ligero) para garantizar **idempotencia** y evitar emitir telemetría duplicada en el siguiente barrido.

### Ola A3: Gobernanza Actuarial (Radamanto)
La telemetría cruda debe transmutarse en gobernanza:
- Modificar `radamanto_batch_core.rs` para suscribirse y acumular los eventos `CI_Job_Failed`.
- Inyectar una nueva clase de cuota en `radamanto.thresholds.json`: `max_ci_failures_per_entity` (ej. 3 fallos en una ventana temporal o ciclo de entrega)[cite: 44, 45].
- Si el umbral se rompe, Radamanto emitirá `Domain_Entity_Degraded`[cite: 44, 45], forzando al ecosistema a abrir un PBI Kintsugi para revisar la entidad/test defectuoso por consumo excesivo de recursos remotos.

## 3. Criterios de Aceptación (Protocolo de Acero)

- [ ] **CA1 (Emisión Remota):** Un test fallido provocado deliberadamente en GitHub Actions genera automáticamente un comentario estructurado `[SDDIA-CI-TELEMETRY]` en el PR.
- [ ] **CA2 (Cruce de Umbral):** `github-bridge-watcher` intercepta el comentario y emite el evento `CI_Job_Failed` en el bus fractal local.
- [ ] **CA3 (Idempotencia):** Barridos consecutivos de `github-bridge-watcher` sobre el mismo comentario fallido no generan eventos duplicados en el bus.
- [ ] **CA4 (Degradación Autónoma):** Superar el umbral `max_ci_failures_per_entity` en `.SddIA/radamanto/stats.json` dispara el evento `Domain_Entity_Degraded` para la entidad asociada.
- [ ] **CA5 (Contratos Actualizados):** El nuevo evento `ci-job-failed.md` se formaliza vía `entity-manager` bajo la familia de telemetría.
