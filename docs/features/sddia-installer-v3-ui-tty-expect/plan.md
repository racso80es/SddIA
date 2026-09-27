---
feature_name: sddia-installer-v3-ui-tty-expect
branch_name: fix/sddia-installer-v3-ui-tty-expect
persist_ref: docs/features/sddia-installer-v3-ui-tty-expect
pbi_ref: docs/todos/pending/[KAIZEN] Installer v3 — smoke TTY presentador (AC-9 y AC-11).md
document_id: PBI-KAIZEN-INSTALLER-V3-UI-TTY-EXPECT
uuid: "5fc72fba-4c31-4c2d-8342-7c84cafb59a3"
---

# Plan

1. `test-sddia-installer-ui-tty.sh` — AC-T1..T3 con `script` + `expect`.
2. Invocación desde `test-sddia-installer.sh` si `expect` y `script` disponibles.
3. CI: `apt-get install expect` en job `sddia-installer-smoke`.
4. Evolution + cierre tras CI verde.
