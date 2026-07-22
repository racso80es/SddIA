---
feature_name: kalma2-pasarela-asincrona-eda
created: "2026-07-22"
updated: "2026-07-22"
process: feature
document_id: PBI-044-KALMA2-PASARELA-ASINCRONA-EDA
execution_id: 002f4e1b-0155-4874-95cd-8e6953ed0f70
items_applied:
  - T1
  - T2
  - T3
  - U1
  - U2
  - S1
  - S2
  - S3
status: evidence_apto
correlation_id_smoke: 6178f1d1-e1d7-4446-bc9b-fca16d79b872
---

# Execution — kalma2-pasarela-asincrona-eda

## Comandos

```bash
cd SddIA && CARGO_TARGET_DIR=target cargo build -p execute-process -p kalma2-bridge
CARGO_TARGET_DIR=target cargo test -p kalma2-bridge
CARGO_TARGET_DIR=target cargo test -p execute-process kalma2
SDDIA_CLIENT_PORT=8765 ./SddIA/target/debug/kalma2-bridge &
# N=12 POST /api/execute → 202; GET /api/status?event_id=<cid>
```

## Evidencia (2026-07-22 re-Tekton)

| ID | Escenario | Resultado |
|----|-----------|-----------|
| B1 | Materialización T1–T3 | **ok** |
| U1 | `cargo test -p kalma2-bridge` | **ok** — 9/9 (fix audit `include_str` autoincriminación) |
| U2 | `cargo test -p execute-process kalma2` | **ok** — 10/10 |
| S1 | Smoke timing N=12 `POST /api/execute` | **ok** — 12× HTTP 202 `accepted`; p99 RTT **4.5 ms** (&lt;50 ms); `duration_ms` bridge p99=3 |
| S2 | Correlación acuse ↔ dominio | **ok** — `.events/domain/6178f1d1-…json` `Kalma2_Process_Requested` con `event_id ≡ cid` |
| S3 | `GET /api/status?event_id=<cid>` | **ok** — HTTP 200 `status=pending` (dominio found; sin watcher → sin PEC terminal; proyección status viva) |
| GIT | `skill:git-manager` status | **ok** — `success: true` |
| AC-R3 | Diff suscripciones vs `main` | **ok** — 0 líneas (`event-domain` / `event-orchestration`) |

### S1 detalle

```text
codes: [202] ×12 · bodies_accepted: 12
min_s=0.002079 max_s=0.004524 p50_s=0.002843 p99_s=0.004524
last_cid=6178f1d1-e1d7-4446-bc9b-fca16d79b872
```

### S2/S3 detalle

```text
S2: .events/domain/6178f1d1-e1d7-4446-bc9b-fca16d79b872.json · Kalma2_Process_Requested
S3: success=true status=pending domain.found=true orchestration.found=false
```

## Código materializado

| Artefacto | Invariante |
|-----------|------------|
| `accept_execute` | spawn + reaper; `Stdio::null`; sin join HTTP |
| `handle_execute` / interact execute | `reply_accept_result` → 202 |
| `kalma2.rs` | honra `correlation_id` UUID; fallback `new_v4` |
| `app.js` | rama `accepted`/202 → `pollStatus` |

## Notas

- H3/R6 Telegram: defer.
- Terminal PEC e2e (watcher+TQM) fuera del smoke lab; cubierto por unit `project_status_completed_from_pec` + S3 dominio.
- Ruido borrado: `SddIA/scripts/_tmp_run_pack_tests.sh` (no entregar).
