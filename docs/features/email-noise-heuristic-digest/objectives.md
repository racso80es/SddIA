---
feature_name: email-noise-heuristic-digest
created: "2026-09-11"
process: feature
branch_name: feat/email-noise-heuristic-digest
persist_ref: docs/features/email-noise-heuristic-digest
pbi_ref: docs/todos/pending/[OPERATIVO] Digest heurístico de ruido de correo (Cuarentena asíncrona).md
execution_id: "8fa709ed-6c4a-4b71-bac1-a2b69ecb0bf3"
document_id: PBI-EMAIL-NOISE-HEURISTIC-DIGEST
pbi_uuid: "bd5eab3f-0693-408a-ab04-de8b43a36c54"
pbi_version: "1.2.0"
status: in-progress
---

# Objetivos — email-noise-heuristic-digest

## Misión

Informe batch determinista de ruido Triaje-C a partir de proofs durables `email-triaged`. El portador ve qué remitentes silenció el muro cold-start. Cero LLM. Cero IMAP. Cero fan-out por evento.

## Alcance (manifiesto)

- Ciclo `feature` inicializado (`execution_id` `8fa709ed-…`). Corte L0: planificación. Código y genoma en L1–L5 del mismo PR.
- Proceso `email-noise-digest` empacado en `codex-kalma2-assistant` (root `[1]`, no Core ni software-engineering).
- Handler nativo en `execute-process`: filtro cerrado C, agregación, cursor, poke Telegram plano.
- Tests `cargo test -p execute-process --lib -- email_noise_digest`.
- Cierre documental en rama + DCC + PR. `accept-pr` condicionado a CI verde.

## Ley aplicada

- Git vía `skill:git-manager`. Troncal `main`. Rama `feat/email-noise-heuristic-digest`.
- DA-2/DA-4: topología `objectives.md` en rama **antes** de mutar process de códice / `library_codexes`.
- `features-documentation-pattern` v1.2.1: un PR; `validacion.md` APTO solo con CA-CI verde (`run_id`).
- `CONSTITUTION_CORE` Filtro A: no Done sobre diffs locales.
- Matriz `email-triage-matrix`: conjunto cerrado de reglas C.
- `process-creator` v1.2.0: alta domain nueva exige `process_jurisdiction: domain` + `process_domain_root` explícito en multi-root.
- Evento `Email_Triaged` v1.1.0: `from`/`subject` OPTIONAL; `body`/`snippet` FORBIDDEN.

## Criterios (PBI v1.2.0)

| ID | Criterio |
|----|----------|
| CA-1 | Filtro completo sobre envelope + payload; conjunto cerrado de reglas C. |
| CA-2 | Normalización `normalize_email_addr`; desempate alfabético de remitente. |
| CA-3 | Moda de regla; asunto del timestamp más reciente; `(sin asunto)` si vacío. |
| CA-4 | 0 matches → silencio + cursor; cero poke. |
| CA-5 | Matches → un mensaje; `"parse_mode": null`. |
| CA-6 | Truncado ≤ 4096 con línea de omisión. |
| CA-7 | Skip / clamp / fallo de poke no avanza cursor. |
| CA-8 | Root kalma2 + membership + dispatch; cero suscripción de dominio. |
| CA-9 | Batería `email_noise_digest` verde. |
| CA-10 | Forja process/códice solo vía `entity-manager` con flags de jurisdicción. |
