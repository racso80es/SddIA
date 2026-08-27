---
feature_name: kaizen-aduana-dlt-relay-supervisado
created: "2026-08-27"
process: feature
branch_name: feat/kaizen-aduana-dlt-relay-supervisado
persist_ref: docs/features/kaizen-aduana-dlt-relay-supervisado
pbi_ref: docs/todos/pending/[KAIZEN] Aduana DLT — relay IOTA supervisado y causa real en anclaje batch.md
document_id: PBI-KAIZEN-ADUANA-DLT-RELAY-SUPERVISADO
uuid: "1243c58b-8e93-4897-ba3e-3efc26564673"
execution_id: "cdd000a0-75d3-4bf9-9a4b-c1d889860ed2"
mayeuta_verdict: ok
laudo: relay-centinela-ambas-jurisdicciones-causa-real-fail-soft
---

# Objetivos — kaizen-aduana-dlt-relay-supervisado

## Misión

Que la ausencia de anclaje DLT sea **imposible de sufrir en silencio**: el relay IOTA vive como centinela supervisado (ambas jurisdicciones), la ignición no declara `ACTIVO` sin territorio verificado, y un fallo de pre-sellado propaga la **causa física** hasta dead-letter + `System_Fracture_Detected`. Rescatar el corpus huérfano 2026-08-25..27 antes de reformar estructura.

## Punto objetivo

> **O-ADUANA-DLT:** Con `SDDIA_LAB_SIMULATE_IOTA=0` y relay loopback, el ecosistema no declara S+ Grade si la aduana DLT no responde. Un relay caído se reinicia sin humano o se grita con `iota-relay-unreachable` (no `batch-missing-merkle-anchor` opaco). Los eventos de la ventana de ceguera quedan en un lote Merkle con acta, `anchored_retroactively: true`, sin reinyección a `pending/`.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Fase 0: rescate Merkle del corpus dead-letter (ventana 25–27 ago) + acta firmada | Panel Espejo de Consciencia (otro PBI; aquí solo señal) |
| Daemon `iota-publish-relay` vía `daemon-creator` / `entity-manager` (DA-2) | Migración Node → Rust (`DT-DLT-RELAY-NODE`) |
| Paridad supervisión `script` (`_start_daemon` + lock/heartbeat) **y** `systemd` (unidad `@%f` + enable en ignición) | Cambios de firma / package ID IOTA |
| Ignición: `_wait_http` al endpoint; sin `ACTIVO` falso; `cleanup()` coherente | Fail-hard del ciclo de entrega |
| `route_domain_batch`: causa real + `System_Fracture_Detected` (`F-DLT-RELAY-SIN-SUPERVISOR`) | Segundo PR documental |
| Cola de re-anclaje fail-soft persistente (Fase 2) | |
| Fase 3: régimen documentado + auditoría `invoke_iota_publisher` | Implementar a la vez fallback y extirpación |

## Objetivos medibles

| ID | Objetivo | Criterio (PBI) |
|----|----------|----------------|
| **O0** | Corpus huérfano anclado o amnistiado con acta | DLT-CA6..CA9 |
| **O1** | Relay = centinela forjado (launcher, lock, heartbeat) | DLT-CA1 |
| **O2** | Ignición no miente; falla si DLT REQUIRED caído | DLT-CA2 |
| **O3** | Causa real en `error_trace` | DLT-CA3 |
| **O4** | Fractura audible en el bus | DLT-CA4 |
| **O5** | Reinicio sin intervención humana | DLT-CA5 |
| **O6** | Cola permanente sustituye rescate manual | DLT-CA10 |

## Orden de ejecución (sello)

1. **Fase 0 — Rescate** (inmediata; criterio de parada si el lote falla).
2. **Fase 1 — Supervisión** (daemon + ignición + cleanup + unidad systemd).
3. **Fase 2 — Percepción** (causa real + fractura + cola re-anclaje).
4. **Fase 3 — Régimen** (peaje documentado + fósil `invoke_iota_publisher`).

## Decisiones Mayeuta (sello)

- **Jurisdicciones:** supervisión en `script` **y** `systemd` (absorbe residual de `kaizen-ignicion-soberana`).
- **REQUIRED condicional:** simulate=0 + URL loopback; irrelevante en consumer / simulación.
- **Régimen:** fail-soft **con** cola de re-anclaje visible; prohibido silencio.
- **Prefijo traza:** `batch-anchor-failed:` + causa física.
- **Fase 0 antes que reforma;** sin backfill parcial sin acta.
- **Honestidad temporal:** `anchored_retroactively: true`.

## No objetivos

- Sustituir el relay Node por cápsula Rust en este PR.
- UI de salud del ecosistema.
- Fail-hard que aborte entregas ya cerradas en soft.
- Mutación manual bajo `SddIA/daemons/` o scripts sin creators.

## Ley aplicada

- `features-documentation-pattern` v1.2.1 / proceso `feature`
- `CONSTITUTION_CORE` — Triaje C/A/B; Verdad Objetiva (causa física > síntoma)
- DA-2/DA-3: genoma vía `entity-manager` / `daemon-creator`; DA-4 topología activa; DA-5 fire-and-forget
- Ceguera espacial: rutas vía `SddIA/core/cumulo.paths.json` (`eda_instance.proofs`, daemons, bus)
- Git exclusivamente vía `skill:git-manager`
- Cierre documental en rama (un PR): PBI → `docs/todos/done/` + `validacion.md` APTO
