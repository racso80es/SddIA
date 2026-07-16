---
feature_name: eda-fractal-dlq-c2
created: "2026-07-16"
process: bug-fix
branch: fix/eda-fractal-dlq-c2
---

# Execution

1. Laudo operador: **B** (`eda_fractal.dead_letter` = `./.events/dead-letter`).
2. Rama `fix/eda-fractal-dlq-c2` desde `main`.
3. Mutación runtime + SSOT (sin genoma protegido DA-2 salvo docs/templates).
4. Build local: `CARGO_TARGET_DIR=SddIA/target cargo build -p sddia-daemon-runtime -p event-sweeper -p event-watcher -p execute-process`.
5. Smoke: `event-sweeper --once --json` sobre backlog domain con stamps `failed`.

## Resultado smoke (2026-07-16)

| Antes | Después |
|-------|---------|
| `.events/domain/` = 2 (ambos con `failed`) | `.events/domain/` = 0 |
| DLQ JSON = 18 | DLQ JSON = 20 (+2 movidos) |
