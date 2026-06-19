---
feature_name: kalma2-mayeuta-llm-router
created: "2026-06-19"
process: feature
branch_name: feat/kalma2-mayeuta-llm-router
persist_ref: docs/features/kalma2-mayeuta-llm-router
uuid: def280fd-73a3-42fe-b485-3258f1e5e426
status: pre-implementation
---

# Blueprint — kalma2-mayeuta-llm-router

## Estrategia

Capa nueva sin regresión: el fallback determinista (`synthesize_mayeuta_response`) permanece como red de seguridad. La ruta LLM y el enrutamiento se añaden en el handler `kalma2`, no en la función compartida con `telegram-fallback`.

```text
Fase A  Skill mayeuta-llm: crate + contrato JSON + lectura SDDIA_LLM_CLI_COMMAND (skeleton)
Fase B  Transductor CLI Cursor (std::process::Command) + SYNTHESIZE + tests con fallback
Fase C  CLASSIFY_INTENT + esquema estricto + umbral de confianza
Fase D  Handler kalma2-interact: integrar Skill + fallback (sin tocar paridad)
Fase E  Enrutamiento asíncrono: allowlist + emisión evento Kalma2_Process_Requested + acuse
Fase E2 Cierre lazo EDA (P1-P3): evento dedicado + suscriptor process + rama dispatcher
Fase F  Cerbero/contexto subproceso + smokes E2E + validacion.md
```

## Fases

### Fase A — Skill (andamiaje) — C1
- Crear `SddIA/skills/mayeuta-llm/{Cargo.toml,src/main.rs}` (miembro `skills/*`).
- Definir contrato stdin/stdout y lectura de `SDDIA_LLM_CLI_COMMAND`.
- `SYNTHESIZE`/`CLASSIFY_INTENT` devuelven stub determinista (sin invocar CLI).

### Fase B — Transductor CLI Cursor — C3
- Ensamblar prompt y ejecutar el CLI vía `std::process::Command` (comando desde bóveda).
- `SYNTHESIZE` real con timeout; capturar stdout; fallback `success:false` si falta comando o falla.
- Tests: con env ausente → fallback; con CLI mock → parseo correcto.

### Fase C — Clasificación de intención
- `CLASSIFY_INTENT` con prompt de sistema + esquema JSON estricto.
- Validación del JSON devuelto; umbral de confianza (def. 0.7).

### Fase D — Handler
- `handlers::kalma2::run`: `filter_c` → CLASSIFY → (SYNTHESIZE | route).
- Degradación a `synthesize_mayeuta_response` si la Skill falla.
- `telegram-fallback-responder` intacto (O12).

### Fase E — Enrutamiento asíncrono — C2
- Allowlist `{bug-fix, feature, refactorization, task-queue-manager}`.
- Proceso autorizado → emitir evento `Kalma2_Process_Requested` (`fractal::write_fractal_event`, patrón telegram-gateway) + acuse inmediato al operador.
- Prohibido despacho síncrono de procesos de ciclo de vida.

### Fase E2 — Cierre del lazo EDA (P1–P3, O14)
- **P1:** crear `SddIA/events/domain/kalma2-process-requested.md` (ECST propio, emisor `kalma2-interact`) + suscriptor `process` en `event-domain-subscriptions.json` (recomendado: fijo `task-queue-manager`).
- **P2:** rama en `dispatch_subscriber` (`route_domain_event_core.py`): si `event_type == Kalma2_Process_Requested`, construir `process_inputs` desde `{process, pbi_ref, raw_text}`, con gate de allowlist.
- **P3:** registrar clase ECST + entrada en `eda-coverage.json`; verificar validación del payload.
- Smoke: emitir evento → `event-watcher` enruta → proceso despachado (lab `SDDIA_LAB_ROUTE_SYNC=1`).

### Fase F — Seguridad y cierre
- Declarar contexto de subproceso local de la Skill; gate Cerbero.
- Smokes: chat, fix→evento, fuera de allowlist, sin comando→fallback.
- `validacion.md` APTO + PBI a `done/` en el mismo PR.

## Gates

| Hito | Gate |
|------|------|
| Fin A | `cargo build -p mayeuta-llm` |
| Fin B | SYNTHESIZE vía CLI con fallback verde |
| Fin C | CLASSIFY_INTENT JSON válido + umbral |
| Fin D | degradación sin comando; paridad telegram intacta |
| Fin E | prompt fix → evento `Kalma2_Process_Requested` emitido (no síncrono) |
| Fin E2 | `event-watcher` enruta el evento a suscriptor `process` (no solo DLT); `dispatch_subscriber` no exige `branch`; ECST APTO |
| Fin F | Argos APTO; sin secretos en logs |

## Decisiones resueltas (C1–C3 — vinculantes)

1. **C1** — Cápsula forjada como **Skill** `mayeuta-llm` (descartada tool).
2. **C2** — Procesos de ciclo de vida despachados **asíncronos** vía evento EDA (`manual-task-requested`); nunca síncrono.
3. **C3** — Motor de inferencia: **CLI de Cursor local** vía `std::process::Command`, comando inyectado por `SDDIA_LLM_CLI_COMMAND` (sin red ni SDK en el Genoma).
4. Allowlist: `{bug-fix, feature, refactorization, task-queue-manager}` — gobierna qué se puede solicitar; todo se despacha por evento.

## Rollback

El fallback determinista permanece operativo; sin `SDDIA_LLM_CLI_COMMAND` se restaura el comportamiento PoC actual. Riesgo acotado.

## Orden de delegación (runtime)

1. `agent:dedalo` — `spec.md` + `plan.md` (este doc).
2. `agent:tekton` — `implementation.md` + forja (cápsula + handler).
3. `agent:argos` — `validacion.md`.
4. `action:execute-process` → `delivery-close-cycle`.
