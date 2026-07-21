---
feature_name: iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin
created: "2026-07-20"
process: feature
branch_name: feat/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin
persist_ref: docs/features/iniciafeatureparaelpbidocstodoskitchenecosistemasddiaeinyeccinin
canonical_feature_name: fractura-core-paciente-0-gesfer
document_id: PBI-ECOSISTEMA-GESFER-PACIENTE-0
correlation_id: 4dd6f7a2-7bbf-4744-8a4c-7ac315ed9a51
phases: 5
agent_planificador: dedalo
target_executor: tekton
rbac_ok: true
---

# Plan / Blueprint — Fractura Core · Fase 1

Blueprint ejecutable para **Tekton** (`process: feature` · fase Ejecución).  
Entrada: `objectives.md`, `clarify.md`, `spec.md`.  
`target_executor_rbac.allowed_policies` homologado a Tekton v1.1.0: `ecosystem-evolution`, `filesystem-ops`, `source-control`, `system-operations`, `chaos-engineering`.

## Viabilidad RBAC (Cerbero)

| Cápsula | context | ¿Permitida? |
|---------|---------|-------------|
| `skill:filesystem-manager` | filesystem-ops | sí |
| `skill:git-manager` | source-control | sí |
| `skill:shell-executor` | system-operations | sí |
| `action:execute-process` | (cadena procesos `ecosystem-evolution`) | sí — vía orquestador |

Ninguna fase delega en cápsulas fuera de esta matriz.

---

## Fases declarativas

```yaml
phases:
  - name: "F1-A SSOT products + Shared Kernel Rust"
    intent: "Registrar products en cumulo.paths.json; crear crate sddia-core reexportando sddia-io; member en workspace."
    delegates_to:
      - "skill:filesystem-manager"
      - "skill:shell-executor"
  - name: "F1-B npm @sddia/core"
    intent: "Materializar packages/sddia-core como fachada npm del Shared Kernel."
    delegates_to:
      - "skill:filesystem-manager"
  - name: "F1-C Sellado capsule-json-io"
    intent: "Purgar literales GESFER_* en norma motor; fijar SDDIA_* ; anotar evolution."
    delegates_to:
      - "skill:filesystem-manager"
  - name: "F1-D Esqueletos Forge y Portal"
    intent: "Crear apps/sddia-forge y apps/sddia-portal vacíos con dependencia inerte a @sddia/core."
    delegates_to:
      - "skill:filesystem-manager"
  - name: "F1-E Verificación local y registro documental"
    intent: "cargo check -p sddia-core; rg anti-GesFer en perímetro; implementation.md + execution.md; commit vía git-manager."
    delegates_to:
      - "skill:shell-executor"
      - "skill:filesystem-manager"
      - "skill:git-manager"
```

---

## F1-A — SSOT + crate `sddia-core`

**Intent:** Jurisdicción Core empaquetable (AC1).

| # | Entregable | Detalle |
|---|------------|---------|
| A1 | `cumulo.paths.json` | Añadir `products` según `spec.md` §5; bump patch `version` del JSON |
| A2 | `SddIA/sddia-core/Cargo.toml` | `name = "sddia-core"`, dep path `sddia-io` |
| A3 | `SddIA/sddia-core/src/lib.rs` | `pub use sddia_io::*;` + módulo `jurisdiction` (markers documentales) |
| A4 | `SddIA/Cargo.toml` | Member `"sddia-core"` |
| A5 | Verify | `skill:shell-executor` → `cargo check -p sddia-core` en cwd workspace `SddIA/` |

**Criterio salida:** crate compila; clave `products.shared_kernel_crate` presente.

---

## F1-B — npm `@sddia/core`

**Intent:** Fachada consumible por cáscaras JS (AC1).

| # | Entregable | Detalle |
|---|------------|---------|
| B1 | `packages/sddia-core/package.json` | name `@sddia/core`, version `0.1.0`, private true |
| B2 | `packages/sddia-core/src/index.ts` (o `.js`) | Stub Shared Kernel (sin lógica GesFer) |
| B3 | `packages/sddia-core/README.md` | Jurisdicción + vínculo a crate / `capsule-json-io` |

**Criterio salida:** path `products.shared_kernel_npm` resoluble; sin literales cliente.

---

## F1-C — Sellado `capsule-json-io`

**Intent:** Tubería hermética + AC2/AC3.

| # | Entregable | Detalle |
|---|------------|---------|
| C1 | `SddIA/norms/capsule-json-io.md` | `GESFER_CAPSULE_REQUEST` → `SDDIA_CAPSULE_REQUEST`; `GESFER_SKIP_STDIN` → `SDDIA_SKIP_STDIN` |
| C2 | Evolution | Entrada breve en `SddIA/evolution/` vinculando `document_id` / `correlation_id` |
| C3 | Scan | `rg -i 'GESFER|gesfer' SddIA/norms/capsule-json-io.md` → vacío |

**Nota aduana:** norma motor (no `library_norms`). Mutación bajo feature activa + evolution (DA-4 / excepción operador documentada). No invocar `norm-creator` (clase táctica Librería).

**Criterio salida:** AC2 en perímetro norma; schema sigue `2.0`.

---

## F1-D — Esqueletos Forge & Portal

**Intent:** AC4 sin producto UI.

| # | Entregable | Detalle |
|---|------------|---------|
| D1 | `apps/sddia-forge/` | `package.json` + dep `@sddia/core` (`file:../../packages/sddia-core` o workspace) + README vacío + `.gitignore` |
| D2 | `apps/sddia-portal/` | Homólogo |
| D3 | Prohibido | Fuentes UI, AST, routers, assets de producto |

**Criterio salida:** ambos `package.json` declaran dependencia al Shared Kernel; README marca esqueleto F1.

---

## F1-E — Verificación y cierre de ejecución

**Intent:** Dejar evidencia para Argos; no abrir PR (eso es `delivery-close-cycle`).

| # | Acción | Cápsula |
|---|--------|---------|
| E1 | `cargo check -p sddia-core` | `skill:shell-executor` |
| E2 | Anti-GesFer en touchpoints F1 (`capsule-json-io`, `packages/sddia-core`, `SddIA/sddia-core`, `apps/*`) | `skill:shell-executor` |
| E3 | `implementation.md` + `execution.md` (frontmatter norma) | `skill:filesystem-manager` |
| E4 | Commit(s) en rama feature | `skill:git-manager` únicamente |

**Opcional O3 (fuera de camino crítico):** promoción PBI kitchen→pending + UUID — solo con orden Racso; no bloquea F1-E.

**Criterio salida:** artefactos documentales de ejecución listos; rama con cambios F1-1…F1-3; Fases 2–4 ausentes del diff.

---

## Orden y commits sugeridos

```text
F1-A ──► F1-B ──► F1-C ──► F1-D ──► F1-E
```

| Commit | Mensaje sugerido |
|--------|------------------|
| 1 | `feat(core): F1-A shared kernel crate sddia-core + products SSOT` |
| 2 | `feat(core): F1-B npm facade @sddia/core` |
| 3 | `fix(norms): F1-C purge GESFER env aliases in capsule-json-io` |
| 4 | `feat(apps): F1-D empty forge/portal shells depending on @sddia/core` |

## Fuera de este blueprint

- `delivery-close-cycle` / PR / `validacion.md` Argos (fases posteriores del proceso `feature`).
- Inyección `.SddIA/` en microservicios GesFer (Fase 2).
- Minteo IOTA / códices dominio (Fase 3).
- Runtime invisible / wallet (Fase 4).
