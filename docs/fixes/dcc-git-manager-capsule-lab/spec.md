---
feature_name: dcc-git-manager-capsule-lab
created: "2026-09-04"
process: bug-fix
base: main
scope: ignition-git-manager-elf
version_spec: "1.0.0"
branch_name: fix/ignition-pre-push-guard
persist_ref: docs/fixes/dcc-git-manager-capsule-lab
pbi_ref: docs/todos/pending/[FIX] delivery-close-cycle — Ola 1 cápsula git-manager (ca3d901fdc9a).md
document_id: PBI-FIX-FRACTURE-ca3d901fdc9a-OLA1
fracture_hash: ca3d901fdc9a
incident_ref: "System_Fracture_Detected — ca3d901fdc9a"
laudo: C
---

# Especificación — Ola 1 cápsula `git-manager` en ignición

## Diagnóstico

DCC `21eccbf3` abortó Snapshot final: `cápsula skill 'git-manager' no encontrada bajo SddIA/target`. `start-sddia.sh` `release_pkgs` no incluía el crate. `--seal-capsules` `names` tampoco. Genoma `SddIA/skills/git-manager.md` documenta `cargo build -p git-manager` → `SddIA/target/debug/git-manager`.

## Corrección

1. `-p git-manager` en lote release de `_ensure_orchestrator`.
2. `cargo build -p git-manager` en el lote debug (junto a `execute-process`).
3. Sello aparte: `write_genome: false`, `write_witness: true`, `names: ["git-manager"]`. Prohibido parchear `{name}.md` (DA-2).

## Fuera de alcance

- `sddia-qa` (Ola 2).
- Suprimir `System_Fracture_Detected` por receta de compile (Ola 3).
- Mutar `SddIA/skills/git-manager.md` (`source_sha256` ausente). Con `SDDIA_CAPSULE_ANCHOR=1` el resolver anclado exigiría ese campo; el incidente fue resolución legado (ancla unset) → `NotFound`.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| DCC-GM-CA1 | ELF ejecutable `SddIA/target/release/git-manager` o `debug/` |
| DCC-GM-CA2 | Orquestador resuelve `git-manager` sin `no encontrada bajo SddIA/target`; `status` vía `--tool` |
| DCC-GM-CA3 | DCC: `failed_phase` ≠ Snapshot final por esa traza. Si Ola 2 abierta, fallo permitido en aduanas `sddia-qa` |
