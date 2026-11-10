---
feature_name: eda-store-lock-contention
created: "2026-11-10"
updated: "2026-11-10"
process: feature
branch_name: feat/eda-store-lock-contention
persist_ref: docs/features/eda-store-lock-contention
pbi_ref: MAN-42
parent_ref: MAN-38
historia_ref: MAN-35
related_refs: ["AUT-67", "AUT-16", "MAN-40"]
agent_planificador: dedalo
target_executor: tekton
phase: diseno-blueprint
status: blueprint-ready
version_plan: "1.0.0"
phases:
  - T1-firma-suppress-pending
  - T2-ownership-budget
  - T-close-ka-lock-validacion
rbac_ok: true
scope: "F-EDA-STORE-LOCK-CONTENTION suppress + delivery pending; 1 DB + N tx; sin reabrir MAN-38 ni subir budget AUT-67"
norm_ref: features-documentation-pattern@1.3.1
tracker_channel: ".SddIA/state/tracker-outbound/MAN-42.jsonl"
execution_id: "cd45182d-1075-43f1-89d5-f2fda26a09f5"
---
# Plan / Blueprint — eda-store-lock-contention

Blueprint ejecutable para **Tekton**. Entrada: `objectives.md`, `clarify.md` (L-*), `spec.md` **v1.0.0** (tracker **MAN-42** / padre **MAN-38** / historia **MAN-35**).

`target_executor_rbac.allowed_policies` alineadas a Tekton: `ecosystem-evolution`, `filesystem-ops`, `source-control`, `system-operations`, `chaos-engineering`, `tracker-operations`.

**Lectura de estado:** síntomas Locked y retry/open_contention vivos; firma `F-EDA-STORE-LOCK-CONTENTION` **ausente** del catálogo; `refine_item` aún emite `System_Fracture_Detected` con `F-REFINE-ITEM-REDB-LOCK`; `forge_pbi` ya usa Pending. Este blueprint pide forja del delta EDA/fractura/delivery, **no** reabrir MAN-38 ni AUT-67 ni sidechannel refine.

## Viabilidad RBAC (cruzada mecánicamente)

| Cápsula | `context` skill | ¿En Tekton? |
|---------|-----------------|-------------|
| `skill:filesystem-manager` | `filesystem-ops` | sí |
| `skill:shell-executor` | `system-operations` | sí (`cargo test` / labs) |
| `skill:git-manager` | `source-control` | sí — **única** vía Git (**L-GIT**) |
| `action:execute-process` → entity-manager | `ecosystem-evolution` | sí **solo si** muta entidad Genoma; catálogo `fracture-signatures.json` = filesystem salvo norma viva exija entity-manager (**L-GENOME**) |

**Prohibido:** TODOs bajo `docs/todos/` (solo `agent:cumulo` / `Kaizen_Alert_Required`).
**Prohibido:** reabrir MAN-38 lifetime; subir open budgets; implementar sidechannel `F-REFINE-ITEM-REDB-LOCK`; tocar EventManagerPort 1.2.
**rbac_ok:** `true` — ninguna fase requiere contexto fuera de Tekton.

---

## Fases declarativas

```yaml
phases:
  - name: "T1 Firma suppress pending"
    intent: "Registrar F-EDA-STORE-LOCK-CONTENTION (suppress); clasificar Locked; cero System_Fracture_Detected/DL; delivery pending."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T2 Ownership y presupuesto"
    intent: "Verificar 1 Database + N tx en paths productivos; preservar MAN-38/AUT-67; defaults budget sin incremento; telemetría KA-LOCK-4."
    delegates_to: ["skill:filesystem-manager", "skill:shell-executor"]
  - name: "T-close Métricas y cierre forja"
    intent: "Tests/lab KA-LOCK-1..4; implementation/execution; evidencia CA sin inventar APTO."
    delegates_to: ["skill:shell-executor", "skill:filesystem-manager", "skill:git-manager"]
```

---

## T1 — Firma + suppress + delivery pending

