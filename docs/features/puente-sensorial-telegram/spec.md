---
feature_name: puente-sensorial-telegram
created: "2026-05-29"
process: feature
base: main
scope: telegram-watcher, telegram-gateway, send-telegram-notification, domain-events, event-subscriptions, env-bovedas, eda-coverage, qa-smokes
pbi_ref: docs/todos/pending/Puente Sensorial Telegram Ingesta Externa y Notificación.md
document_id: PBI-PUENTE-SENSORIAL-TELEGRAM
---

# Especificación técnica — Puente Sensorial Telegram

## 1. Contexto

Estado actual: el ecosistema no expone canal Telegram. El PBI exige separación estricta **física (Capa 0)** vs **lógica (Gateway ED)** vs **actuación ciega (tool eferente)**.

```text
  [Telegram Cloud]
        │
        │ getUpdates (long poll)
        ▼
  telegram-watcher.py ──filtro chat_id──► execute-process telegram-gateway
        │                                      │
        │                                      ▼
        │                              transmutación regex
        │                                      │
        │                                      ▼
        │                              .events/pending/ (domain)
        │                                      │
        │                                      ▼
        │                              event-watcher → Ola C
        │
  event-subscriptions ──► execute-action/tool ──► send-telegram-notification
        │                                              │
        │                                              ▼
        └──────────────────────────────────────► sendMessage API
```

## 2. Configuración y secretos

### 2.1 Variables

| Variable | Obligatoria | Descripción |
|----------|:-----------:|-------------|
| `TELEGRAM_BOT_TOKEN` | Sí | Token BotFather |
| `TELEGRAM_ALLOWED_CHAT_ID` | Sí | Chat o usuario autorizado (string decimal) |

### 2.2 Plantilla operador (no versionada)

Fichero: `.SddIA/.dev/.env` (ejemplo para README feature / runbook):

```env
TELEGRAM_BOT_TOKEN=<token>
TELEGRAM_ALLOWED_CHAT_ID=<chat_id>
```

### 2.3 Validación arranque

| Componente | Si falta variable |
|------------|-------------------|
| `telegram-watcher.py` | Exit 2, stderr `[telegram-watcher] TELEGRAM_* no configurado` |
| `send-telegram-notification` | Exit 2, envelope JSON error |

## 3. Tool — `send-telegram-notification`

### 3.1 Contrato genoma

| Campo | Valor |
|-------|-------|
| `name` | `send-telegram-notification` |
| `contract` | `tools-contract v1.2.0` |
| `context` | `ecosystem-evolution` |
| `implementation_path_ref` | `scripts/tools/send-telegram-notification` |

### 3.2 Input (stdin JSON)

```json
{
  "message": "Texto a enviar",
  "parse_mode": "MarkdownV2"
}
```

| Campo | Tipo | Obligatorio | Notas |
|-------|------|:-----------:|-------|
| `message` | string | Sí | Longitud ≤ 4096 (límite API) |
| `parse_mode` | string \| null | No | Default `MarkdownV2`; valores `HTML`, `null` (plain) |

### 3.3 Ejecución

- Endpoint: `POST https://api.telegram.org/bot{token}/sendMessage`
- Body: `chat_id`, `text`, `parse_mode` (omitir clave si plain / refugio)
- **Prohibido:** leer ficheros del repo, inspeccionar bus, ramificar por tipo de evento.
- **Prohibido:** devolver `success: false` por error de parsing si el reintento plain aún no se intentó.

#### 3.3.1 Pipeline de envío (máximo 2 POST)

```text
Intento 1 — según input.parse_mode (default MarkdownV2)
  ├─ Si parse_mode == MarkdownV2 → aplicar escape_markdown_v2(message) al text enviado
  ├─ POST con parse_mode en body (omitir si null)
  └─ OK → success true, attempt 1, fin

  └─ FAIL clasificado parsing → Intento 2 (Refugio)
        ├─ Mismo message original (sin re-escape; texto crudo del caller)
        ├─ POST sin parse_mode (plain)
        └─ OK → success true, degraded_plain_fallback true, attempt 2, fin
              FAIL → success false, attempts 2, fin
```

#### 3.3.2 Clasificación error parsing (Telegram)

Considerar **fallo de parsing** cuando la API responde HTTP 400 **y** el cuerpo JSON incluye `description` que coincide (case-insensitive) con alguno de:

- `can't parse entities`
- `can't parse message text`
- `character` + `is reserved` (MarkdownV2)
- `wrong html` / `can't parse` (modo HTML)

Cualquier otro 4xx/5xx → **no** activar refugio (S6 `clarify.md`).

#### 3.3.3 Táctica del Refugio (restricción dura Fase A)

