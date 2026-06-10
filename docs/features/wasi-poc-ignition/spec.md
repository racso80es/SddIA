---
feature_name: wasi-poc-ignition
created: "2026-06-01"
process: feature
base: main
scope: wasi-capsule-poc
version_spec: "1.0.0"
persist_ref: docs/features/wasi-poc-ignition
branch_name: feat/wasi-poc-ignition-9366362475876501103
---

# Especificación — Prueba de Concepto: SddIA WASI Ignition

## 1. Naturaleza y propósito

Validar la viabilidad de compilar y ejecutar cápsulas SddIA (Skills/Tools) bajo **WebAssembly System Interface (WASI)**. Sustituir binarios nativos acoplados al SO por módulos `.wasm` con aislamiento (sandbox), ejecución determinista y portabilidad.

## 2. Fronteras del dominio (restricciones duras)

| Restricción | Norma |
|-------------|-------|
| Aislamiento I/O | `SddIA/norms/capsule-json-io.md` v2.0 — solo `stdin` (JSON) / `stdout` (JSON) |
| Ceguera espacial | Sin acceso al FS del anfitrión salvo montaje explícito |
| Táctica del refugio | Runtime ligero (Wasmtime) vía wrapper mínimo; sin orquestadores corporativos |

## 3. Criterios de aceptación (S+ Grade)

1. Cápsula Rust `wasi-poc` compila al target `wasm32-wasip1` (sucesor de `wasm32-wasi`).
2. Wasmtime ejecuta el `.wasm` con payload JSON simulado por stdin.
3. La cápsula devuelve envelope JSON válido por stdout sin romper el sandbox.

## 4. Artefactos

| Artefacto | Ruta |
|-----------|------|
| Cápsula Rust | `SddIA/tools/wasi-poc/` |
| Build | `SddIA/tools/wasi-poc/scripts/build-wasi.sh` |
| Run | `SddIA/tools/wasi-poc/scripts/run-wasi.sh` |
| Salida WASM | `SddIA/tools/wasi-poc/target/wasm32-wasip1/release/wasi-poc.wasm` |

## 5. Contrato de I/O

**Petición (stdin):**

```json
{
  "meta": {
    "schemaVersion": "2.0",
    "entityKind": "tool",
    "entityId": "wasi-poc"
  },
  "request": { "ping": true }
}
```

**Respuesta (stdout, una línea):**

```json
{
  "meta": { "schemaVersion": "2.0", "entityKind": "tool", "entityId": "wasi-poc" },
  "success": true,
  "exitCode": 0,
  "message": "WASI capsule executed in sandbox",
  "result": {
    "echo": { "ping": true },
    "wasi_status": "S+ Grade_Sealed",
    "sandbox": "wasm32-wasip1"
  }
}
```
