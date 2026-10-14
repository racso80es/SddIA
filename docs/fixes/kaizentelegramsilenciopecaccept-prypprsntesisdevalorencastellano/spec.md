---

feature_name: kaizentelegramsilenciopecaccept-prypprsntesisdevalorencastellano
created: "2026-10-14"
updated: "2026-10-14T16:27:00Z"
process: bug-fix
branch_name: fix/kaizentelegramsilenciopecaccept-prypprsntesisdevalorencastellano
persist_ref: docs/fixes/kaizentelegramsilenciopecaccept-prypprsntesisdevalorencastellano
pbi_ref: docs/todos/done/[KAIZEN] Telegram — silencio PEC accept-pr y PPR, síntesis de valor en castellano.md
pbi_ref_inject: docs/todos/pending/[KAIZEN] Telegram — silencio PEC accept-pr y PPR, síntesis de valor en castellano.md
document_id: PBI-KAIZEN-TELEGRAM-PEC-SILENCE-VALUE-ES
uuid: "7f3a9c2e-1b84-4d6f-a9e0-2c5d8b1f4a63"
scope: telegram-pec-silence-value-es
base: main
design_status: sealed
correlation_id: "09a61ea3-b2d9-43c3-b65d-b815385e9a22"
agent: dedalo
phase: Diseño del fix
input_precedence: bug_summary
input_note: "objectives.md ausente en persist_ref; semilla = cuerpo PBI (bug_summary). Inject pending/ inexistente; PBI real en done/ (status: done)."
prior_persist_ref: docs/fixes/kaizen-telegram-pec-silence-value-es
prior_pr_url: "https://github.com/racso80es/SddIA/pull/346"
tg_ca7_language_input: discarded
design_pass: "reaffirm-dedalo-7c65bbcf"
code_coherence: "L1/L2 materializados en route_domain_core.rs + notify_humanized_pr_merged.rs; relectura estática 7c65bbcf"
execution_id: "7c65bbcf-c809-4eb2-9fcd-3bec42aba37c"
---
# Spec — Telegram: silencio PEC accept-pr/PPR + Síntesis de Valor en castellano

## Problema

Tras `PBI-KAIZEN-PR-ADUANA-FRICTION-20261012` el titular PEC ya es legible (`PEC {status}: {process}`), pero el operador sigue recibiendo dos cierres sin valor operativo. Además, el aviso `PullRequest_Merged` omite o emite en inglés el apartado **Síntesis de Valor**.

Fuente: `PBI-KAIZEN-TELEGRAM-PEC-SILENCE-VALUE-ES` (TG-CA1–TG-CA7). `objectives.md` no materializado en este `persist_ref`; se consume el cuerpo del PBI como `bug_summary`.

