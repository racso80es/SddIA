---
feature_name: email-triage-heuristic-cold-start
created: "2026-09-06"
process: feature
branch_name: feat/email-triage-heuristic-cold-start
persist_ref: docs/features/email-triage-heuristic-cold-start
pbi_ref: docs/todos/pending/[OPERATIVO] Bucle de Triaje Heurístico y Asimilación de Contexto (Cold-Start).md
execution_id: "5b530130-8225-4904-98f0-a894523f9c7e"
document_id: PBI-EMAIL-TRIAGE-HEURISTIC
pbi_uuid: "2d939386-db39-44f0-804f-1d5ab6ed78c2"
pbi_version: "1.4.0"
slice: 1
status: in-progress
---

# Objetivos — email-triage-heuristic-cold-start

## Misión

Capa de contexto histórico (preferencias del portador) sobre el circuito de triaje ya productivo. Cold-start = matriz vigente (cero regresión). Con hábitos `status: active` en el store JSON, el gateway resuelve `mute`/`priority` sin LLM cuando el patrón es inequívoco, y conjuga preferencias parciales en Clasificacion cuando no lo es.

## Alcance (manifiesto)

- Ciclo `feature` inicializado (`execution_id` `5b530130-…`).
- Slice 1: Triaje-P (`memory:pref-query`) + `P-EXEMPT-C` + `P-MUTE-SENDER` + conjugación de prompt.
- Bump menor de `email-triage-gateway`, `email-triage-matrix`, documentación de `Email_Triaged.decision_path`.
- Tests unitarios del orden §3 y de cold-start.
- Cierre documental en rama + DCC + PR. `accept-pr` condicionado a CI verde.

## Ley aplicada

- Git vía `skill:git-manager`. Troncal `main`.
- DA-2/DA-4: topología `objectives.md` en rama antes de mutar genoma (`library_norms`, `events`, process de códice).
- `features-documentation-pattern` v1.2.1: un PR; `validacion.md` APTO solo con CA-CI verde (`run_id`).
- `CONSTITUTION_CORE` Filtro A: no declarar Done sobre diffs locales.
- Matriz `email-triage-matrix` prevalece sobre heurística del ejecutor o del LLM.
- L-NO-QUERY-EVENT / L-FAIL-POLICY / L-NO-DLT-VALUE de `memoria-preferencias-usuario`.

## Criterios (PBI Slice 1)

| ID | Criterio |
|----|----------|
| CA1 | Cold-start / fail-open: mismos veredictos que handler actual; cero LLM extra vs baseline. |
| CA2 | Mute P activo + C no concluyó → `noise` / `decision_path: preference` / sin `llm:interact`. |
| CA3 | Preferencias parciales viajan en `user_preference_context` v1.0.0. Sin `body`. Sin elevar `actionable` sin `title`+`datetime`. |
| CA5 | `verdict` ∈ {`noise`,`passive`,`actionable`}. |
| CA6 | `decision_path` ∈ {`deterministic`,`llm`,`preference`} coherente con quien cerró. |
| CA7 | Cero mutación IMAP. |
| CA9 | `QuerySpec.subject_key` nunca es la dirección en claro. |
| CA10 | Store caído → bloque vacío versionado → degradación a CA1. |
| CA11 | Sin hábito: `noreply@` → `C-NOREPLY`. Con `explicit_user`+priority max\|high: C no cierra. |

CA4 / CA8 (Slice 2): fuera de este PR.
