---
feature_name: sddia-domain-abstraction
created: "2026-08-05"
process: feature
base: main
scope: sddia-domain-abstraction
branch_name: feat/sddia-domain-abstraction
persist_ref: docs/features/sddia-domain-abstraction
pbi_ref: docs/todos/done/[ARQUITECTURA] Separación de Dominio SddIA y Abstracción del Contexto de Ejecución.md
document_id: PBI-SDDIA-DOMAIN-ABSTRACT-01
execution_id: 758d4440-2889-47a9-b412-ffab00ba0c1d
version_spec: "1.0.0"
status: dedalo_locked
laudo: L-SPLIT-A-mvp-profile-git-gate
agents: dedalo
---

# Especificación — sddia-domain-abstraction

## 1. Misión técnica (MVP L-SPLIT-A)

Hacer que el orquestador arranque y materialice contexto de tarea **sin Git obligatorio**, gobernado por un **perfil de dominio activo** (instancia + override de input), y demostrar denegación Cerbero controlada cuando falta autoridad. Fuera: migración process→códice (ABSTRACT-02).

## 2. Hallazgo I7 (cerrado)

```text
feature.md (requires_capability: proc:git-sync)
  → capability_di_resolver::resolve_phase_bindings
  → phase_with_effective_delegates  ⇒  delegates_to: [skill:git-manager]
  → is_workspace_init_phase(true)
  → workspace_init::run
```

| Veredicto | Detalle |
|-----------|---------|
| **No es bug de ignición** | DI sintetiza `delegates_to`; detector opera sobre fase **efectiva** |
| **Fragilidad** | Con `SDDIA_LAB_SKIP_CAPABILITY_DI=1` no hay síntesis → no hay handler nativo |
| **Deuda** | Git solo se salta con `SDDIA_LAB_SKIP_GIT` (lab), no por perfil/códice |

## 3. Laudos Dedalo

| Ref | Pregunta | Laudo |
|-----|----------|-------|
| **D1** | ¿Nuevo process / migrar feature.md? | **No** en este PR |
| **D2** | ¿Perfil activo? | Archivo instancia `.SddIA/active-domain-profile.json` + override input `execution_profile` / `active_codex_ref` |
| **D3** | ¿Git gate? | `workspace_init::run` consulta `git_required`; lab env permanece como override |
| **D4** | ¿Detector? | Endurecer: además de `delegates_to∋skill:git-manager`, aceptar `requires_capability∋proc:git-sync` **o** `resolved_provider` post-DI |
| **D5** | ¿AC-BOOT vía Telegram/Manual? | Esos ECST **no** pasan por `workspace_init`; AC-BOOT/WSINIT = harness `feature`/`workspace_init` con `git_required:false`. AC-DENY = Cerbero DI RBAC sin `source-control` |
| **D6** | ¿Forjar códice software/PA? | **No**; perfil puede referenciar slug existente o `null` + flags |
| **D7** | ¿Mutar genoma norms/codexes? | **Mínimo**: solo motor `execute-process` + fixture/docs en `persist_ref`. Sin `entity-manager` salvo que Tekton demuestre necesidad de norma indexada |

## 4. Contrato `active-domain-profile`

### 4.1 Ubicación

| Fuente | Precedencia |
|--------|-------------|
| Input proceso `execution_profile` (objeto) | 1 (máxima) |
| Input `active_codex_ref` + defaults | 2 |
| Instancia `.SddIA/active-domain-profile.json` | 3 |
| Default | `git_required: true` (compat software-first) |

Path instancia resuelto relativo al repo root (cieguera: no hardcodear fuera de convención `.SddIA/` ya usada por `eda_instance` / workspaces).

### 4.2 Schema JSON (mínimo)

```json
{
  "codex_slug": "codex-frontend-product-splus | null",
  "codex_uuid": "uuid | null",
  "git_required": false,
  "allowed_policies": ["ecosystem-evolution", "filesystem-ops", "knowledge-management"]
}
```

- `git_required: false` → `workspace_init` **no** invoca `git-manager` (equivalente productivo a skip-git sin depender del env lab).
- `git_required: true` o ausente → comportamiento actual (fetch/checkout/pull) salvo `SDDIA_LAB_SKIP_GIT`.
- Resolución de slug contra `directories.library_codexes` es **opcional en MVP** (validación soft: warn/log si slug ausente; no abortar arranque). Autoridad dura = Cerbero + `allowed_policies` en fases DI.

### 4.3 Precedencia Git

```text
SDDIA_LAB_SKIP_GIT=1  →  skip (lab)
else execution_profile.git_required == false  →  skip
else active-domain-profile.git_required == false  →  skip
else  →  git sync
```

## 5. Cambios de motor

### 5.1 `workspace_init.rs`

1. **Detector:** `is_workspace_init_phase` verdadero si:
   - nombre de fase = `Inicialización de Espacio de Trabajo`, y
   - process ∈ {feature, bug-fix, refactorization}, y
   - task name resoluble, y
   - (`delegates_to`∋`skill:git-manager` **∨** `requires_capability`∋`proc:git-sync` **∨** `resolved_provider` contiene `skill:git-manager`).
2. **`run`:** leer perfil; si git no requerido → `git_steps` con `skipped` + `reason: profile_git_not_required`; seguir creando `objectives.md`.
3. Tests unitarios: (a) ciego `proc:git-sync` sin delegates estáticos; (b) perfil `git_required:false` sin env lab; (c) default git on.

### 5.2 Lector de perfil

Módulo pequeño (p.ej. `domain_profile.rs` bajo `engine/`) o fn en `workspace.rs`:

- `resolve_execution_profile(repo, inputs) -> ExecutionProfile`
- Sin mutación de genoma; solo lectura FS instancia + inputs.

### 5.3 AC-DENY

Sin código nuevo de Cerbero: reutilizar `cerbero_di_rbac` — fase con binding a `skill:git-manager` y `allowed_policies` sin `source-control` → `CERBERO_RBAC_DENIED` + DLQ, sin panic. Smoke: test existente o harness documentado en `execution.md`.

## 6. Fuera de alcance (reafirmado)

- Migrar `feature`/`bug-fix`/`refactorization` a códice (ABSTRACT-02)
- Alta `codex-personal-assistant` / nuevos ECST Email/Prompt
- Vaciar `SddIA/process/`
- Reescritura de watchers Telegram/Kalma2
- Derogar `SDDIA_LAB_SKIP_GIT`

## 7. Criterios de aceptación (mapeo)

| AC | Evidencia |
|----|-----------|
| **AC-WSINIT** | Test/run: perfil `git_required:false` → skip git **sin** `SDDIA_LAB_SKIP_GIT` |
| **AC-BOOT** | Mismo run materializa `objectives.md` / success fase init |
| **AC-CODEX** | Schema + lector documentados; slug opcional resuelto vía Cúmulo path |
| **AC-DENY** | Cerbero DI deny sin panic (test o smoke) |
| **AC-BUILD** | `cargo build -p execute-process --release` (o workspace release) OK |
| **AC-DOC** | Cascada + PBI `done/` + `pbi_archived: true` |

## 8. Handoff Tekton

Implementar §5 en `SddIA/engine/execute-process`; fixtures bajo `persist_ref` (`_smoke-*.json` si aplica); `implementation.md` + `execution.md`; no tocar ABSTRACT-02.
