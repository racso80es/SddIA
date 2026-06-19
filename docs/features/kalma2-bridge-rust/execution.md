---
feature_name: kalma2-bridge-rust
created: "2026-06-19"
process: feature
branch_name: feat/kalma2-bridge-rust
uuid: 2afb1f2f-667c-4c39-ae5f-7bd7f626c7e2
status: executed
---

# Ejecución — kalma2-bridge

## Build

```bash
cd SddIA
cargo build -p kalma2-bridge -p execute-process
```

## Arranque manual

```bash
export SDDIA_REPO_ROOT=/ruta/al/repo/SddIA
SddIA/target/debug/kalma2-bridge
```

## Smoke

```bash
curl -sf -o /dev/null -w '%{http_code}\n' http://127.0.0.1:8765/
curl -sf -XPOST http://127.0.0.1:8765/api/interact \
  -H 'Content-Type: application/json' -d '{"prompt":"hola"}'
curl -sf -o /dev/null -w '%{http_code}\n' http://127.0.0.1:8765/../Cargo.toml
```

## Ecosistema completo

```bash
./start-sddia.sh
```

## CLI orquestador (paridad shell)

```bash
./sddia-run.sh --process kalma2-interact --inputs '{"prompt":"hola"}'
```

## Evidencia

Ejecutado 2026-06-19: build/test verdes; smoke HTTP 200/404; POST retorna envelope orquestador con `data.response`.
