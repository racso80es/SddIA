---
feature_name: dcc-dns-unresolved-d0cfd5b66ff1
created: "2026-08-30"
process: bug-fix
base: main
scope: dcc-net-overescalation-mayeuta-token
branch_name: fix/dcc-dns-unresolved-d0cfd5b66ff1
persist_ref: docs/fixes/dcc-dns-unresolved-d0cfd5b66ff1
pbi_ref: docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (d0cfd5b66ff1).md
document_id: PBI-FIX-FRACTURE-d0cfd5b66ff1
execution_id: "83cc0b40-f863-4a2a-95bf-8743d6faa56f"
---

# Especificación — fractura `d0cfd5b66ff1` (DNS ≠ colapso)

## Problema

`delivery-close-cycle` fase **Publicación remota** abortó con:

```
fatal: no es posible acceder a 'https://github.com/racso80es/SddIA.git/': Could not resolve host: github.com
```

El abort de `git-manager push` es **correcto**. Defectos:

| ID | Defecto |
|----|---------|
| F1 | Detonante infra: DNS no resuelve `github.com`. No es código. |
| F2 | `emit_dcc_phase_fractures` escala todo `failed` no-aduana a `System_Fracture_Detected` (hueco F4b). |
| F3 | `analyze_fracture_kaizen` concatena `process_name` al blob y busca `delivery-close` → toda fractura DCC sale como «recursión hook». |

La guarda `SDDIA_HOOK_DELIVERY_CLOSE` **ya existe**. Prohibido reimplementarla.

## Reproducción (esta sesión)

1. DCC original: `Publicación remota` → hash `d0cfd5b66ff1`.
2. `./sddia-run.sh --process bug-fix` + `SDDIA_AGENT_RELAY_IDE=1`: workspace-init **failed** en fetch con la misma traza DNS (`execution_id` `34ecf78b`). Barrera simulated cortó DCC. Sin PBI Kintsugi nuevo (el emisor de fractura es DCC, no el executor genérico).
3. Re-inyección con `SDDIA_LAB_SKIP_GIT=1` + checkout local `git-manager`: `execution_id` `83cc0b40`; Diseño `simulated`; Ejecución…Cierre `skipped` (`prior_agent_phase_not_executed`).

## Cambio requerido

### F2 — Taxonomía de red (F4c, motor, no DA-2)

`SddIA/engine/execute-process/src/engine/delivery_close.rs`. No mutar genoma `delivery-close-cycle.md` salvo nota operativa opcional.

**Predicado de traza** (`dcc_transient_network_trace`), case-insensitive, sobre `error`/`message`:

| Token |
|-------|
| `could not resolve host` |
| `temporary failure in name resolution` |
| `name or service not known` |
| `network is unreachable` |
| `connection timed out` |

**Supresión** (`dcc_net_block_suppresses_fracture`), hermana de `dcc_gate_block_suppresses_fracture` (F4b intacto):

| Condición | Emite `System_Fracture_Detected` |
|-----------|----------------------------------|
| F4b: aduana evolution/EDA `blocked` | **No** (ya) |
| `Publicación remota` o `Apertura en forja` + `failed`/`blocked` + traza de red | **No** (nuevo) |
| `failed` de forja/cápsula no-red (p. ej. `pr_url` opaco) | **Sí** |
| `fail_soft: true` | No (ya excluido) |

Al clasificar: sellar `friction_id: F-DCC-DNS-UNRESOLVED` en el phase report. Status **`blocked`** (cierre no cruzó umbral físico; operador reinyecta). **Prohibido** `fail_soft` (ocultaría el fallo de entrega). **Prohibido** `offline: true` como éxito de push.

Envelope raíz de DCC: `success: false` (no hay `pr_url` / `delivery_push`). Señal accionable, no Kintsugi.

No clasificar en `git-manager`. No retry/backoff (DA-5).

### F3 — Matcher Kaizen

`SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs`:

1. La regla de recursión hook **no** concatena `process_name` al blob. Tokens `recurs` / `pre-push` / `hook` / `re-entrada` se evalúan solo contra `error_trace` + `attempted_action`.
2. Eliminar el token unario `delivery-close` de esa regla (colisión con el nombre del proceso).
3. Conservar: traza `pre-push hook` → `refactor_tool` + texto de recursión.

## Criterios de aceptación

| ID | Criterio |
|----|----------|
| CA-1 | Traza DNS en `Publicación remota` `failed`/`blocked` **no** materializa `System_Fracture_Detected` en `eda_bus.pending` |
| CA-2 | Mismo caso en `Apertura en forja` (simetría `gh`) |
| CA-3 | Phase report lleva `friction_id: F-DCC-DNS-UNRESOLVED`; status `blocked`; envelope DCC `success: false` |
| CA-4 | F4b intacto: aduana evolution `blocked` sigue sin fractura; `Apertura en forja` failed **no-red** sigue emitiendo |
| CA-5 | `analyze_fracture_kaizen("delivery-close-cycle", DNS, "Publicación remota", …)` **no** contiene «Recursión o re-entrada» |
| CA-6 | Traza `pre-push hook blocked` sigue clasificando recursión (`refactor_tool`) |
| CA-7 | Tests unitarios `execute-process`: `dcc_fracture` (red) + `analyze_fracture_kaizen` (DNS vs hook) |
| CA-8 | Cascada spec/plan/implementation/execution/validacion APTO; PBI en `done/` en el mismo PR (cierre posterior a este corte) |

## Fuera de alcance (este PR)

- Resolver DNS del host / sandbox.
- Skip silencioso del push (`offline: true`).
- Retry/polling en `git-manager` o Tekton.
- Reimplementar `SDDIA_HOOK_DELIVERY_CLOSE` / `SDDIA_SKIP_HOOKS`.
- Mutar fases/contrato `proc:git-sync` del genoma DCC.
- Ampliar F4b a otras aduanas (`Aduana integridad índices`) salvo que compartan traza de red.
- Ejecución de código y DCC en **este** corte (mandato: spec + plan + commit).
