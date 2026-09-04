---
feature_name: dcc-sddia-qa-lab
created: "2026-09-04"
process: bug-fix
base: main
scope: ignition-sddia-qa-elf
version_spec: "1.0.0"
branch_name: fix/ignition-pre-push-guard
persist_ref: docs/fixes/dcc-sddia-qa-lab
pbi_ref: docs/todos/pending/[FIX] delivery-close-cycle — Ola 2 sddia-qa (ca3d901fdc9a).md
document_id: PBI-FIX-FRACTURE-ca3d901fdc9a-OLA2
fracture_hash: ca3d901fdc9a
incident_ref: "System_Fracture_Detected — ca3d901fdc9a"
laudo: C
---

# Especificación — Ola 2 `sddia-qa` en ignición

## Diagnóstico

DCC `ad403cf3` / `21eccbf3`: Aduana evolution e integridad índices fallan con `sddia-qa no encontrado`. `resolve_sddia_qa_bin` y `hook_common.resolve_sddia_qa` solo aceptan ELF bajo `SddIA/target/{debug,release}/sddia-qa`. Ignición no construía el crate.

## Corrección

1. `-p sddia-qa` en `release_pkgs` de `_ensure_orchestrator`.
2. Mismo crate en el lote debug (`execute-process`, `git-manager`, `sddia-qa`).
3. Sin `--seal-capsules`: no hay `{name}.md` de tool; el resolver de aduana no usa ancla de cápsula. DA-2: no forjar genoma.

## Fuera de alcance

- `git-manager` (Ola 1, cerrada).
- Suprimir fractura por binario ausente (Ola 3).
- Mutar `hook_common.sh` paths.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| DCC-QA-CA1 | ELF `SddIA/target/release/sddia-qa` o `debug/` ejecutable |
| DCC-QA-CA2 | `sddia-qa gate-evolution --json --range --if-touched --sync-base` arranca (no receta «no encontrado») |
| DCC-QA-CA3 | DCC: Aduana evolution e integridad índices sin traza `sddia-qa no encontrado` |
