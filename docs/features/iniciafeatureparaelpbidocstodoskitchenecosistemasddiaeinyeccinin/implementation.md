---
feature_name: iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin
created: "2026-07-20"
updated: "2026-07-21"
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
  - id: F1-A-lock
    path: SddIA/Cargo.lock
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
    status: done
  - id: F1-E-tests
    path: SddIA/sddia-core/src/lib.rs
    status: done
  - id: F1-E-git
    path: skill:git-manager
    status: done
tekton_verdict: ok
---

# Implementation — Fractura Core F1

## Sentencia

Materialización **F1-A…F1-E completa** con evidencia reproducible:

- Shared Kernel Rust + npm + SSOT `products`
- Sellado `SDDIA_*` en `capsule-json-io`
- Cáscaras Forge/Portal
- `Cargo.lock` actualizado; `cargo check/test --locked`
- Tests de markers / reexport
- Commits en rama; `git-manager status` exitoso

## Touchpoints

| ID | Artefacto | Cambio |
|----|-----------|--------|
| F1-A | `SddIA/core/cumulo.paths.json` | `version` 1.5.0→1.5.1; bloque `products` |
| F1-A | `SddIA/Cargo.toml` | member `sddia-core` |
| F1-A | `SddIA/Cargo.lock` | entrada `sddia-core` 0.1.0 |
| F1-A | `SddIA/sddia-core/` | crate + tests jurisdicción |
| F1-B | `packages/sddia-core/` | npm `@sddia/core` 0.1.0 private |
| F1-C | `SddIA/norms/capsule-json-io.md` | `SDDIA_CAPSULE_REQUEST` / `SDDIA_SKIP_STDIN` |
| F1-C | `SddIA/evolution/4dd6f7a2-….md` | registro evolution |
| F1-D | `apps/sddia-forge/` / `apps/sddia-portal/` | esqueletos + dep inerte |
| Higiene | `.gitignore` | ignora `.SddIA/daemons/status/` |

## Anti-GesFer (perímetro F1)

Scan en `capsule-json-io`, `packages/sddia-core`, `SddIA/sddia-core`, `apps/*` → **0 matches**.

## Veredicto

**ok** — frontera F1 certificable bajo AC1–AC6 refinados; PBI kitchen maestro permanece sin archivar (O3).
