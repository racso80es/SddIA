---
feature_name: kalma2-mvp-paciente-0
created: "2026-08-17"
process: feature
base: main
scope: kalma2-mvp-paciente-0
version_spec: "1.0.0"
document_id: PBI-KALMA2-MVP-01
uuid: "d7d00838-9ee6-472f-a164-95dcba2ceb80"
persist_ref: docs/features/kalma2-mvp-paciente-0
branch_name: feat/kalma2-mvp-paciente-0
entities_to_forge: 11
ssot_mutations: 4
laudo: "cumulo-genoma-kalma2-mvp"
---

# Especificación del Genoma — Kalma2 MVP (Paciente 0)

Toda entidad se forja vía `execute-process --process entity-manager`. Los `uuid` de esta especificación son **reservas de identidad de Cúmulo**: si la forja emite uno propio, el emitido prevalece y este documento se actualiza en la misma transacción lógica. `hash_signature` lo calcula `cryptography-manager` sobre el canon del contrato de cada familia; aquí figura como `sha256:pending-forge`.

## 1. Topología

Rutas resueltas exclusivamente por `SddIA/core/cumulo.paths.json`.

```
SddIA/                                      ← Genoma (Core, agnóstico)
├── events/domain/
│   ├── email-received.md                   ← nuevo
│   └── email-triaged.md                    ← nuevo
├── daemons/
│   ├── email-watcher.md                    ← nuevo
│   └── email-watcher/                      ← cápsula Rust (delivery)
├── templates/systemd/
│   └── sddia-email-watcher@.service.template  ← nuevo
├── library/
│   ├── norms/email-triage-matrix.md        ← nuevo (LA LEY)
│   └── codexes/
│       ├── codex-kalma2-assistant.md       ← nuevo (EL ACTIVO)
│       └── codex-kalma2-assistant/process/
│           └── email-triage-gateway.md     ← nuevo (proceso empacado)
├── process/sync-client-assets.md           ← nuevo
├── actions/download-remote-asset.md        ← nuevo
├── tools/github-raw-fetcher.md             ← nuevo (cápsula temporal)
├── skills/agenda-manager.md                ← nuevo
└── interfaces/kalma2-bridge/               ← mutación (ruta /api/sync-assets)

{instancia}/.SddIA/                         ← Periferia (cliente, fuera de Git)
├── .dev/.env                               ← credenciales IMAP
├── daemons/state/email-watcher.json        ← watermark de idempotencia
├── inbox/{message_uid}.eml                 ← cuerpos de correo (nunca en bus ni Git)
└── library/codexes/codex-kalma2-assistant.md  ← activo inyectado por H4
```

## 2. Circuito objetivo

```
[IMAP]
   │  sondeo read-only
   ▼
email-watcher (Centinela · ceguera lógica)
   │  escribe ECST
   ├──► ./.events/domain/{event_id}.json      Email_Received
   └──► ./.events/telemetry/                  Daemon_Heartbeat
   │
   ▼
event-watcher (ya existente) ──► execute-process --process route-domain-event
   │
   │  suscripción declarada en event-domain-subscriptions.json
   ▼
email-triage-gateway (proceso empacado en el códice)
   │
   ├── Fase 1 · Triaje-C determinista ─── veredicto noise ──► salida temprana (0 tokens)
   │
   ├── Fase 2 · Clasificación LLM (solo si Fase 1 no concluye)
   │      requires_capability: llm:interact → skill:mayeuta-llm
   │
   ├── Fase 3 · Extracción + asiento en agenda (solo vía actionable)
   │      requires_capability: agenda:persist → skill:agenda-manager
   │
   └── Fase 4 · Emisión ECST Email_Triaged  (veredicto + decision_path + coste)
                     │
                     ▼
              WUI Kalma2 · GET /api/status
```

Cadena de sincronización de activos (H4):

