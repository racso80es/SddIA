---
feature_name: smokepasarelaasyncpbi-044lab
created: "2026-07-23"
updated: "2026-07-23"
process: feature
base: main
scope: lab-smoke-evidence-pasarela-async-pbi-044
version_spec: "1.1.0"
uuid: 7f2c9e1a-4b8d-4e6f-9a3c-1d5e8b0f2a47
status: dedalo_locked
document_id: PBI-044-SMOKE-PASARELA-ASYNC-LAB
pbi_uuid: 8c71b50f-7067-472a-a149-40041920b054
branch_name: feat/smokepasarelaasyncpbi-044lab
persist_ref: docs/features/smokepasarelaasyncpbi-044lab
pbi_ref: docs/todos/done/[ARQUITECTURA] PBI-044 — Pasarela asíncrona Kalma2 y desacople por bus de eventos.md
correlation_id: e92ee44d-9992-4d1b-9384-b5aba5de1acc
prior_correlation_ids:
  - 33f4a9ee-290c-40af-8634-ae69c1445642
  - 54e86a6b-2bec-4010-8da8-ea50f2e86973
  - 6178f1d1-e1d7-4446-bc9b-fca16d79b872
  - 978397b0-c509-4678-a69c-3c69a4acaef7
  - 97af9687-41d5-4d6a-b094-bf2d4b678da8
  - ae3bba9e-ccd7-4d9a-a106-401c9897828f
  - e6bf6120-fb76-49c5-982d-b8e914e26174
phase: Diseño de Blueprint
agents: dedalo
laudo: lab-smoke-evidence-only-no-redesign
reentry: "Post Mayeuta D14 + Argos FAIL_EVIDENCE_GAP (ciclo e6bf6120) — reafirmación blueprint T-GATE+T1–T5; sin relajar AC-L-*; sin reinventar vectores (Q6)"
depends_on:
  - docs/features/kalma2-pasarela-asincrona-eda
  - docs/features/kalma2-event-bus-integration
  - docs/features/kalma2-process-dispatch
  - docs/features/kalma2-full-cycle
---

# Especificación — smokepasarelaasyncpbi-044lab

## 1. Naturaleza del ciclo

**Lab de evidencia física**, no forja de pasarela. El contrato H1+H2 (202/`accepted`/spawn; ceguera espacial; `correlation_id ≡ event_id`; UI poll) ya está en `docs/features/kalma2-pasarela-asincrona-eda/`. Este ciclo **re-materializa** smokes/units/auditorías bajo este `persist_ref` de forma reproducible y auditable.

**Reingreso (D14 / Q6):** Mayeuta reafirmó L-S1…L-DOC tras Argos `NO_APTO` / `FAIL_EVIDENCE_GAP` (ciclo `e6bf6120-…` + esta sesión cid `e92ee44d-…`). Dedalo **reafirma** el blueprint T-GATE+T1–T5 (sin reinventar vectores) — el toll es capacidad de materializar (Unlock RBAC), no requisito inestable.

```text
Unlock RBAC (system-operations + source-control)
  → Lab controlado
  → build kalma2-bridge (+ execute-process si L-U2)
  → N× POST /api/execute (intención válida)
  → medir 202 / accepted / cid / RTT (sin await ciclo en el request)
  → rastro Kalma2_Process_Requested (cid≡event_id) + audit no-write-bus
  → GET /api/status?event_id=<cid>
  → cargo test bridge + kalma2
  → persistir evidencia → Argos
```

**Invariantes:** paths vía `SddIA/core/cumulo.paths.json` (`paths.featurePath` → `docs/features`); bridge sin writes EDA; git solo `skill:git-manager`; KM/`docs/todos/` solo Cumulo / `Kaizen_Alert_Required`; ausencia física = **NO_APTO** (prohibido inventar éxito).

## 2. Laudos Dedalo (cierran D4 / Q1–Q6 + D9–D14)

