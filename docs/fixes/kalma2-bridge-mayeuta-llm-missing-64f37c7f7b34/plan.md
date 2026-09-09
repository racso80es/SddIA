---
feature_name: kalma2-bridge-mayeuta-llm-missing-64f37c7f7b34
created: "2026-09-09"
process: bug-fix
phases:
  - host-release-mayeuta-llm
  - resolver-release-first
  - launcher-preflight-warn
  - hermetic-tests
  - chat-no-regression
  - evolution-register
  - archive-pbi-validacion
  - delivery-close-cycle
  - confirm-ci
  - accept-pr
branch_name: fix/kalma2-bridge-mayeuta-llm-missing-64f37c7f7b34
persist_ref: docs/fixes/kalma2-bridge-mayeuta-llm-missing-64f37c7f7b34
execution_id: "aee88988-ea8f-434d-a0bd-8bc67e5748b9"
---

# Plan — prótesis `mayeuta-llm` (`64f37c7f7b34`)

Blueprint Tekton. Mutación acotada: `resolve_mayeuta_llm`, tests en `kalma2-bridge`, preflight en `kalma2-bridge.sh`. Cero `app.js`. Cero cubo Kaizen (CA5 diferido).

## Fase 0 — Diseño (hecho, Dedalo)

- `spec.md` + este `plan.md` bajo `persist_ref`.
- Init: `execution_id` `aee88988-ea8f-434d-a0bd-8bc67e5748b9`; rama `fix/kalma2-bridge-mayeuta-llm-missing-64f37c7f7b34`.
- Commit de diseño vía `skill:git-manager`. Working tree sucio (`app.js`, `ecosystem-health.json`, instrumentation email-inbox) **fuera** del commit.

## Fase 1 — Host CA1

```bash
cd SddIA && cargo build --release -p mayeuta-llm
test -x target/release/mayeuta-llm && file target/release/mayeuta-llm | grep -q ELF
```

No entra en el PR.

## Fase 2 — Resolver CA2

En `resolve_mayeuta_llm`:

```text
for rel in [
  "SddIA/target/release/mayeuta-llm",
  "SddIA/target/debug/mayeuta-llm",
]
```

Override env sin cambio. Mensaje de error default **idéntico** (sello inmutable).

## Fase 3 — Preflight CA4

Tras resolver `BRIDGE_BIN` y **antes** de `exec`:

1. Candidatos: `$SDDIA_MAYEUTA_LLM_BIN` si set; si no, `SddIA/target/release/mayeuta-llm` luego `debug`.
2. Si ninguno pasa `_sddia_is_native_elf`: `echo "[WARN] mayeuta-llm … Compilar: cd SddIA && cargo build --release -p mayeuta-llm" >&2`.
3. Continuar `exec` del puente. Prohibido `exit 1` por esta ausencia.

Verificación: script con ELF puente presente y prótesis ausente (staging); proceso arranca; stderr contiene el aviso.

## Fase 4 — Tests CA3

En `#[cfg(test)]` de `main.rs`:

| Caso | Predicado |
|------|-----------|
| Override env + fixture `\x7fELF` | `Ok` hacia ese path |
| Ambos perfiles en tempdir | elige `release/` |
| Sin candidatos | `Err` == traza literal |
| Override no-ELF | traza `SDDIA_MAYEUTA_LLM_BIN no es ELF nativo` |

Aislar env (`remove_var` / restore). Fixture mínimo: 4 bytes `\x7fELF` (existencia + magia; no hace falta ELF válido de ejecución).

```bash
cargo test --manifest-path SddIA/interfaces/kalma2-bridge/Cargo.toml --bin kalma2-bridge -- resolve_mayeuta_llm
```

## Fase 5 — CA6 no-regresión

Rebuild `kalma2-bridge` (el órgano vivo no carga el diff hasta recycle). Probe:

```bash
curl -sS -m 5 -X POST "http://127.0.0.1:8765/api/chat" \
  -H "Content-Type: application/json" \
  -d '{"prompt":"sello 64f37c7f7b34"}'
```

Predicado: HTTP ≠500 **o** `message` ≠ traza literal. Timeout/SSE sin 500 cumple. No exigir tokens LLM.

Si el órgano lab no recicla en esta ola: el test hermético cubre el resolver; CA6 se sella con probe al binario bajo test (puerto lab o órgano post-recycle). Documentar evidencia en `execution.md`.

## Fase 6 — Documentación + evolution

`implementation.md` + `execution.md`. `sddia-qa evolution-register` (`modificacion`: puente + lanzador). Paths: persist_ref, PBI, `main.rs`, `kalma2-bridge.sh`.

## Fase 7 — Cierre documental en rama

- PBI pending → `docs/todos/done/` (mismo `document_id`).
- `validacion.md`: CA1–CA4, CA6; CA-CI = `PENDIENTE-CI` hasta run verde (norma v1.2.1).
- Tras CI verde: `global: APTO`, `ci_run_id`, `pbi_archived: true` en la **misma rama** antes de `accept-pr`.

## Fase 8 — Entrega

`./sddia-run.sh --process delivery-close-cycle` con `source_process: bug-fix`, `persist_ref`, `branch_name`. Git vía `skill:git-manager`. Fire-and-forget tras acuse JSON (DA-5).

## Fase 9 — CI post-PR

Un chequeo de checks del PR (no bucle). Si rojo: parche local + un push (DA-6). Si verde: sellar CA-CI y Fase 10.

## Fase 10 — accept-pr

`./sddia-run.sh --process accept-pr` con `source_branch: fix/kalma2-bridge-mayeuta-llm-missing-64f37c7f7b34`. Solo con CA-CI APTO.

## Orden

```text
spec/plan (Dedalo, commit de diseño)
  → Fase 1 host release
    → Fase 2–4 código + tests
      → Fase 5 CA6 probe
        → Fase 6 implementation.md + execution.md + evolution
          → Fase 7 validacion.md + archivo PBI
            → Fase 8 delivery-close-cycle → PR
              → Fase 9 CI run_id verde
                → Fase 10 accept-pr
```

## Fuera de este plan

Cubo léxico `enrich_fracture_pbi_kaizen.rs`; `resolve_orchestrator`; WUI; 503 Gemini; 404 Aiúa.
