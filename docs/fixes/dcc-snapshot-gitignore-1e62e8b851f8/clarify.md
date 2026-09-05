---
feature_name: dcc-snapshot-gitignore-1e62e8b851f8
created: "2026-09-05"
process: bug-fix
document_id: PBI-FIX-FRACTURE-1e62e8b851f8
---

# Clarify — 1e62e8b851f8

| ID | Laudo |
|----|-------|
| L-SSOT | Traza Cúmulo = gitignore `.dev`. Mayeuta Head sha = cubo ajeno. |
| L-SKIP | Snapshot omite vault `.dev` y `starter-kit/.SddIA`. No `git add -f`. |
| L-GM | `git-manager` commit salta paths ignorados; I/O congelado intacto. |
| L-HEAD | Resolver `HEAD` a rama `*`; hook `symbolic-ref`. |
| L-PR | Mismo PR #262 (bloquea push/DCC de ola 2). |
