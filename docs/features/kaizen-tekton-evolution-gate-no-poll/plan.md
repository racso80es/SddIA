---
feature_name: kaizen-tekton-evolution-gate-no-poll
created: "2026-08-27"
process: feature
phases: "T0-docs,T1-tests-capsule,T2-fidelity,T3-rehash,T4-format-verdict,T5-all-ci,T6-fossils,T7-hook,T8-da6-contract,T9-aduana-doc"
uuid: "07dc027a-fdb5-487a-9fea-1a5dd67d38ca"
persist_ref: docs/features/kaizen-tekton-evolution-gate-no-poll
branch_name: feat/kaizen-tekton-evolution-gate-no-poll
execution_id: "96471044-003a-457a-bf59-041e94053b12"
dedalo_verdict: ok
---

# Plan — kaizen-tekton-evolution-gate-no-poll

Orden: tests de semántica **antes** de cablear hooks/CI bloqueante (R3). Saneamiento **antes** de `--all` en el workflow (R2). Un PR.

## T0 · Documentación (esta entrega)

- `clarify.md`, `objectives.md`, `spec.md`, `plan.md` bajo `persist_ref`.
- Topología DA-4 activa. Tekton no arranca en T0.

## T1 · Cápsula: formato, newline, L-SELF

Crate `sddia-evolution-register`:

1. Tests T-FMT, T-NL, T-DELTA (rojo → implementación T4).
2. `canonical_hash` intocado; T-NL documenta el comportamiento vigente.

`entity-manager` update skill al cerrar T3/T4 (operation `rehash`, `audit`). No bisturí del `{name}.md` antes del creator.

## T2 · Fidelidad de captura

1. `parse_frontmatter_from_str` en `parser.rs` + test.
2. `read_blob(repo, rel, rev: &str)`; `build_registry` parsea fm del `raw`.
3. Test T-FIDEL (tmp git): HEAD bueno / WT malo y viceversa.
4. `run_gate`: `--range` → `HEAD`; default → `":"`.

No tocar `pre_commit_gate.sh` salvo que el binario nuevo rompa el JSON (no debería).

## T3 · `evolution-rehash`

1. Cápsula `operation: rehash` (L-REHASH-SURGICAL).
2. `sddia-qa`: subcomando, `--id`, `--json`, `--dry-run`; persistencia solo detalle.
3. Test T-REHASH.
4. Forja contrato skill (`entity-manager` update).

## T4 · Formato + `verdict` delta

1. Regex antes de recompute (L-FORMAT-FIRST).
2. L-DELTA-VALIDATE: correlators validados aunque `material` vacío.
3. Tests T-FMT / T-DELTA verdes.

## T5 · `--all` (código, CI aún no bloquea)

1. Flag `--all`; `request.audit: "universe"`.
2. Enumeración UUID vía Cúmulo; blobs HEAD.
3. Test T-ALL.
4. **No** añadir el step CI hasta T6.

## T6 · Fósiles + CI universo

1. `evolution-rehash` de los 4 UUID (spec §10).
2. Verificar `gate-evolution --all` exit 0 en local.
3. Step `evolution gate (universe)` en `sddia-index-qa.yml` **en el mismo PR**.

## T7 · Pre-push

1. `--if-touched` en `run_gate`.
2. `pre_push_gate.sh`: invocación **antes** de `invoke_process route-domain-event`.
3. Verificación K-LOCAL: rama tmp con registro corrupto en el rango → push bloqueado (lab; no `SDDIA_SKIP_HOOKS`).

## T8 · Norma y contrato

1. `evolution_contract.md` 1.1.2 (semántica hash; `--all`; `evolution-rehash`; veto ad hoc).
2. `./sddia-run.sh --process entity-manager` update `norm` `external-ai-constraints` v1.6.0 (DA-6).
3. Extender `.cursor/rules/tekton-fire-and-forget.mdc`.

## T9 · Cierre documental

- `implementation.md` / `execution.md`.
- `validacion.md` APTO, `pbi_archived: true`.
- Mover PBI a `docs/todos/done/`.
- Evolution register del propio hito vía `evolution-register` / cápsula (hash real, no placeholder).
- `delivery-close-cycle` (Tekton + Racso).

## Dependencias

```
T0
 └─ T1 (tests rojos)
      └─ T2 fidelidad
           └─ T3 rehash
                └─ T4 formato/verdict (T1 verde)
                     └─ T5 --all código
                          └─ T6 fósiles + CI
                               ├─ T7 hook
                               └─ T8 norma
                                    └─ T9 cierre
```

T7 puede paralelizarse con T8 tras T5 (hook no exige `--all`). T6 **antes** de merge.

## Prohibido en ejecución

- Recalcular hashes con `sha256sum` / Python.
- `sleep` / `gh pr checks` / `gh run rerun` mismo SHA (DA-6, anticipado).
- Activar `--all` CI con fósiles vivos.
- Rule Cursor nueva.
