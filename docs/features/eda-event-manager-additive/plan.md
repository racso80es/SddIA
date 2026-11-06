---
feature_name: eda-event-manager-additive
created: "2026-11-06"
updated: "2026-11-06"
process: feature
branch_name: feat/eda-event-manager-additive
persist_ref: docs/features/eda-event-manager-additive
pbi_ref: MAN-32
document_id: PBI-EDA-001
execution_id: "5513eab2-a948-4e5b-9476-3ff561266b95"
related_adr: ADR-EDA-002
execution_mode: ADDITIVE_NON_BREAKING
agent_planificador: dedalo
target_executor: tekton
phase: diseno-blueprint
status: blueprint-ready
phases:
  - T0-genoma-contrato
  - T1-puerto-kernel
  - T2-adapter-redb
  - T3-adapter-fs
  - T4-puente-factory
  - T5-verificacion-sello
rbac_ok: true
scope: "Pasos 1–2 aditivos EventManager — Genoma + puerto + adapters + suite"
norm_ref: features-documentation-pattern@1.3.1
---

# Plan / Blueprint — eda-event-manager-additive

Blueprint ejecutable para **Tekton**. Entrada: `objectives.md`, `clarify.md` (L-*), `spec.md` v1.1.0.

`target_executor_rbac.allowed_policies` esperadas: `ecosystem-evolution`, `filesystem-ops`, `source-control`, `quality-assurance`.

## Viabilidad RBAC

| Cápsula | ¿Permitida? |
|---------|-------------|
| `skill:filesystem-manager` | sí (`filesystem-ops` / `ecosystem-evolution`) |
| `skill:shell-executor` | sí (cargo test / clippy) |
| `skill:git-manager` | sí (commits bajo orden; **única** vía Git — **L-GIT**) |
| `action:execute-process` → entity-manager / evolution | sí si el runtime lo inyecta; alternativa T5 = Write atómico de norma + sello EDA |

Si `quality-assurance` no viene en RBAC del ejecutor, T5 degrada a verificación documentada vía `shell-executor` solo si el contexto del skill lo permite; si no, **abortar fase** y escalar — no improvisar.

---

## Fases declarativas

```yaml
phases:
  - name: "T0 Genoma — contrato y catálogo"
    intent: "Materializar schema eda.event_manager.v1, alta taxonomía eda:event-manager y binding multi-rol aditivo sin tocar filas vigentes."
    delegates_to: ["skill:filesystem-manager"]
  - name: "T1 Puerto Shared Kernel"
    intent: "Forjar ports/event_manager en sddia-core: trait, errores, version_gate, InMemoryEventManagerMock, EventManagerDiFactory kernel-safe."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T2 Adapter Redb"
    intent: "Crate redb_event_adapter + ficha + censo; ACID eventos/leases/entregas/DLQ; manifiesto dual-version."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T3 Adapter Filesystem"
    intent: "Crate fs_event_adapter + ficha + censo; write-temp+rename; árboles pending/claimed/dlq/traces."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T4 Puente NFT-ready y wiring factory"
    intent: "ExternalCapsuleEventAdapter (stdin/stdout, BYPASS thermo) + registro de builders en factory + dummy E2E."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T5 Verificación y sello"
    intent: "Contract suite tripartita, version_gate, puente, clippy -D warnings, AC-1 diff limpio; implementation/execution; evolution EDA."
    delegates_to: ["skill:shell-executor", "skill:filesystem-manager", "skill:git-manager"]
```

---

## T0 — Genoma (contrato / taxonomía / binding)

| # | Entregable | Detalle |
|---|------------|---------|
| T0.1 | `eda.event_manager.v1.schema.json` | Draft 2020-12; ops L-OPS; payloads §3.1 spec; I/O `capsule-json-io` |
| T0.2 | `capability-taxonomy.md` | `1.0.9`→`1.0.10`; catalog `eda:event-manager`; hash recalculado; tabla legible |
| T0.3 | `library/norms/index.md` | Fila taxonomía `1.0.9`→`1.0.10` |
| T0.4 | `capability-bindings.md` | `1.7.0`→`1.8.0`; **solo** bloque nuevo multi-rol (`default` redb, `fallback` fs, `test` mock, `capsule:*`) |

**Salida:** AC-2 documental listo. **Prohibido** mutar bindings previos / `di.binding.schema.json`.

**Commit** (`skill:git-manager`): `feat(eda): sellar contrato eda.event_manager v1 en Genoma`.

---

## T1 — Puerto en `sddia-core`

| # | Entregable | Detalle |
|---|------------|---------|
| T1.1 | `src/ports/event_manager/*` | contract, port, error, version_gate, memory_mock, event_manager_factory |
| T1.2 | `lib.rs` | `pub mod ports` |
| T1.3 | Tests unitarios gate + mock | `^1.0.0` OK minor/patch; MAJOR → `ContractVersionMismatch` |
| T1.4 | Cargo.toml sddia-core | **Sin** dependencia `redb` ni adapters (**L-FACTORY** / **L-PORT-HOST**) |
| T1.5 | Factory skeleton | Mapa de builders inyectable; rol `test` → mock in-kernel |

