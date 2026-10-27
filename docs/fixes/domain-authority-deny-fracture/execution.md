---
feature_name: domain-authority-deny-fracture
created: "2026-10-27"
process: bug-fix
branch_name: fix/domain-authority-deny-fracture
persist_ref: docs/fixes/domain-authority-deny-fracture
pbi_ref: AUT-8
execution_id: "383795cc-6dee-478f-bcb1-6cbbc670f10c"
---

# Ejecución

1. `SDDIA_AGENT_RELAY_IDE=1 SDDIA_LAB_ALLOW_DIRTY=1 ./sddia-run.sh --process bug-fix` — workspace-init, rama `fix/domain-authority-deny-fracture`, `pbi_ref: AUT-8`.
2. Parche `emit_fracture_for_denial` + hook en `run_process`.
3. `cargo test -p execute-process domain_authority:: --lib`.
4. Manifiesto: `tracker.project_id` → UUID proyecto Linear SddIA (`359ebe2e-311b-46f9-8623-fa665179bac3`).
5. `sddia-qa evolution-register` alta `6d0e423c-f418-416c-9878-7d19eaf2ee7e`.
6. `delivery-close-cycle` (parcial): snapshot `9611c6b6`, PR https://github.com/racso80es/SddIA/pull/402 (aduana evolution pendiente de evolution en rama).
