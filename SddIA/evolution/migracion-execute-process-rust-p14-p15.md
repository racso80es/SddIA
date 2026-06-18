---
id: migracion-execute-process-rust-p14-p15
uuid: c8f2a1b3-4d5e-6f78-9012-3456789abcde
type: action
version: 1.0.0
feature: migracion-execute-process-rust
related_entity_uuid: 95b5ac3a-061f-458d-bfb6-69f91a1c1731
---

# Evolución — P14/P15 documentación orquestador

## Hito

Cierre documental P14 (`README.md`) y P15 (`external-ai-constraints.md` v1.2.0 — DA-3 SSOT `orchestrator_resolve`).

## Artefactos

| Artefacto | Cambio |
|-----------|--------|
| `README.md` | Wrapper `./sddia-run.sh`, SSOT, §Aduana Universal |
| `SddIA/norms/external-ai-constraints.md` | v1.2.0 — tabla DA-3 con invocación canónica |

## Gate residual

P17 (poda `.py` + bridges) pendiente de smokes E2E y CA-7/CA-8.
