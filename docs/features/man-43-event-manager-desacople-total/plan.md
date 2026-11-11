---
feature_name: man-43-event-manager-desacople-total
created: "2026-11-11"
updated: "2026-11-11"
process: feature
branch_name: feat/man-43-event-manager-desacople-total
persist_ref: docs/features/man-43-event-manager-desacople-total
pbi_ref: MAN-43
parent_ref: MAN-40
historia_ref: MAN-40
document_id: PBI-0f4f0f5f
agent_planificador: dedalo
target_executor: tekton
phase: diseno-blueprint
status: blueprint-ready
version_plan: "1.0.1"
phases:
  - H0-precondiciones-forja
  - H1-contrato-eventuri-v2
  - H2-absorcion-entregas-testigos
  - H3-watcher-sweeper-puerto
  - H4-perifericos-sin-fs
  - H5-lab-demolicion-gates
  - H6-convergencia-documental
rbac_ok: true
scope: "EventManagerPort v2 / EventUri única frontera; cero dualismo Redb+FS ni topología runtime `.events/`"
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-43.jsonl"
refine_workspace: ".SddIA/workspaces/refine-item/c7976303-3b56-4175-9303-415e8b187623"
execution_id: "3278ec5b-21ae-4fe0-af14-62a7e5ba0e07"
seed: continue-tekton
---
# Plan / Blueprint — man-43-event-manager-desacople-total

Blueprint ejecutable para **Tekton**. Entrada: `objectives.md`, `clarify.md` (L-*), `spec.md` **v1.0.1** (ids **MAN-43** / **PBI-0f4f0f5f** / padre **MAN-40**).

Semilla **`continue tekton`** (**L-CONTINUE-TEKTON**): reconfirmación Dedalo; el *qué* no cambia; Ejecución consume este plan en solo lectura. `rbac_ok: true` intacto.

`target_executor_rbac.allowed_policies` alineadas a Tekton: `ecosystem-evolution`, `filesystem-ops`, `source-control`, `system-operations`, `chaos-engineering`, `tracker-operations`.

**Lectura de estado:** puerto 1.2 + bóveda MAN-36 vivos; deuda F-*/L-P*/R* por refine + `spec.md` §0; auditorías desacople/legacy **ausentes en disco** (**L-AUDIT-DISK**) — H6 rematerializa antes de AC-13. Este blueprint pide forja del desacople total v2, no reabrir MAN-36.

## Viabilidad RBAC (cruzada mecánicamente)

| Cápsula | `context` skill | ¿En Tekton? |
|---------|-----------------|-------------|
| `skill:filesystem-manager` | `filesystem-ops` | sí |
| `skill:shell-executor` | `system-operations` | sí (`cargo test` / clippy / gate) |
| `skill:git-manager` | `source-control` | sí — **única** vía Git (**L-GIT**) |
| `action:execute-process` → entity-manager | `ecosystem-evolution` | sí si muta Genoma/docs normativos (**L-GENOME**) |

**Prohibido:** TODOs bajo `docs/todos/` (solo `agent:cumulo` / `Kaizen_Alert_Required`).
**Prohibido:** reabrir bóveda/precedencia MAN-36; dualidad productiva v1/v2; imponer UUID a `event_id`; eliminar `SddIA/events/` o `.SddIA/events/`.
**rbac_ok:** `true` — ninguna fase requiere contexto fuera de Tekton.

---

## Fases declarativas

```yaml
phases:
  - name: "H0 Precondiciones de forja"
    intent: "Cerrar P1–P6 (inventario v1, ops v2, purga, cutover, tests) y sellar forge_key si el runtime lo exige."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "H1 Contrato EventManager 2.0 + EventUri"
    intent: "PublishSeal.event_uri; eliminar logical_rel/target_path; schema/bindings/taxonomía/version_gate v2; suite mock/Redb/externo; rechazo v1."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor", "action:execute-process"]
  - name: "H2 Absorción entregas/testigos"
    intent: "processing/processed/DLQ detrás del puerto; fin de escrituras FS en eda_bus_topology."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "H3 Watcher/sweeper sin FS"
    intent: "claim/listado/cierre/purga solo puerto; cero doble despacho; cero read_dir/is_file SSOT."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "H4 Periféricos sin fallback FS"
    intent: "actions/Kalma2/workspace-server/github_bridge limpios; retirar runtime_db_path/path_is_resolved_redb de consumidores."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "H5 Lab + demolición + gates"
    intent: "Migrar matrices; eliminar eda_port_legacy y sddia-event-store; allowlists vacías; cargo/clippy/suite/gates verdes."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "H6 Convergencia documental"
    intent: "README/Constitución/cumulo/evolution; rematerializar auditorías ausentes; APTO_TOTAL solo con evidencia."
    delegates_to: ["skill:filesystem-manager", "action:execute-process", "skill:git-manager"]
```

