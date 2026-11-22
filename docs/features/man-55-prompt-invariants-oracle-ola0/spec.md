---
feature_name: man-55-prompt-invariants-oracle-ola0
created: "2026-11-22"
process: feature
base: main
scope: Invariantes de prompts/handoff (KZ-PRM-01..06) y ratchet de allowlists en sddia-qa (KZ-EST-01)
branch_name: feat/man-55-prompt-invariants-oracle-ola0
persist_ref: docs/features/man-55-prompt-invariants-oracle-ola0
execution_id: "6931ec92-babc-4ef7-a4d1-a28d5148d33c"
tracker_ref: MAN-55
---

# Especificación Técnica — man-55-prompt-invariants-oracle-ola0

## 1. Oráculo primero en Ola 0 (`AC-1` / `K-ORA` / `KZ-PRM-01`)

- **`SddIA/templates/spec-template/spec.md`**:
  - Añadir columna `Oráculo mecánico ejecutable (Ola 0)` a la tabla de Criterios de Aceptación.
  - Exigir captura en rojo (`oracle-red.log` bajo `persist_ref`) antes de autorizar cambios productivos en Ola 1, y prohibir `grep` manual ad-hoc como único método de verificación.
- **`SddIA/library/norms/features-documentation-pattern.md`** (vía `entity-manager` `norm`):
  - Incorporar regla explícita en `spec.md` / `plan.md`: cada `AC-1..AC-N` debe declarar su oráculo mecánico ejecutable en Ola 0 con captura en rojo previa a Ola 1.
- **`SddIA/agents/dedalo.md`** (vía `entity-manager` `agent`):
  - Exigir que `technical_specification_md` (`spec.md`) y `process_blueprint_md` (`plan.md`) incluyan oráculo mecánico ejecutable por cada AC y fase Ola 0 de captura en rojo.

## 2. Anti-evasión léxica y ratchet de allowlists (`AC-2` / `K-EVA` + `K-RATCH`)

- **`SddIA/tools/sddia-qa/src/gate_allowlist_ratchet.rs`** + **`SddIA/tools/sddia-qa/src/main.rs`**:
  - Nuevo subcomando `sddia-qa gate-allowlist-ratchet [--json] [--base <ref>]` (por defecto `origin/main`).
  - Inspecciona y contabiliza entradas en:
    1. `VIRTUAL_PATH_ALLOW` en `SddIA/engine/execute-process/src/engine/eda_event_token_hygiene.rs`
    2. `SCAN_EXCLUDE` en `SddIA/engine/execute-process/src/engine/eda_event_token_hygiene.rs`
    3. `INTEGRATION_SRC` en `SddIA/tools/sddia-qa/src/gate_eda_unit_no_real_backend.rs`
    4. Arrays de strings en `SddIA/tools/sddia-qa/allowlists/*.toml`
  - Compara cardinalidad actual vs `base` (`git show <base>:<path>`). Si `current > baseline`, falla con diagnóstico salvo que exista token `RATCHET_LAUDO` en un `docs/{features,fixes}/*/clarify.md` modificado/creado respecto a `base`.
  - Tests unitarios: conteo determinista, rechazo de crecimiento sin laudo, aceptación con laudo `RATCHET_LAUDO` en `clarify.md`, y ejecución sobre el repositorio real.
- **`SddIA/norms/external-ai-constraints.md`** y **`SddIA/agents/tekton.md`**:
  - Cláusula explícita anti-evasión léxica (`KZ-PRM-02`): prohibido `concat!`, interpolación dinámica, cambio de prefijo manteniendo `std::fs`, o ampliación de allowlists sin laudo previo en `clarify.md`.

## 3. Baseline hermético Ola 0 y protocolo `entity-manager` (`AC-3` / `K-HERM` + `K-GEN`)

- **`SddIA/library/norms/features-documentation-pattern.md`** (§ `execution.md`) + **`SddIA/norms/external-ai-constraints.md`** + **`SddIA/agents/tekton.md`**:
  - **Baseline Ola 0 (`KZ-PRM-03`):** exigir registro en `execution.md` de: (1) `git status --porcelain` vacío; (2) ausencia física de `.events`, `orchestration`, `telemetry`, `_lab` en raíz; (3) estado de `systemctl --user list-units 'sddia-*'`; (4) baseline de tests con `env -u SDDIA_EVENT_STORE -u SDDIA_EXECUTE_PROCESS_BIN`.
  - **Protocolo `entity-manager` (`KZ-PRM-04`):** mutaciones de genoma exclusivamente vía `./sddia-run.sh --process entity-manager` (`"lifecycle_operation": "update"`), verificando `handoff_hash_signature_new != handoff_hash_signature_old`; prohibido aceptar `idempotent` o editar manualmente `.md` como workaround; revisar frontmatter, tablas y diagramas `mermaid`.

## 4. Paridad build/host y separación Tekton/Argos (`AC-4` / `K-HOST` + `K-ROLE`)

- **`SddIA/norms/external-ai-constraints.md`**, **`SddIA/agents/tekton.md`**, **`SddIA/agents/argos.md`** y **`.cursor/rules/task-closure-documental.mdc`**:
  - **Paridad build/host (`KZ-PRM-05`):** si el diff toca crates Rust que producen o alimentan binarios residentes `sddia-*`, compilar debug y `--release`, reiniciar `systemctl --user restart 'sddia-*@*.service'` y re-ejecutar `cargo test -p execute-process --lib eda_event_token_hygiene` tras el reinicio antes de `delivery-close-cycle` (`D-P6`).
  - **Separación Tekton/Argos (`KZ-PRM-06`):** prohibido recortar ACs o diferir hijos del plan sin `refine-item` previo; prohibido degradar `F3_TECH_GATE` o filas `PARCIAL` a `non_blocking_findings`; Tekton jamás escribe `global: APTO` en `validacion.md` (exclusivo de Argos / verificación determinista).

## 5. Oráculos mecánicos por AC (Ola 0)

| AC | Oráculo mecánico ejecutable (Ola 0) |
|----|-------------------------------------|
| **AC-1** | `cargo test --manifest-path SddIA/Cargo.toml -p sddia-qa gate_prompt_invariants::tests::prompt_invariants_ac1_oracle_ola0_present` |
| **AC-2** | `cargo test --manifest-path SddIA/Cargo.toml -p sddia-qa gate_allowlist_ratchet::tests` y `SddIA/target/debug/sddia-qa gate-allowlist-ratchet --json` + `gate_prompt_invariants::tests::prompt_invariants_ac2_anti_evasion_present` |
| **AC-3** | `cargo test --manifest-path SddIA/Cargo.toml -p sddia-qa gate_prompt_invariants::tests::prompt_invariants_ac3_hermetic_and_entity_manager_present` |
| **AC-4** | `cargo test --manifest-path SddIA/Cargo.toml -p sddia-qa gate_prompt_invariants::tests::prompt_invariants_ac4_host_parity_and_role_separation_present` |
