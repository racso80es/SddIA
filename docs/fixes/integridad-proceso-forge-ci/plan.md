---
feature_name: integridad-proceso-forge-ci
created: "2026-08-29"
process: bug-fix
branch_name: fix/integridad-proceso-forge-ci
persist_ref: docs/fixes/integridad-proceso-forge-ci
pbi_ref: docs/todos/pending/[FIX] Integridad de proceso — parse_frontmatter ciego, hash forge divergente y aduana CI opaca.md
document_id: PBI-FIX-INTEGRIDAD-PROCESO-FORGE-CI
uuid: 4d417acc-bb85-46ea-ab0e-20c017d16d6f
phases:
  - l1-test-parse-frontmatter-embedded-delimiter
  - l2-test-forge-hash-phases-parity
  - l3-ci-job-rename-sddia-index-integrity
  - l4-dcc-aduana-integridad-indices
  - l5-optional-workspace-template
  - smoke-ca4-block-before-push
---

# Plan — Integridad de proceso (forge, CI, DCC)

Orden de Ejecución: L1 → L2 → L3 → L4 → L5 (opt) → smoke CA4. Este commit **solo** sella Diseño (`spec.md` + `plan.md`). Cierre documental y `delivery-close-cycle` **después** de Ejecución/Argos.

## Fase L1 — Test CA1

Archivo: `SddIA/engine/execute-process/src/forges/common.rs` (`#[cfg(test)]`).

1. Tempdir + `.md` con `workspace_template` terminado en `/---` pegado (réplica DCC).
2. `uuid` y `hash_signature` no nulos tras `parse_frontmatter`.
3. Comando: `cd SddIA && cargo test -p execute-process parse_frontmatter -- --nocapture` (filtrar nombre del test).

No tocar `core/parser.rs` salvo test gemelo si se quiere paridad Core; el contrato del PBI es el wrapper de forja.

## Fase L2 — Test CA2

Archivo: `SddIA/engine/execute-process/src/forges/factory.rs` tests (o `common.rs` si `run_process_forge` se importa desde ahí).

1. Fixture process en tmp **dentro de un repo mínimo** si `locate_existing_process_path` lo exige; si no, path inyectable.
2. `run_process_forge(repo, { process_name, lifecycle: update, markdown_body_replacements: [{from, to}] })`.
3. Leer `hash_signature` escrito; igualar a `sha256_phases_integrity` del `phases` parseado.
4. Comando: `cargo test -p execute-process` acotado al test nuevo.

## Fase L3 — Workflow (CA3)

Archivo: `.github/workflows/sddia-index-qa.yml`.

1. Clave job L19: `verify-tools-index` → `sddia-index-integrity`.
2. Steps intactos.
3. Mencionar en `implementation.md` / cuerpo de PR: actualizar required checks de `main`.

## Fase L4 — Genoma DCC + handler (CA4)

1. `./sddia-run.sh --process entity-manager` — insertar fase `Aduana integridad índices` entre `Aduana EDA genómica` y `Publicación remota`. Domain root. Prohibido editar `delivery-close-cycle.md` a mano.
2. Handler `capsule_index_integrity_audit_gate` en `phase_capsules.rs`: `resolve_sddia_qa_bin` → `verify-process-integrity` luego `verify-tools-index` (o flags CLI si existen equivalentes in-process; preferir el mismo binario que CI).
3. `delivery_close.rs`: rama `if phase_name == "Aduana integridad índices"` **antes** de `execute_delivery_close_phase` (el match actual no cubre aduanas Argos). `blocked` aborta el ciclo (no `fail_soft`). `dcc_friction_id` → `F-DCC-INDEX-INTEGRITY`.
4. `residual_runner.rs`: misma rama que Aduana evolution.
5. Test unitario: mock o binario real sobre fixture con hash corrupto → `status: blocked`; no llega a handler de push.
6. Documentar skip `SDDIA_LAB_SKIP_INDEX_INTEGRITY` en `delivery-close-cycle.md` (tabla perfil laboratorio) **vía entity-manager** (body/frontmatter según contrato), no Write.

## Fase L5 — CA5 (opcional)

Mismo `entity-manager` update: `workspace_template` sin `---` terminal. Hash de fases puede no cambiar (campo fuera de `phases`); si el forge bumpa versión, aceptar el sello nuevo.

## Fase smoke CA4

Tras binarios: hash corrupto en un process de prueba **fuera** de genoma productivo, o env que apunte el verificador a fixture. DCC bloquea antes de push. No en este commit.

## Cierre (fuera de esta parada)

`implementation.md` + `execution.md` → Argos `validacion.md` → PBI a `done/` → `delivery-close-cycle`.