---

## H0 — Precondiciones de forja (L-FORGE)

| # | Entregable | Detalle |
|---|------------|---------|
| H0.1 | Inventario consumidores v1 | Lista cerrada en `implementation.md` o anexo del feature (motor/daemons/bridges/tools/tests) |
| H0.2 | Ops v2 checklist | Confirmar §4.3 de `spec.md` cubre sweeper/testigos; ampliar contrato si falta algo **antes** de H3 |
| H0.3 | Política residuos `.events/` | Migrar ECST vivos → puerto; higiene FS ops; nunca SSOT |
| H0.4 | Orden cutover | Validar §8 `spec.md`; no watcher huérfano |
| H0.5 | Estrategia tests por hijo | Mapear AC→comandos; contract suite + gates |
| H0.6 | `forge_key` | Materializar si el runtime lo exige antes de código (**ausente** hoy) |

**Commit (si aplica docs-only):** `docs(eda): precondiciones forja MAN-43`.

---

## H1 — Contrato EventManager 2.0 + EventUri

| # | Entregable | Detalle |
|---|------------|---------|
| H1.1 | `EventFamily` + `EventUri` | Enum + parse/serialización/validación; rechazo familias-estado |
| H1.2 | `PublishSeal` v2 | Solo `event_uri` (+ accessors); eliminar `logical_rel`/`target_path` |
| H1.3 | Ops reales | `list_deliveries`, listados por familia/tipo/estado, barrido terminales; `PublishEcstRequest` con familia/URI |
| H1.4 | Constantes | `CONTRACT_VERSION=2.0.0`, `REQUIRED_CONTRACT_RANGE=^2.0.0` |
| H1.5 | Schema/bindings/taxonomía | Genoma vía entity-manager |
| H1.6 | `version_gate` / handshake | Rechazo explícito consumidores v1 |
| H1.7 | Contract suite | mock + Redb + adaptador externo verdes |
| H1.8 | Adapters host | Implementar ops nuevas sin exponer path |

**Commit:** `feat(eda): EventManagerPort 2.0 + EventUri (MAN-43 H1)`.

**AC:** AC-1, AC-2, AC-3.

---

## H2 — Absorción de entregas y testigos

| # | Entregable | Detalle |
|---|------------|---------|
| H2.1 | `eda_bus_topology` | Sin `write_json_atomic` de testigos/cabeceras; estado vía puerto |
| H2.2 | Terminal / witnesses | Solo puerto; sin OR `is_file` |
| H2.3 | `fractal_bus` | Sin mutación fichero residual en prod |
| H2.4 | github DL fallback | Preferir `move_to_dlq` (puede completarse en H4) |

**Commit:** `refactor(eda): entregas/testigos detrás del puerto (MAN-43 H2)`.

**AC:** AC-5, AC-7 (parcial hasta H3/H4).

**Bloqueo:** no iniciar H3 si ops de listado/terminal de H1 no están verdes.

---

## H3 — Watcher y sweeper sin filesystem

| # | Entregable | Detalle |
|---|------------|---------|
| H3.1 | `event-watcher` L-P1 | Eliminar lote `read_dir`; solo `claim_pending` |
| H3.2 | L-P2 | Cierre/DLQ sin FS |
| H3.3 | `eda_sweep` L-P3/F-5 | Barrido solo puerto |
| H3.4 | L-P4 | Consultas dominio vía `list_events` |
| H3.5 | Anti doble despacho | Un enrutado por evento/tick |
| H3.6 | Higiene L-H* | Purga vía `purge_event`; scripts ops fuera de SSOT |

**Commit:** `refactor(eda): watcher/sweeper 100% EventManagerPort v2 (MAN-43 H3)`.

**AC:** AC-4, AC-5, AC-6.

---

## H4 — Consumidores periféricos

| # | Entregable | Detalle |
|---|------------|---------|
| H4.1 | F-1 `actions` | Sin doble materialización FS |
| H4.2 | F-2 Kalma2 | Sin fallback FS |
| H4.3 | F-3 workspace-server telemetry | Publicación puerto `telemetry` |
| H4.4 | F-4 github_bridge | DLQ por puerto |
| H4.5 | AC-8 `eda_port` | Retirar `runtime_db_path` / `path_is_resolved_redb` de consumidores |