```
WUI [Sincronizar Genoma]
   │  POST /api/sync-assets
   ▼
kalma2-bridge ──► execute-process --process sync-client-assets  (202, fire-and-forget)
   │
   ├── Fase 1 · Lectura del manifiesto local del cliente
   ├── Fase 2 · action:download-remote-asset
   │              requires_capability: asset:fetch  ← ÚNICO punto de pivote DLT
   │              provider actual: tool:github-raw-fetcher
   │              provider futuro:  tool:iota-ipfs-fetcher
   ├── Fase 3 · Validación de hash canónico (discordancia = abortar sin escribir)
   └── Fase 4 · skill:filesystem-manager sobrescribe en {instancia}/.SddIA/library/codexes/
```

## 3. Clases ECST

### 3.1 `email-received` (familia domain)

```yaml
---
uuid: "9a9694fb-3eae-4379-b67c-43e0d802f4d3"
name: "email-received"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Email_Received"
context: "peripheral-sensing"
capabilities:
  - "email_received"
hash_signature: "sha256:pending-forge"
---
```

| Campo payload | Obligatorio | Descripción |
|---------------|:-----------:|-------------|
| `message_uid` | Sí | UID IMAP; clave de idempotencia |
| `mailbox` | Sí | Buzón de origen |
| `from` | Sí | Remitente |
| `subject` | Sí | Asunto |
| `received_at` | Sí | ISO-8601 UTC |
| `snippet` | Sí | Cuerpo truncado (512 caracteres por defecto) |
| `body_ref` | No | Ruta relativa de instancia al `.eml` completo |
| `list_headers` | No | Cabeceras de lista/bulk detectadas — insumo del Triaje-C |

**PROHIBIDO en payload:** cuerpo íntegro, adjuntos, credenciales, ruta absoluta del host.

**Emisor autorizado:** Centinela `email-watcher`, exclusivamente.
**Suscriptor:** proceso `email-triage-gateway`.

### 3.2 `email-triaged` (familia domain)

```yaml
---
uuid: "915726b1-0805-42e1-a1f8-9d730eaf27f9"
name: "email-triaged"
version: "1.0.0"
contract: "events-contract v1.1.0"
event_family: "domain"
event_type: "Email_Triaged"
context: "ecosystem-evolution"
capabilities:
  - "email_triaged"
hash_signature: "sha256:pending-forge"
---
```

| Campo payload | Obligatorio | Descripción |
|---------------|:-----------:|-------------|
| `message_uid` | Sí | Correlación con `Email_Received` |
| `verdict` | Sí | `noise` \| `passive` \| `actionable` |
| `decision_path` | Sí | `deterministic` \| `llm` — habilita el criterio de peaje termodinámico |
| `matched_rule` | No | Identificador de la regla de la norma que resolvió |
| `thermodynamic_cost` | Sí | `{ tokens_in, tokens_out, duration_ms }`; ceros cuando `decision_path: deterministic` |
| `agenda_entry_id` | No | Presente solo si `verdict: actionable` y el asiento tuvo éxito |

**Emisor autorizado:** proceso `email-triage-gateway`.
**Suscriptor MVP:** ninguno (consumo por proyección `GET /api/status` de la WUI).

## 4. Centinela `email-watcher`

```yaml
---
uuid: "e32bc42b-365a-4ee1-a2ae-55ea4237f440"
name: "email-watcher"
version: "1.0.0"
contract: "daemons-contract v1.0.0"
context: "peripheral-sensing"
hash_signature: "sha256:pending-forge"
capabilities:
  - "imap-mailbox-poll"
  - "email-stimulus-injection"
execution:
  entrypoint: "SddIA/daemons/email-watcher.sh"
  runtime: "native-rust"
  heartbeat_interval_seconds: 30
jurisdiction: "Aislada — Ceguera Lógica. Solo inyecta eventos físicos en el bus"
telemetry_provided: true
telemetry_schema:
  - "uptime_seconds"
  - "pid"
  - "status"
---
```

