---
feature_name: kaizen-ciclo-jurisdiccion-todos
created: "2026-08-29"
process: feature
base: main
scope: kaizen-ciclo-jurisdiccion-todos
version_spec: "1.0.0"
---

# Especificación — kaizen-ciclo-jurisdiccion-todos

## Hito 1 — Forge conforme (CA1)

Archivo: `SddIA/engine/execute-process/src/forges/factory.rs` (`run_norm_forge`).

| Input | Destino |
|-------|---------|
| `tactical_norm_dependencies` | Frontmatter `dependencies:` YAML list (default `[]`) |
| `tactical_norm_friction` | `## Directriz Core` |
| `tactical_norm_hard_constraints` (nuevo) | `## Restricciones Duras (Aduana de Fricción)` |

`entity_manager.rs` (`entity_class: norm`): reenviar `tactical_norm_hard_constraints` (default `""`). Si vacío, el bloque existe con cuerpo `Ninguna.` (contrato exige dos bloques; no sepultar en Directriz Core).

`norms_contract_version` default del merge: alinear a `1.1.0` en el seed (hoy `1.0.0`).

Test: módulo existente de forges / `factory.rs`. Semilla con deps + restricciones; assert `dependencies:` contiene el UUID; assert ambos headings. Caso deps `[]` + constraints vacío → `dependencies: []` + heading presente.

No re-forjar las 9 normas conformes.

## Hito 2 — Update `todos-jurisdiction` (CA2)

```text
./sddia-run.sh --process entity-manager --inputs-file .tmp/entity-manager-<uuid>.json
```

| Campo | Valor |
|-------|-------|
| `entity_class` | `norm` |
| `lifecycle_operation` | `update` |
| `entity_name` | `todos-jurisdiction` |
| `tactical_norm_version` | `1.1.0` |
| `tactical_norm_dependencies` | `["4c448c82-de41-460f-b24f-82a84fa5ed69"]` |
| `tactical_norm_friction` | Destilar Directriz Core actual (tabla de buckets + portador deuda) |
| `tactical_norm_hard_constraints` | Las cuatro prohibiciones del PBI §1 (tercer Done; TQM a inertes; mutar sin EM; reimplementar fan-out) |

Prefijo RAW Kernel. Verificar `objectives.md` de esta feature (DA-4). Índice `library/norms/index.md` actualiza versión en la misma transacción del creator.

## Hito 3 — Sello hash real (CA3)

`entity-manager` `update` sobre:

| Entidad | class | uuid |
|---------|-------|------|
| `github-raw-fetcher` | `tool` | `66daf19f-217a-4874-b417-99e5be2571f3` |
| `download-remote-asset` | `action` | `6175f5cd-7844-4d0c-aa93-d2ce3a41d18e` |

Post-condición: frontmatter `hash_signature` ≠ `sha256:pending-forge`; `eda-coverage.json` refleja el hash nuevo (upsert motor). Evidencia: `sddia-qa audit-eda-coverage --scan --json` → `orphan_count: 0` y ambos UUIDs sin pending-forge.

No cambiar semántica de las entidades.

## Hito 4 — Documentar excepción backfill (CA3b)

Completar `delivery-close-cycle.md` (vía `entity-manager` `entity_class: process`, `update`, name `delivery-close-cycle`) y el párrafo de `features-documentation-pattern` (§ Ruido de Sistema) para que coincidan con `backfill_manifest_active`:

| Campo | Efecto |
|-------|--------|
| Path | `{persist_ref}/backfill-manifest.json` |
| `correlation_id` presente | necesario |
| `merkle_anchored: true` | **desactiva** la excepción |
| Ausencia de `merkle_anchored` o valor ≠ true | activa |
| Veredicto | `warn`; `argos_noise`: `backfill Fase C en curso` |

Sin segunda semántica. Sin `Write` directo al process/norma de librería.

## Hito 5 — Fractura en DCC (CA4)

En el handler de fases de `delivery-close-cycle` (`delivery_close.rs` / `phase_capsules.rs`): si status `blocked` o `failed`, llamar helper compartido (extraer el de `workspace_init` o duplicar mínimo) → `materialize_pending_domain_event(..., "System_Fracture_Detected", ...)`.

Payload mínimo: `process_name`, `error_trace`, `agent_emitter: execute-process`, `attempted_action` = nombre de fase, `friction_id` estable por fase (p. ej. `F-DCC-EDA-ORPHAN-BLOCK`, `F-DCC-EVOLUTION-GATE`).

Idempotencia: no depositar segundo evento si ya existe pendiente con mismo `friction_id` + `process_name` (resolutor fan-out).

Test: mock/bloqueo de aduana EDA o invocación del helper; assert evento en pending (tempdir). Evidencia de integración: provocar block y `ls .events/pending` — solo en lab controlado, no en CI contra el bus del operador.

## Hito 6 — Gate evolution (CA5)

`SddIA/skills/sddia-evolution-register` / `gate-evolution`: path canónico `SddIA/core/eda-coverage.json` no dispara `EVOL_MATERIAL_UNREGISTERED`. Motivo en el finding omitido o código `EVOL_ENGINE_DERIVED`. Cualquier otro path material sigue igual.

Test unitario del gate: diff que solo toca `eda-coverage.json` → `EVOL_OK`. Diff que toca `eda-coverage.json` + otro material sin registro → sigue rojo por el otro.

## Hito 7 — `.gitignore` (CA6)

Añadir `**/.tmp/`. Conservar `/.tmp`. Verificar `git check-ignore -v docs/features/kaizen-ciclo-jurisdiccion-todos/.tmp/pr-body.md`.

## Hito 8 — Colapso mudo (CA7)

Parche `SddIA/norms/obediencia-procesos.md` § Escalado: subsección **Colapso mudo**. Si el proceso oficial falla y `.events/pending/` no contiene `System_Fracture_Detected` del ciclo, el operador **emite** por vía canónica (`execute-process` / acción de emisión documentada) y **detiene**. Prohibido improvisar `git push` / `gh pr create`. Bump versión de la norma (1.1 → 1.2). Evolution correlaciona el path.

## Fuera de alcance

- Rediseñar el scan EDA a granularidad de PR.
- Rehabilitar vía de apertura del PR #219.
- Re-forja masiva del catálogo de normas.