**Commit:** `refactor(eda): periféricos sin fallback FS (MAN-43 H4)`.

**AC:** AC-8, AC-9.

---

## H5 — Laboratorio, demolición y gates

| # | Entregable | Detalle |
|---|------------|---------|
| H5.1 | Migrar matrices | Fuera de `sddia-event-store`; harness/mock/contract suite |
| H5.2 | Borrar `eda_port_legacy` | Módulo + ramas cfg |
| H5.3 | Borrar crate `sddia-event-store` | Workspace + deps |
| H5.4 | Allowlists legacy vacías | Gates detectan reintroducción |
| H5.5 | Gates v2 | Veto `.events/` runtime, símbolos path, store legacy, major 1 |
| H5.6 | Verificación | `cargo test --workspace`, clippy, contract suite, gates |

**Commit(s):** `test(eda): demolición store legacy + gates v2 (MAN-43 H5)`.

**AC:** AC-10, AC-11, AC-12.

**Condición L-7:** no borrar store hasta H5.1–H5.4 listos.

---

## H6 — Convergencia documental

| # | Entregable | Detalle |
|---|------------|---------|
| H6.1 | README / Constitución / procesos / contratos | Narrativa `EventUri` + puerto v2; cero topología runtime `.events/` |
| H6.2 | `cumulo.paths.json` | Solo IDs/tablas/suscripciones lógicas |
| H6.3 | Auditorías | Rematerializar/recuperar paths ausentes (**L-AUDIT-DISK**); parental + legacy → `APTO_TOTAL` **solo con evidencia** |
| H6.4 | `SddIA/evolution/` | Registro cutover 2.0.0 |
| H6.5 | Docs forja | `implementation.md` + `execution.md` |

**Commit:** `docs(eda): convergencia EventManager v2 / APTO_TOTAL (MAN-43 H6)`.

**AC:** AC-13, AC-14.

---

## Orden de ejecución Tekton

1. **H0** precondiciones (+ `forge_key` si aplica).
2. **H1** contrato/URI/suite/gate versión — desbloquea el resto.
3. **H2** entregas/testigos (fuente de estado).
4. **H3** watcher/sweeper (cutover sin huérfano; mismo tren que deja de necesitar espejos).
5. **H4** periféricos (parcialmente paralelizable tras H1; F-3 temprano OK).
6. **H5** lab + demolición (tras consumidores limpios).
7. **H6** docs/evolution/auditorías (incl. rematerializar evidencia) → handoff Argos/`validacion.md`.

No declarar AC-1…AC-14 sin evidencia. No seeds Kaizen desde Tekton. Git solo `skill:git-manager`.

## Criterios de salida del blueprint

- AC-1…AC-14 según `spec.md` / `objectives.md` (O1…O10).
- Laudos L-1…L-7, L-CHILDREN, L-SUPERSEDE-T0, L-AUDIT-GAP, L-AUDIT-DISK, L-CONTINUE-TEKTON, L-NON-GOALS, L-PROCESS, L-GIT, L-GENOME, L-TRACKER, L-FORGE respetados.
- Inventarios F-1…F-6 / L-P* / R1…R3 **cerrados** o reducidos a higiene ops no-SSOT.
- Tracker: **MAN-43** / documento **PBI-0f4f0f5f** / padre **MAN-40** / `execution_id` **3278ec5b-21ae-4fe0-af14-62a7e5ba0e07**.

## Riesgos y mitigaciones

| Riesgo | Mitigación |
|--------|------------|
| Consumidores externos aún en v1 | `version_gate` v2 + inventario H0; rechazo handshake |
| Ops insuficientes para sweeper | Ampliar puerto en H1 **antes** de H3 (**L-6**) |
| Eventos residuales solo en `.events/` | Política H0.3; higiene ≠ SSOT |
| Watcher sin fuente en cutover | H2 completo + H3 atómico (**L-4**) |
| Cambio simultáneo CLI/procesos | H1 primero; H4/H6 alinean después |
| README/Constitución con `.events/` permitido | H6 obligatorio (AC-14) |
| Reabrir MAN-36 | Diff review: cero cambios de precedencia/bóveda salvo API faltante |
| Inventar APTO / auditorías ausentes | H6 rematerializa (**L-AUDIT-DISK**); Argos declara; Tekton solo evidencia |
| `forge_key` ausente | H0.6 antes de código; no inventar éxito de forja |
