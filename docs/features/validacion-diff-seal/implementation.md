---
feature_name: validacion-diff-seal
created: "2026-10-30"
process: feature
pbi_ref: AUT-39
---

# Implementación

| Touchpoint | Rol |
|------------|-----|
| `SddIA/engine/execute-process/src/engine/diff_seal.rs` | Digest y sello YAML |
| `SddIA/engine/execute-process/src/engine/delivery_close.rs` | Fase DCC + supresión fractura |
| `SddIA/engine/execute-process/src/engine/mod.rs` | Export módulo |
| `SddIA/library/codexes/.../delivery-close-cycle.md` | Fase + v1.6.0 |
