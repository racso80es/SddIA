---
feature_name: kalma2-mvp-sync-activos
created: "2026-08-19"
process: feature
phase: Diseño de Blueprint
agents: dedalo
document_id: PBI-KALMA2-MVP-01B
uuid: "ed2f20b8-6e3d-4dbf-931c-d62e53ddf7c4"
version: "1.0.0"
persist_ref: docs/features/kalma2-mvp-sync-activos
branch_name: feat/kalma2-mvp-sync-activos
dossier_ref: docs/features/kalma2-mvp-paciente-0
ssot_spec: docs/features/kalma2-mvp-paciente-0/spec.md
ssot_plan: docs/features/kalma2-mvp-paciente-0/plan.md
scope: "T6-T8 + T9b"
entities_to_forge: 3
ssot_mutations: 1
status: blueprint
dedalo_verdict: ok
---

# Especificación de Blueprint — Kalma2 MVP 01B · Sincronización de Activos

> Esta especificación cubre exclusivamente la **Ola B** (T6–T8 + T9b). El arco arquitectónico completo, incluyendo topología, circuito email, SSOT mutations R-01/R-02 y plan general, reside en el dossier compartido (`ssot_spec`). No se duplica; se referencia.

---

## 1. Precondiciones verificadas (post-merge PR #182)

| # | Hecho | Impacto en Ola B |
|---|-------|-----------------|
| F-B2 | `codex-kalma2-assistant` presente en `SddIA/library/codexes/` | Activo a sincronizar existe; T7 tiene carga válida |
| F-B5 | `capability-bindings.md` **no** registra `asset:fetch` | T6b lo añade como primer paso bloqueante de T7 |
| F-B6 | `kalma2-bridge` expone `/api/execute` + SSE | T8 añade `/api/sync-assets` sobre la misma base |
| F-B7 | `github-raw-fetcher` ausente en `SddIA/tools/` | T6a lo forja desde cero |
| F-B8 | `download-remote-asset` ausente en `SddIA/actions/` | T7 lo forja desde cero |
| F-B9 | `sync-client-assets` ausente en `SddIA/process/` | T7 lo forja desde cero |
| F-B10 | `filesystem-manager` v1.1.0 operativo | T7 fase Inyeccion lo reutiliza sin cambios |

---

## 2. Topología de Ola B

```
SddIA/
├── tools/github-raw-fetcher.md          ← T6a (cápsula temporal)
├── actions/download-remote-asset.md     ← T7
├── process/sync-client-assets.md        ← T7
├── core/capability-bindings.md          ← T6b (mutation: +asset:fetch)
├── interfaces/kalma2-bridge/            ← T8 (nueva ruta /api/sync-assets)
└── evolution/                           ← T9b (cicatriz digital)

interfaces/kalma2/
├── index.html                           ← T8 (botón "Sincronizar Genoma")
└── app.js                               ← T8 (emisor POST + observador SSE)
```

---

## 3. Cadena de sincronización (circuito H4)

```
WUI [Sincronizar Genoma]
   │  POST /api/sync-assets  { asset_id, asset_family }
   ▼
kalma2-bridge  →  generate UUID v4 (= correlation_id = execution_id)
   │  202 { accepted: true, correlation_id }   (< 100 ms, fire-and-forget)
   ▼
execute-process --process sync-client-assets
   │
   ├── Fase 1 · Manifiesto-Local
   │     requires_capability: fs:persist  ← filesystem-manager
   │     Lee versión y hash del activo presente en {instancia}/.SddIA/library/codexes/
   │
   ├── Fase 2 · Reclamacion
   │     delegates_to: action:download-remote-asset
   │     input: { asset_id, asset_family }
   │     output: { content, declared_hash, origin_kind }
   │     (download-remote-asset no nombra github-raw-fetcher; usa capability asset:fetch)
   │
   ├── Fase 3 · Aduana-Integridad
   │     sha256(content) == declared_hash  ?  continuar  :  abortar sin escribir
   │
   └── Fase 4 · Inyeccion
         requires_capability: fs:persist  ← filesystem-manager
         Ruta destino resuelta por motor vía cumulo.paths.json + asset_family
         Cero rutas absolutas en genoma (Ceguera Espacial)
```

---

## 4. Entidad T6a — Tool `github-raw-fetcher`

SSOT: `spec.md` §8.3 del dossier compartido.

