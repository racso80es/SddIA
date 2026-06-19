---
feature_name: kalma2-bridge-rust
created: "2026-06-19"
process: feature
branch_name: feat/kalma2-bridge-rust
persist_ref: docs/features/kalma2-bridge-rust
uuid: 2afb1f2f-667c-4c39-ae5f-7bd7f626c7e2
status: pre-execution
---

# Blueprint — kalma2-bridge

## Estrategia

Coexistencia con paridad: el puente Python permanece como **oráculo** hasta verde. Switch de touchpoints (`start-sddia.*`) atómico. Poda del `.py` al final.

```text
Fase A  Andamiaje crate + registro workspace
Fase B  Servidor estático (GET) con anti-traversal
Fase C  Enrutador POST /api/interact + subproceso orquestador
Fase D  Switch touchpoints (start-sddia.sh/.md) + restaurar sddia-run.sh
Fase E  Paridad vs .py + poda sddia-client-bridge.py
```

## Fases

### Fase A — Andamiaje
- `SddIA/Cargo.toml`: añadir `"interfaces/*"` a members (vía autorizada).
- Crear `SddIA/interfaces/kalma2-bridge/{Cargo.toml,src/main.rs}`.
- `cargo build -p kalma2-bridge` (esqueleto que escucha en 8765).

### Fase B — Superficie estática
- Resolver `REPO_ROOT` + raíz bundle `interfaces/kalma2/`.
- `GET /` → index; `GET /{rel}` con canonicalización anti-traversal; MIME map.

### Fase C — Enrutador de voluntad
- `POST /api/interact`: validar `{"prompt": string}` no vacío.
- Resolver binario orquestador (SSOT §4 spec).
- `std::process::Command` síncrono, captura stdout/stderr, timeout.
- Passthrough de última línea JSON.

### Fase D — Switch touchpoints
- `start-sddia.sh`: reemplazar `python3 .SddIA/client/sddia-client-bridge.py &` por resolución/arranque del binario `kalma2-bridge`.
- `start-sddia.md`: actualizar tabla de componentes, requisitos (build crate), diagnóstico.
- Restaurar `sddia-run.sh` (wrapper CLI → `orchestrator_resolve.py`).

### Fase E — Paridad y poda
- Smoke comparativo: mismo prompt vs `.py` y vs binario → respuestas equivalentes.
- Retirar `.SddIA/client/sddia-client-bridge.py` (+ `__pycache__`).
- `validacion.md` APTO + PBI a `done/` en el mismo PR.

## Gates

| Hito | Gate |
|------|------|
| Fin A | `cargo build -p kalma2-bridge` verde |
| Fin B | `curl GET /` = index.html; traversal bloqueado |
| Fin C | `POST /api/interact` = JSON orquestador |
| Fin D | `start-sddia.sh` 4/4 + Kalma2 200 sin python en puente |
| Fin E | Paridad `.py`≡binario; working tree sin `.py` |

## Rollback

El `.py` y su touchpoint permanecen hasta Fase D. Revertir = no aplicar switch; el binario convive inerte. Riesgo acotado a un PR.

## Orden de delegación (runtime)

1. `agent:dedalo` — `spec.md` + `plan.md` (este doc).
2. `agent:tekton` — `implementation.md` + forja física (Fase A–C).
3. `agent:argos` — `validacion.md`.
4. `action:execute-process` → `delivery-close-cycle`.
