---
feature_name: kaizen-paciente0-redeploy-fricciones
created: "2026-08-25"
process: feature
purpose: Estabilización Mayeuta — PBI-KAIZEN-PACIENTE0-REDEPLOY-20260824
branch_name: feat/kaizen-paciente0-redeploy-fricciones
persist_ref: docs/features/kaizen-paciente0-redeploy-fricciones
pbi_ref: docs/todos/pending/[KAIZEN] Paciente 0 SddIA_AP — redeploy y fricciones operativas.md
document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260824
uuid: "56aff1d3-d5f6-4502-9b5b-e5a57dc718e3"
execution_id: "c95fa63f-be71-481b-a927-475e7c885fd0"
mayeuta_verdict: ok
laudo: absorber-parches-core-un-pr
---

# Clarificación — kaizen-paciente0-redeploy-fricciones

Transcript Mayeuta (2026-08-25). Semilla PBI v1.1.0 con bitácora empírica, métricas y criterios de cierre embebidos → requisito termodinámico. Filtro A contra genoma vigente; no se implementa a ciegas.

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` v1.3.2 (Kaizen multi-componente: gates nuevos + corrección de degradación; no `bug-fix` aislado) |
| `feature_name` | `kaizen-paciente0-redeploy-fricciones` |
| Rama | `feat/kaizen-paciente0-redeploy-fricciones` |
| `persist_ref` | `docs/features/kaizen-paciente0-redeploy-fricciones` |
| `document_id` | `PBI-KAIZEN-PACIENTE0-REDEPLOY-20260824` |
| Init lab | `./sddia-run.sh --process feature` + `SDDIA_LAB_SKIP_PBI_ARCHIVE=1` + `SDDIA_LAB_SKIP_DELIVERY_CLOSE=1` + `SDDIA_AGENT_RUNTIME_COMMAND=` |
| `execution_id` | `c95fa63f-be71-481b-a927-475e7c885fd0` |
| Fase | Estabilización Mayeuta (esta sesión) |
| Antecesor | `kaizen-consumer-ignition-filtro-c` (APTO) — no reabrir salvo mención residual `.dev` `AGENT_RUNTIME_*` |
| Ensayo | `docs/todos/done/[LABORATORIO] MVP Paciente 0 SddIA_AP.md` |

**Toll:** un `persist_ref`, un PR. Prohibido fragmentar F-DEP-* y F-TRIAGE-* en ciclos documentales paralelos.

---

## D1 — F-TRIAGE-03 (inbox pasivo)

| Semilla | Hecho | Laudo |
|---------|-------|-------|
| Inbox WUI filtra solo `actionable` | Coherente con ensayo Paciente 0 F-03 archivado | **Fuera.** PBI UX distinto |
| Fricción usuario: reunión solo en proofs | Efecto de F-TRIAGE-01/02 mal cerrado, no de falta de historial | Este ciclo **corrige el veredicto**, no la superficie |

---

## D2 — Wizard / F-DEP-05 bóveda

| Semilla | Laudo |
|---------|-------|
| Usuario pidió config desde `Proyectos/.dev`; Telegram solo en preprod | Operador: merge en vault staging. Core: **documentar inventario mínimo** consumidor en ONBOARDING/bundle |
| `DT-CONFIG-UX-ONBOARDING` | **Fuera.** No wizard |

---

## D3 — F-TRIAGE-01 vs `email-triage-matrix`

| Semilla PBI | Norma v1.0.0 | Laudo |
|-------------|--------------|-------|
| LLM `passive` no reevalúa asunto | §1 desempate: `actionable` gana a `passive` **solo** con extracción completa; `noise` C-* prevalece | **Congruente.** Clasificacion no puede degradar señal estructural inequívoca |
| Tests `extract_actionable_from_encoded_meeting_subject` ya cubren el patrón | El hueco es el **guard** post-LLM (`verdict.is_empty` OR `actionable && datetime.is_none`) | Dedalo fija el contrato del guard; Mayeuta no diseña el if |
| `decision_path` | §5: refleja el camino que **cerró** | Si eleva extracción post-LLM, Dedalo declara cómo se etiqueta (no silenciar la elevación) |

**Prohibido:** inventar `datetime`; extraer incompleto ⇒ permanece `passive`.

---

## D4 — F-TRIAGE-02 peaje cero

Proof `6e552199-…`: `decision_path: llm`, `tokens_*=0`, `duration_ms: 0`, `verdict: passive`. Peaje cero = inferencia no medible.

| Ítem | Laudo |
|------|-------|
| Auditoría runtime | Verificar invocación efectiva de `mayeuta-llm` / `SDDIA_LLM_*` (sin secretos) |
| Sin LLM | Solo Triaje-C + extracción de asunto (D3). Documentar en ONBOARDING |
| `SDDIA_LLM_REQUIRE_INFER=1` | No emitir `passive` silencioso: degradar a extracción o marcar `classification-degraded` |
| Prompt matriz | Dedalo alinea si el gap es prompt vs guard; Mayeuta no reescribe prompts |

---

## D5 — Redeploy Paciente 0 como gate

| Pregunta | Laudo |
|----------|-------|
| ¿Demo opcional como en Filtro C? | **No.** Criterios de cierre PBI §6: redeploy **sin** parche `{instancia}/start-sddia.sh` + Gate G5 reunión son **O9** de este ciclo |
| Instancia | `/home/racso/Proyectos/SddIA_AP` — canal canónico bundle + `instance-creator` |
| Secretos | Bóveda fuera de git (`.dev` / deploy-vault). Prohibido versionar valores |

---

## D6 — Auditoría empírica (PBI §9)

Al finalizar: documento de auditoría con bitácora redeploy 2026-08-24, métricas §0ter, cadena UID 104579, fricciones absorbidas vs residuales. Locus: `cumulo.paths.json` → `paths.auditsPath` (`docs/audits`). No sustituye `validacion.md`.

---

## D7 — Jurisdicción de mutación

| Locus | Régimen |
|-------|---------|
| `SddIA/process/instance-creator.md`, normas distribución / matriz | DA-2: `entity-manager` |
| `SddIA/engine/.../instance_creator.rs`, `email_triage.rs` | Motor; Dedalo/Tekton en este ciclo |
| `SddIA/scripts/build-release-bundle.sh`, `start-sddia.sh` | Scripts Core; Dedalo fija si el script de instancia se proyecta desde bundle |
| `{instancia}/start-sddia.sh` parche 2026-08-24 | **Prohibido** consolidar como SSOT |

---

## D8 — Parches ad-hoc 2026-08-24

La secuencia A–E del PBI es **evidencia de deuda**, no runbook. Dedalo absorbe el efecto en (1) bundle, (2) `instance-creator`, (3) `start-sddia.sh` del Core.

---

## D9 — Orden de forja

PBI §7 es secuencia de **riesgo**, no sugerencia cosmética. Dedalo no invierte (1) gate ELF antes de (6) redeploy. F-TRIAGE-03 no entra.

---

## D10 — Cierre documental

`task-closure-documental`: PBI → `docs/todos/done/` + `validacion.md` APTO `pbi_archived: true` **en la rama del PR**. Prohibido segundo PR `docs/cerrar-pbi-*`.

---

## D11 — Filtro C residual `.dev` raíz

`SDDIA_AGENT_RUNTIME_*` en `/home/racso/Proyectos/.dev` no se poda en este ciclo (bóveda operador fuera del repo). Instancia consumidor ya debe omitirlos. Mención documental si ONBOARDING toca inventario; no es O-medible.

---

## D12 — L-BUNDLE-STALE v2 (anclaje SHA-256)

Estímulo Racso 2026-08-25: mtime frágil ante checkout/stash; sustituir por hash de fuentes.

| Afirmación | Filtro A | Laudo |
|------------|----------|-------|
| mtime no sobrevive git checkout / stash / clone CI | **Congruente** | Abandonar L-BUNDLE-STALE v1 |
| SHA-256 del working tree vs testigo junto al ELF | **Congruente** | Fail-closed: mismatch o testigo ausente → abort `--skip-build` |
| «El orquestador compila» | **Alucinación de actor** | Compila `cargo` vía `build-release-bundle.sh`, no `execute-process` |
| Cicatriz = `src/` + `Cargo.toml` del crate hoja | **Incompleto** | `execute-process` → `sddia-io`; `event-watcher` → `sddia-daemon-runtime`. Sin cierre `path =` + `Cargo.lock`, skip-build daría falso fresco |
| Purgar `find -newer` en el script | **Alucinación de código** | T1 no forjado; el script vigente **no** evalúa mtime. Purgar la **spec** v1; no inventar delete de lógica inexistente |
| SHA-256 o MD5 | **Incoherente** | Solo SHA-256 (`sha256:<hex>`). MD5 rechazado |
| «Hard Override» como producto | Retórica | `exit ≠ 0` + mensaje: omitir `--skip-build` |

Contrato Dedalo: `spec.md` §1.1. Residual explícito: drift de `rustc` no entra en la cicatriz.

---

## Veredicto

**ok** — semilla + D12. Gate ELF = cicatriz SHA-256 (cierre de compilación). Guard post-LLM y `classification-degraded` siguen en Dedalo/Tekton.