```yaml
---
uuid: "66daf19f-217a-4874-b417-99e5be2571f3"  # reserva; prevalece UUID real de forja
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

### 4.1 Esquema E/S estricto (`capsule-json-io` schema 2.0)

**Petición (stdin):**
```json
{
  "meta": {
    "schemaVersion": "2.0",
    "entityKind": "tool",
    "entityId": "github-raw-fetcher"
  },
  "request": {
    "asset_path": "<ruta relativa en el repo maestro>",
    "ref": "main"
  }
}
```

**Respuesta exitosa (stdout, exitCode 0):**
```json
{
  "meta": { "schemaVersion": "2.0", "entityKind": "tool", "entityId": "github-raw-fetcher" },
  "success": true,
  "exitCode": 0,
  "result": {
    "content": "<texto plano del activo>",
    "declared_hash": "sha256:<hex64>",
    "origin_kind": "git-raw"
  }
}
```

**Respuesta fallida (stdout, exitCode != 0):**
```json
{
  "meta": { "schemaVersion": "2.0", "entityKind": "tool", "entityId": "github-raw-fetcher" },
  "success": false,
  "exitCode": 1,
  "error": { "code": "<ERROR_CODE>", "message": "<descripción>" }
}
```

Invariantes de cápsula:
- `exitCode: 0 ⟺ success: true` (tubería hermética G6).
- `declared_hash` calculado como `sha256` sobre el texto plano de `content` (mismo corpus que la aduana).
- Base remota configurable por variable de entorno (defecto: `https://raw.githubusercontent.com/racso80es/SddIA/`). Cero secretos; solo lectura pública.
- La cápsula **no** escribe en ningún fichero local; solo stdin → stdout.

---

## 5. Mutación T6b — `capability-bindings.md` (→ v1.5.0)

SSOT: `spec.md` §9.1 del dossier compartido.

Filas a añadir:
```yaml
- capability_id: "asset:fetch"
  contract: "asset.fetch"
  provider: "tool:github-raw-fetcher"
  provider_version: ">=1.0.0"
```

`asset:fetch` es el **único punto de pivote DLT**: sustituir `provider` migra a IOTA/IPFS sin tocar acción ni proceso.

---

## 6. Entidad T7a — Acción `download-remote-asset`

SSOT: `spec.md` §8.2 del dossier compartido.

```yaml
---
uuid: "6175f5cd-7844-4d0c-aa93-d2ce3a41d18e"  # reserva; prevalece UUID real de forja
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

### 6.1 Contrato de interfaz

| Input | Obligatorio | Descripción |
|-------|:-----------:|-------------|
| `asset_id` | Sí | UUID del activo solicitado |
| `asset_family` | Sí | `library_codexes` \| `library_norms` |

| Output | Descripción |
|--------|-------------|
| `content` | Texto plano del activo |
| `declared_hash` | `sha256:<hex>` declarado por el origen |
| `origin_kind` | Etiqueta opaca del proveedor (`git-raw`, `dlt-ipfs`, …) — solo telemetría |

**Restricción de acero:** la acción **no** nombra `github-raw-fetcher` en ningún punto. No ramifica lógica según `origin_kind`. Verificable por construcción.

---

## 7. Entidad T7b — Proceso `sync-client-assets`

SSOT: `spec.md` §8.1 del dossier compartido.

```yaml
---
uuid: "0f6bf2ff-a067-46fb-9175-ee97e6a5dcd8"  # reserva; prevalece UUID real de forja
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
    intent: Comparar sha256(content) contra declared_hash; discordancia aborta sin escribir.
  - name: Inyeccion
    intent: Sobrescribir activo en {instancia}/.SddIA/library/ resolviendo ruta por motor.
    requires_capability:
      - id: fs:persist
        contract: fs.persist
        version: '>=1.0.0'
