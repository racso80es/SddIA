---
document_id: PBI-EMAIL-NOISE-HEURISTIC-DIGEST
uuid: "bd5eab3f-0693-408a-ab04-de8b43a36c54"
title: "[OPERATIVO] Digest heurístico de ruido de correo (Cuarentena asíncrona)"
format: markdown
version: "1.0.0"
created: "2026-09-06"
updated: "2026-09-06"
status: "propuesta"
refinement_status: unrefined
priority: media
type: operativo
process: feature
dispatch: false
suggested_branch: feat/email-noise-heuristic-digest
persist_ref_suggested: docs/features/email-noise-heuristic-digest
depends_on: []
spawned_by: PBI-EMAIL-TRIAGE-HEURISTIC
related:
  - docs/todos/pending/[OPERATIVO] Bucle de Triaje Heurístico y Asimilación de Contexto (Cold-Start).md
  - docs/todos/pending/[OPERATIVO] Réplica del digest de ruido → preferencias.md
  - SddIA/engine/execute-process/src/engine/handlers/email_triage.rs
  - SddIA/events/domain/email-triaged.md
  - SddIA/core/cumulo.paths.json
  - SddIA/tools/send-telegram-notification.md
  - SddIA/library/codexes/codex-kalma2-assistant.md
  - SddIA/library/codexes/codex-kalma2-assistant/process/email-triage-gateway.md
  - SddIA/process/radamanto-batch.md
  - SddIA/interfaces/kalma2-bridge/src/main.rs
---

### [OPERATIVO] Digest heurístico de ruido de correo (Cuarentena asíncrona)

#### Origen

Laudo D2 de `PBI-EMAIL-TRIAGE-HEURISTIC` v1.4.0: el muro Triaje-C permanece en cold-start; el crecimiento de contexto no se hace correo a correo. Este PBI materializa el **informe batch**. La acción de respuesta humana es `PBI-EMAIL-DIGEST-PREFERENCE-REPLY` (fuera de alcance).

#### Filtro A — qué no es este activo

| Tentación | Hecho SSOT |
|-----------|------------|
| «Telemetría del Core» | `Email_Triaged` es `event_family: domain`. Telemetría = `Raw_Execution_Finished` / `Daemon_Heartbeat` en `./.events/telemetry/` |
| `radamanto-batch` / agente Radamanto | Consume `Raw_Execution_Finished`; `llm_profile: none`; no interpreta correo. Prohibido reutilizarlo |
| Job de Argos-juez | Argos hoy es agente del **tool** `send-telegram-notification` en el fan-out de `Email_Triaged`, no un batch diario |
| Escanear `./.events/domain/` | Bus fractal volátil (sweeper). Fuente durable: `eda_instance.proofs` → `.SddIA/proofs/email-triaged/` |
| «Estado Discovery» | No existe en genoma (ni veredictos ni `PreferenceStatus`) |
| «Hoy purgué N remitentes» | Prohibido IMAP STORE. El veredicto es `noise`. Léxico: clasificados / silenciados |
| Solo `C-NOREPLY` | El muro incluye `C-LIST` y `C-SUBJECT-NOISE`. Un newsletter no es necesariamente noreply |
| Depender de Triaje-P | El gateway **ya** escribe proof de `noise`. Este PBI puede implementarse en paralelo a Slice 1 del padre |
| LLM para «destacan» | Ranking por frecuencia es determinista. Cero `llm:interact` |
| Cron en el Core | No hay `OnCalendar`/timer en el repositorio. El Core es agnóstico: el proceso recibe `since`/`until`. El estímulo periódico es de **instancia** |

#### Circuito propuesto

```
estímulo instancia (timer/systemd local, fuera del genoma Core)
  → execute-process email-noise-digest
       inputs: { since, until }   # ISO-8601; ventana cerrada-abierta [since, until)
  → leer proofs .SddIA/proofs/email-triaged/
  → agregar
  → send-telegram-notification (un mensaje; vacío ⇒ success silencioso, cero poke)
```

- **Empaque:** `SddIA/library/codexes/codex-kalma2-assistant/process/` (semántica de correo; no `SddIA/process/`). Alta en `process_membership` del códice vía `entity-manager`.
- **Agente titular:** `cumulo` (precedente `email-triage-gateway`). Tool eferente: `send-telegram-notification`.
- **Handler nativo** en `execute-process` (mismo patrón que el gateway). No cápsula nueva de triaje.

#### Filtro de inclusión

Proofs cuyo payload cumple **todo**:

- `verdict == noise`
- `decision_path == deterministic`
- `matched_rule` ∈ {`C-LIST`, `C-NOREPLY`, `C-SUBJECT-NOISE`}
- `timestamp` ∈ `[since, until)` (campo del proof / evento; no del IMAP)

**Excluir:** `matched_rule: P-MUTE-SENDER` (hábito ya asimilado). `passive` fuera de alcance. Sin `from`: agrupar bajo clave `_unknown` (no inventar remitente).

#### Agregación (determinista)

Por `from` (texto táctico ya presente en `Email_Triaged` v1.1.0 OPTIONAL; el poke accionable ya lo muestra en Telegram):

- `count`
- `matched_rule` (si mixto, el de mayor count)
- `subject` de la ocurrencia más reciente (no `snippet`; FORBIDDEN en el evento)

Salida humana, techo API Telegram 4096 chars; si no cabe, N remitentes + top K + «+M omitidos». Distinguir **eventos** vs **remitentes únicos**. Ejemplo de léxico lícito:

```text
Ruido Triaje-C 2026-09-06
eventos=12 remitentes=4
- envios@proveedor.example (3, C-NOREPLY) Pedido entregado
- newsletter@marca.example (7, C-LIST) Ofertas
¿Inyectar priority:max? Respuesta en ciclo aparte.
```

Idempotencia: cursor de instancia (p.ej. bajo `.SddIA/daemons/state/`, gitignored) con `until` de la última ventana enviada. Re-ejecutar la misma ventana ⇒ skip (success, cero segundo mensaje).

#### Contratos

- Proceso nuevo: inputs `since`, `until`; outputs `events_scanned`, `senders`, `notified`.
- **No** nuevo evento de dominio en MVP (estímulo = invocación CLI). Evento `Email_Noise_Digest_Requested` = deuda opcional, no gate.
- `capsule-json-io` solo si se invoca el tool de Telegram (ya lo cumple).
- Cero mutación de `email-triage-matrix` (el digest no cambia veredictos).

#### Criterios de aceptación (borrador)

- [ ] Ventana con proofs `noise` C-* produce un único mensaje Telegram; agrega por `from`.
- [ ] Ventana vacía o solo `P-MUTE-SENDER` / `actionable` / `passive`: `success`, cero mensaje.
- [ ] No lee `./.events/domain/` como fuente; no invoca `radamanto-batch`; no LLM; no IMAP.
- [ ] Re-run idempotente de la misma ventana.
- [ ] Truncado seguro ≤ 4096 chars.
- [ ] Empaque bajo códice kalma2; Core sin cron cableado.
