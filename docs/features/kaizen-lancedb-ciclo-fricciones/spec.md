---
feature_name: kaizen-lancedb-ciclo-fricciones
created: "2026-08-31"
process: feature
base: main
scope: dcc-workflow-scope-halt-mayeuta-relacionado-ingest-ci-ca
branch_name: feat/kaizen-lancedb-ciclo-fricciones
persist_ref: docs/features/kaizen-lancedb-ciclo-fricciones
pbi_ref: docs/todos/pending/[KAIZEN] Post-ciclo LanceDB — PAT-workflow, Kintsugi saltado, Mayeuta ciego y correlato evolution.md
document_id: PBI-KAIZEN-LANCEDB-CICLO-FRICCIONES
uuid: "12250eca-49c6-4008-ac50-5c5722a7fe91"
execution_id: "b97c39ce-f5d6-4e26-92c6-68de26eedcf0"
---

# Especificación — kaizen-lancedb-ciclo-fricciones

## Hito 1 — Envelope workflow scope (CA2)

`delivery_close.rs` (paridad `stamp_dcc_network_block` / `stamp_dcc_hook_evol_block`):

- Predicado: traza contiene `without` + `workflow` + `scope` (GitHub: `without \`workflow\` scope`).
- Fase: `Publicación remota`.
- Efecto: `status=blocked`, `friction_id=F-DCC-WORKFLOW-SCOPE`.
- `emit_dcc_phase_fractures` suprime (como DNS / hook-evol).
- Mensaje accionable: unificar credential helper git→`gh` (`gh auth setup-git`); `gh auth refresh -s workflow` solo si git ya delega en gh.

No reimplementar `SDDIA_HOOK_DELIVERY_CLOSE`.

## Hito 2 — Halt (CA2b)

`delivery_close.rs` `run()`: si una fase `Publicación remota` queda `failed` o `blocked`, no ejecutar fases posteriores (`Apertura en forja`, `Sello Presentación ECST`, `Higiene local`). Marcarlas `skipped` con `reason=prior_push_not_ok`. Un solo payload de fractura causal si aplica (workflow-scope no emite).

Aduanas previas (`blocked` evolution/EDA) **no** cambian: ya suprimen fractura y no son este halt.

## Hito 3 — Mayeuta (CA1, CA4)

`analyze_fracture_kaizen`:

1. Regresión: fixtures `without workflow scope` y specimen `Head sha can't be blank` + `Apertura en forja` → sección **sin** «Recursión o re-entrada».
2. Cubo `credential_workflow_scope` si traza contiene workflow-scope.
3. Cubo `remote_branch_absent` si `Head sha can't be blank` o `Head ref must be a branch` (post-push-rejected).
4. Prioridad: estos cubos **antes** del token genérico `failed`→`prompt_adjustment`.

Propuesta: no `Implementar guarda SDDIA_HOOK_DELIVERY_CLOSE`.

## Hito 4 — Normas CA3 (EM)

`external-ai-constraints` y `obediencia-procesos`: DCC `failed`/`blocked` (incl. `F-DCC-WORKFLOW-SCOPE`) = colapso de **credencial** u operación remota; prohibido `git push`/`gh` raw; wait laudo. Prefijo RAW. Prohibido Write directo.

## Hito 5 — Helper relacionado (CA6)

En `sddia-evolution-register` (lib, no `.md` de skill): dado el set de paths del diff, sugerir complementos:

- Si toca `SddIA/**/Cargo.toml` o crate bajo `SddIA/` → incluir `SddIA/Cargo.lock` si está en el diff.
- Si toca `SddIA/infrastructure/adapters/<crate>/` → incluir ficha `SddIA/infrastructure/adapters/<kebab>.md` e `index.md` si están en el diff.
- API: función pura `suggest_relacionado_complements(diff_paths, relacionado) -> Vec<String>` de paths en diff no listados.

Gate: test unitario con fixture lockfile sin path en relacionado → UNREGISTERED; con path → OK.

## Hito 6 — Ingest (CA7)

EM `update` `memory-evolution-ingest`: bump SemVer; intent/cuerpo: persistir vía puerto `EvolutionStore` en `{paths.vectorStore}/lancedb/`; retirar JSON SSOT. `markdown_body_replacements` + `process_phases` intent.

## Hito 7 — Patrón CA8 (EM)

`features-documentation-pattern`: un CA cuyo medio de verificación sea CI/GitHub Actions no puede ser `APTO` sin `run_id`/URL de check verde, o el check queda `PENDIENTE-CI` y entonces `global` no puede ser `APTO` si ese CA es gate.

## Hito 8 — PBI fractura (CA5)

`01c9040df256`: diagnóstico §1+§1b; propuesta Mayeuta marcada errónea; `uuid` si se conserva; archivar a `done/` en el mismo PR.

## Invariantes

| Regla | Motivo |
|-------|--------|
| No reimplementar guarda hook | Ya existe; 0c5268362b9a |
| Halt no aplica a fail-soft post-`pr_url` | L-FAILSOFT-OLA2 |
| Helper no exime `Cargo.lock` del gate | Fuera de alcance del PBI |
| EM para genoma | DA-2 |
