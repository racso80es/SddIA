---
feature_name: migracion-execute-process-rust
created: "2026-06-18"
process: feature
purpose: Estabilización de requisitos — migración execute-process.py a Rust nativo
---

# Clarificación — migracion-execute-process-rust

Transcript de decisiones (2026-06-18). Estabiliza el alcance del PBI `PBI-MIGRACION-EXECUTE-PROCESS-RUST` y resuelve ambigüedades de sustrato, empaquetado y frontera de paridad antes del diseño de blueprint.

---

## D1 — Naturaleza de la feature

| Pregunta | Decisión |
|----------|----------|
| ¿Rediseño funcional o porte 1:1? | **Porte de paridad estricta.** La lógica ya fue certificada en `refactor-execute-process-engine` (PR #9); se transmuta sustrato, no comportamiento. |
| ¿Proceso? | `feature` v1.3.0 |
| Rama | `feat/migracion-execute-process-rust` |
| `persist_ref` | `docs/features/migracion-execute-process-rust` |

---

## D2 — Sustrato de compilación (la decisión raíz)

| Opción | Decisión |
|--------|----------|
| Cápsula WASI `wasm32-wasip1` (como skills/tools) | **Rechazada.** El orquestador hace *subprocess spawning* (`wasmtime run`, `git`, `execute-action`); WASI no soporta spawn con `--dir=.` (lección `migracion-rust-wasi` D8). |
| **Binario nativo Rust** | **Adoptada.** Homólogo a los centinelas (`SddIA/daemons/*` usan `std::process::Command`). El runtime de producción de cápsulas sigue siendo WASI; el orquestador que las invoca es nativo. |

**Corolario:** el orquestador es el **host** de las cápsulas WASI, no una cápsula más.

---

## D3 — Empaquetado y desacoplamiento

| Pregunta | Decisión |
|----------|----------|
| ¿Enlazar el orquestador como crate/librería dentro de los daemons? | **No.** Viola la Ceguera Espacial: los centinelas son Despertadores Inertes. |
| ¿Cómo lo consumen centinelas, hooks, wrapper y Kalma2? | **Binario independiente** invocado vía `std::process::Command` / shell. El contrato `--process/--inputs → stdout JSON` no cambia. |
| Ubicación del crate | Nuevo miembro del workspace `SddIA/Cargo.toml` (p. ej. `engine/execute-process` o `orchestrator/`), a estabilizar en `plan.md`. |

---

## D4 — Frontera de paridad de E/S

| Pregunta | Decisión |
|----------|----------|
| ¿Reusar `SddiaResponse` de `sddia-io`? | **Parcial.** Se reutiliza para lectura de `stdin`, semántica de `exit` y primitivas de error. |
| ¿El envelope del orquestador es el de `SddiaResponse`? | **No.** El orquestador emite un envelope **más rico**: `{success, status_code, data, error, execution_report, exitCode}`. Debe preservarse **byte-compatible** (mismos campos, mismo orden semántico) porque centinelas y el puente Kalma2 parsean la **última línea JSON** de stdout. |
| Criterio de paridad | Golden tests: misma entrada → mismo envelope que el Python actual (salvo campos no deterministas: UUID, timestamps, `duration_ms`). |

---

## D5 — Cores satélite y dependencias subprocess

| Componente | Decisión |
|------------|----------|
| `route_domain_event_core`, `kalma2_interact_core`, `telegram_fallback_responder_core`, `daemon_kill_switch_core` | **Portar a submódulos Rust** (`engine::handlers::*`) con paridad. |
| `execute-action.py` | **No se porta en esta feature.** Se sigue invocando como subprocess Python desde el orquestador nativo (deuda separada documentada). |
| `env_loader.py` (jerarquía de bóvedas) | Portar a `core::env`; preservar precedencia SO > `.dev/.env` > `.SddIA/.dev/.env` y el log `[CONFIG] Jerarquía detectada…`. |

---

## D6 — PyYAML / `requirements.txt`

| Pregunta | Decisión |
|----------|----------|
| ¿Eliminar `requirements.txt` al cerrar la feature? | **Condicional** (heredada de `migracion-rust-wasi` D3). Tras el porte, el orquestador deja de usar PyYAML, pero scripts QA residuales (`verify-process-integrity.py`, `execute-action.py`, `audit-doc-parity.py`) aún lo consumen. |
| Acción | Auditar consumidores residuales; mantener `requirements.txt` documentado mientras exista al menos uno. La poda total es objetivo de una feature posterior. |

---

## D7 — Mutación de genoma y documentación viva

| Pregunta | Decisión |
|----------|----------|
| Contratos de proceso / normas que citan el CLI | Mutación vía **proceso autorizado / `entity-manager`** (DA-2), nunca bisturí directo del IDE. |
| Documentos históricos (`docs/todos/done/`, `evolution/`, features cerradas) | **No se reescriben.** Se citan como antecedente. |
| `README.md` | Editable (instancia, fuera de genoma) — actualizar §«Aduana Universal (CLI)» y ejemplos de invocación. |

---

## D8 — Criterios de Done

```text
Done = PR mergeado en main
     + binario orquestador miembro del workspace, cargo build sin warnings
     + paridad de envelope verificada (golden tests) para feature/bug-fix/route-domain-event/delivery-close-cycle/entity-manager
     + centinelas + hooks + sddia-run.sh + Kalma2 operan E2E contra el binario
     + ningún flujo de orquestación requiere Python/PyYAML; requirements.txt reevaluado (D6)
     + validacion.md global: APTO, pbi_archived: true
     + PBI en docs/todos/done/ en la misma rama
```

---

## Alcance de la entrega actual

Esta sesión materializa la cascada documental **hasta `implementation.md`** inclusive. La forja física del binario (`execution.md`) y la auditoría Argos (`validacion.md`) pertenecen a la fase de Ejecución posterior.
