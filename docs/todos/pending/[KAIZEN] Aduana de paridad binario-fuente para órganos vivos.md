---
document_id: PBI-KAIZEN-RUNTIME-BINARY-PARITY-GATE
uuid: "a432ee20-44ee-4a05-aae2-0495e1d19229"
title: "[KAIZEN] Aduana de paridad binario/fuente para órganos vivos (execute-process y kalma2-bridge)"
format: markdown
version: "1.0.0"
created: "2026-10-14"
status: pending
priority: alta
process: feature
dispatch: true
suggested_branch: feat/kaizen-runtime-binary-parity-gate
persist_ref: docs/features/kaizen-runtime-binary-parity-gate
source_audit: docs/audits/kalma2-wui-hu-b07-core-self-20261014.md
related_audits:
  - docs/audits/kalma2-wui-tormentosa-chat-20260909.md
  - docs/audits/kalma2-pbi-telegram-pec-silence-20261013.md
  - docs/audits/kalma2-wui-hu-b07-core-self-20261014.md
related:
  - SddIA/engine/execute-process/src/engine/capsule_seal.rs
  - SddIA/engine/execute-process/src/engine/accept_pr.rs
  - SddIA/engine/execute-process/src/engine/handlers/instance_health_verify.rs
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - start-sddia.sh
---

# [KAIZEN] Aduana de paridad binario/fuente para órganos vivos

## 1. Problema sistémico (patrón de reincidencia)

Tres auditorías distintas han documentado el mismo fallo estructural bajo síntomas diferentes:

1. **2026-09-09** (`kalma2-wui-tormentosa-chat-20260909.md`): `POST /api/aiua/interact` devolvía 404 porque `kalma2-bridge` ejecutaba un ELF compilado tres días antes que el frontend.
2. **2026-08-28** (`6a49e0ad310e-R1`): `resolve_capsule_native` servía un ELF release fósil de `iota-immutable-publisher`. Se creó `SDDIA_CAPSULE_ANCHOR` + `--seal-capsules` para cápsulas/daemons, pero **se dejó fuera** al propio orquestador (`execute-process`) y al puente HTTP (`kalma2-bridge`): sus testigos `.sha256` en `SddIA/target/release/` siguen con fecha `2026-09-28`.
3. **2026-10-14** (`kalma2-wui-hu-b07-core-self-20261014.md`): el PR #347 cerró `PBI-FIX-KALMA2-BRANCH-AND-STATUS` con tests en verde, pero `kalma2-bridge` (release, 18:00) y `execute-process` (debug, 18:02) siguieron vivos sin recompilar tras el merge de las 18:36. La siguiente prueba en WUI reprodujo exactamente el bug que acababa de darse por corregido.

**Causa raíz:** la definición de Done valida el código fuente en CI (`cargo test`), pero el ecosistema local opera con demonios persistentes (`kalma2-bridge`, `event-watcher`) y un binario `execute-process` en disco que nadie recompila ni recicla al cerrar un PR, y cuya paridad `source_sha256 == elf_sha256` no se verifica ni al despachar ni en `/api/system-health`.

## 2. Acción Kaizen

Extender el patrón de anclaje (`capsule_seal`) a los dos órganos de entrada del ciclo (`execute-process` y `kalma2-bridge`) en tres puntos de control:

| Punto | Mecanismo | Efecto |
|-------|-----------|--------|
| **K-1 · Sello en ignición y post-merge** | En `start-sddia.sh` y al cierre de `accept-pr` (cuando el diff del PR toca `SddIA/engine/execute-process/**`, `SddIA/interfaces/kalma2-bridge/**` o `SddIA/sddia-*`), recompilar `execute-process` y `kalma2-bridge`, actualizar sus `.sha256` (`write_witness: true`) y reiniciar `sddia-kalma2-bridge@` / `sddia-event-watcher@`. | Ningún PR mergeado en `main` deja al demonio vivo corriendo el ELF anterior al merge. |
| **K-2 · Gate pre-dispatch en `/api/execute`** | Antes de `accept_execute` en `kalma2-bridge`, verificar el testigo `execute-process.sha256` y el propio `kalma2-bridge.sha256` contra el `source_sha256` actual del crate (`capsule_seal`). Si divergen, responder `409 Conflict` (`STALE_RUNTIME_BINARY`) con el nombre del binario desalineado en vez de aceptar `202` y ejecutar lógica vieja en silencio. | Imposible despachar un PBI desde WUI con binarios anteriores al árbol actual. |
| **K-3 · Visibilidad en Espejo de consciencia (`/api/system-health`)** | Añadir fila `runtime_binary_parity` en `system-health` (comparando `/proc/self/exe` de `kalma2-bridge` y el `execute-process` resuelto contra sus testigos `.sha256`). Si hay drift, `map_status: DEGRADED` visible en la portada de Kalma2 antes de pulsar ningún botón. | El operador ve en la propia WUI que el puente o el motor están desfasados respecto a `HEAD`. |
| **K-4 · Smoke post-fix sobre el binario vivo** | Para PBIs con `surface: kalma2-wui` o que toquen `kalma2-bridge` / `workspace_init`, `validacion.md` exige un check de paridad del binario en ejecución (`/api/system-health` → `runtime_binary_parity: ok`), no solo `cargo test` sobre un binario efímero de test. | Cierra la brecha entre «pasa `cargo test`» y «funciona al pulsar Forjar». |

## 3. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| KZ-CA1 | `--seal-capsules` en `start-sddia.sh` incluye `execute-process` y `kalma2-bridge` (`write_witness: true`); sus `.sha256` quedan actualizados en cada ignición. | `cargo test -p execute-process capsule_seal` |
| KZ-CA2 | `POST /api/execute` con `execute-process` o `kalma2-bridge` divergente respecto a su `.sha256` devuelve `409` con código `STALE_RUNTIME_BINARY` (configurable bypass solo con `SDDIA_ALLOW_STALE_RUNTIME=1` en lab). | `cargo test -p kalma2-bridge stale_runtime` |
| KZ-CA3 | `GET /api/system-health` expone `runtime_binary_parity` (`ok: true/false`, `stale: [...]`) y degrada el espejo en WUI cuando el proceso vivo (`/proc/self/exe`) o `execute-process` no coinciden con el cierre de compilación actual. | `cargo test -p kalma2-bridge system_health` |
| KZ-CA4 | `accept-pr` (o hook post-merge local) dispara rebuild + seal + restart de `kalma2-bridge` y `event-watcher` cuando el merge modifica crates nativos del motor o del puente. | Test de integración `accept_pr` |