| # | Entregable | Detalle |
|---|------------|---------|
| T1.1 | Catálogo | Añadir `F-EDA-STORE-LOCK-CONTENTION` en `SddIA/core/fracture-signatures.json` con `consumers` incl. `dcc_suppress` + `mayeuta`, `fracture_policy: suppress`, match L-SYMPTOM (ver `spec.md` §3.1) |
| T1.2 | No-goals catálogo | **No** reescribir entry `F-REFINE-ITEM-REDB-LOCK` |
| T1.3 | Helper match | Una función/módulo compartido de síntomas lock (literales + `StoreError::Locked` / `STORE_LOCK_CONTENTION`) |
| T1.4 | Silencio fractura | `refine_item::emit_store_lock_fracture` (y cualquier emisor EDA equivalente): **dejar de** materializar `System_Fracture_Detected` ante lock; clasificar firma EDA |
| T1.5 | Anti-DL | Rutas claim/ACK/error open: Locked ≠ `move_to_dlq` / dead-letter |
| T1.6 | Delivery | Ante contención: estado `pending` (contraste `forge_pbi::ForgedEmit::Pending` intacto) |
| T1.7 | Tests unit | Match → id firma; emisión fractura = 0; DL = 0 ante fixture Locked |

**Commit:** `fix(eda): F-EDA-STORE-LOCK-CONTENTION suppress + pending (MAN-42)`.

---

## T2 — Ownership 1 DB + N tx / presupuesto

| # | Entregable | Detalle |
|---|------------|---------|
| T2.1 | Auditoría ownership | Grep/evidencia: paths productivos host/watcher/eda_port respetan scoped owner; cero remedio vía segundo `Database::create` |
| T2.2 | MAN-38 | Diff review: no reescribir `with_productive_port` / scopes claim-ACK / `port-lifetime.md` salvo bug bloqueante de T1 (documentar) |
| T2.3 | Budget | Confirmar `DEFAULT_OPEN_BUDGET_MS = 10_000` y host default 2000 **sin** incremento; test AUT-67 sigue verde |
| T2.4 | Telemetría | `open_contention` / literales alineados a KA-LOCK-4 |
| T2.5 | Hook | `hook_common.sh` `STORE_LOCK_CONTENTION` preservado (**L-COMPAT-67**) |

**Commit:** `test(eda): ownership+budget lock contention (MAN-42)` (o chore si solo asserts).

---

## T-close — KA-LOCK + cierre forja

| # | Hecho | Verificación |
|---|--------|----------------|
| C.1 | KA-LOCK-1 | Fixture/lab: síntoma → `F-EDA-STORE-LOCK-CONTENTION` |
| C.2 | KA-LOCK-2 | 0× `System_Fracture_Detected` por Locked |
| C.3 | KA-LOCK-3 | 0× DL por contención |
| C.4 | KA-LOCK-4 | 1 DB+N tx + budgets sin incremento |
| C.5 | Docs forja | `implementation.md` + `execution.md` |
| C.6 | Git | Solo `skill:git-manager` / `./sddia-run.sh --tool git-manager` |

**Commit(s):** verificación; handoff Argos/`validacion.md` — **no** declarar APTO sin evidencia.

---

## Orden de ejecución Tekton

1. **T1** catálogo + helper + silencio emisores + anti-DL + pending.
2. **T2** auditoría ownership/budget/telemetría/hook (puede solapar asserts con T1.7).
3. **T-close** métricas + artefactos forja → Argos.

No declarar CA-1…CA-8 sin evidencia. No seeds Kaizen desde Tekton. No inflar presupuestos.

## Criterios de salida del blueprint

- CA-1…CA-8 según `spec.md` / `objectives.md` (O1…O7; O8 = validación Argos).
- Laudos L-DOMAIN, L-SYMPTOM, L-SIGNATURE, L-SUPPRESS, L-NOT-FRACTURE, L-NOT-DL, L-DELIVERY-PENDING, L-ONE-DB-N-TX, L-NO-REOPEN-38, L-NO-BUDGET, L-COMPAT-67, L-METRICS, L-NON-GOALS, L-PROCESS, L-GIT, L-GENOME, L-TRACKER respetados.
- Tracker: **MAN-42** / `execution_id` **cd45182d-1075-43f1-89d5-f2fda26a09f5**.

## Riesgos y mitigaciones

| Riesgo | Mitigación |
|--------|------------|
| Confundir silencio refine con sidechannel AUT-52 | T1 solo deja de emitir fractura; entry `F-REFINE-ITEM-REDB-LOCK` intacta |
| Suppress en catálogo sin gate de emisión | Cablear emisores EDA (T1.4); catálogo solo no basta |
| Reabrir MAN-38 al «arreglar» lock | Diff gate: cero rediseño lifetime; retry = escudo |
| Subir budget por presión operativa | Assert AUT-67 + CA-7; L-NO-BUDGET |
| Inventar APTO | Argos declara; Tekton solo evidencia |
| Mutar genoma por error | Catálogo core vía filesystem salvo norma entity-manager; Git solo git-manager |