### 4.1 Invariantes de ceguera

| Permitido | Prohibido |
|-----------|-----------|
| Conectar IMAP en modo lectura | Cualquier comando IMAP de escritura (`STORE`, `EXPUNGE`, `MOVE`, `COPY`) |
| Escribir ECST en `./.events/domain/` y `./.events/telemetry/` | Invocar `execute-process`, `entity-manager` o acciones |
| Persistir `.eml` en `.SddIA/inbox/` y watermark en `.SddIA/daemons/state/` | Leer o mutar `SddIA/` (genoma) |
| Escribir su `.lock` en `.SddIA/daemons/status/` | Emitir juicio, veredicto o clasificación |
| Terminar limpio ante SIGTERM | Escalar privilegios; hardcodear rutas del host |

### 4.2 Configuración de instancia

Leída de `{instancia}/.SddIA/.dev/.env` (`env_hierarchy.instance`). Ningún valor por defecto apunta a un proveedor concreto.

| Variable | Obligatoria | Defecto |
|----------|:-----------:|---------|
| `SDDIA_EMAIL_IMAP_HOST` | Sí | — |
| `SDDIA_EMAIL_IMAP_PORT` | No | `993` |
| `SDDIA_EMAIL_IMAP_USER` | Sí | — |
| `SDDIA_EMAIL_IMAP_SECRET` | Sí | — |
| `SDDIA_EMAIL_MAILBOX` | No | `INBOX` |
| `SDDIA_EMAIL_POLL_SECONDS` | No | `60` |
| `SDDIA_EMAIL_SNIPPET_CHARS` | No | `512` |

### 4.3 Idempotencia

`{instancia}/.SddIA/daemons/state/email-watcher.json`:

```json
{
  "mailbox": "INBOX",
  "last_uid": 0,
  "updated_at": "<ISO-8601 UTC>"
}
```

Regla: solo se emite ECST para `uid > last_uid`; el watermark se persiste **después** de escribir el evento. Un fallo entre ambos pasos produce, como máximo, un duplicado detectable por `message_uid`; nunca una pérdida.

## 5. Norma táctica `email-triage-matrix` (la ley)

```yaml
---
uuid: "d4a29f1f-ba90-44d7-94c8-5fba9eaef33b"
name: "email-triage-matrix"
version: "1.0.0"
nature: "tactical-norm"
author: "norm-creator"
scope: "agnostic"
category: "workflow"
dependencies: []
---
```

Cuerpo normativo obligatorio:

1. **Matriz de tres vías** — definición semántica de `noise`, `passive`, `actionable` con criterio de desempate explícito.
2. **Reglas deterministas del Triaje-C** — señales que resuelven sin LLM (cabeceras de lista/bulk, remitentes en lista de ruido, patrones de asunto). Cada regla con identificador estable para `matched_rule`.
3. **Contrato de extracción** — campos que la vía `actionable` debe extraer (`title`, `datetime`, `source_ref`) y comportamiento ante extracción incompleta: degradar a `passive`, nunca inventar fecha.
4. **Blindaje antiverbosidad** — la longitud, el tono comercial o la urgencia declarada en el correo **no** elevan la prioridad. Solo elevan señales estructurales.
5. **Prioridad de conflicto** — el Triaje-C determinista prevalece sobre el veredicto LLM cuando ambos concluyen.

## 6. Códice `codex-kalma2-assistant` (el activo)

```yaml
---
uuid: "f50ebb1d-8765-4aff-becc-6048951d4a1e"
name: "SddIA Codex Kalma2 Personal Assistant"
version: "1.0.0"
nature: "domain-codex"
author: "codex-creator"
target_environment: ["personal-assistant", "email", "kalma2"]
certification_grade: "Pendiente"
process_membership:
  - email-triage-gateway
composition:
  - norm: "d4a29f1f-ba90-44d7-94c8-5fba9eaef33b"
    path: "../norms/email-triage-matrix.md"
  - norm: "4c448c82-de41-460f-b24f-82a84fa5ed69"
    path: "../norms/features-documentation-pattern.md"
dlt:
  asset_class: "domain-codex"
  mint_status: "pre-mint"
  ledger: "iota-rebased-testnet"
  canonical_hash: "sha256:pending-forge"
  token_id: null
  owner_vertex: "biological-vertex"
hash_signature: "sha256:pending-forge"
---
```

