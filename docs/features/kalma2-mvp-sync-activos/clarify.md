---
feature_name: kalma2-mvp-sync-activos
created: "2026-08-19"
process: feature
purpose: "Estabilización Mayeuta — Filtro Antientrópico sobre PBI-KALMA2-MVP-01B (Sincronización de activos · Ola B)"
phase: Estabilización de Requisitos
agents: mayeuta
document_id: PBI-KALMA2-MVP-01B
uuid: "ed2f20b8-6e3d-4dbf-931c-d62e53ddf7c4"
version: "1.0.0"
persist_ref: docs/features/kalma2-mvp-sync-activos
branch_name: feat/kalma2-mvp-sync-activos
dossier_ref: docs/features/kalma2-mvp-paciente-0
spec_ref: docs/features/kalma2-mvp-paciente-0/spec.md
status: stabilized
mayeuta_verdict: ok
open_decisions: 0
ratification_required: 0
ratification_granted: 0
---

# Clarificación Mayeuta — Filtro Antientrópico · Ola B (Sync Activos)

## D0. Apertura

Esta clarificación opera sobre la **segunda ola** del PBI paraguas `PBI-KALMA2-MVP-01`. El dossier compartido (`clarify.md` + `spec.md` + `plan.md`) bajo `docs/features/kalma2-mvp-paciente-0/` ya estabilizó el arco completo (13 fisuras, 13 decisiones, 2 ratificaciones). Esta clarificación **no duplica** ese trabajo; se limita a:

