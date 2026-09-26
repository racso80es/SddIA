---
feature_name: sddia-installer-v3-io-contract
created: "2026-09-26"
process: refactorization
branch_name: refactor/sddia-installer-v3-io-contract
persist_ref: docs/features/sddia-installer-v3-io-contract
document_id: PBI-ARQUITECTURA-INSTALLER-V3-IO-CONTRACT
---

# Implementación — installer v3 I/O

- `SddIA/scripts/installer/installer_io.py` — estado, envelope, progreso, PTC.
- `SddIA/scripts/installer/installer_io_lib.sh` — integración bash.
- Motor y fachada emiten un envelope; fachada live fusiona verify/eventos vía `merge-facade`.
- Cúmulo `instance.installer_logs`; norma `sddia-installer-contract` 1.2.0.
