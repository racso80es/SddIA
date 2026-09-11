---
feature_name: email-noise-heuristic-digest
created: "2026-09-11"
process: feature
purpose: Estabilización Filtro A del PBI v1.2.0; corte planificación (L0)
version_clarify: "1.0.0"
execution_id: "8fa709ed-6c4a-4b71-bac1-a2b69ecb0bf3"
pbi_ref: docs/todos/pending/[OPERATIVO] Digest heurístico de ruido de correo (Cuarentena asíncrona).md
document_id: PBI-EMAIL-NOISE-HEURISTIC-DIGEST
pbi_uuid: "bd5eab3f-0693-408a-ab04-de8b43a36c54"
pbi_version: "1.2.0"
slice: all
---

# Clarificación — email-noise-heuristic-digest

Init: `./sddia-run.sh --process feature` + `SDDIA_AGENT_RELAY_IDE=1` + skips archive/delivery. `execution_id` `8fa709ed-6c4a-4b71-bac1-a2b69ecb0bf3`. Rama `feat/email-noise-heuristic-digest`. Mayeuta…Argos: simulated / phase-barrier; relevo IDE.

Semilla: PBI v1.2.0 (Filtro A residual sobre v1.1.0). Corte L0 = clarify + objectives + spec + plan + commit. Sin mutar genoma ni handler en esta parada.

## Decisiones

| ID | Laudo |
|----|-------|
| L-BATCH | Proceso batch CLI/timer de instancia. Cero alta en `event-domain-subscriptions.json`. Cero Clase ECST nueva. Cero agente titular. El «cumulo precedente del gateway» de D2 aplica a `Email_Received`, no a este digest. |
| L-HANDLER | Handler nativo `email_noise_digest.rs` en `execute-process`. Cero cápsula nueva. Reutilizar `proofs_root()` / `eda_instance.proofs` + subdir `email-triaged`. |
| L-FILTER | Conjunto cerrado `{C-LIST, C-NOREPLY, C-SUBJECT-NOISE}` ∧ `verdict=noise` ∧ `decision_path=deterministic`. Prohibido glob `C-*`. Excluir `preference` / `P-MUTE-SENDER` / `llm` / `actionable|passive`. |
| L-NORM | `normalize_email_addr` = trim + `<addr>` + ASCII lowercase. RFC 2047 ya aplicado al persistir el proof. Fallback `_unknown`. Orden `(count DESC, normalized_from ASC)`. Moda de regla con desempate lexicográfico. Asunto del `timestamp` máximo; empate → `event_id` ASC. |
| L-CURSOR | Estado `{daemons_instance.state}/email-noise-digest.json`. Skip si `until <= last_until`. Clamp `effective_since = last_until` si solape. Cursor avanza solo en éxito (vacío o poke OK). Fallo de cápsula → cursor intacto. |
| L-TG | Un mensaje plano. JSON `"parse_mode": null`. Techo 4000 / API 4096. Truncado K + `+ M remitentes omitidos`. `invoke_capsule_json` como `route_domain_core`. |
| L-FORGE | EM create process: `process_jurisdiction: domain`, `process_domain_root: SddIA/library/codexes/codex-kalma2-assistant/process`, `process_contract_version: 1.4.0`. EM update códice membership. Prohibido default `[0]` (software-engineering). Handler/tests ≠ DA-2. |
| L-METRIC | `events_scanned` = matches del filtro completo, no ficheros leídos. |
| L-IMAP | Cero STORE/expunge. Léxico: silenciados / clasificados. |
| L-LLM | Cero `llm:interact` / `mayeuta-llm`. |
| L-CHILD | Pie «¿Inyectar priority:max?» es copy. Réplica = `PBI-EMAIL-DIGEST-PREFERENCE-REPLY`. |
| L-CI | `validacion.md` no `global: APTO` hasta `run_id` verde. |

## Fuera (este ciclo y siempre este PBI)

Réplica humana → preferencias; botonera inline; nuevo daemon; cron en Core; `radamanto-batch`; escanear `./.events/domain/`; membership de `email-quick-action-ingest`; alinear cápsula Telegram al default documental MarkdownV2.
