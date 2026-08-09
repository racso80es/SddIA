---
feature_name: sddia-codex-software-engineering
created: "2026-08-09"
process: feature
base: main
scope: sddia-codex-software-engineering
branch_name: feat/sddia-codex-software-engineering
persist_ref: docs/features/sddia-codex-software-engineering
pbi_ref: docs/todos/done/[ARQUITECTURA] PBI-SDDIA-DOMAIN-ABSTRACT-02 — Migración process software a códice.md
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-02
execution_id: c76c5d95-b066-49ca-834b-78a4f9443a62
version_spec: "1.0.0"
status: dedalo_locked
laudo: L-MVP-A-codex-gate-no-physical-move
agents: dedalo
---

# Especificación — sddia-codex-software-engineering

## 1. Misión técnica

Forjar el Códice `codex-software-engineering`, declarar membresía de process software-lifecycle y **exigir autoridad de dominio** en el orquestador antes de ejecutar esos process. Relocalización física de `.md` **diferida** (ABSTRACT-03).

## 2. Laudos Dedalo

| Ref | Pregunta | Laudo |
|-----|----------|-------|
| **D1** | ¿Mover `feature.md` fuera de Core? | **No** en este PR (AC-MOVE → ABSTRACT-03). Resolución sigue `directories.process`. |
| **D2** | ¿Membresía PR cycle? | **Sí:** `pull-request-review`, `accept-pr`, `delivery-close-cycle` + `feature`/`bug-fix`/`refactorization`. |
| **D3** | ¿Gate dónde? | Pre-fases en `run_generic` (`executor.rs`): `domain_authority::assert_process_allowed`. |
| **D4** | ¿Compat software-first? | `git_required: true` ∧ `codex_slug` ausente ⇒ **autoridad software implícita** (legado ABSTRACT-01). Deny si: (a) `codex_slug` ≠ software, o (b) `git_required: false` ∧ slug ≠ software. |
| **D5** | ¿Forja códice? | `entity-manager` → `codex-creator`. Composition: normas library `features-documentation-pattern`, `patterns-in-planning-implementation-execution`, `pr-acceptance-protocol`. |
| **D6** | ¿Membresía en contrato? | Campo opcional frontmatter `process_membership: [string]` + tabla en cuerpo. Runtime lee frontmatter; fallback constante alineada. |
| **D7** | Creators Core | Fuera de membresía. |

## 3. Códice `codex-software-engineering`

| Campo | Valor |
|-------|-------|
| slug / archivo | `codex-software-engineering.md` |
| name | `SddIA Codex Software Engineering` |
| nature | `domain-codex` |
| target_environment | `["software-engineering", "git", "pull-request"]` |
| certification_grade | `Pendiente` |
| process_membership | ver §3.1 |

### 3.1 Membresía (SSOT runtime)

```text
feature
bug-fix
refactorization
pull-request-review
accept-pr
delivery-close-cycle
```

### 3.2 Composition (UUIDs library)

| Norma | UUID | path relativo cantera |
|-------|------|----------------------|
| features-documentation-pattern | `4c448c82-de41-460f-b24f-82a84fa5ed69` | `../norms/features-documentation-pattern.md` |
| patterns-in-planning-implementation-execution | `1c6af49c-3091-4648-aa54-bbf6bcb90f82` | `../norms/patterns-in-planning-implementation-execution.md` |
| pr-acceptance-protocol | `7c18fe07-9567-4f06-8d2b-a58e04608171` | `../norms/pr-acceptance-protocol.md` |

## 4. Regla de autoridad

```text
si process ∉ membership → allow
profile = resolve_execution_profile(repo, inputs)
slug = profile.codex_slug
si slug == Some("codex-software-engineering") → allow
si slug == Some(otro) → DENY DomainAuthorityDenied
si slug == None && profile.git_required → allow (legado software-first)
si slug == None && !profile.git_required → DENY DomainAuthorityDenied
```

Deny: `success: false`, `status_code: 1`, error código `DOMAIN_AUTHORITY_DENIED`, **sin panic**, sin fases.

## 5. Touchpoints motor

| Path | Cambio |
|------|--------|
| `engine/domain_authority.rs` | **Nuevo** — membership + assert |
| `engine/domain_profile.rs` | sin cambio de contrato (reuso) |
| `engine/executor.rs` | llamar assert al inicio de `run_generic` |
| `engine/mod.rs` | `pub mod domain_authority` |

## 6. Criterios ↔ evidencia

| AC | Evidencia |
|----|-----------|
| AC-CODEX | archivo + fila `index.md` |
| AC-MEMBER | frontmatter `process_membership` + constante |
| AC-GATE | test: profile `git_required:false` sin slug software → deny |
| AC-ALLOW | test/default o slug software → allow |
| AC-BUILD | `cargo build -p execute-process --release` |
| AC-DOC | cascada + PBI done |
| AC-MOVE | **Diferido** ABSTRACT-03 (este PR) |

## 7. Handoff Tekton

1. Forjar códice vía `entity-manager` (create/codex); si runtime creator falla → materializar conforme contrato + index + evolution, luego reintentar sellado EDA.
2. Implementar `domain_authority` + tests.
3. Smokes documentados en `execution.md`.
