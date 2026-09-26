---
feature_name: sddia-installer-v3-io-contract
created: "2026-09-26"
process: refactorization
version_spec: "1.0.0"
pbi_ref: docs/todos/pending/[ARQUITECTURA] Installer v3 — contrato de entrada-salida del comando.md
document_id: PBI-ARQUITECTURA-INSTALLER-V3-IO-CONTRACT
---

# Especificación — installer v3 I/O

SSOT: PBI `PBI-ARQUITECTURA-INSTALLER-V3-IO-CONTRACT` y historia `HU-INSTALLER-V3-IO-CONTRACT-UX` §4.

- Entrada: argv + `--request-file` / stdin JSON / `SDDIA_CAPSULE_REQUEST`.
- Salida: `capsule-json-io` 2.0, `meta.entityId=sddia-installer`.
- Códigos 1–7 según tabla `error.code` del PBI.
- Invariantes I-DEP-SINGLE-STDOUT, I-UX-NOPROMPT-MOTOR en norma 1.2.0.