| Ref | Pregunta | Laudo | Justificación |
|-----|----------|-------|---------------|
| **L1** | N disparos timing | **N=12** | Precedente padre; ∈[10,30]; suficiente para p99 estable |
| **L2** | Puerto / entorno lab | **`SDDIA_CLIENT_PORT` efímero** (default lab **18765**) + **`SDDIA_LAB_SKIP_GIT=1`** en el entorno del bridge/hijo | Evita colisión 8765; reduce peaje git en spawn lab |
| **L3** | Método p99 RTT | Medir RTT cliente wall-clock hasta cuerpo 202 parseado; **excluir cold-start** del binario (1 warm-up descartado opcional; documentar en `execution.md`) | Aísla ruido de build/arranque |
| **L4** | Payload execute | `{"prompt":"lab-smoke-pbi044-<i>","mode":"execute"}` (prompt no vacío; i=1..N) | Contrato padre §3.1 intacto |
| **L5** | Correlación S2 | Tras último (o cid fijado) 202: localizar `eda_fractal.domain` / `.events/domain/<cid>.json` con `event_type=Kalma2_Process_Requested` y `event_id≡cid`; si dominio aún vacío, documentar evidencias de spawn + techo lab | Hereda nota padre S3 sin PEC |
| **L6** | Status S3 | `GET /api/status?event_id=<cid>` → HTTP 200 con proyección viva (`pending`/`completed`/`failed`); si `orchestration.found=false` sin watcher, **APTO lab** documentando techo (PEC e2e fuera mínimo) | AC-L-S3 / D3 |
| **L7** | Forja código | **Solo** bugfix mínimo si smoke falla con causa demostrable (Q4); **prohibido** reabrir H3/ECST/allowlist/F4 | Alcance lab |
| **L8** | PBI-044 | **No re-archivar**; ya en `done/`. Done lab = `validacion.md` APTO + evidencia en este `persist_ref` | Q1 / AC-DONE-LAB |
| **L9** | Artefactos evidencia | Obligatorios bajo `persist_ref`: `execution.md`, `implementation.md` (aunque forja=0: declarar `items: []` / baseline intacto), fixtures `_smoke-*.json` / logs capturados si se generan | AC-L-DOC; Argos marcó gap runtime |
| **L10** | Git evidencia | Invocar `./sddia-run.sh --tool git-manager` (JSON stdin). Si el entorno rechaza la cápsula: **declarar blocked** en `execution.md` — no bypass Shell destructivo | Q5 |
| **L11** | Preflight RBAC | **Gate duro antes de T1:** ejecutor con `system-operations` (shell-executor), `filesystem-ops`, `source-control` (git-manager). Sin Unlock → **abortar** con `blocked` honesto; **no** bajar AC-L-* | D9–D14 / Argos correction |
| **L12** | Post `FAIL_EVIDENCE_GAP` | Rematerializar T1–T5 + re-Argos; lectura estática **no** sustituye L-BLIND/L-U; **prohibido** reclasificar APTO narrativo | D9–D14 / Q6 |

## 3. Contrato de entorno lab

| Variable | Valor lab | Rol |
|----------|-----------|-----|
| `SDDIA_CLIENT_PORT` | `18765` (efímero; cambiar si ocupado) | Bind bridge |
| `SDDIA_LAB_SKIP_GIT` | `1` | Skip peaje git en caminos lab documentados |
| `CARGO_TARGET_DIR` | `SddIA/target` (relativo repo) | Build determinista |
| Working dir build | `SddIA/` | Crates `kalma2-bridge`, `execute-process` |

**Binarios (topología producto, no host abs):**

| Artefacto | Path lógico |
|-----------|-------------|
| Bridge | `SddIA/target/debug/kalma2-bridge` (tras `cargo build -p kalma2-bridge`) |
| Motor | `SddIA/target/debug/execute-process` |
| Suscripciones dominio | `eda_fractal.domain_subscriptions` → `SddIA/core/event-domain-subscriptions.json` |
| Suscripciones orquestación | `eda_fractal.orchestration_subscriptions` |
| Dominio eventos | `eda_fractal.domain` → `./.events/domain` |
| Feature docs | `paths.featurePath` → `docs/features/smokepasarelaasyncpbi-044lab` |

## 4. Vectores de evidencia (qué medir)

### 4.1 L-S1 — Timing no bloqueante

| Campo | Regla |
|-------|-------|
| Método | `POST http://127.0.0.1:$PORT/api/execute` × **N=12** |
| Esperado HTTP | **202** |
| Cuerpo | `success:true`, `status:"accepted"`, `correlation_id` UUID |
| Gate | p99 RTT cliente **&lt; 50 ms** |
| Prohibido | Await del hijo / Argos / cascada en el mismo request |

Persistir en `execution.md`: códigos, count accepted, min/p50/p99, `last_cid`.

### 4.2 L-S2 — Correlación ECST

Tras S1 (usar `last_cid` o cid de un disparo anclado):

1. Buscar archivo bajo `eda_fractal.domain` con nombre/`event_id` = cid.
2. Verificar `event_type` = `Kalma2_Process_Requested` y `event_id ≡ correlation_id`.
3. Si solo hay evidencia de spawn correlacionado sin write aún: documentar ventana + no declarar `emitted` en HTTP.

### 4.3 L-S3 — Proyección status

`GET /api/status?event_id=<cid>` → proyección viva. Documentar techo si sin PEC/watcher.

### 4.4 L-U1 / L-U2 — Units baseline

```text
CARGO_TARGET_DIR=target cargo test -p kalma2-bridge
CARGO_TARGET_DIR=target cargo test -p execute-process kalma2
```

(Desde `SddIA/`. Capturar stdout resumen en `execution.md`.)

### 4.5 L-BLIND — Ceguera espacial

Audit **ejecutado** (no solo Grep IDE) en crate `SddIA/interfaces/kalma2-bridge/`:

