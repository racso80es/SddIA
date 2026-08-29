---
feature_name: kaizen-espejo-consciencia-observabilidad
created: "2026-08-29"
process: feature
branch_name: feat/kaizen-espejo-consciencia-observabilidad
persist_ref: docs/features/kaizen-espejo-consciencia-observabilidad
pbi_ref: docs/todos/pending/[KAIZEN] Espejo de Consciencia: Proyección de Salud y Observabilidad del Ecosistema.md
document_id: PBI-KAIZEN-ESPEJO-CONSCIENCIA-001
uuid: 97d96117-49cf-4db7-b860-acd65bee216a
execution_id: "a15ad28b-27a3-491c-902e-f78c100ffd43"
status: blueprint_locked
mayeuta_verdict: ok
depends_on:
  - docs/features/telemetria-cognitiva-llm-kalma2
  - docs/features/kalma2-bridge-rust
---

# Objetivos — kaizen-espejo-consciencia-observabilidad

## Misión

Erradicar la ceguera espacial del Vértice Biológico: proyectar en Kalma2 el cruce **mapa esperado × territorio de instancia** (Centinelas, skills/tools, revocación) con semántica de color estricta, sin parsear el genoma desde el puente y sin anidar orquestación en los batches críticos.

## Punto objetivo

> **O-ESPEJO:** Abrir Kalma2 muestra, junto al Pulso cognitivo, una matriz de salud: daemons vivos/caídos y entidades activa/degradada/revocada/letargo, derivada solo de artefactos de instancia + map-snapshot.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Map-snapshot Cúmulo (tools, skills, daemons) | Walk `SddIA/**` desde bridge/WUI |
| Fusión `query-ecosystem-health` (Core Rust) | Cápsula WASI nueva (DD-5 b descartada) |
| `GET /api/system-health` + panel WUI | SSE de salud (opcional post-MVP) |
| Semántica VERDE/AMARILLO/ROJO/GRIS | Intervención UI (kill/restart) |
| Suscripción mapa a `Domain_Entity_{Created\|Updated\|Deleted}` | `Ecosystem_State_Changed` |
| | Familia `infrastructure_adapters` en el panel (Fase 2) |
| | Detección semántica placeholder |
| | Anidar `execute-process` en heartbeat-audit / radamanto-batch |

## Ley aplicada

- PBI DD-1…DD-7 y OBS-CA1…OBS-CA7.
- DA-2/DA-3: proceso/acción/evento vía `entity-manager`; `cumulo.paths.json` y bridge/WUI = Tekton directo.
- DA-5: sin polling post-acuse; GET pull como el cognitivo.
- Constitución: Filtro A — no alucinar rutas; SSOT Cúmulo.

## Criterios (contrato Argos)

OBS-CA1…OBS-CA7 del PBI. Ver `spec.md`.
