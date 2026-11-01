---
feature_name: kaizen-disk-thermodynamics-retention
created: "2026-11-01"
process: feature
purpose: "Decisiones de diseño post-refine-item y post-auditoría código (Fix_Events_Multiple / main)"
tracker_ref: AUT-49
refine_execution_id: "71a4fc2f-1f35-4379-9203-33c0215f0eae"
---

# Clarificación — kaizen-disk-thermodynamics-retention

Refinamiento Mayeuta sellado en workspace `71a4fc2f-…` (`verdict: refined`). Auditoría 2026-11-01 contrastó el refinamiento con `main` + rama `Fix_Events_Multiple` (gate I/O AUT-56 mergeado en main; watcher residual ampliado en rama).

| Afirmación | Verificación | Decisión |
|---|---|---|
| Syslog kaizen no cubre `target` ni `.events` | PR #291 / sibling `kaizen-disk-thermodynamics-syslog` | Se mantiene |
| `ephemeral-cache-purger` jail `/tmp/cursor-sandbox-cache`, veto `/home` | `jail.rs` sin diff desde refinamiento | L-CAPSULE: cápsula nueva |
| Gate «no escribir espejo FS en redb» | Entregado `189dc7dd` (AUT-56) en `main` | **Fuera de AUT-49**; slice B = solo residuo histórico |
| Watcher `F-EDA-RESIDUAL-*` ante JSON en `redb` | `event-watcher` no enruta; emite fractura vía store | Purga FS debe **abstenerse** si watcher vivo; no competir con `routed_ok` en memoria |
| Topología FS `.events` | `load_bus_topology` / `load_eda_bus` defaults + overrides cumulo | Allowlist de purga usa esas APIs, no tablas `eda_bus.store` |
| `dead_letter_index` | Tabla redb | Candidato FS con `event_id` indexado → `kept` |
| Palanca «4 KB por JSON ~1 KB» | Mitigación futura = dejar de acumular residuo + purga; no reabrir emisores | Documentado; no criterio de cierre duplicado |
| Rama de entrega | `feat/kaizen-disk-thermodynamics-retention` desde `main` | No mezclar con `Fix_Events_Multiple` salvo cherry-pick puntual |

## Laudos

| ID | Laudo |
|----|-------|
| L-SLUG | `persist_ref` = `docs/features/kaizen-disk-thermodynamics-retention` |
| L-CAPSULE | Crate `repo-retention-purger`; prohibido mutar jail del purger sandbox |
| L-TWO-PHASE | Plan JSON acusado (`plan_digest`) → apply; abortar apply sin digest |
| L-TARGET | ≤1 generación deps/artefacto; ≤2 incremental/crate; abstención `cargo`/`rustc` |
| L-EVENTS-REDB | Slice B solo `StoreMode::Redb`; `processing/**` excluido |
| L-WATCHER | Apply slice B requiere ausencia de `event-watcher` (proceso) sobre el repo |
| L-NATIVE | `native-debug-bins` fuera de slice A |
| L-CI | KA-RET-8 / APTO solo con CI verde |
