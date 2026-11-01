---
feature_name: refine-item-mayeuta-envelope
created: 2026-11-01
process: bug-fix
branch_name: Fix_Events_Multiple
persist_ref: docs/fixes/refine-item-mayeuta-envelope
tracker_ref: AUT-37
execution_id: 1f6f7316-ed62-4ded-922a-6b95fbb9d44a
global: APTO
pbi_archived: false
branch: Fix_Events_Multiple
approval_status: aprobado
verdict: aprobado
checks:
  AUT37-T1-ENTRY-ERROR: APTO
  AUT37-T2-TRANSCRIPT: APTO
  AUT37-T3-EVOLUTION: APTO
git_changes:
- SddIA/engine/execute-process/src/engine/agent_runtime.rs
- SddIA/scripts/tools/kalma2-agent-runtime-cursor.py
- SddIA/scripts/tools/test_kalma2_runtime_timeout.py
- SddIA/evolution/c3d4e5f6-a7b8-4901-8c2d-3e4f5a6b7c8d.md
- docs/fixes/refine-item-mayeuta-envelope/
- docs/audits/AUT-37.md
- .SddIA/state/tracker-outbound/AUT-37.jsonl
validated_base_sha: 2d14642644f1acd7ebfa1fa32bac0f941f817736
validated_head_sha: 62a62d41e1a14d68657d134d307e683f5f61c0e3
validated_tree_sha: 072fba07ffd09bcac5c848f04a0f87cdda7d6ecc
diff_digest: sha256:c0b4d4f5ea07ed2983bcd69e199a4ec738633dcd36ba72ab4163d223ace5ffb2
---

# Validación — AUT-37

## Veredicto

**APTO** — `data.message` se propaga a `entry.error` en `blocked`/`failed`; el fallo del CLI escribe `transcript.md` sin `persist_ref`; tests de regresión en Rust y Python.

## Evidencia

| Check | Medio | Resultado |
|-------|--------|-----------|
| T1 | `cargo test -p execute-process agent_runtime::tests --lib` | 24/24 OK |
| T2 | `test_kalma2_runtime_timeout.py` (transcript) | 3/3 OK |
| T3 | Nota `SddIA/evolution/c3d4e5f6-…` | OK |

## Cierre tracker

Provider `linear`: **AUT-37** → Done con el PR que incluya este diff; sin fichero en `docs/todos/done/`.