**Salida:** `cargo test -p sddia-core` verde en tests nuevos.

**Commit:** `feat(eda): puerto EventManagerPort y version_gate en sddia-core`.

---

## T2 — RedbEventAdapter

| # | Entregable | Detalle |
|---|------------|---------|
| T2.1 | Crate `redb_event_adapter/` | `crate_name`: `sddia-infrastructure-redb-event`; impl `EventManagerPort`; dep `redb` local |
| T2.2 | Ficha `redb-event-adapter.md` | Frontmatter `adapters-contract` §1; `status: placeholder`→`active` al verde |
| T2.3 | `index.md` + `SddIA/Cargo.toml` | Fila censo; miembro workspace |
| T2.4 | Manifiesto dual-version | `capsule_version` + `implements_contract` 1.0.0 |
| T2.5 | Tests tempdir | Hook a `run_event_manager_contract_suite` |

**Commit:** `feat(eda): RedbEventAdapter ACID para eda:event-manager`.

---

## T3 — FsEventAdapter

| # | Entregable | Detalle |
|---|------------|---------|
| T3.1 | Crate `fs_event_adapter/` | `crate_name`: `sddia-infrastructure-fs-event`; write-temp + rename; dirs canónicos |
| T3.2 | Ficha + censo + workspace | Paridad T2 |
| T3.3 | Manifiesto dual-version | Igual forma |
| T3.4 | Tests tempdir | Misma suite |

**Commit:** `feat(eda): FsEventAdapter atómico para eda:event-manager`.

---

## T4 — Puente + wiring factory

| # | Entregable | Detalle |
|---|------------|---------|
| T4.1 | `external_capsule_event_adapter/` | `crate_name`: `sddia-infrastructure-external-capsule-event`; JSON ↔ hijo stdin/stdout; env `SDDIA_BYPASS_THERMO_TOLL=1` |
| T4.2 | Ficha + censo + workspace | Paridad adapters-contract |
| T4.3 | Builders host/test | Registrar redb/fs/external en `EventManagerDiFactory` **desde fuera** de `sddia-core` |
| T4.4 | Binario/fixture dummy | Handshake + eco op; test E2E puente |

**Commit:** `feat(eda): puente ExternalCapsuleEventAdapter y DI factory`.

---

## T5 — Verificación y sello

| # | Hecho | Verificación |
|---|--------|----------------|
| T5.1 | `run_event_manager_contract_suite` | 100% MemoryMock + Redb + Fs (**AC-3**) |
| T5.2 | Tests `version_gate` | SemVer (**AC-4**) |
| T5.3 | Test puente | NFT-ready (**AC-4**) |
| T5.4 | `cargo clippy` alcance forjado | `-- -D warnings` limpio |
| T5.5 | AC-1 | `git-manager` status/diff: sin lógica activa en `execute-process`, `event-watcher`, `sddia-event-store` |
| T5.6 | Docs de forja | `implementation.md` + `execution.md` (frontmatter features-documentation-pattern) |
| T5.7 | Evolution / EDA | Registro feature + `Domain_Entity_Updated` / cobertura para norma taxonomía mutada |

**Commit(s):** cierre verificación documental; Git **solo** `skill:git-manager`.

**Prohibido:** materializar semillas Kaizen / TODOs bajo `docs/todos/` (eso es Cumulo / evento `Kaizen_Alert_Required`).

---

## Orden de ejecución Tekton

1. T0 (Genoma puro — no rompe runtime).
2. T1 (puerto + mock + gate + factory skeleton).
3. T2 → T3 (adapters built-in; serializar commits).
4. T4 (puente + wiring builders).
5. T5 (suite + clippy + AC-1 + docs + EDA).

## Criterios de salida del blueprint

- AC-1…AC-4 según `spec.md` §7 / `objectives.md`.
- Laudos L-DOMAIN, L-TAXON, L-LOCUS, L-PORT-HOST, L-BIND-ROLES, L-FACTORY, L-PROVIDER-KIND, L-OPS, L-VERIFY, L-NONBREAK, L-GIT respetados.
- Sin cableado ADR `eda.event_store`; sin tokenización NFT real.

## Riesgos y mitigaciones

| Riesgo | Mitigación |
|--------|------------|
| Contaminar `sddia-core` con `redb` | **L-FACTORY** kernel-safe + deps solo en crates adapter |
| Romper `load_binding_rows` (duplicados / schema) | Una sola fila nueva; roles extensión solo para EventManagerDiFactory |
| Drift ADR tools vs infra | **L-LOCUS**; convergencia diferida |
| Norma Genoma huérfana EDA | T5.7 cobertura / `Domain_Entity_Updated` |
| MAN-32 cuerpo Linear ausente | No bloquea; cierre Linear en delivery-close |