Cuerpo obligatorio por `codex-contract` §2: **Estrategia de Dominio** (por qué esta ley y este proceso forman un activo coherente para un asistente personal) e **Instrucciones de Prioridad** (`email-triage-matrix` prevalece sobre cualquier heurística del agente ejecutor).

### 6.1 Identidad de Activo (NFT lógico)

El bloque `dlt` exige `codex-contract` **v1.2.0** (bloque opcional; ver `clarify.md` L-06 / R-01). Invariantes:

| Invariante | Regla |
|-----------|-------|
| `canonical_hash` | Idéntico a `hash_signature`. El minteo ancla, no recalcula |
| `mint_status` | `pre-mint` hasta que `iota-immutable-publisher` devuelva `token_id` |
| `token_id` | `null` mientras `mint_status: pre-mint` |
| `uuid` | Inmutable de por vida. Un cambio de UUID es un activo distinto, no una versión |
| `version` | SemVer; cambio de contenido ⇒ nueva versión ⇒ nuevo `canonical_hash` |

## 7. Proceso empacado `email-triage-gateway`

Reside en `SddIA/library/codexes/codex-kalma2-assistant/process/`, no en `SddIA/process/`: la semántica de correo no pertenece al Core.

```yaml
---
uuid: "815215e7-fd7f-4500-baff-6801a53842ea"
name: email-triage-gateway
version: "1.0.0"
contract: process-contract v1.4.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
context:
  - external-ingest
hash_signature: sha256:pending-forge
inputs:
  - event_file_path: Ruta de la instancia ECST Email_Received
outputs:
  - verdict: noise | passive | actionable
  - decision_path: deterministic | llm
  - emitted: true si se escribió Email_Triaged
phases:
  - name: Triaje-C
    intent: Reglas deterministas de email-triage-matrix; salida temprana en noise sin gasto de inferencia.
  - name: Clasificacion
    intent: Clasificación semántica solo para correo no resuelto por Triaje-C.
    requires_capability:
      - id: llm:interact
        contract: llm.interact
        version: '>=1.0.0'
  - name: Asiento-Agenda
    intent: Extracción estructurada y asiento local; solo vía actionable.
    requires_capability:
      - id: agenda:persist
        contract: agenda.persist
        version: '>=1.0.0'
  - name: Emision
    intent: Escritura de Email_Triaged en ./.events/domain/ con veredicto y coste.
minteo_maximo: null
porcentaje_de_exito: null
---
```

**Gate termodinámico:** la fase `Clasificacion` no se ejecuta si `Triaje-C` concluyó. Verificable en `execution_report`.

## 8. Tubería de sincronización de activos

### 8.1 Proceso `sync-client-assets`

```yaml
---
uuid: "0f6bf2ff-a067-46fb-9175-ee97e6a5dcd8"
name: sync-client-assets
version: "1.0.0"
contract: process-contract v1.4.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
context:
  - ecosystem-evolution
  - knowledge-management
hash_signature: sha256:pending-forge
inputs:
  - asset_id: UUID del activo solicitado
  - asset_family: library_codexes | library_norms
outputs:
  - synced: true si el activo local fue actualizado
  - local_version: Versión resultante en la instancia
  - hash_verified: Resultado de la validación de integridad
phases:
  - name: Manifiesto-Local
    intent: Leer versión y hash del activo presente en la instancia.
    requires_capability:
      - id: fs:persist
        contract: fs.persist
        version: '>=1.0.0'
  - name: Reclamacion
    intent: Obtener el activo vigente del repositorio maestro (origen opaco).
    delegates_to:
      - action:download-remote-asset
  - name: Aduana-Integridad
    intent: Comparar hash canónico recibido contra el declarado; discordancia aborta sin escribir.
  - name: Inyeccion
    intent: Sobrescribir el activo en {instancia}/.SddIA/library/ del cliente.
    requires_capability:
      - id: fs:persist
        contract: fs.persist
        version: '>=1.0.0'
minteo_maximo: null
porcentaje_de_exito: null
---
```

