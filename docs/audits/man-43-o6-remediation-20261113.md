---
id: man-43-o6-remediation-20261113
uuid: "a3d5e7f9-1b2c-4d5e-8f9a-0b1c2d3e4f5a"
type: audit
version: 1.0.0
created: "2026-11-13"
updated: "2026-11-17"
tracker_ref: MAN-43
parent_audit: docs/audits/man-32-lineage-man-43-remediation-20261111.md
baseline_commit: 877c1791
scope: "Re-auditoría post O5.2 + O6 documental y cierre O15 (rama docs/man-43-post-merge-handoff)"
verdict: APTO_TOTAL
milestone_o5: APTO
milestone_o6_narrative: APTO
ac_13: closed
ac_14: closed
dossier_cierre: docs/audits/man-43-o14d-reaudit-ac13-20261116.md
---

# Re-auditoría MAN-43 — post O5.2 / O6

## Dictamen

El tren **O0–O5.2** en rama cierra demolición del store legacy, gates v2 y consumidores migrados al puerto con evidencia de tests. **O6** alinea README y Constitución con ese runtime. El feature MAN-43 **no** alcanza `APTO_TOTAL` (AC-13): persisten deudas documentales en genoma `SddIA/process/`, referencias históricas a `SDDIA_EVENT_STORE`, y posible split contractual 1.2 vs 2.0 (F-02 del laudo 20261111).

No se reescribe el veredicto de `docs/audits/man-32-lineage-man-43-remediation-20261111.md`; este informe es una **capa posterior** al commit `877c1791`.

## Evidencia ejecutada (2026-11-13)

| Comando | Resultado |
|---------|-----------|
| `test ! -d SddIA/sddia-event-store` | Directorio ausente |
| `rg sddia-event-store SddIA/Cargo.toml` | Sin miembro workspace |
| `cargo test -p execute-process --lib -- --test-threads=8` | 816 passed |
| `cargo test -p event-bus-audit` | 12 passed |
| `cargo run -p sddia-qa -- gate-event-manager-port` | OK (212 ficheros, tras fix `dlt_telemetry.rs`) |
| `rg '\"pending\":\\s*\"\\./\\.events' SddIA/core/cumulo.paths.json` | Sin coincidencias (tablas lógicas) |

## Hallazgos vs F-01…F-12 (linaje 20261111)

| ID | Estado en rama | Nota |
|----|----------------|------|
| F-01 | Cerrado (O0) | Entregas por igualdad exacta en adapters |
| F-03 | Parcial | Productivo sin espejo FS en emisores clave (O4–O5); `event-bus-audit` aún default `.events/pending` en ausencia de clave |
| F-04–F-07 | Cerrado en adapter/suite (O1) | Ver `cargo test -p sddia-core event_manager` en PR |
| F-09–F-10 | Parcial | Watcher por puerto; periféricos migrados; genomo process aún cita FS |
| **F-11** | **Cerrado** | `sddia-event-store` y `eda_port_legacy` eliminados (`877c1791`) |
| F-02 | Abierto | Taxonomía/bindings pueden seguir en 1.2; handshake 2.0 en lab |
| F-12 | Mitigado | `validacion.md` H1 no reescrito; feature sigue `NO_APTO_TOTAL` |

## Inventario genoma (sin mutación — L-GENOME)

Referencias legacy a cola FS o `SDDIA_EVENT_STORE` detectadas (solo inventario):

- `SddIA/process/event-bus-audit.md` — modos filesystem/redb
- `SddIA/process/route-domain-event.md` — `event_file_path` como token `.events/pending/`
- Codex/library bajo `SddIA/library/` — citas históricas `writes_store(SDDIA_EVENT_STORE)` (refinar vía `entity-manager`, no en O6)

## AC trazabilidad

| AC | Estado |
|----|--------|
| AC-13 (`APTO_TOTAL`) | **Abierto** — este laudo `NO_APTO_TOTAL` |
| AC-14 (evolution + docs constitucionales) | **Parcial** — README, Constitución, evolution `c4e8f2a1-…`; genomo process pendiente |

## Siguiente

Convergencia genoma (`entity-manager`), alineación contractual F-02, y cierre de defaults `event-bus-audit` antes de reclamar AC-13.

## Cierre Argos O15 (2026-11-17)

Completado en **O7–O14d** (genoma forjado vía `entity-manager`, `eda.event_manager` 2.0.0, 17 contratos con `x-sddia`, `event_uri` único en daemon/motor, poda total de `.events/` productivo, `cargo test --workspace` 1226/0 y `clippy` 0 avisos). Veredicto actualizado por Argos a **`APTO_TOTAL`** sobre el dossier [man-43-o14d-reaudit-ac13-20261116.md](file:///home/racso/Proyectos/SddIA/docs/audits/man-43-o14d-reaudit-ac13-20261116.md).
