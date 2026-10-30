---
feature_name: llm-audit-peaje-toll
created: "2026-10-30"
process: bug-fix
branch_name: fix/llm-audit-peaje-toll
persist_ref: docs/fixes/llm-audit-peaje-toll
tracker_ref: AUT-47
---

# Especificación — peaje auditoría LLM (AUT-47)

## Objetivo

Que cada ciclo `feature` | `bug-fix` | `refactorization` deje `docs/audits/{tracker_ref}.md` con fila `process` o declare el motivo en `thermodynamic_toll.llm_process_peaje`.

## Cambios

1. `tracker_pbi_meta::resolve_tracker_ref_for_lifecycle` — misma cadena que `attach_tracker_ref_from_persist`; `pbi_ref: AUT-n` no se abre como path.
2. `maybe_record_process_peaje` — devuelve JSON observable; sin `let _ =`.
3. Ausencia de `llm_audit` en la descripción del issue = active para peaje; `inactive` → `skipped` / `llm_audit_inactive`.
4. Errores de `fetch_issue` o frontmatter ilegible en el peaje.

## Fuera de alcance

Filas por skill/tool/action; `llm_audit` en `forge-pbi`.
