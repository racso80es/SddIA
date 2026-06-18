---
feature_name: migracion-execute-process-rust
created: "2026-06-18"
process: feature
branch_name: feat/migracion-execute-process-rust
persist_ref: docs/features/migracion-execute-process-rust
---

# Blueprint — Migración orquestador a Rust nativo

## 1. Estrategia: porte por capas con red de paridad

Migración incremental con el `.py` vigente como **oráculo de paridad** hasta verde total. El cambio de touchpoints es el **último paso atómico** (un PR), evitando coexistencia rota.

```text
Fase A  Andamiaje crate + golden harness
Fase B  Porte core (parser/resolver/env)
Fase C  Porte engine (executor/capsules/handlers)
Fase D  Porte forges
Fase E  Switch de touchpoints + docs viva
Fase F  Poda condicional + cierre
```

## 2. Ubicación del crate (decisión)

Nuevo miembro del workspace bajo `SddIA/engine/execute-process/` (patrón `crate-binario`, homólogo a `daemons/*`):

```toml
# SddIA/Cargo.toml (members) — añadir
"engine/*"
```

```toml
# SddIA/engine/execute-process/Cargo.toml
[package]
name = "execute-process"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "execute-process"
path = "src/main.rs"

[dependencies]
sddia-io = { path = "../../sddia-io" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"
```

> Mutación de `SddIA/Cargo.toml` y `SddIA/process/*` se realiza por la vía autorizada; el resto es código bajo workspace (no genoma indexado).

## 3. Fases de ejecución

### Fase A — Andamiaje y red de paridad
- Crear crate `execute-process` y registrarlo en el workspace.
- Definir `OrchestratorEnvelope` (serde) con el esquema rico de §4.2 spec.
- **Golden harness:** script que ejecuta N casos (procesos + inputs reales) contra `python3 execute-process.py` y captura envelopes de referencia (normalizando no-deterministas).

### Fase B — `core`
- `core::parser`: `serde_yaml` para frontmatter de `SddIA/process/*.md`.
- `core::resolver`: resolución `name`/`aliases`, `normalize_request`, conjuntos `RUNTIME_INJECTED_INPUTS` / `DEFAULTABLE_INPUTS`.
- `core::env`: jerarquía de bóvedas + log `[CONFIG] Jerarquía detectada…`.
- Tests unitarios contra fixtures de procesos reales.

### Fase C — `engine`
- `engine::executor`: `run_process`, `execute_phase`, `run_workspace_init`, ramas de suite.
- `engine::capsules`: invocación `wasmtime`, Peaje Termodinámico (telemetría + orquestación), registro de acciones → `execute-action.py` (subprocess).
- `engine::handlers`: `route-domain-event`, `kalma2-interact`, `telegram-fallback-responder`, kill-switch.
- Validar emisión de eventos en `./.events/{telemetry,orchestration}`.

### Fase D — `forges`
- `forges::factory`: `tool`/`action`/`process` con UUID, `hash_signature`, append a índices.
- Tests de diff de índices + idempotencia.

### Fase E — Switch de touchpoints (atómico)
- Reapuntar consumidores al binario (ver `implementation.md` tabla T-*).
- Actualizar `README.md` y norma DA-3 (vía proceso autorizado).
- Smokes E2E verdes con el binario.

### Fase F — Poda condicional y cierre
- Auditar consumidores residuales de PyYAML (D6); podar `requirements.txt` solo si procede.
- Retirar `execute-process.py` y módulos `execute_process_*.py` tras CA-7 verde.
- `validacion.md` APTO + PBI a `done/` en el mismo PR.

## 4. Secuencia de verificación

| Hito | Gate |
|------|------|
| Fin Fase B | `cargo test` core verde + golden de resolución |
| Fin Fase C | Golden de envelope para `feature`/`route-domain-event` |
| Fin Fase D | Golden de forjas + índices |
| Fin Fase E | Smokes E2E (centinelas, hooks, Kalma2) verdes |
| Fin Fase F | Argos APTO; sin PyYAML en orquestación |

## 5. Orden de delegación (runtime)

1. `skill:git-manager` — inicialización (ya ejecutada en esta sesión).
2. `agent:dedalo` — `spec.md` + `plan.md` (este documento).
3. `agent:tekton` — `implementation.md` + `execution.md` (forja física, fase posterior).
4. `agent:argos` — `validacion.md`.
5. `action:execute-process` → `delivery-close-cycle` (cierre).

## 6. Rollback

El `.py` permanece intacto hasta Fase E. Revertir = no aplicar el switch de touchpoints; el binario convive como artefacto inerte sin consumidores. Riesgo de regresión acotado a un único PR.
