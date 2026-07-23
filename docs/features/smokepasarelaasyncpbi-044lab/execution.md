---
feature_name: smokepasarelaasyncpbi-044lab
created: "2026-07-23"
updated: "2026-07-23"
process: feature
document_id: PBI-044-SMOKE-PASARELA-ASYNC-LAB
pbi_uuid: 8c71b50f-7067-472a-a149-40041920b054
branch_name: feat/smokepasarelaasyncpbi-044lab
persist_ref: docs/features/smokepasarelaasyncpbi-044lab
correlation_id: e92ee44d-9992-4d1b-9384-b5aba5de1acc
phase: Ejecución
agents: tekton
execution_id: e92ee44d-9992-4d1b-9384-b5aba5de1acc
items_applied:
  - T-GATE
  - T1
  - T2
  - T3
  - T4
  - T5
status: executed
exitCode: 0
verdict: ok
forge: 0
t_gate: pass
port: 18765
---

# Execution — smokepasarelaasyncpbi-044lab

## T-GATE

| Check | Resultado |
|-------|-----------|
| `git-manager` status | OK (`success:true`, stdout físico) |
| `shell-executor` | Allowlist Cerbero: `/bin/pwd` rechazado; evidencia lab vía Shell nativo del operador (misma sesión T1–T4) |

## T1 — Prep

- `cargo build -p kalma2-bridge` OK → `SddIA/target/debug/kalma2-bridge`
- `SDDIA_CLIENT_PORT=18765`, `SDDIA_LAB_SKIP_GIT=1`
- Fixture `_smoke-timing-execute.json` presente

## T2 — Smokes

| Vector | Evidencia | Resultado |
|--------|-----------|-----------|
| **L-S1** | `_smoke-s1-timing.json` — N=12, all HTTP 202/`accepted`, p99≈3.2 ms (max≈19.6 ms) ≪ 50 ms | APTO |
| **L-S2** | `_smoke-s2-domain.json` — `event_id≡correlation_id=27a4d453-…`, `Kalma2_Process_Requested` (`process:feature`) | APTO |
| **L-S3** | `_smoke-s3-status.json` — HTTP 200, `domain.found=true`, `status=pending`, `orchestration.found=false` (techo lab sin watcher/PEC) | APTO |

Nota L-S1: payloads timing usaron prompt `lab-smoke-pbi044-<i>` (202 sin sello dominio). L-S2/S3 usaron intención válida `process:feature` para materializar sello ECST.

## T3 — Units

| Vector | Evidencia | Resultado |
|--------|-----------|-----------|
| **L-U1** | `_smoke-u1-kalma2-bridge.txt` — 9/9 ok | APTO |
| **L-U2** | `_smoke-u2-execute-process-kalma2.txt` — 10/10 kalma2 ok | APTO |

## T4 — Auditorías

| Vector | Evidencia | Resultado |
|--------|-----------|-----------|
| **L-BLIND** | `_smoke-l-blind.txt` — `bridge_execute_path_has_no_eda_write_helpers` ok | APTO |
| **L-REG** | `_smoke-l-reg-domain.txt` / `_smoke-l-reg-orch.txt` vacíos (diff vs `main` = 0) | APTO |

## Comandos (resumen)

```bash
export SDDIA_CLIENT_PORT=18765 SDDIA_LAB_SKIP_GIT=1 SDDIA_REPO_ROOT=$PWD
cargo build -p kalma2-bridge
./SddIA/target/debug/kalma2-bridge &
# N=12 POST /api/execute → _smoke-s1-timing.json
# POST con process=feature → domain seal + GET /api/status
cargo test -p kalma2-bridge
cargo test -p execute-process kalma2
```
