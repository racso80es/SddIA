---
feature_name: kalma2-bridge-aiua-interact-stale-elf
created: "2026-09-09"
process: bug-fix
phases:
  - verify-live-organ
  - document-implementation-execution
  - evolution-register
  - archive-pbi-validacion
  - delivery-close-cycle
  - confirm-ci
  - accept-pr
branch_name: fix/kalma2-bridge-aiua-interact-stale-elf
persist_ref: docs/fixes/kalma2-bridge-aiua-interact-stale-elf
execution_id: "4ccdf721-f1d4-40e1-b7ca-40002e9d5da5"
---

# Plan — sello documental ELF `kalma2-bridge`

Blueprint Tekton. Cero mutación de `SddIA/interfaces/kalma2-bridge/src/main.rs` y `interfaces/kalma2/app.js`.

## Fase 0 — Diseño (hecho, Dedalo)

- `spec.md` + este `plan.md` bajo `persist_ref`.
- Init: `execution_id` `4ccdf721-f1d4-40e1-b7ca-40002e9d5da5`; rama `fix/kalma2-bridge-aiua-interact-stale-elf`.
- Commit de diseño vía `skill:git-manager`. Working tree sucio (`main.rs`, `app.js`, `ecosystem-health.json`) **fuera** del commit.

## Fase 1 — Verificar órgano vivo

```bash
UNIT=sddia-kalma2-bridge@home-racso-Proyectos-SddIA.service
PID=$(systemctl --user show -p MainPID --value "$UNIT")
ls -l --time-style=full-iso "/proc/$PID/exe" SddIA/target/release/kalma2-bridge
strings SddIA/target/release/kalma2-bridge | grep -E "aiua/intera|handle_aiua_interact"
```

Probe HTTP (predicado ≠404, timeout 120 s):

```bash
curl -sS -i -m 120 -X POST "http://127.0.0.1:8765/api/aiua/interact" \
  -H "Content-Type: application/json" \
  -d '{"prompt":"prueba de latido sddia"}'
```

No-regresión: `POST /api/chat` (no exigir combustión Gemini). Resolvedor al spawn:

```bash
bash -c 'source SddIA/scripts/common/sddia_shell_lib.sh && _sddia_resolve_daemon_binary "$(pwd)" kalma2-bridge'
```

Rebuild+restart **solo** si ELF < fuente o MainPID ausente. Si el órgano cumple: no tocar runtime.

## Fase 2 — Documentación de ejecución

`implementation.md` + `execution.md`. Registrar PID/mtime/probe. Incluir `docs/audits/kalma2-wui-tormentosa-chat-20260909.md` si permanece untracked (evidencia de origen).

## Fase 3 — Evolution

`sddia-qa evolution-register` (`alta` o `modificacion` según cápsula). Paths: persist_ref, PBI, auditoría.

## Fase 4 — Cierre documental en rama

- PBI pending → `docs/todos/done/` (mismo `document_id`).
- `status: cerrado` + `fix_ref` de este persist_ref.
- `validacion.md`: checks CA-1…CA-6; CA-CI = `PENDIENTE-CI` hasta run verde (norma v1.2.1: no `global: APTO` con CA-CI sin `run_id`).
- Tras CI verde: actualizar `validacion.md` (`global: APTO`, `ci_run_id`) en la **misma rama** antes de `accept-pr`.

## Fase 5 — Entrega

`./sddia-run.sh --process delivery-close-cycle` con `source_process: bug-fix`, `persist_ref`, `branch_name`. Git vía `skill:git-manager`. Fire-and-forget tras acuse JSON (DA-5). No incluir dirty `main.rs`/`app.js`.

## Fase 6 — CI post-PR

Un chequeo de checks del PR (no bucle). Si rojo: parche local + un push (DA-6). Si verde: sellar CA-CI y seguir a Fase 7.

## Fase 7 — accept-pr

`./sddia-run.sh --process accept-pr` con `source_branch: fix/kalma2-bridge-aiua-interact-stale-elf`. Solo con CA-CI APTO.

## Orden

```text
spec/plan (Dedalo, commit de diseño)
  → Fase 1 verify live (rebuild iff fósil)
    → Fase 2 implementation.md + execution.md + auditoría
      → Fase 3 evolution-register
        → Fase 4 validacion.md + archivo PBI
          → Fase 5 delivery-close-cycle → PR
            → Fase 6 CI run_id verde
              → Fase 7 accept-pr
```

## Fuera de este plan

Sanitizar 503 en epidermis; failover multi-LLM; fractura `mayeuta-llm` `64f37c7f7b34`; mutar dispatcher Rust.