minteo_maximo: null
porcentaje_de_exito: null
---
```

**Restricción de acero:** el proceso **no** nombra `github-raw-fetcher` en ningún punto. Ruta de instancia resuelta por motor vía `cumulo.paths.json`; cero rutas cableadas en genoma.

---

## 8. Mutación T8 — `kalma2-bridge` + WUI

### 8.1 Nueva ruta en `kalma2-bridge`

| Método | Ruta | Delegación | Respuesta |
|--------|------|-----------|-----------|
| `POST` | `/api/sync-assets` | `execute-process --process sync-client-assets --inputs '{"asset_id":"…","asset_family":"library_codexes"}'` | `202 { accepted: true, correlation_id }` |

Cuerpo de petición: `{ "asset_id": "<uuid>", "asset_family": "library_codexes" }`

El `correlation_id` es un UUID v4 generado por `kalma2-bridge` en el momento de aceptar la petición. Coincide con el `execution_id` del workspace del proceso (trazabilidad completa). Respuesta en < 100 ms; el proceso corre desacoplado (fire-and-forget, DA-5).

### 8.2 WUI (`interfaces/kalma2/`)

- `index.html`: añadir botón **"Sincronizar Genoma"** en el panel de control existente.
- `app.js`: función `syncGenome()` que:
  1. Emite `POST /api/sync-assets` con `asset_id` y `asset_family`.
  2. Extrae `correlation_id` de la respuesta 202.
  3. Abre `GET /api/progress/stream?correlation_id=<id>` en el canal SSE existente para observar progreso.

---

## 9. T9b — Aduana de sincronización y cicatriz digital

### 9.1 Aduana de integridad (materializada en Fase 3 de `sync-client-assets`)

| Condición | Comportamiento |
|-----------|---------------|
| `sha256(content) == declared_hash` | Continuar a Inyeccion |
| `sha256(content) != declared_hash` | **Abortar**; no escribir fichero; emitir error con `hash_mismatch` |

La aduana se ejecuta **antes** de cualquier escritura. No hay rollback post-escritura; la escritura directamente no ocurre.

### 9.2 Cicatriz digital en `SddIA/evolution/`

Al completar T9b, registrar entrada en `SddIA/evolution/` con:
- UUID de cada entidad nueva (`github-raw-fetcher`, `download-remote-asset`, `sync-client-assets`).
- Referencia al PR de cierre.
- Fecha de forja.
- Hito: `kalma2-mvp-sync-activos · Ola B · T6-T9b`.

---

## 10. Índices a reconciliar

| Índice | Fila nueva |
|--------|-----------|
| `SddIA/tools/index.md` | `github-raw-fetcher` |
| `SddIA/actions/index.md` | `download-remote-asset` |
| `SddIA/process/index.md` | `sync-client-assets` |

Reconciliación automática vía `Domain_Entity_Created` → `sync-entity-index`; se verifica, no se asume.

---

## 11. Gates de Ola B

| Gate | Fases | Condición de paso |
|------|-------|------------------|
| G5 | T6 | `github-raw-fetcher` forjado + `asset:fetch` registrado en `capability-bindings.md` |
| G6 | T7 | Tubería `sync-client-assets → download-remote-asset → github-raw-fetcher` trazable; la cápsula cumple `capsule-json-io` 2.0 |
| G7 | T7 | `grep github-raw-fetcher` en `sync-client-assets.md` y `download-remote-asset.md` = 0 resultados |
| G8 | T8 | `POST /api/sync-assets` devuelve 202 + `correlation_id`; WUI muestra botón; progreso observable vía SSE |
| G9 | T9b | Aduana aborta en discordancia SHA-256 sin escribir; cicatriz digital en `SddIA/evolution/` |

---

## 12. Matriz de verificación de Reglas de Acero (Ola B)

| Regla | Mecanismo verificable |
|-------|-----------------------|
| Pivote DLT sin fractura (G7) | `grep github-raw-fetcher` en acción y proceso = 0; solo en `capability-bindings.md` |
| Tubería hermética (G6) | `exitCode: 0 ⟺ success: true` en `github-raw-fetcher` |
| Fire-and-forget (G8) | `/api/sync-assets` devuelve 202 en < 100 ms; proceso desacoplado |
| Sin credenciales | `github-raw-fetcher` opera en lectura pública; cero secretos en genoma |
| Cicatriz Digital | 3 entidades con `uuid` v4, SemVer, `contract`, `hash_signature`, fila en índice |
| Aduana de integridad | SHA-256 verificado antes de `fs:persist`; fichero local intacto en discordancia |
| Abstracción de origen | `download-remote-asset` exige `asset_id`, devuelve `content`; no inspecciona `origin_kind` |
| Ceguera Espacial | Ruta de instancia resuelta por motor; nunca cableada en genoma |

---

## 13. Decisiones de diseño (resumen ejecutivo)

| ID | Decisión | Origen |
|----|----------|--------|
| D-B01 | T6 bifurcado: T6a forja cápsula → T6b registra binding `asset:fetch` | L-B01 |
| D-B02 | `origin_kind` es etiqueta de salida opaca; prohibida su inspección en la acción | L-B02 |
| D-B03 | Hash = SHA-256 sobre texto plano del activo; corpus idéntico en origen y aduana | L-B03 |
| D-B04 | Ruta de inyección resuelta por motor vía `cumulo.paths.json` + `asset_family` | L-B04 |
| D-B05 | `correlation_id` = UUID v4 de `kalma2-bridge`; coincide con `execution_id` del proceso | L-B05 |