**Contexto de reentrada:** misma `correlation_id` y mismo `document_id` que la entrega previa bajo `prior_persist_ref` (PR #346). Este artefacto sella el diseño para `execution_id` `7c65bbcf-c809-4eb2-9fcd-3bec42aba37c` y el slug de rama/`persist_ref` inyectados por el runtime.

## Causas → capas

| ID | Hecho | Capa |
|----|-------|------|
| F-PEC-NOISE-ACCEPT-PR | Humanizador always-on para todo PEC; `accept-pr` success duplica `PullRequest_Merged` | L1 |
| F-PEC-NOISE-PPR-FAILED | PEC `failed` de `pull-request-review` (peaje F2 / reintentos) es ruido de aduana | L1 |
| F-SYNTH-LANG-EN | Prompt de síntesis en inglés → modelo responde en inglés | L2 |
| F-SYNTH-ABSENT-OPAQUE | Fail-soft Gemini omite el apartado sin señal; operador no distingue «sin valor» de «inferencia no corrió» | L2 |

## Solución

### L1 — Silencio selectivo en humanizador (TG-CA1, CA2, CA3)

**Único punto:** `build_telegram_message_from_event` en `route_domain_core.rs`, rama `Process_Execution_Completed`.

**Predicado (antes de ensamblar líneas):**

```text
pec_telegram_silent(process_name, status) ⇔
  (process_name == "accept-pr" ∧ status == "success")
  ∨ (process_name == "pull-request-review" ∧ status == "failed")
```

- Si silencioso → `return None`.
- El despacho de `send-telegram-notification` trata `None` como `skipped-empty-message` / exit 0 (**no** invoca la cápsula). Eso satisface «no produce `send-telegram-notification`».

**Conservar** el contrato ADU-CA11–CA14 para todo PEC no silenciado (incl. `accept-pr` failed, `pull-request-review` success, cualquier otro proceso).

**Prohibido:**

- Borrar o estrechar la suscripción orquestación `Process_Execution_Completed` → `send-telegram-notification`.
- Leer `.SddIA/workspaces/` o `.events/` para decidir silencio; solo `process_name` y `status` del payload.
- Silenciar PEC `success` de PPR o PEC `failed` de `accept-pr`.

**Regresión de tests:** unitarios CA1/CA2 esperan `None`; CA3 + ADU-CA11–13 cubren no-silencio.

### L2 — Síntesis de valor: castellano + omisión explícita (TG-CA4, CA5, TG-CA7)

**Archivo:** `notify_humanized_pr_merged.rs` (+ alineación documental de acción solo si se toca genoma vía `entity-manager`).

#### L2.a — Idioma (TG-CA4, TG-CA7)

**Decisión TG-CA7: descartado** el input de idioma vehicular.

- Motivo: un solo llamador de síntesis PR-merged; cláusula estable en el prompt basta (§3.1 PBI, primer bullet).
- No mutar contrato de `gemini-http-infer`.
- No nuevo campo en `notify-humanized-pr-merged` inputs EDA.

**Prompt** (`build_synthesis_prompt_with_git`): instrucción en castellano:

```text
{PROMPT_KERNEL}
Devuelve solo el valor de negocio de este merge, en castellano.
No reiteres hash, auditor, rama ni correlación.
No inventes ficheros, commits ni intención ausentes del CONTEXT.
CONTEXT:
…
```

`PROMPT_KERNEL` (anti-verbosidad) se conserva. Tests afirman presencia de «castellano» / ausencia de `Return only business value`.

#### L2.b — Omisión explícita (TG-CA5)

| Resultado inferencia | Mensaje |
|----------------------|---------|
| Texto no vacío (tras truncate ≤2 líneas, ≤400 chars) | `…\n\n🧠 Síntesis de Valor: {texto}` |
| Fallo cápsula / exit≠0 / success≠true / timeout | `…\n\n🧠 Síntesis de Valor: (omitida — inferencia no disponible)` |
| Cuerpo vacío tras parse | `…\n\n🧠 Síntesis de Valor: (omitida — texto vacío)` |

**API:** `SynthesisOutcome { Ready(String), OmittedInferUnavailable, OmittedEmptyText }` → `assemble_humanized_message(static, outcome)`.

- Fail-soft respecto al **colapso del suscriptor**: fallo Gemini **no** es `System_Fracture_Detected`.
- Prohibido inventar valor de negocio en la omisión.
- Bloque estático (rama, commit, integrador, auditor, URL, correlación) intacto.

#### L2.c — Genoma

Sin input de idioma → **no** forge de contrato cápsula. Si Tekton alinea el prompt documentado en `SddIA/actions/notify-humanized-pr-merged.md`, esa mutación documental sale **solo** vía `entity-manager` (bump cuerpo; sin inputs nuevos). Fase **opcional**.

## Criterios de aceptación (Argos)

| ID | Predicado verificable | Medio |
|----|----------------------|-------|
| TG-CA1 | PEC `accept-pr` + `success` → `build_telegram_message_from_event` = `None`; sin invoke send-telegram | unit `tg_ca1_*` |
| TG-CA2 | PEC `pull-request-review` + `failed` → `None` | unit `tg_ca2_*` |
| TG-CA3 | PEC `accept-pr` failed, PPR success, y PEC de otro proceso → mensaje ADU-CA11–CA14 | unit `tg_ca3_*` + regresión ADU |
| TG-CA4 | Prompt contiene instrucción de castellano; ensamblaje Ready incluye `Síntesis de Valor` sin repetir hash/rama/auditor/correlación en el cuerpo sintético | unit prompt + assemble |
| TG-CA5 | Sin inferencia / texto vacío → línea `(omitida — …)` tipada; suscriptor no colapsa | unit assemble / outcome |
| TG-CA6 | Batería CA1–CA5 presente en humanizador + `notify_humanized_pr_merged` | inventario tests + `cargo test` |
| TG-CA7 | Decisión registrada: input idioma **descartado**; prompt ES; sin mutación `gemini-http-infer` | este spec + diff sin contrato cápsula |

## Fuera de alcance

- Retirar suscripción genérica PEC → Telegram.
- Silenciar PEC de otros procesos, PEC success de PPR, o PEC failed de `accept-pr`.
- Sustituir bloque estático de `PullRequest_Merged` o el fail-soft que evita fractura del suscriptor.
- Versionar ECST `PullRequest_Merged` con diffs o segundo envío Telegram.
- Otros idiomas distintos de castellano.

## Mapa CA → capa

| CA | Capa |
|----|------|
| TG-CA1, CA2, CA3 | L1 |
| TG-CA4, CA5, CA7 | L2 |
| TG-CA6 | L1+L2 (tests) |

## Nota de coherencia (Dedalo)

Pasada `7c65bbcf`: relectura estática PBI (`done/`) + código. Símbolos L1/L2 presentes y alineados: `pec_telegram_silent` + early `None` (`route_domain_core.rs`); `build_synthesis_prompt_with_git` (cláusula castellano), `SynthesisOutcome`, `assemble_humanized_message`, `try_infer_synthesis` (`notify_humanized_pr_merged.rs`); tests `tg_ca1_*`/`tg_ca2_*`/`tg_ca3_*` y CA4/CA5. Sin cambio de arquitectura respecto al sello en `prior_persist_ref` / PR #346 ni pasadas Dedalo `064dae08`/`449293ee`. PBI archivado en `docs/todos/done/`. **No** sustituye batería unitaria ejecutada ni `validacion.md` de esta ejecución (Argos; git vía `skill:git-manager`). Sin mutación Dedalo bajo `docs/todos/`.