1. Verificar que el estado del repositorio después del merge de la Ola A (PR #182) es coherente con los supuestos de la Ola B.
2. Identificar cualquier fisura nueva o dependencia no resuelta que bloquee T6–T8+T9b.
3. Confirmar que el vector de entrega es ejecutable sin ratificaciones adicionales.

## D1. Hechos verificados post-merge Ola A (PR #182)

| # | Hecho | Relevancia para Ola B |
|---|-------|-----------------------|
| F-B1 | PR #182 mergeado en `main`; `PBI-KALMA2-MVP-01A` archivado | Desbloquea la dependencia declarada en el PBI-01B |
| F-B2 | `codex-kalma2-assistant` forjado y presente en `SddIA/library/codexes/` | El activo a sincronizar existe; T7 tiene carga válida |
| F-B3 | `codex-contract` elevado a v1.2.0 con bloque `dlt` opcional (R-01) | Invariantes de Cicatriz Digital vigentes para el activo |
| F-B4 | `cumulo.paths.json` `process_domain_roots` incluye el proceso empacado (R-02) | Jurisdicción de Cúmulo intacta |
| F-B5 | `capability-bindings.md` **no** registra aún `asset:fetch` | T6 debe añadirlo como primera acción bloqueante de T7 |
| F-B6 | `kalma2-bridge` expone `/api/execute` y `/api/progress/stream` | T8 añade `/api/sync-assets` sobre la misma base; canal SSE reutilizable |
| F-B7 | No existe `github-raw-fetcher` en `SddIA/tools/` | T6 lo forja desde cero |
| F-B8 | No existe `download-remote-asset` en `SddIA/actions/` | T7 lo forja desde cero |
| F-B9 | No existe `sync-client-assets` en `SddIA/process/` | T7 lo forja desde cero |
| F-B10 | `filesystem-manager` v1.1.0 operativo (`file-write`, `create-directory`) | T7 fase Inyeccion lo reutiliza sin cambios (D-12 de Ola A) |

## D2. Análisis de fisuras Ola B

### L-B01 — Orden de dependencia dentro de T6 · **operativa**

`capability-bindings.md` debe registrar `asset:fetch` **antes** de forjar `download-remote-asset`, ya que el contrato de acción declara `requires_capability: asset:fetch`. Forjar la acción sin el binding provoca una acción con dependencia no satisfecha desde el nacimiento.

**Resolución:** T6 se divide en dos micro-pasos atómicos:
1. T6a — Forjar `github-raw-fetcher` y obtener su UUID real.
2. T6b — Mutar `capability-bindings.md` registrando `asset:fetch → provider: tool:github-raw-fetcher` con el UUID real. Solo entonces T7 puede proceder.

### L-B02 — `origin_kind` como etiqueta de telemetría, no parámetro de control · **grave si se incumple**

La semilla del PBI describe `origin_kind` como "abstracción de origen". Riesgo: que `download-remote-asset` ramifique lógica según `origin_kind`, acoplando la acción al proveedor y rompiendo el pivote DLT (G7).

**Resolución confirmada (ya en spec.md §8.2):** `origin_kind` fluye exclusivamente como etiqueta opaca de salida para telemetría. La acción no inspecciona ese campo en ningún punto de su lógica interna. Verificable por construcción: `download-remote-asset.md` no puede contener lógica condicional sobre `origin_kind`.

### L-B03 — Aduana de hash en T9b: ¿quién calcula el hash local? · **operativa**

La fase `Aduana-Integridad` de `sync-client-assets` compara `declared_hash` (devuelto por `download-remote-asset`) contra el "hash canónico" del activo. El proceso necesita saber cómo computar el hash del contenido recibido antes de escribirlo. Sin especificar el algoritmo y el corpus, la aduana es inútil.

**Resolución:** el hash se calcula sobre el texto plano del campo `content` con `sha256`. El `declared_hash` en la respuesta de `github-raw-fetcher` se calcula sobre el mismo corpus (texto plano del fichero en raw). La aduana compara ambos SHA-256; discordancia → abortar sin escribir. El algoritmo queda declarado en el contrato de `github-raw-fetcher` (`capsule-json-io` §result.declared_hash: "sha256:<hex>").

### L-B04 — Ruta de destino de la inyección · **operativa**

`sync-client-assets` fase `Inyeccion` escribe en `{instancia}/.SddIA/library/codexes/`. La ruta de instancia no puede estar cableada en el genoma (Ceguera Espacial). ¿Cómo la conoce el proceso?

**Resolución:** el proceso recibe `asset_family` como input (`library_codexes`). La ruta destino se resuelve mediante `cumulo.paths.json` en tiempo de ejecución por el motor (`execute-process`), que inyecta la ruta de instancia como variable de entorno del workspace. El proceso declara la familia; el motor resuelve la ruta. Cero rutas absolutas en el genoma.

### L-B05 — Fire-and-forget en T8: ¿`correlation_id` de quién? · **menor**

`POST /api/sync-assets` devuelve `202 { accepted: true, correlation_id: "<uuid>" }`. El PBI no especifica quién genera ese `correlation_id`.

**Resolución:** `kalma2-bridge` genera un UUID v4 en el momento de aceptar la petición y lo pasa como input a `execute-process --process sync-client-assets`. Este mismo UUID es el `execution_id` del workspace del proceso, garantizando correlación trazable. El cliente lo usa para observar el progreso vía `GET /api/progress/stream?correlation_id=`.

## D3. Decisiones estabilizadas Ola B

| ID | Decisión | Origen |
|----|----------|--------|
| D-B01 | T6 bifurcado: T6a forja cápsula → T6b registra binding `asset:fetch` | L-B01 |
| D-B02 | `origin_kind` es etiqueta de salida opaca; prohibida su inspección interna en la acción | L-B02 |
| D-B03 | Hash = SHA-256 sobre texto plano del activo; corpus idéntico en origen y aduana | L-B03 |
| D-B04 | Ruta de inyección resuelta por motor vía `cumulo.paths.json` + `asset_family`; cero rutas cableadas | L-B04 |
| D-B05 | `correlation_id` = UUID v4 generado por `kalma2-bridge`; coincide con `execution_id` del proceso | L-B05 |

## D4. Verificación de Reglas de Acero (Ola B)

| Regla | Mecanismo verificable |
|-------|-----------------------|
| Pivote DLT sin fractura (G7) | `grep github-raw-fetcher` en `sync-client-assets.md` y `download-remote-asset.md` devuelve 0 coincidencias; solo en `capability-bindings.md` |
| Tubería hermética (G6) | `github-raw-fetcher` cumple `capsule-json-io` 2.0: `exitCode: 0 ⟺ success: true` |
| Fire-and-forget (G8) | `/api/sync-assets` devuelve 202 sin bloquear; progreso por SSE existente |
| Sin credenciales | `github-raw-fetcher` opera en lectura pública (`https://raw.githubusercontent.com/`); cero secretos en genoma |
| Cicatriz Digital | Las 3 entidades nuevas con `uuid` v4, SemVer, `contract`, `hash_signature`, fila en índice |
| Aduana de integridad | Discordancia SHA-256 aborta antes de `fs:persist`; fichero local intacto |
| Abstracción de origen | `download-remote-asset` exige `asset_id`, devuelve `content`; ignora procedencia |
| Ceguera Espacial | Ruta de instancia nunca en genoma; resuelta por motor en runtime |

## D5. Gates de Ola B

| Gate | Condición de paso |
|------|------------------|
| G5 (T6) | `github-raw-fetcher` forjado, `asset:fetch` registrado en `capability-bindings.md` |
| G6 (T7) | Tubería `sync-client-assets` → `download-remote-asset` → `github-raw-fetcher` trazable sin nombrar cápsula en acción/proceso |
| G7 (T7) | `grep github-raw-fetcher` en acción y proceso = 0 |
| G8 (T8) | `/api/sync-assets` devuelve 202; WUI muestra botón y observa progreso SSE |
| G9 (T9b) | Aduana hash aborta en discordancia; cicatriz digital registrada en `SddIA/evolution/` |

## D6. Handoff a Dédalo (Ola B)

Sin ratificaciones pendientes: todas las decisiones de arquitectura compartida (R-01, R-02) fueron concedidas en la Ola A y están materializadas en `main`.

Restricciones que Dédalo no puede relajar:
- `download-remote-asset` nunca nombra `github-raw-fetcher` directamente.
- `sync-client-assets` nunca nombra `github-raw-fetcher` directamente.
- La aduana debe abortar **antes** de escribir; no hay rollback post-escritura.
- `POST /api/sync-assets` devuelve 202 en < 100 ms; el proceso corre desacoplado.
- Cero credenciales en genoma ni en el payload de la cápsula.

Continúa en `spec.md` del dossier compartido (§8) y `plan.md` §Ola B (T6–T9).
