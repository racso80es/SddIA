---
feature_name: iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin
created: "2026-07-20"
process: feature
branch_name: feat/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin
persist_ref: docs/features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin
canonical_feature_name: fractura-core-paciente-0-gesfer
document_id: PBI-ECOSISTEMA-GESFER-PACIENTE-0
correlation_id: 4dd6f7a2-7bbf-4744-8a4c-7ac315ed9a51
scope: "F1 — Fractura Core (1.1 + 1.2 + 1.3-esqueleto)"
agent: tekton
items:
  - id: F1-A
    path: SddIA/sddia-core
    status: done
  - id: F1-A-ssot
    path: SddIA/core/cumulo.paths.json
    status: done
  - id: F1-B
    path: packages/sddia-core
    status: done
  - id: F1-C
    path: SddIA/norms/capsule-json-io.md
    status: done
  - id: F1-D-forge
    path: apps/sddia-forge
    status: done
  - id: F1-D-portal
    path: apps/sddia-portal
    status: done
  - id: F1-E-cargo
    path: SddIA/sddia-core
    status: blocked
  - id: F1-E-git
    path: skill:git-manager
    status: blocked
tekton_verdict: blocked
block_reason: shell_allowlist_no_cargo_no_git_manager
---

# Implementation — Fractura Core F1

## Sentencia

Materialización **F1-A…F1-D completa** vía `skill:filesystem-manager` (LLM-native).  
**F1-E incompleto:** entorno de sesión solo permite `ls` en Shell; `cargo check` y `skill:git-manager` no invocables. No se inventa éxito de compilación ni commit.

## Touchpoints

| ID | Artefacto | Cambio |
|----|-----------|--------|
| F1-A | `SddIA/core/cumulo.paths.json` | `version` 1.5.0→1.5.1; bloque `products` |
| F1-A | `SddIA/Cargo.toml` | member `sddia-core` |
| F1-A | `SddIA/sddia-core/` | crate Shared Kernel; `pub use sddia_io::*` + `jurisdiction` |
| F1-B | `packages/sddia-core/` | npm `@sddia/core` 0.1.0 private + README |
| F1-C | `SddIA/norms/capsule-json-io.md` | `SDDIA_CAPSULE_REQUEST` / `SDDIA_SKIP_STDIN` |
| F1-C | `SddIA/evolution/4dd6f7a2-7bbf-4744-8a4c-7ac315ed9a51.md` | registro evolution |
| F1-D | `apps/sddia-forge/` | esqueleto + dep `file:../../packages/sddia-core` |
| F1-D | `apps/sddia-portal/` | homólogo |

## Anti-GesFer (perímetro F1)

Scan Grep en `capsule-json-io`, `packages/sddia-core`, `SddIA/sddia-core`, `apps/*` → **0 matches**.  
Library códices/normas de dominio **no** tocados (fuera de perímetro).

## Residual operador (F1-E)

```bash
cd SddIA && cargo check -p sddia-core
# commit vía skill:git-manager (no git crudo)
```

## Veredicto

**blocked** — forja física lista; sellado verify/commit pendiente de shell/cápsulas.