### 8.2 Acción `download-remote-asset`

Abstracción de negocio: **exige `asset_id`, devuelve contenido, ignora la procedencia**.

```yaml
---
uuid: "6175f5cd-7844-4d0c-aa93-d2ce3a41d18e"
name: download-remote-asset
version: "1.0.0"
contract: actions-contract
context: knowledge-management
hash_signature: sha256:pending-forge
capabilities:
  - "remote-asset-reclamation"
  - "asset-integrity-declaration"
requires_capability:
  - id: asset:fetch
    contract: asset.fetch
    version: '>=1.0.0'
---
```

| Input | Obligatorio |
|-------|:-----------:|
| `asset_id` | Sí |
| `asset_family` | Sí |

| Output | Descripción |
|--------|-------------|
| `content` | Texto plano del activo |
| `declared_hash` | Hash canónico declarado por el origen |
| `origin_kind` | Etiqueta opaca del proveedor resuelto (`git-raw`, `dlt-ipfs`, …) para telemetría |

La acción **no** nombra `github-raw-fetcher` en ningún punto.

### 8.3 Tool `github-raw-fetcher` (pieza temporal)

```yaml
---
uuid: "66daf19f-217a-4874-b417-99e5be2571f3"
name: github-raw-fetcher
version: "1.0.0"
contract: tools-contract
context: system-operations
hash_signature: sha256:pending-forge
capabilities:
  - "github-raw-fetcher"
  - "asset-fetch"
  - "capsule-json-io"
provides_capability:
  - id: asset:fetch
    contract: asset.fetch
    version: "1.0.0"
deprecation_pivot: "tool:iota-ipfs-fetcher (fase DLT)"
---
```

E/S estricta `capsule-json-io` schema `2.0`.

Petición: `{ "meta": { "schemaVersion": "2.0", "entityKind": "tool", "entityId": "github-raw-fetcher" }, "request": { "asset_path": "<ruta relativa en el repo maestro>", "ref": "main" } }`

Respuesta: `{ "meta": {…}, "success": true, "exitCode": 0, "result": { "content": "<texto>", "declared_hash": "sha256:…", "origin_kind": "git-raw" } }`

Base remota configurable por entorno (defecto `https://raw.githubusercontent.com/racso80es/SddIA/`). Sin credenciales: solo lectura pública.

### 8.4 Skill `agenda-manager`

```yaml
---
uuid: "0219a3a5-a5a9-4bb5-a225-77a501e8fba0"
name: agenda-manager
version: "1.0.0"
contract: skills-contract
context: filesystem-ops
hash_signature: sha256:pending-forge
capabilities:
  - "agenda-entry-create"
  - "agenda-entry-list"
provides_capability:
  - id: agenda:persist
    contract: agenda.persist
    version: "1.0.0"
---
```

Persistencia local en `{instancia}/.SddIA/agenda/`. Agenda local pura: cero integración con calendarios externos en MVP.

## 9. Mutaciones de SSOT

### 9.1 `SddIA/core/capability-bindings.md` (→ v1.5.0)

```yaml
- capability_id: "asset:fetch"
  contract: "asset.fetch"
  provider: "tool:github-raw-fetcher"
  provider_version: ">=1.0.0"
- capability_id: "agenda:persist"
  contract: "agenda.persist"
  provider: "skill:agenda-manager"
  provider_version: ">=1.0.0"
```

