---
feature_name: kaizen-paciente0-redeploy-20260825
created: "2026-08-25"
process: feature
purpose: Estabilización Mayeuta — PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825
branch_name: feat/kaizen-paciente0-redeploy-20260825
persist_ref: docs/features/kaizen-paciente0-redeploy-20260825
pbi_ref: docs/todos/pending/[KAIZEN] Paciente 0 SddIA_AP — redeploy 20260825 y fricciones.md
document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825
uuid: "d4f13e9a-5d91-4ab8-a2f5-be2e6b8c4815"
execution_id: "7fd0a353-d2fe-4895-8abe-d7f5b34f652c"
mayeuta_verdict: ok
laudo: absorber-fricciones-post-absorcion-un-pr
---

# Clarificación — kaizen-paciente0-redeploy-20260825

Transcript Mayeuta (2026-08-25). Semilla PBI v1.0.0 con bitácora empírica del redeploy post-Kaizen T6. Filtro A contra genoma vigente (handler release absorbido; regresión por **ELF debug stale**). No se implementa a ciegas.

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` v1.3.2 (Kaizen multi-componente residual; no `bug-fix` aislado) |
| `feature_name` | `kaizen-paciente0-redeploy-20260825` |
| Rama | `feat/kaizen-paciente0-redeploy-20260825` |
| `persist_ref` | `docs/features/kaizen-paciente0-redeploy-20260825` |
| `document_id` | `PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825` |
| Init lab | `./sddia-run.sh --process feature` + `SDDIA_LAB_SKIP_PBI_ARCHIVE=1` + `SDDIA_LAB_SKIP_DELIVERY_CLOSE=1` + `SDDIA_AGENT_RUNTIME_COMMAND=` + pin `SDDIA_EXECUTE_PROCESS_BIN` **release** |
| `execution_id` | `7fd0a353-d2fe-4895-8abe-d7f5b34f652c` |
| Fase | Estabilización Mayeuta (esta sesión) |
| Antecesor | `kaizen-paciente0-redeploy-fricciones` (T6 mergeado). **No** reabrir F-DEP-01…04 ni F-TRIAGE-* |
| Ensayo | Redeploy Paciente 0 `20260825T120132Z`; auditoría T6 `docs/audits/kaizen-paciente0-redeploy-20260825.md` |

**Toll:** un `persist_ref`, un PR. Prohibido fragmentar F-DEP-07/08/09 y F-SMOKE-01 en ciclos paralelos.

---

## D1 — F-DEP-07 (debug antes que release)

| Semilla | Filtro A | Laudo |
|---------|----------|-------|
| `_sddia_resolve_orchestrator` prueba `target/debug/` y luego `release` | **Congruente.** Lab 2026-08-25: debug `2026-08-24 19:41` (pre-Kaizen) vs release `2026-08-25 14:01` | **Dentro.** No seleccionar debug más viejo que release / cicatriz vigente |
| Override `SDDIA_EXECUTE_PROCESS_BIN` como SSOT operador | Evidencia de **deuda de sesión**, no contrato de producto | Pin ad-hoc **prohibido** como runbook. Dedalo fija política en el resolver |
| «Usar siempre release» | Puede romper lab de desarrollo intencional | Dedalo elige: preferir release, ELF cuya cicatriz `.sha256` coincida, o fail-closed si debug es más viejo. Mayeuta no diseña el if |

---

## D2 — F-DEP-08 (stub `{}`)

| Semilla | Filtro A | Laudo |
|---------|----------|-------|
| `materialize_local_paths`: si el fichero existe, `return Ok` | **Congruente** con `instance_creator.rs` vigente | Existencia ≠ validez |
| Stub `{}` de 1.ª pasada debug | Overlay vacío; F-DEP-04 no cubre este hueco | **Dentro.** Sustituir `{}` / overlay vacío por starter-kit |
| `unlink` manual antes del 2.º creator | Corrección ad-hoc | **Prohibido** consolidar como SSOT |

---

## D3 — F-DEP-09 (herencia env ignición)

| Semilla | Filtro A | Laudo |
|---------|----------|-------|
| `start-sddia.sh` / `_ensure_orchestrator` honra `SDDIA_EXECUTE_PROCESS_BIN` ya exportado | **Congruente.** `_sddia_resolve_orchestrator` retorna al primer hit de env | Orquestador de ignición = forja, no bundle instancia |
| Centinelas bajo `{instancia}/SddIA/target/` | Observado 2026-08-25 | El gap es **orquestador de rutas**, no spawn de daemons |
| `env -u` en el operador | Deuda de sesión | **Dentro:** ignición de instancia no hereda pin de forja; o pin en bóveda instancia al ELF del bundle. Dedalo elige el locus (`start-sddia` vs lib vs vault) |

---

## D4 — F-SMOKE-01 (`Local_QA_Requested`)

| Semilla | Filtro A | Laudo |
|---------|----------|-------|
| Clase ECST exige `payload.branch` REQUIRED | **Congruente** `events/orchestration/local-qa-requested.md` | Emisión incompleta → dead-letter |
| Smoke nativo emite `payload` sin `branch` | **Congruente** handler | **Dentro:** payload ECST válido **o** no emitir esa clase |
| Emisor autorizado = `git-hook-pre-push` | Contrato de clase vs `instance-creator` | Dedalo: completar payload **y/o** dejar de emitir clase de orquestación no autorizada. Mayeuta no elige |

Padres en `pending/` (skip routed-ok) son síntoma, no criterio de cierre.

---

## D5 — F-SYS-01 (systemd user enable)

| Semilla | Laudo |
|---------|-------|
| Creator renderiza unidad; `enable --now` es operador | Residual del antecesor |
| Alcance | **Opcional.** No bloquea Done. Si Dedalo estima coste bajo: `install_user_unit` opt-in. `skip_ignition` no debe forzar enable |

---

## D6 — F-DEP-05 / bóveda `.dev`

| Semilla | Laudo |
|---------|-------|
| Fase E del PBI copió Telegram + extras IMAP + perfil consumer/systemd a `.dev` | **Cerrado a nivel operador** (criterio PBI §4 operador `[x]`) |
| Auto-merge bóvedas en Core | **Fuera.** Laudo D2 del antecesor intacto |
| Wizard | `DT-CONFIG-UX-ONBOARDING` **fuera** |

---

## D7 — Fuera de este ciclo

- Gate G5 correo reunión (cerrado en T6; no re-auditar).
- F-TRIAGE-01/02/03.
- F-DEP-01…04: absorbidos en handler **release**; no re-parchear si ELF = genoma actual.
- Consolidar pasos 3–5 del PBI §1.2 como procedimiento canónico.

---

## D8 — Redeploy smoke como gate

Un **solo** `instance-creator` (sin unlink de `{}`, sin pin ELF) sobre Paciente 0 debe dejar `ExecStart` bajo `{instance_root}/` y `local.paths.json` no vacío. Canal canónico: bundle + creator + ignición. Secretos fuera de git.

G5 reunión **no** es gate de este ciclo.

---

## D9 — Auditoría

Al cierre: documento **nuevo** bajo `paths.auditsPath` (`docs/audits`) con bitácora 2026-08-25 (F-DEP-07…) y qué quedó absorbido. **Prohibido** reescribir/duplicar `kaizen-paciente0-redeploy-20260825.md` (T6).

---

## D10 — Jurisdicción de mutación

| Locus | Régimen |
|-------|---------|
| `SddIA/process/instance-creator.md` | DA-2: `entity-manager` |
| `SddIA/engine/.../instance_creator.rs` | Motor; este ciclo |
| `SddIA/scripts/common/sddia_shell_lib.sh`, `start-sddia.sh` | Scripts Core (fuera DA-2) |
| `{instancia}/` parches 2026-08-25 | **Prohibido** consolidar como SSOT |

---

## D11 — Orden de forja

PBI §5 es secuencia de **riesgo**. Dedalo no invierte (1) F-DEP-07 antes de (6) redeploy smoke.

```text
(1) F-DEP-07 resolver orquestador
(2) F-DEP-08 overlay {} no es no-op
(3) F-DEP-09 aislamiento env ignición
(4) F-SMOKE-01 ECST Local_QA
(5) F-SYS-01 opcional
(6) Redeploy smoke — un instance-creator
```

---

## D12 — Cierre documental

`task-closure-documental`: PBI → `docs/todos/done/` + `validacion.md` APTO `pbi_archived: true` **en la rama del PR**. Un PR. Sin segundo PR documental.

---

## Veredicto

**ok** — semilla estable. Huecos Dedalo: política concreta del resolver (D1), validez de overlay (D2), locus de aislamiento env (D3), emitir vs no emitir `Local_QA_Requested` (D4), opt-in systemd (D5).
