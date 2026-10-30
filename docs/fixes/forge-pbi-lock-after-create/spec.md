---
feature_name: forge-pbi-lock-after-create
created: "2026-10-30"
process: bug-fix
base: main
branch_name: fix/forge-pbi-scan-sin-ledger
persist_ref: docs/fixes/forge-pbi-lock-after-create
pbi_ref: AUT-29
tracker_ref: AUT-29
execution_id: "b7f670f6-35d3-411b-9e70-9fb68fb4fc61"
---

# Spec — forge-pbi tras lock de events.redb (AUT-29)

## Hecho

`forge-pbi` crea la issue y después emite `PBI_Forged` con `?`. Si `events.redb` responde lock (`Database already open` / `Cannot acquire lock`), el proceso falla sin `tracker_ref` ni `document_id`. Un segundo `forge-pbi` del mismo `raw_idea` vuelve a llamar a `create_issue`.

La autoridad de dominio con `project_slug=core` queda cubierta por `.SddIA/active-domain-profile.json` (AUT-33). Este fix no la reabre. El título de la issue sigue en AUT-28.

## Corte

1. Identidad estable `forge_key` (sha256 de `project_slug`, `parent_ref`, `process`, `raw_idea`) en el frontmatter de la descripción.
2. Antes de `create_issue`, reutilizar issue previa: ledger `.SddIA/state/forge-pbi/{forge_key}.json` en la raíz del proyecto, o hija del padre cuya descripción lleve el mismo `forge_key`. El mock inline (`lab_state`) persiste `description`, `children` y `update_issue_description`. Con `SDDIA_LAB_MOCK_OUTBOUND=1`, `invoke_tool_for_process` usa el binario **nativo** de `linear-tracker-adapter` para que `fetch_issue` lea `.SddIA/lab-linear-store.json` (evita WASM desfasado que caía al fixture sin hijos). Test `retry_without_ledger_reuses_child_by_forge_key`: primer alta, borrar solo el ledger, segundo `forge-pbi` → mismo `tracker_ref` y un solo `create_issue`.
3. El ledger se escribe en cuanto hay `tracker_ref`, antes de emitir el evento.
4. Si la emisión de `PBI_Forged` falla por lock y ya hay `tracker_ref`, el envelope es éxito con `document_id`, `tracker_ref` y `pbi_forged_pending: true`. El evento queda para el reintento (mismo `raw_idea`).
5. Otro error de emisión, con issue ya creada, devuelve fallo pero conserva `tracker_ref` y `document_id`.
6. Sin `tracker_ref` (padre ausente), el lock sigue tumbando la emisión: no hay alta que preservar.

## Verificación

```bash
cd SddIA && cargo test -p execute-process forge_pbi --lib -- --test-threads=1
```

`SDDIA_LAB_SIMULATE_STORE_LOCK=1` solo en el test del corte 4: sintetiza el lock sin tocar el `events.redb` vivo.
