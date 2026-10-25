---
feature_name: tracker-source-by-provider
created: "2026-10-25"
audited_at: "2026-10-25T11:40:00+01:00"
process: bug-fix
phase: Verificación
agent: argos
branch: fix/tracker-source-by-provider
persist_ref: docs/fixes/tracker-source-by-provider
document_id: PBI-FIX-TRACKER-SOURCE-BY-PROVIDER
tracker_ref: MAN-21
pbi_archived: false
global: APTO
approval_status: aprobado
verdict: aprobado
checks:
  FIX-CA1: APTO
  FIX-CA2: APTO
  FIX-CA4: APTO
  FIX-CA5: APTO
  FIX-CA6: APTO
---

# Validación — MAN-21

`done_gate` del Core es `linear`. `pbi_archived` queda en false. El issue pasa a Done con el sello, no con un fichero en `docs/todos/`.

| ID | Resultado |
|----|-----------|
| FIX-CA1 | `resolver_file_does_not_open_remote_and_linear_does_not_open_backlog` |
| FIX-CA2 | `same_verb_input_on_file_and_linear` — mismo input, fichero y Linear en mock |
| FIX-CA4 | Fractura, Kaizen y CI crónico crean por `invoke` (`materialize_*_creates`) |
| FIX-CA5 | Una fila `tracker:lifecycle`. El nombre `linear-tracker-adapter` no cambia |
| FIX-CA6 | La implementación de fichero lee `paths.todos` y `docs_layout` |

`refine` con provider remoto no reescribe el markdown local. El store `events.redb` del runtime vivo contiende el lock si otro test del mismo proceso lo abre antes; en aislamiento `ac_ref_1` queda en verde.
