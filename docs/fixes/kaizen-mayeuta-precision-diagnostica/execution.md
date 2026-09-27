---
feature_name: kaizen-mayeuta-precision-diagnostica
created: "2026-09-27"
process: bug-fix
branch_name: fix/kaizen-mayeuta-precision-diagnostica
persist_ref: docs/fixes/kaizen-mayeuta-precision-diagnostica
execution_id: "39778d5c-5085-4eb8-b6ad-5860c2b92a33"
---

# Ejecución

```bash
export TMPDIR=/home/racso/Proyectos/SddIA/.tmp
cd SddIA && cargo test -p execute-process --lib
```

Resultado: 509+ tests `execute-process` en verde (incl. `fracture_corpus_regression`).