`asset:fetch` es el **único punto de pivote DLT**: sustituir su `provider` migra el sistema a IOTA/IPFS sin tocar proceso ni acción.

### 9.2 `SddIA/core/event-domain-subscriptions.json`

```json
"Email_Received": [
  {
    "agent": "cumulo",
    "process": "email-triage-gateway",
    "intent": "Aduana cognitiva del canal aferente de correo; triaje de tres vías bajo codex-kalma2-assistant."
  }
]
```

`Email_Triaged` se registra sin suscriptores en MVP (consumo por proyección de estado).

### 9.3 `SddIA/core/cumulo.paths.json`

`directories.process_domain_roots` += `"SddIA/library/codexes/codex-kalma2-assistant/process"` (R-02).

### 9.4 `SddIA/library/codexes/codex-contract.md` (→ v1.2.0)

Bloque `dlt` opcional documentado en §1, con la tabla de invariantes de §6.1 de esta especificación. Retrocompatible: los 4 códices existentes siguen siendo válidos sin el bloque.

## 10. Mutación de `kalma2-bridge`

Nueva ruta, homóloga a `/api/execute` (fire-and-forget, 202):

| Método | Ruta | Delegación |
|--------|------|-----------|
| `POST` | `/api/sync-assets` | `execute-process --process sync-client-assets --inputs '{"asset_id":"…","asset_family":"library_codexes"}'` |

Cuerpo de petición: `{ "asset_id": "<uuid>", "asset_family": "library_codexes" }`
Respuesta `202`: `{ "accepted": true, "correlation_id": "<uuid>" }`

WUI (`interfaces/kalma2/`): botón **Sincronizar Genoma** en `index.html` + emisor en `app.js`. Progreso observable por `GET /api/progress/stream?correlation_id=`, canal ya existente.

## 11. Índices a reconciliar

| Índice | Filas nuevas |
|--------|-------------|
| `SddIA/events/domain/index.md` | `email-received`, `email-triaged` |
| `SddIA/daemons/index.md` | `email-watcher` |
| `SddIA/library/norms/index.md` | `email-triage-matrix` |
| `SddIA/library/codexes/index.md` | `codex-kalma2-assistant` |
| `SddIA/library/codexes/codex-kalma2-assistant/process/index.md` | `email-triage-gateway` (índice nuevo) |
| `SddIA/process/index.md` | `sync-client-assets` |
| `SddIA/actions/index.md` | `download-remote-asset` |
| `SddIA/tools/index.md` | `github-raw-fetcher` |
| `SddIA/skills/index.md` | `agenda-manager` |
| `SddIA/templates/index.md` | plantilla systemd (si el índice de la familia lo exige) |

Entidad física sin fila en su índice = Ruido de Sistema. La reconciliación es automática vía `Domain_Entity_Created` → `sync-entity-index`; se verifica, no se asume.

## 12. Matriz de verificación de las Reglas de Acero

| Regla | Mecanismo en el genoma | Verificable en |
|-------|------------------------|----------------|
| Ceguera Espacial | `WorkingDirectory=%f` como único acoplamiento; cero rutas de cliente en `SddIA/` | Inspección del template + grep de rutas absolutas |
| Ceguera Lógica del Centinela | Sin invocación de orquestador; sin lectura de genoma | §4.1 + inspección de la cápsula |
| Identidad de Activo | Bloque `dlt` + `canonical_hash` == `hash_signature` + `uuid` inmutable | §6.1 |
| Pivote DLT sin fractura | `asset:fetch` en `capability-bindings.md` | §9.1 |
| Tubería hermética | `capsule-json-io` schema 2.0 en toda cápsula | §8.3 |
| No destructividad | Cero comandos IMAP de escritura | §4.1 |
| Eficiencia termodinámica | Salida temprana determinista + `decision_path` + `thermodynamic_cost` | §3.2, §7 |
| Privacidad del bus | `body_ref` en lugar de cuerpo | §3.1 |