| Principio | Regla |
|-----------|-------|
| Prioridad | **Entrega táctica** > estética del mensaje |
| Motivación | Reportes Argos/Mayeuta con `_`, `*`, URLs o bloques sin escapar no deben tumbar la alerta |
| Reintentos | Exactamente **un** refugio plain por invocación; sin tercer intento |
| Ceguera | El fallback no inspecciona quién generó el texto; solo reacciona al código API |

#### 3.3.4 Output envelope

```json
{
  "success": true,
  "message_id": 12345,
  "attempt": 2,
  "degraded_plain_fallback": true,
  "parse_mode_requested": "MarkdownV2",
  "error": null
}
```

| Campo | Cuándo |
|-------|--------|
| `attempt` | `1` o `2` |
| `degraded_plain_fallback` | `true` solo si el éxito fue en intento 2 tras parsing fallido |
| `parse_mode_requested` | Valor del input (trazabilidad) |
| `error` | Poblado solo si `success: false` tras ambos intentos o error no-parsing |

### 3.4 Smoke cápsula

```bash
python SddIA/scripts/tools/send-telegram-notification/main.py <<'EOF'
{"message": "SddIA smoke puente-sensorial-telegram"}
EOF
```

### 3.5 Smoke Táctica del Refugio (obligatorio Fase A)

Mensaje **deliberadamente inválido** en MarkdownV2 (p. ej. `"Alerta: PR #42 _sin_cerrar"` o texto con `*` suelto). Mock o bóveda real:

- Intento 1 → 400 parsing (verificar en log/mock).
- Intento 2 plain → `success: true`, `degraded_plain_fallback: true`.

Artefacto: `_smoke-telegram-refugio-markdown.json` en `persist_ref`.

## 4. Proceso — `telegram-gateway`

### 4.1 Contrato proceso

| Campo | Valor |
|-------|-------|
| `name` | `telegram-gateway` |
| `version` | `1.0.0` |
| `contract` | `process-contract v1.4.0` |
| Fases | Una: **Transmutación e inyección** |

### 4.2 Inputs

| Input | Tipo | Obligatorio |
|-------|------|:-----------:|
| `text` | string | Sí |

### 4.3 Reglas de transmutación (MVP)

| Orden | Condición | `event_type` | Payload mínimo |
|-------|-----------|--------------|----------------|
| 1 | `re.match(r'^\s*TODO:\s*(.+)$', text, re.I)` | `Kaizen_Idea_Captured` | `idea_text`, `source: "telegram"`, `raw_text` |
| 2 | `text.strip()` no vacío | `Manual_Task_Requested` | `task_text`, `source: "telegram"`, `raw_text` |
| 3 | vacío / solo espacios | — | No emitir; exit 0 |

### 4.4 Emisión bus

- Función: `write_fractal_event(repo, event_dict, "domain")` (`execute_process_capsules` / `eda_bus_utils`).
- Instancia ECST: UUID v4, `event_type` según tabla, `emitter_agent: "telegram-gateway"`, timestamp ISO.
- Validación: schemas cargados tras forja de Clases en `SddIA/events/domain/`.

### 4.5 Invocación lab

```bash
python SddIA/scripts/qa/execute-process.py --process telegram-gateway --inputs '{"text":"TODO: Revisar auditorías"}'
```

## 5. Daemon — `telegram-watcher.py`

### 5.1 Responsabilidades

| Hace | No hace |
|------|---------|
| Long polling `getUpdates` | Interpretar TODO/Kaizen |
| Filtrar `chat.id` | Escribir en `.events/` |
| Persistir `last_update_id` | Invocar IA |
| Invocar `execute-process` con texto | Loggear token |

### 5.2 Estado persistente

Path: `.SddIA/daemons/state/telegram-watcher.json` (añadir a `.gitignore` global si no existe):

```json
{ "last_update_id": 123456789 }
```

### 5.3 Bucle polling

1. `GET .../getUpdates?offset={last+1}&timeout=30`
2. Por cada `result[]`: validar chat → extraer `message.text` (ignorar sin texto)
3. `subprocess` → `execute-process.py` (mismo patrón que `event-watcher.py` § `_invoke_route_process`)
4. Actualizar `last_update_id` al máximo `update_id` procesado (incluso si gateway falla — evitar bucle infinito; registrar error stderr)

### 5.4 CLI

```text
telegram-watcher.py              # bucle
telegram-watcher.py --once       # un poll (lab)
telegram-watcher.py --dry-run    # imprimir decisión sin execute-process (lab)
```

## 6. Eventos de dominio (genoma)

### 6.1 `Manual_Task_Requested`

