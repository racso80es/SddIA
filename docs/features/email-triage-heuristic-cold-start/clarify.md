---
feature_name: email-triage-heuristic-cold-start
created: "2026-09-06"
process: feature
purpose: Estabilización Filtro A del PBI v1.4.0; laudos D1–D3; corte Slice 1
version_clarify: "1.0.0"
execution_id: "5b530130-8225-4904-98f0-a894523f9c7e"
pbi_ref: docs/todos/pending/[OPERATIVO] Bucle de Triaje Heurístico y Asimilación de Contexto (Cold-Start).md
document_id: PBI-EMAIL-TRIAGE-HEURISTIC
pbi_uuid: "2d939386-db39-44f0-804f-1d5ab6ed78c2"
pbi_version: "1.4.0"
slice: 1
---

# Clarificación — email-triage-heuristic-cold-start

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id` `5b530130-8225-4904-98f0-a894523f9c7e`. Rama `feat/email-triage-heuristic-cold-start`. Mayeuta…Argos: simulated / phase-barrier; relevo IDE.

Semilla: PBI v1.4.0 (Filtro A aplicado). Dirty de `source_sha256` en daemons/tools restaurado a HEAD antes del init (no entra en este ciclo).

## Decisiones

| ID | Laudo |
|----|-------|
| L-SLICE | Este ciclo = **Slice 1** (Triaje-P + conjugación). Slice 2 (botonera) bloqueado por `PBI-TELEGRAM-INLINE-KEYBOARD`. CA4/CA8 = N/A. |
| L-HANDLER | Extender handler nativo `email_triage.rs`. Cero cápsula nueva de triaje. Input sigue `event_file_path`. |
| L-ORDER | Orden canónico D2: query P (siempre, fail-open) → `P-EXEMPT-C` → muro C → `P-MUTE-SENDER` → Clasificacion + `user_preference_context` → Asiento/Emision. |
| L-EXEMPT | `P-EXEMPT-C` solo si hábito `status: active` ∧ `authority: explicit_user` ∧ `predicate: priority` ∧ `value.level` ∈ {`max`,`high`}. `inferred`/`proposed` no eximen. No cierra `actionable`. |
| L-MUTE | `P-MUTE-SENDER` solo si C no concluyó. `predicate: mute` activo (`value.muted: true`; `until` futuro o null). `decision_path: preference`. Skip LLM. |
| L-HASH | `subject_key` = SHA-256 hex UTF-8 de `normalize_email_addr(from)`: RFC2047 decode; extraer `local@domain` de `Nombre <addr>`; lowercase; trim. Función en `user-preference-core`. Nunca addr en claro en QuerySpec/ECST. |
| L-FAIL | Store caído / query error → bloque `{schema_version: "1.0.0", preferences: []}` = CA1. Vacío ≠ permitir todo. |
| L-PROMPT | Conjugación: prompt Clasificacion += bloque JSON `user_preference_context` v1.0.0. Sin `body`. Sin `sensitivity: personal` en logs CLI. Cold-start (bloque vacío) = prompt actual. |
| L-PATH | `decision_path` de quien **cerró**: `preference` (mute), `deterministic` (C), `llm` (Clasificacion). `P-EXEMPT-C` no es cierre. |
| L-FORGE | Genoma vía `entity-manager` `update`: proceso empacado (`process_jurisdiction: domain`, root códice kalma2), norma `email-triage-matrix` 1.1.0, evento `email-triaged` replacements (UUID inmutable). Handler/crate ≠ DA-2. |
| L-EVENT-VER | Replacements de evento **no** bumpan SemVer de frontmatter (precedente L-FORGE telegram). Cuerpo documenta enum `preference`. Hash sí se recálcula. |
| L-IMAP | Cero STORE/expunge/delete en todos los caminos. |
| L-CI | `validacion.md` no `global: APTO` hasta `run_id` verde. `accept-pr` solo tras checks verdes del PR. |

## Fuera (este ciclo)

Slice 2 botonera; LanceDB; digest de cuarentena; réplica digest; FSM «ajustar regla»; ampliar `actionable` sin `title`+`datetime`; override C por `inferred`.
