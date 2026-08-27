## Summary
- Rehabilita `accept-pr` en Cerbero/Radamanto (ola A1 Yunque Rúnico) tras re-revocación post-#200 / PPR #203.
- Corta re-muerte: fail_soft `"Sincronización y Limpieza"` post-`merge_commit_hash` (ola A2 motor).
- Cascada documental dual persist_ref + evolution `b7e4a91c-…` + PBI archivado en `docs/todos/done/`.

## Test plan
- [x] A1 instancia: `accept-pr` ∉ revoked/permanent · stats `healthy` laudo #203
- [x] A2 motor: `t_a2_sync_*` + regresión `t_a2_seal_*` → 10/10
- [x] Smoke: `accept-pr` exit 0 · sin re-revocación Cerbero
- [x] `validacion.md` APTO · `pbi_archived: true`
- [x] Lateral `refactorization` intacto