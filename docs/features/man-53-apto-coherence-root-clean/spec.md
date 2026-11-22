---
feature_name: man-53-apto-coherence-root-clean
created: "2026-11-22"
process: feature
branch_name: feat/man-53-apto-coherence-root-clean
persist_ref: docs/features/man-53-apto-coherence-root-clean
tracker_ref: MAN-53
base: main
scope: "SddIA/engine/execute-process/src/engine/{apto_coherence.rs,pull_request_review.rs,qa_attestation.rs,delivery_close.rs,accept_pr.rs,residual_runner.rs}"
---
# Especificación Técnica — man-53-apto-coherence-root-clean

## 1. `apto_coherence.rs` (Regla C5 + Veto Checks Troncales)

- Nuevas constantes:
  - `pub const FRICTION_AC_PARTIAL: &str = "apto_coherence_ac_partial";`
  - `pub const FRICTION_TRUNK_GATE: &str = "apto_coherence_trunk_gate";`
- `check_c5_ac_matrix(repo, persist)`:
  - Lee `execution.md` y `validacion.md` en `persist_ref`.
  - Extrae filas de matriz AC/CA (tablas Markdown o ítems de lista cuyo identificador sea `AC[-_]?\d+` o `CA[-_]?\d+`).
  - Si alguna celda/token de estado coincide con `PARCIAL`, `FAIL`, `NO_APTO`, `ABIERTO`, `PENDIENTE` (o `PENDIENTE-CI`), retorna `CoherenceFault` con `friction_id = FRICTION_AC_PARTIAL`.
- `check_trunk_gates(validacion_fm)`:
  - Inspecciona `checks:` en el frontmatter de `validacion.md`.
  - Si alguna clave troncal (`F2_DOC_GATE`, `F3_TECH_GATE`, `F4_RBAC_GATE`, `ROOT_TOPOLOGY_CLEAN`, `DOC_HONESTY_*`) tiene valor `NO_APTO`, retorna `CoherenceFault` con `friction_id = FRICTION_TRUNK_GATE`.

## 2. `pull_request_review.rs`, `qa_attestation.rs` y `residual_runner.rs`

- Antes de aprobar en `pull-request-review` (`handoff_accept_pr`, `qa_attestation::try_write_from_ppr` y cierre de `residual_runner` cuando `process_name == "pull-request-review"`), si `persist_ref` está presente, invocar `apto_coherence::validate_apto_coherence(repo, &persist_ref)`.
- Si falla, bloquear la aprobación/atestación/handoff propagando el `CoherenceFault` accionable (sin degradar a `non_blocking_findings`).

## 3. Verificación física de topología efímera en raíz (`qa_attestation.rs`, `delivery_close.rs`, `accept_pr.rs`)

- Helper compartido `root_ephemeral_dir_violations(repo: &Path) -> Vec<String>` sobre `[".events", "orchestration", "telemetry", "pending", "domain", "dead-letter", "progress", "_lab"]`.
- `delivery_close.rs` (`capsule_gate_pre_pr_qa`): además de `worktree_violations_except_validacion`, verifica `root_ephemeral_dir_violations(repo)` y bloquea con `friction_id: "root_topology_dirty"` si hay coincidencias.
- `accept_pr.rs` (`Auditoría Genómica` y `Fusión Soberana`): verifica `root_ephemeral_dir_violations(repo)` y bloquea con error accionable `root_topology_dirty: ...` si hay coincidencias.
