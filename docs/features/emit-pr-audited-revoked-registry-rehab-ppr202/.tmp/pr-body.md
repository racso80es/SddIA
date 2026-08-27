## Summary
- Rehabilita `emit-pr-audited-event` en Cerbero/Radamanto (A1 Yunque Rúnico).
- Restaura emisión `PullRequest_Audited` en aduana `pull-request-review`.
- Cascada documental + evolution `c2e8f4a1-…` + PBI archivado en `docs/todos/done/`.

## Test plan
- [x] A1 instancia: `emit-pr-audited-event` ∉ revoked/permanent
- [x] Smoke: `./sddia-run.sh --action emit-pr-audited-event` → exit 0
- [x] `validacion.md` APTO · `pbi_archived: true`
- [x] Lateral `refactorization` intacto (L-OUT)