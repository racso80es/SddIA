---
dependencies:
  - node
  - npm
  - pwsh
  - cargo
implementation_path: scripts/skills-rs/src/bin/verify_pr_protocol.rs
inputs: []
language: rust
name: Verify PR Protocol
outputs:
  - description: 0 for success, non-zero for failure.
    name: Exit Code
    type: integer
security_context: Karma2Token
skill_id: verify-pr-protocol
---
# Skill: verify-pr-protocol

## Propósito
Esta skill ejecuta de manera atómica los pasos del **Protocolo de Aceptación de PR** para **GesFer.Product.Front** (ver `SddIA/norms/pr-acceptance-protocol.md`).

## Responsabilidad
- Validar la nomenclatura de la rama actual (`scripts/validate-nomenclatura.ps1`).
- **Lint:** `npm run lint` (working directory: `src/`).
- **Build:** `npm run build` (working directory: `src/`).
- **Tests:** `npm run test` (working directory: `src/`).
- Retornar **éxito (0)** solo si **todos** los pasos son exitosos.

## Implementación
- **Lenguaje:** Rust.
- **Binario:** `verify_pr_protocol`.
- **Ruta fuente:** `scripts/skills-rs/src/bin/verify_pr_protocol.rs`.

## Uso
```powershell
# Desde la raíz del repositorio
cargo run --manifest-path scripts/skills-rs/Cargo.toml --bin verify_pr_protocol
```

## Salidas
- **Standard output:** Logs por fase (1/4 … 4/4).
- **Exit code:** `0` (éxito), `1` (fallo).
