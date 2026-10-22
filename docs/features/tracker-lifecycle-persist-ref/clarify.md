---
feature_name: tracker-lifecycle-persist-ref
created: "2026-10-22"
process: feature
purpose: Emisor localizado y traza sin estado done
execution_id: "76a499d1-59eb-4b02-8e18-5586b41190f0"
document_id: PBI-ARQUITECTURA-TRACKER-LIFECYCLE-04
---

# Clarificación — tracker-lifecycle-persist-ref

Init: `execution_id` `76a499d1-59eb-4b02-8e18-5586b41190f0`. Rama `feat/tracker-lifecycle-persist-ref`.

## Decisiones

| ID | Laudo |
|----|-------|
| L-EMIT | El emisor es `emit_work_initiated_fail_soft` en `workspace_init.rs`. Se dispara al entrar en la primera fase `agent:tekton` de `feature`, `bug-fix` o `refactorization`. El payload lleva `branch`, `persist_ref` y, si hay `pbi_ref`, `tracker_ref`. |
| L-CONSUME | El consumidor es `tracker-stamp` (`Work_Initiated` en `event-domain-subscriptions.json`). Comenta la rama y el `persist_ref` y, si hace falta, pasa el issue a `in_progress`. No lo pasa a `done`. |
| L-FILE | `provider: file` corta el sello antes de la cápsula. No hay comentario. |
| L-GENOME | Esos tres ficheros ya hacen el trabajo. Este PBI no los edita. El diff de genoma es cero. |
