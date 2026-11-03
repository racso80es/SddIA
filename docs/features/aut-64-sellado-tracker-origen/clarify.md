---
feature_name: aut-64-sellado-tracker-origen
created: "2026-11-03"
purpose: decision
process: refactorization
branch_name: refactor/man-26-ciclo-vida-software
persist_ref: docs/features/aut-64-sellado-tracker-origen
pbi_ref: AUT-64
execution_id: "exec-aut-64-20261103"
---

# Clarificaciones — AUT-64

## Decisiones

1. El sellado tracker (estado Linear + comentario idempotente) ocurre en el proceso de origen (`forge-pbi`, `forge-hu`, `refine-item`, `refine` legacy), no en el suscriptor asíncrono `tracker-stamp`.
2. Los eventos `PBI_Forged`, `HU_Forged`, `PBI_Refined`, `HU_Refined` dejan de tener suscripción EDA a `tracker-stamp`; el handler conserva no-op defensivo si aún llegan por bus.
3. Sumidero `file`: paridad vía `stamp_file_sink_event` dentro de `seal_*_at_origin`.
4. Fallos de sync remoto: fail-soft (`Tracker_Sync_Failed`) en origen; no bloquean emisión de dominio salvo error duro no prefijado con `warn:`.
