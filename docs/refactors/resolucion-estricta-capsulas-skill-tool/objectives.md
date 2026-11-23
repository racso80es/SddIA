---
feature_name: resolucion-estricta-capsulas-skill-tool
created: "2026-11-23"
process: refactorization
branch_name: refactor/resolucion-estricta-capsulas-skill-tool
persist_ref: docs/refactors/resolucion-estricta-capsulas-skill-tool
tracker_ref: AUT-118
---
# Objetivos y Criterios de Aceptación — resolucion-estricta-capsulas-skill-tool (`AUT-118`)

- [x] **K-CROSS:** 0 invocaciones cruzadas `tool` vía `invoke_capsule_json` o `skill` vía `invoke_tool_capsule_json` en `SddIA/` (corregidos `notify_humanized_pr_merged.rs`, `route_domain_core.rs` y `handlers/email_noise_digest.rs`).
- [x] **K-RT:** `resolve_capsule` y Cerbero RBAC rechazan de forma inmediata con `CapsuleKindMismatch` + incidente registrado ante cualquier cruce `skill`↔`tool` contra `SddIA/skills/index.md` y `SddIA/tools/index.md`.
- [x] **K-QA:** Gate estático `gate-capsule-kind` (e integrado en `verify-tools-index`) activo en `sddia-qa` con `violation_count: 0`.
- [x] **K-TEST:** Tests unitarios en verde (`execute-process` y `sddia-qa`) cubriendo `tool→skill`, `skill→tool` y caso válido sin warnings (`RUSTFLAGS="-Dwarnings"`).