- Preferente: unit `bridge_execute_path_has_no_eda_write_helpers` (stdout capturado).
- Complemento: cero APIs nuevas de write bajo paths EDA (`eda_bus.*`, `eda_fractal.*`, `.events/**`) en camino execute (`accept_execute` / spawn+reaper).

Lectura estática sola = **NO_APTO** (precedente Argos).

### 4.6 L-REG — No regresión nervio

Diff de `event-domain-subscriptions.json` y `event-orchestration-subscriptions.json` vs `main` (vía `skill:git-manager` `diff` o evidencia equivalente): **0** líneas no justificadas.

### 4.7 L-DOC / AC-DONE-LAB

| Artefacto | Obligatorio |
|-----------|-------------|
| `execution.md` | Sí — tabla S1–S3, U1–U2, BLIND, REG; sin inventar ok |
| `implementation.md` | Sí — touchpoints; si forja=0: baseline intacto explícito |
| `validacion.md` | Argos (fase siguiente); lab **no** mueve PBI-044 |
| `_smoke-timing-execute.json` | Plantilla fixture (ya materializada; mantener coherente L4) |

## 5. Fixture plantilla

`docs/features/smokepasarelaasyncpbi-044lab/_smoke-timing-execute.json`:

```json
{
  "prompt": "lab-smoke-pbi044",
  "mode": "execute"
}
```

Inputs operativos one-shot → `.tmp/` (norma artefactos efímeros); no dejar basura en `persist_ref`.

## 6. Superficie de mutación (Tekton)

| Artefacto | Acción esperada | Condición |
|-----------|-----------------|-----------|
| Código bridge/handler/UI | **Ninguna** | Happy path: baseline padre verde |
| Bugfix mínimo acotado | Solo si L-S* / L-U* falla con causa | L7 |
| `execution.md` / `implementation.md` | **Obligatorio** (reescritura post-rematerialización) | Siempre |
| `_smoke-*.json` | Mantener plantilla L4 | Recomendado |
| Genoma indexado / subscriptions | **Prohibido** | L-REG |
| `docs/todos/` | **Prohibido** (Tekton) | Solo Cumulo/Kaizen |

## 7. Criterios de aceptación (Argos)

| ID | Criterio | Liga |
|----|----------|------|
| **AC-L-S1** | N=12 → 202+`accepted`+cid; p99 RTT &lt; 50 ms; evidencia en `execution.md` | O1 |
| **AC-L-S2** | `event_id ≡ correlation_id` en dominio (o spawn correlacionado documentado) | O2 |
| **AC-L-S3** | `GET /api/status` proyección viva; techo lab documentado si sin PEC | O3 |
| **AC-L-U** | Units bridge + kalma2 verdes con stdout capturado | O4 |
| **AC-L-BLIND** | Audit/unit ejecutado; cero writes EDA desde bridge (camino execute) | O5 |
| **AC-L-REG** | Diff suscripciones = 0 injustificado vía git-manager | O6 |
| **AC-L-DOC** | Cascada lab + fixtures/logs runtime; ausencia = NO_APTO | O7 |
| **AC-DONE-LAB** | `validacion.md` APTO en rama; **sin** mover PBI-044 | O8 |

## 8. Fuera de alcance (ratificado)

Re-diseño H1+H2 · H3 Telegram · chat SSE · waiting-for-shell Cursor · rehabilitación F4 PR #146 · PBI-043 DI · PPR#136 F3 · PEC e2e watcher+TQM como gate mínimo · mutación allowlist/subscriptions · escritura `docs/todos/` por Tekton/Argos · inventar segundo evento de intención · relajar AC-L-* por Shell/RBAC Rejected.

## 9. Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Shell/git-manager Rejected en IDE | **L11** Unlock primero; si falla → `execution.md` blocked; no narrar APTO |
| Puerto ocupado | Cambiar `SDDIA_CLIENT_PORT`; documentar valor usado |
| 404 status post-acuse breve | Reintento corto; ventana documentada (padre) |
| Cold-start infla p99 | Warm-up + excluir del p99; N=12 |
| Dominio vacío sin spawn real | NO_APTO S2; no inventar archivo |
| Grep IDE como “BLIND” | **Veto** — exige unit/stdout (L12) |

## 10. Handoff Tekton

Consumir este `spec.md` v1.1.0 + `plan.md` (T-GATE → T1→T5) + `objectives.md`/`clarify.md` (D14).

1. Ejecutar **T-GATE** (Unlock RBAC). Si Rejected → `execution.md` `verdict: blocked` + `block_reason` explícito; **stop**.
2. Si Unlock ok → T1→T5 captura física; git solo `skill:git-manager`.
3. Semillas Kaizen/TODOs solo Cumulo / `Kaizen_Alert_Required`.
4. Si no hay evidencia física capturable: **blocked** explícito — no inventar métricas/cids.
