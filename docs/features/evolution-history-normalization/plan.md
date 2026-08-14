---
feature_name: evolution-history-normalization
created: "2026-08-14"
process: refactorization
phase: Diseño de refactor
agents: dedalo
branch_name: refactor/evolution-history-normalization
persist_ref: docs/features/evolution-history-normalization
pbi_ref: docs/todos/done/[REFACTOR] Evolution — migrar históricos y extraer borradores (EV-AUD-002-007).md
document_id: 7bb37ff1-decd-4ec5-968b-344a5334f9eb
correlation_id: 4b9de6b3-c400-49c8-86f2-55f08ec64ce4
phases: "T0-manifest T1-L1 T2-L2 T3-L3 T4-L4 T5-index T6-validator T7-refs T8-evolution T9-argos"
status: blueprint
---

# Plan — evolution-history-normalization

Blueprint de ejecución. Orden estricto: **manifiesto congelado → lotes con pruebas → índice → idempotencia → cascada**.

| ID | Fase | Touchpoints | Done |
|----|------|-------------|------|
| **T0** | Manifiesto | `sddia-qa migrate-evolution-history manifest`; `persist_ref/migration-manifest.json`; evidencia git freeze | Manifiesto congelado; 0 ítems bloqueados |
| **T1** | L1 INV-A | ~35 registros; normalizar FM; calcular hash; tests unitarios clasificador+hash | AC parcial L1; validador pre/post por lote |
| **T2** | L2 INV-L | ~18 registros; enum L-TIPO; hash compute; tests | AC parcial L2 |
| **T3** | L3 NOMBRE/UUID-INV | Renombres + SIN_FECHA git; 7 UUID `manifest_v4`; tests rename+collision | AC parcial L3 |
| **T4** | L4 BORRADOR | Extract a `docs/audits/evolution/drafts/`; refs cruzadas borradores | AC-DRAFT |
| **T5** | Índice + contrato | `Evolution_log.md` 64 filas CANONICO; `evolution_contract.md` §3 | AC-INDEX |
| **T6** | Validador + idempotencia | `validate-evolution-contract --universe official`; `migrate-evolution-history verify` | AC-CANON, AC-IDEM, AC-AUDIT |
| **T7** | Cascada refs | grep docs/SddIA; actualizar punteros legacy | refs alineadas |
| **T8** | Evolution hito | Registro canónico del ciclo vía `gate-evolution`; `implementation.md`, `execution.md` | trazabilidad |
| **T9** | Argos + cierre | `validacion.md` APTO; PBI → `docs/todos/done/`; PR único | AC-PR |

## Orden Tekton (estricto)

```text
T0 manifest freeze
  → STOP si blocked_items > 0 o colisión UUID
T1 apply --lote L1 + cargo test migrator L1
T2 apply --lote L2 + cargo test migrator L2
T3 apply --lote L3 + cargo test migrator L3
T4 apply --lote L4
T5 rebuild Evolution_log + contract §3
T6 verify + validate --universe official
T7 refs barrido
T8 self-register evolution
T9 Argos → validacion.md → cierre documental
```

Prohibido `apply` sin manifiesto congelado. Prohibido lote N+1 si tests del lote N fallan.

## Delegaciones

| Necesidad | Vía |
|-----------|-----|
| Motor migrador / validador | Tekton → `SddIA/tools/sddia-qa`, dependencia `sddia-evolution-register` (hash) |
| Renames / extract | `./sddia-run.sh --tool git-manager` (JSON stdin); fallback `filesystem-manager` solo con evidencia git |
| Hash parity | Test compartido: mismo vector que `sddia-evolution-register` tests |
| Semillas Kaizen residuales | Solo `agent:cumulo` / `Kaizen_Alert_Required` |
| Docs tarea | `persist_ref` (jurisdicción documental) |

## Gates anti-entropía

```text
si manifest blocked_items > 0        → STOP T0
si apply sin frozen_at en manifest   → FAIL
si UUID colisiona en manifest        → FAIL T0
si verify diff != vacío              → FAIL T6
si official universe CANONICO < 64   → FAIL T6
si *-temp* permanece en evolution/   → FAIL T4/T6
si mapa bajo directories.evolution   → FAIL (recontaminación)
```

## Evidencia por fase

| Fase | Artefacto |
|------|-----------|
| T0 | `migration-manifest.json`, `_manifest-freeze.json` (commit hash) |
| T1–T3 | `_qa-lot-L{n}.json` snapshots validador |
| T6 | `_qa-validate-evolution-official.json`, salida `verify --json` |
| T9 | `validacion.md` |

## Handoff Argos

Consumir `objectives.md` § AC + `spec.md` § Criterios técnicos. Verificar universo 64, 0 legacy en oficiales, manifiesto reversible, idempotencia y cierre documental en rama.
