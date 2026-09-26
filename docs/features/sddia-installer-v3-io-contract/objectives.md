---
feature_name: sddia-installer-v3-io-contract
created: "2026-09-26"
process: refactorization
version_objectives: "1.0.0"
pbi_ref: docs/todos/pending/[ARQUITECTURA] Installer v3 — contrato de entrada-salida del comando.md
document_id: PBI-ARQUITECTURA-INSTALLER-V3-IO-CONTRACT
---

# Objetivos — sddia-installer-v3-io-contract

Un único envelope JSON en stdout por invocación; `result.plan` conserva los 14 campos del dry-run v2; errores tipados; progreso JSONL; logs en `.SddIA/logs/installer/`; smoke y CI verdes.
