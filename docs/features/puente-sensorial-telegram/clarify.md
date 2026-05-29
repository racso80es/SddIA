---
feature_name: puente-sensorial-telegram
created: "2026-05-29"
process: feature
purpose: Cierre de decisiones arquitectónicas — Puente Sensorial Telegram (ingesta + notificación)
version_clarify: "1.1.0"
---

# Clarificación — Puente Sensorial Telegram

Transcript de decisiones (2026-05-29) para estabilizar requisitos antes de `spec.md` / `plan.md`.

---

## D1 — Inicio formal

| Pregunta | Decisión |
|----------|----------|
| ¿Proceso de inicio? | **`feature`** v1.3.0 |
| Nombre operativo | **Puente Sensorial Telegram** |
| Rama | `feat/puente-sensorial-telegram` |
| `persist_ref` | `docs/features/puente-sensorial-telegram` |
| PBI | `docs/todos/pending/Puente Sensorial Telegram Ingesta Externa y Notificación.md` |
| Inputs SSOT | `_init-feature.json` |
| API externa | [Telegram Bot API](https://core.telegram.org/bots/api) |

---

## D2 — Variables de entorno (Bóvedas)

| Variable | Alcance | Decisión |
|----------|---------|----------|
| `TELEGRAM_BOT_TOKEN` | Secreto | Solo bóveda **local** `.SddIA/.dev/.env` (nunca commit). Plantilla documentada en `spec.md` §2 |
| `TELEGRAM_ALLOWED_CHAT_ID` | Identidad | Mismo fichero; comparación **estricta** string (chat id numérico como texto) |
| Carga runtime | Daemon + tool | `load_hierarchical_env(repo_root)` al arranque (patrón `event-watcher.py`) |
| Ausencia de token/chat | Fail-fast | Tool y daemon **exit ≠ 0** con mensaje claro; sin llamadas a API ni `execute-process` |

**Motivo:** alinear con `ampliacion-configuracion-entornos`; el PBI nombra `.env` del nodo local → resolución canónica vía jerarquía de bóvedas.

---

## D3 — Capa 0: Demonio físico (`telegram-watcher`)

| Pregunta | Decisión |
|----------|----------|
| Ubicación | `SddIA/scripts/daemons/telegram-watcher.py` |
| Mecánica | Long polling `getUpdates`; persistir `last_update_id` en `.SddIA/daemons/state/telegram-watcher.json` (gitignored) |
| Filtro intruso | Si `message.chat.id` ≠ `TELEGRAM_ALLOWED_CHAT_ID` → **descartar** silenciosamente (log stderr mínimo, sin CLI, sin IA) |
| Payload al Core | Solo texto plano: `execute-process.py --process telegram-gateway --inputs '{"text":"<texto_limpio>"}'` |
| Ceguera | El daemon **no** parsea TODO/Kaizen; no escribe en `.events/` |
| Modos CLI | `--once` (lab/smoke) + bucle por defecto (operación) |
| ED Centinela (kitchen) | **Fuera de alcance MVP** — no bloquear esta feature por `daemons-contract` / `governance-daemon-manager`; contrato Centinela queda en PBI kitchen |

---

## D4 — Capa 1: Proceso lógico (`telegram-gateway`)

| Pregunta | Decisión |
|----------|----------|
| Naturaleza | Proceso SddIA nuevo bajo `SddIA/process/telegram-gateway.md` |
| Input | `text` (string, REQUIRED) |
| Transmutación MVP | **Determinística** (sin LLM): regex/prefijos acordados en `spec.md` §4 |
| Eventos mínimos | Forjar e inyectar al menos **`Manual_Task_Requested`**; **`Kaizen_Idea_Captured`** si patrón `TODO:` (case-insensitive, trim) |
| Depósito bus | `write_fractal_event(..., "domain")` → `./.events/pending/` (topología EDA existente) |
| Consultas libres | **Fuera de alcance MVP** — mensajes sin patrón → log + exit 0 sin evento (no consumir orquestador) |
| Mayeuta/Tekton en runtime | Reservado para evolución post-MVP (PBI menciona orquestador; no obligatorio en primera entrega) |

---

## D5 — Capa eferente: Tool `send-telegram-notification`

| Pregunta | Decisión |
|----------|----------|
| Naturaleza | Tool inerte (`tools-contract`); cápsula Python bajo `SddIA/scripts/tools/send-telegram-notification/` |
| Input JSON | `{"message": string, "parse_mode": "MarkdownV2" \| "HTML" \| null}` — default `MarkdownV2` |
| Ceguera espacial | La tool **no** lee `event_type`, PR ni motivo; solo POST `/sendMessage` |
| Consumo inicial | Suscriptor EDA opcional en `event-subscriptions.json` para `PullRequest_Presented` y `System_Fracture_Detected` → invocación vía `execute-action` / handler lab (no hardcode en Argos `.md`) |
| Mensaje PR | Plantilla fija: rama + `pr_url` si presente en payload del evento |
| Mensaje fractura | Plantilla fija: `trace_hash` / `incident_ref` del payload ECST |
| **Táctica del Refugio** (Fricción de Acero) | La tool **no puede fallar por formato**. Si Telegram rechaza el envío por parsing (`400`, p. ej. `can't parse entities`), la cápsula reintenta **una vez** de inmediato con el **mismo `message`** y **sin `parse_mode`** (texto plano). Directriz suprema: el mensaje táctico llega al dispositivo; la estética es prescindible |
| Orden de defensa | (1) Intento con `parse_mode` solicitado (+ `escape_markdown_v2` si `MarkdownV2`); (2) solo si fallo **clasificado como parsing**, degradación plain; (3) `success: false` únicamente si ambos intentos fallan |

---

## D6 — Eventos de dominio (genoma)

| event_type | Disparador texto (MVP) | Emisor autorizado |
|------------|------------------------|-------------------|
| `Manual_Task_Requested` | Cualquier texto válido tras filtro Centinela que **no** coincida con `TODO:` | `telegram-gateway` |
| `Kaizen_Idea_Captured` | Prefijo `TODO:` (regex `^\s*TODO:\s*(.+)$`, i) | `telegram-gateway` |

| Decisión | Detalle |
|----------|---------|
| Familia | `domain` / `event_family: domain` |
| Contrato | `events-contract v1.1.0` |
| Suscripciones | **No** añadir suscriptores obligatorios en MVP salvo notificación eferente (D5); Ola C enruta vía `event-watcher` si se suscribe después |
| EDA coverage | UUIDs nuevas Clases → upsert `eda-coverage.json` en Tekton |

---

## D7 — Idempotencia y seguridad

| ID | Regla |
|----|-------|
| S1 | Tras procesar update `id`, persistir `last_update_id`; siguiente poll usa `offset = last_update_id + 1` |
| S2 | Reinicio daemon: no re-ejecutar updates ya confirmados por Telegram |
| S3 | Intrusiones: cero invocaciones `execute-process` y cero escrituras bus |
| S4 | Tool: timeout HTTP acotado (30s); **máximo 2 POST** por invocación (formato + refugio plain); sin bucles adicionales |
| S5 | **Fricción de Acero — parsing:** error `400` de Telegram por entidades/markup inválido → **obligatorio** reintento inmediato sin `parse_mode`; no propagar fallo al bus ni perder alerta Argos/Mayeuta |
| S6 | Errores **no** de parsing (401 token, 403, 5xx, red) → **sin** degradación plain; `success: false` en envelope |
| S7 | `escape_markdown_v2` es **primera línea**, no sustituto del refugio: reportes con `_`, `*`, URLs sin escapar deben entregar igual vía fallback |

---

## D8 — Alcance vs deuda relacionada

| Artefacto | Relación | Decisión |
|-----------|----------|----------|
| `docs/todos/kitchen/ED Centinela Soberanía...` | Daemon gobernado + heartbeat | **Dependencia blanda** — esta feature entrega watcher mínimo; refactor a Centinela en feature futura |
| `docs/todos/kitchen/Transmutación de Telemetría SddIA.MD` | Telemetría | Sin bloqueo |
| `docs/todos/pending/Argos_Eda_Emision.md` | Emisión Argos | Notificación Telegram vía tool suscrita; no duplicar lógica de emisión ECST |

---

## D9 — Criterios de aceptación PBI → AC feature

| PBI DoD | AC feature |
|---------|------------|
| Inmunidad chat_id | AC1 smoke intruso |
| Idempotencia update_id | AC2 reinicio daemon |
| Ceguera tool salida | AC3 contrato tool + test cápsula |
| Desacople gateway → bus | AC4 smoke `Manual_Task_Requested` en `pending/` |
| Táctica del Refugio | AC7 smoke mensaje MarkdownV2 defectuoso → entrega plain + `degraded_plain_fallback: true` |

---

## D10 — Estado de planificación

| Fase feature | Estado |
|--------------|--------|
| Inicialización git | ✅ Rama `feat/puente-sensorial-telegram` |
| Clarificación (Mayeuta) | ✅ este documento (v1.1.0 — Fricción de Acero / Refugio) |
| Objetivos | ✅ `objectives.md` |
| Especificación (Dedalo) | ✅ `spec.md` |
| Plan (Dedalo) | ✅ `plan.md` |
| Ejecución (Tekton) | ✅ T1–T6 |
| Cierre documental | ✅ `validacion.md` APTO + PBI en `done/` |

**Siguiente:** `delivery-close-cycle` → PR único (operador).
