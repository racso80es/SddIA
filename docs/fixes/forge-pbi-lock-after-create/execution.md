---
feature_name: forge-pbi-lock-after-create
created: "2026-10-30"
process: bug-fix
branch_name: fix/forge-pbi-scan-sin-ledger
persist_ref: docs/fixes/forge-pbi-lock-after-create
pbi_ref: AUT-29
tracker_ref: AUT-29
execution_id: "b7f670f6-35d3-411b-9e70-9fb68fb4fc61"
items_applied:
  - forge_key-ledger-idempotency
  - pbi_forged_pending-on-store-lock
  - lab-native-linear-tracker-adapter
  - lab-native-required-no-wasm-fallback
  - retry_without_ledger_reuses_child_by_forge_key
---

# Ejecución — seguimiento AUT-29

1. Rama `fix/forge-pbi-scan-sin-ledger` desde `main` (`4ef0d08c`).
2. Parche: scan por `forge_key` sin ledger, mock lab con `description`/`children`, cápsula nativa de `linear-tracker-adapter` bajo `SDDIA_LAB_MOCK_OUTBOUND`.
3. `cargo test -p execute-process forge_pbi --lib -- --test-threads=1` — reclamado 14/14 OK (Tekton; **no** reafirmado por Argos ~13:16 — peaje Shell muerto).
4. `cargo test -p execute-process core_project_slug_allowed --lib` — reclamado 1/1 OK (idem; sin stdout Argos).
5. Alta de evolution y PR de seguimiento. AUT-29 ya está mergeado; este corte no reabre el issue.

Argos ~13:16+01 (Verificación, inyección `?`): Shell+Task peaje muerto → `validacion.md` `NO_APTO` / `integridad_fallida`. Sin reafirmación de §3–4.

## Tekton 13:17+01 (kalma2; inyección `process/phase=?`)

Peaje Shell IDE Rejected ×2; Task `best-of-n-runner` `shell_ok: false`; MCP sddia ausente. **No** re-ejecutado cargo ni `git-manager` este turno — claims §3–4 **no** reafirmados aquí. Grep estático: `retry_without_ledger_reuses_child_by_forge_key` + lab-nativo presentes. Veredicto turno: **blocked**.
