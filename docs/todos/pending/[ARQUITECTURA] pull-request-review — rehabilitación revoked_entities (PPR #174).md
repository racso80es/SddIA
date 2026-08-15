---
document_id: PBI-PPR-174-REVOKED-REGISTRY
title: "[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #174)"
format: markdown
version: "1.0.0"
created: "2026-08-15"
updated: "2026-08-15T10:57:00Z"
status: abierto
priority: media
process: bug-fix
uuid: e7a1c4b2-9f36-4d8e-a215-6c0b8d3e5f17
source_feature: docs/features/kalma2-phase-barrier-timeout-persist
source_correlation_id: 6vw31k4eoXBCpXXfNWB4EL4FvtxYDbtZao4J47vwVPf8
source_audit: docs/features/kalma2-phase-barrier-timeout-persist/validacion.md
incident_ref: "RBAC_PROCESS_REGISTRY:NO_APTO — pull-request-review re-revocado since 2026-08-15T08:40:55Z (success_rate_below_threshold)"
related:
  - .SddIA/cerbero/revoked_entities.json
  - .SddIA/radamanto/stats.json
  - SddIA/process/pull-request-review.md
  - SddIA/engine/execute-process/src/engine/radamanto_batch_core.rs
  - docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #124).md
  - docs/todos/done/[ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #125).md
  - docs/fixes/kaizen-regex-lookahead-panic/validacion.md
sightings:
  - "PPR #174 · CID 6vw31k4eo… (origen seed)"
  - "PPR #175 · CID 7mDbYBoZi… · Cosecha Kaizen 2026-08-15 — dedup; misma revocación since 2026-08-15T08:40:55Z"
  - "PPR #175 · CID 83b18b3a… · Cosecha Kaizen 2026-08-15T10:57:00Z — dedup; misma revocación since 2026-08-15T08:40:55Z"
---

# [ARQUITECTURA] pull-request-review — rehabilitación revoked_entities (PPR #174)

## Incidente (Cosecha Kaizen · Cúmulo)

Re-revocación empírica tras rehabilitación histórica PPR #124/#125.

| Campo | Valor |
|-------|--------|
| Entidad | `pull-request-review` |
| Registro | `.SddIA/cerbero/revoked_entities.json` → `revoked.pull-request-review` |
| `entity_type` (instancia) | `tool` (entropía: es proceso) |
| `reason` | `success_rate_below_threshold` |
| `since` | `2026-08-15T08:40:55Z` |
| Radamanto | `status: pending_redemption` · `degraded_at` = mismo timestamp · `recovery_attempts: 2` · `rehab_laudo` legado `PBI-PPR-124+125-REVOKED-REGISTRY` |
| ECST origen | `6vw31k4eoXBCpXXfNWB4EL4FvtxYDbtZao4J47vwVPf8` · PR #174 |
| Sighting adicional | `7mDbYBoZiQTE5dsrB5WHpcybGmuesR4CzGd3ExmZe5R` · PR #175 (Cosecha Kaizen — dedup; sin seed nueva) |
| Sighting adicional | `83b18b3a-b3ae-47ad-8948-77d5dbb52067` · PR #175 (Cosecha Kaizen 2026-08-15T10:57:00Z — dedup; sin seed nueva) |
| Check aduana | `RBAC_PROCESS_REGISTRY: NO_APTO` (Cerbero F4; no bloqueante) |

## Mandato

1. Laudo Cerbero: rehabilitar o confirmar revocación permanente según política Radamanto.
2. Alinear instancia: retirar/confirmar clave en `revoked_entities.json`; corregir `entity_type` si procede (`process` ≠ `tool`).
3. Reset/redención stats Radamanto (`pending_redemption` → `healthy`) o política anti-recurrencia para `success_rate` en procesos multi-fase de larga duración (exención actual solo cubre latency).
4. Cascada `docs/fixes/` + `validacion.md` APTO + PBI → `done/`.

## Fuera de alcance

- Residual Kalma2 Shell/`git-manager` (dedup OPERATIVO PPR #136 done).
- Merge histórico PR #174 / handoff `accept-pr` (merge hermano `dbbcabb4` ↔ CID `2b466b03`).


---

## Refinamiento: Resolución de Impacto S+ Grade (15-08-2026)

Tras someter el incidente al Protocolo de Acero, se diagnostica que el colapso del proceso no es un defecto de código aislado, sino una fractura en la arquitectura de auditoría: Radamanto está evaluando un macro-proceso multi-fase con la inflexibilidad estadística (`success_rate`) de una herramienta atómica.

Se establecen las siguientes directrices de intervención:

### Vías de Implementación Inmediata (Mandato Activo)
1. **Auditoría de Umbrales Diferenciados:** Se procederá a alinear la ontología (corrigiendo `entity_type` a `process` en Cerbero) y se instruirá a Radamanto con una tabla de umbrales diferenciada. Los procesos de larga duración y múltiples fases gozarán de mayor tolerancia estadística en la tasa de éxito para evitar falsos positivos y revocaciones prematuras.
2. **Resiliencia de Fase (Kintsugi Ontológico):** Se aplicará un patrón de falla controlada (*fail-soft*) a nivel interno del proceso `pull-request-review`. Si el agente encargado detecta fricción o ruptura en una sub-fase (ej. caída de API externa o error de lectura puntual), deberá absorber el error y registrar la fricción sin causar el colapso termodinámico lineal de la ejecución completa, posibilitando un resultado de éxito parcial.

### Constancia de Evolución Futura (Faro Kaizen)
3. **Desacople Reactivo (Fragmentación EDA):** Queda registrada en el Cúmulo la posibilidad arquitectónica de trocear `pull-request-review` en eventos atómicos puros a través del Bus Fractal (ej. `PullRequest_Analysis_Started` → `PullRequest_Diff_Extracted`). **Esta vía queda explícitamente descartada en el ciclo actual** por aplicación del Filtro C (Eficiencia) para evitar incurrir en sobreingeniería y saturación termodinámica, pero se mantiene como ruta teórica de optimización S+ Grade a largo plazo en caso de que la complejidad del proceso escale.