| Campo payload | Tipo | REQUIRED |
|---------------|------|:--------:|
| `task_text` | string | Sí |
| `source` | string | Sí (const `telegram`) |
| `raw_text` | string | Sí |

### 6.2 `Kaizen_Idea_Captured`

| Campo payload | Tipo | REQUIRED |
|---------------|------|:--------:|
| `idea_text` | string | Sí (grupo captura TODO) |
| `source` | string | Sí |
| `raw_text` | string | Sí |

Archivos: `SddIA/events/domain/manual-task-requested.md`, `kaizen-idea-captured.md` (kebab-case). Actualizar `SddIA/events/domain/index.md`.

## 7. Notificaciones eferentes (suscripciones)

Añadir en `SddIA/core/event-subscriptions.json`:

| event_type | Suscriptor | Mecanismo |
|------------|------------|-----------|
| `PullRequest_Presented` | tool `send-telegram-notification` | Handler existente de fan-out (mismo patrón que tools en orquestación) o entrada nueva con `intent` notificación humana |
| `System_Fracture_Detected` | idem | Mensaje con `trace_hash` / resumen corto |

**Plantilla mensaje PR** (puede ir con `parse_mode: MarkdownV2`; la tool aplica refugio si el agente inyecta caracteres sin escapar):

```text
PR presentado: {branch}
{pr_url opcional}
```

**Plantilla fractura:**

```text
Fractura detectada: {trace_hash}
```

Implementación Tekton: preferir **acción delgada** `notify-telegram-human` que solo mapea payload→mensaje y delega en la tool (mantiene ceguera de la tool). Si el tiempo apremia, handler inline en `execute_process_capsules` documentado en `implementation.md`.

## 8. Criterios de aceptación (AC)

| AC | Descripción | Verificación |
|----|-------------|--------------|
| AC1 | Chat no autorizado bloqueado | `--dry-run` con fixture JSON chat intruso → 0 invocaciones process |
| AC2 | Idempotencia | Dos arranques `--once` con mismo fixture mock → un solo `execute-process` |
| AC3 | Ceguera tool | Revisión estática: cápsula sin imports de `eda_bus`, `event-subscriptions` |
| AC4 | Evento en bus | Gateway con `TODO: x` → fichero en `.events/pending/` tipo `Kaizen_Idea_Captured` |
| AC5 | Manual task | Gateway con `Hacer backup` → `Manual_Task_Requested` |
| AC6 | Notificación | Smoke lab emite `PullRequest_Presented` sintético → tool invocada (mock HTTP opcional en CI) |
| AC7 | **Táctica del Refugio** | Mensaje MarkdownV2 inválido → 2º POST plain exitoso; `degraded_plain_fallback: true`; nunca `success: false` solo por parsing |

## 9. Tests y smokes

| ID | Comando / artefacto |
|----|---------------------|
| S1 | `_smoke-telegram-gateway-todo.json` en `persist_ref` |
| S2 | `_smoke-telegram-intruder-chat.json` |
| S3 | `test_telegram_watcher_idempotency.py` (mock API) — opcional si tiempo |
| S4 | `audit-entity-eda-coverage.py --scan` post-forja |
| S5 | `_smoke-telegram-refugio-markdown.json` — AC7 refugio plain |

## 10. Touchpoints previstos (Tekton)

| Path | Acción |
|------|--------|
| `SddIA/tools/send-telegram-notification.md` | CREATE |
| `SddIA/scripts/tools/send-telegram-notification/main.py` | CREATE |
| `SddIA/process/telegram-gateway.md` | CREATE |
| `SddIA/scripts/qa/execute_process_capsules.py` | Handler fase gateway + notify |
| `SddIA/process/index.md` | Índice |
| `SddIA/tools/index.md` | Índice |
| `SddIA/events/domain/*.md` | 2 Clases |
| `SddIA/core/event-subscriptions.json` | Suscriptores notify |
| `SddIA/core/eda-coverage.json` | Upsert |
| `SddIA/scripts/daemons/telegram-watcher.py` | CREATE |
| `.gitignore` | state daemon |
| `docs/features/puente-sensorial-telegram/_smoke-*.json` | Plantillas (incl. `_smoke-telegram-refugio-markdown.json`) |

## 11. Riesgos

| Riesgo | Mitigación |
|--------|------------|
| MarkdownV2 rompe envío (Argos/Mayeuta) | **Fricción de Acero:** refugio plain automático (§3.3); `escape_markdown_v2` como primera línea; AC7 obligatorio |
| Rate limit Telegram | Backoff exponencial en watcher (max 3 reintentos) |
| Proceso gateway no registrado en lab | Registrar en `execute_process_capsules` + `process/index.md` |
| CI sin token | Smokes usan `--dry-run` / mock `urllib` |
