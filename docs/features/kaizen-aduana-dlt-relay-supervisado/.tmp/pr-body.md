## Summary
- Centinela `iota-publish-relay` (supervisor Rust + hijo Node)
- Ignición L-REQUIRED con `/health`; causa real en `batch-anchor-failed:` y fractura×1 por lote
- Cola `.SddIA/dlt/reanchor-queue`; rescate Merkle 28 eventos (ventana 2026-08-25..27)
- PBI archivado; `validacion.md` global APTO

## Test plan
- [x] Smokes DLT-CA1..CA10 documentados en `validacion.md`
- [x] `cargo test -p execute-process batch_anchor`
- [ ] CI GitHub Actions