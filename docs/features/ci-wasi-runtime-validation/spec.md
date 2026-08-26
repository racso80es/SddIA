---
feature_name: ci-wasi-runtime-validation
created: "2026-06-11"
process: feature
branch_name: feat/ci-wasi-runtime-validation
persist_ref: docs/features/ci-wasi-runtime-validation
---

# Especificación — CI WASI runtime validation

## 1. Propósito

Complementar `eda-bus-e2e-smoke` (resiliencia vía fallback Python) con un job que **obliga** la ejecución de cápsulas `wasm32-wasip1` mediante `wasmtime` en el runner Ubuntu de GitHub Actions.

## 2. Workflow

Archivo: `.github/workflows/sddia-index-qa.yml`

Nuevo job `wasi-runtime-smoke`:

```yaml
wasi-runtime-smoke:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
      with:
        targets: wasm32-wasip1
    - name: Install wasmtime
      run: |
        curl https://wasmtime.dev/install.sh -sSf | bash
        echo "$HOME/.wasmtime/bin" >> "$GITHUB_PATH"
    - uses: actions/cache@v4
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          SddIA/target
        key: wasi-${{ runner.os }}-${{ hashFiles('SddIA/Cargo.lock') }}
    - name: Build WASI capsules (skills/tools; exclude native daemons)
      run: SddIA/scripts/qa/build-wasi-capsules.sh
    - uses: actions/setup-python@v5
      with:
        python-version: "3.12"
    - run: pip install pyyaml
    - name: WASI CI smoke
      env:
        SDDIA_CI_REQUIRE_WASI: "1"
        SDDIA_LAB_SIMULATE_IOTA: "1"
        SDDIA_LAB_SIMULATE_SYNC_INDEX: "1"
        SDDIA_LAB_ROUTE_SYNC: "1"
      run: python SddIA/scripts/qa/run-wasi-ci-smoke.py --json
```

## 3. Código de soporte

| Componente | Descripción |
|------------|-------------|
| `run-wasi-ci-smoke.py` | Orquestador: verifica toolchain, PoC, crypto WASM, E2E opcional |
| `SDDIA_CI_REQUIRE_WASI` | En `crypto()` — prohibe fallback cuando está activo |
| `wasi-poc/scripts/run-wasi.sh` | Reutilizado para sellado envelope |

## 4. Separación de responsabilidades

| Job | Modo | Propósito |
|-----|------|-----------|
| `eda-bus-e2e-smoke` | Fallback permitido | Resiliencia EDA bus en runners sin Rust |
| `wasi-runtime-smoke` | WASI obligatorio | Certificación migración cápsulas |
