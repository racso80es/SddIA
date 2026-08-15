---
feature_name: kaizen-regex-lookahead-panic
created: "2026-08-15"
process: bug-fix
branch_name: fix/kaizen-regex-lookahead-panic
persist_ref: docs/fixes/kaizen-regex-lookahead-panic
pbi_ref: docs/todos/done/[FIX] enrich-fracture-pbi-kaizen — panic regex look-ahead (5b135a1d).md
document_id: 5b135a1d-480d-4e8c-abca-3cca8fda97e9
uuid: 91afa02f-d339-4347-827d-7070b74f2d0f
scope: kaizen-upsert-no-lookahead
base: main
correlation_id: 91884ac3-d226-4046-b887-bc373bc7c869
---

# Spec — panic look-ahead Kaizen en route-domain-event

## Problema

`upsert_fracture_kaizen_section` compilaba en runtime un patrón con look-ahead `(?=\n## |\Z)`. El crate `regex` no soporta look-around; `.expect("regex kaizen section")` panica. Se dispara al re-enriquecer PBIs que ya tienen síntesis Mayeuta (no placeholder).

Cascada: panic del hilo suscriptor → `PoisonError` en `event.lock().unwrap()` y `event_arc.lock().unwrap()`.

`start-sddia.sh` no es la causa del look-ahead; sí es el **vector empírico de cierre** (CA5).

## Causa raíz

| Pieza | Hecho |
|-------|--------|
| Handler | `enrich_fracture_pbi_kaizen.rs` — regex con look-ahead |
| Trigger | PBI pending con sección Kaizen ya rellenada (re-upsert) |
| Cascada | `route_domain_core.rs` — `Mutex::lock().unwrap()` sin recuperación de poison |

## Solución

1. Recorte de sección por delimitadores Markdown (`split_once` + `find("\n## ")`). Sin `regex`.
2. `recover_lock`: `unwrap_or_else(PoisonError::into_inner)`.
3. `catch_unwind` en dispatch async: panic de suscriptor → `failed: subscriber panicked`.
4. Ignición: `start-sddia.sh` recompila `execute-process` en `SddIA/target` y exporta `SDDIA_EXECUTE_PROCESS_BIN`.

## CA5 — Validación empírica de `./start-sddia.sh` (bloqueante)

Protocolo:

```bash
./start-sddia.sh
```

**APTO** solo si en el mismo arranque:

| Señal | Obligatorio |
|-------|-------------|
| `[SddIA] Asegurando orquestador nativo` + path a `SddIA/target/.../execute-process` | sí |
| `[SddIA] Ecosistema S+ Grade operativo.` | sí |
| Centinelas obligatorios ACTIVO; heartbeats `missed_cycles<3` | sí |
| Ausencia de `regex kaizen section` / `look-around` / `PoisonError` | sí |
| Ausencia de `[WATCHER] route-domain-event falló` por panic del orquestador | sí |

Cualquier otro fallo de ignición (Kalma2, IOTA, gate de latidos, panic distinto) **bloquea** CA5. No se declara Done con tests unitarios solos.

## Fuera de alcance

- Causa raíz de fracturas heartbeat históricas de centinelas, salvo que impidan el gate de ignición (entonces entran por CA5).
---
