---
document_id: PBI-FIX-FRACTURE-d0cfd5b66ff1
uuid: "c2263a19-0af2-4164-a2b2-230825e2c35f"
title: "[FIX] delivery-close-cycle — DNS en Publicación remota (fractura sobreescalada)"
format: markdown
version: "1.2.0"
created: "2026-08-30"
updated: "2026-08-30"
closed: "2026-08-30"
status: "cerrado"
refinement_status: "refinado"
priority: alta
process: bug-fix
fracture_hash: d0cfd5b66ff1
fracture_process: delivery-close-cycle
friction_id: F-DCC-DNS-UNRESOLVED
friction_ids:
  - F-DCC-DNS-UNRESOLVED
  - F-DCC-NET-OVERESCALATION
  - F-MAYEUTA-DCC-TOKEN-COLLISION
incident_ref: "System_Fracture_Detected — d0cfd5b66ff1"
resolution_ref: docs/fixes/dcc-dns-unresolved-d0cfd5b66ff1/
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - SddIA/engine/execute-process/src/engine/capsules.rs
  - SddIA/skills/git-manager/src/main.rs
  - docs/todos/done/[FIX] delivery-close-cycle — barrera de fase simulated (c51acf014c0f).md
  - docs/todos/done/[FIX] delivery-close-cycle — fractura sistémica (c339de406e29).md
  - docs/todos/done/[FIX] bug-fix — fractura sistémica (1d4115c57471).md
  - docs/fixes/dcc-dns-unresolved-d0cfd5b66ff1/validacion.md
  - SddIA/evolution/add08452-fbff-4768-b906-9b0eb2baa9e3.md
---

# [FIX] delivery-close-cycle — DNS en Publicación remota (fractura sobreescalada)

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `delivery-close-cycle` |
| Emisor | `execute-process` |
| Acción intentada | `Publicación remota` |
| Fase física | `capsule_delivery_remote_push` (`phase_capsules.rs`) |
| Cápsula | `skill:git-manager` → `operation_type: push` |
| Clasificación | `F-DCC-DNS-UNRESOLVED` (detonante) + `F-DCC-NET-OVERESCALATION` (deuda) |

## Traza de error

```
fatal: no es posible acceder a 'https://github.com/racso80es/SddIA.git/': Could not resolve host: github.com
```

`getaddrinfo` falló. No es timeout HTTP, no es auth, no es hook pre-push, no es `gh`.

## Errata Mayeuta (síntesis auto-generada v1.0.0)

El enriquecimiento Kaizen es **falso positivo recurrente**. `analyze_fracture_kaizen` concatena `error_trace + attempted_action + process_name` y hace `blob.contains("delivery-close")`. El `process_name` **es** `delivery-close-cycle` → **toda** fractura DCC recibe «recursión hook» + propuesta `SDDIA_HOOK_DELIVERY_CLOSE`, con independencia de la traza.

Esa guarda **ya existe** (`in_delivery_close_cycle`, `SDDIA_HOOK_DELIVERY_CLOSE=1`, Ola B / `6d64bcc7` §3.7). Misma errata documentada en `c51acf014c0f` y `c339de406e29`. **Prohibido** reimplementarla.

## Cadena de fricciones

El síntoma emitido (`System_Fracture_Detected`) es el **último eslabón**, no la causa raíz del push.

| # | Friction | Naturaleza | Emite evento |
|---|----------|-----------|:---:|
| F1 | **DNS no resuelve `github.com`** | Detonante infra / entorno | No (stderr git) |
| F2 | **Push fallido escala a colapso sistémico** | Sobre-escalado (hueco F4b) | Sí (`d0cfd5b66ff1`) |
| F3 | **Mayeuta: token `delivery-close` en `process_name`** | Ceguera clasificadora | Contamina el PBI |

### F1 — DNS unresolved (detonante; no es defecto de motor)

`capsule_delivery_remote_push` invoca `git-manager` `push` `{remote: origin, branch, force: false}`. Git aborta con `Could not resolve host: github.com`. El abort es **correcto**: no hay publicación posible.

Naturaleza: fallo de resolución DNS (red local, sandbox, `resolv.conf`, outage transitorio). Simétrico inverso al `offline` que `workspace_init` ya tolera en **fetch**; el push **no** debe skippearse (ocultaría un cierre de entrega).

**Operativa inmediata (fuera de código):** cuando DNS resuelva, re-inyectar `delivery-close-cycle` sobre el mismo `persist_ref`/`branch_name`. No es causa raíz accionable en genoma.

### F2 — `failed` de red → `System_Fracture_Detected` (deuda accionable)

F4b (`c51acf014c0f` / `c339de406e29`) silenció fractura solo para `blocked` de **Aduana evolution** / **Aduana EDA**. `dcc_gate_block_suppresses_fracture` no cubre `Publicación remota` ni status `failed`.

Entregado: predicado hermano `dcc_net_block_suppresses_fracture` + `stamp_dcc_network_block` (`F-DCC-DNS-UNRESOLVED`).

### F3 — Colisión de token Kaizen (deuda accionable, tercer incidente)

Entregado: regla hook sobre `error_trace`+`attempted_action`; token unario `delivery-close` retirado.

## Veredicto evolutivo

**Kaizen de discriminación** entregado (`add08452-fbff-4768-b906-9b0eb2baa9e3`).

## Criterio de cierre

- [x] Diagnóstico v1.0.0 («recursión hook») marcado erróneo en este PBI
- [x] F2: `emit_dcc_phase_fractures` no escala DNS/red transitoria de `Publicación remota` a `System_Fracture_Detected`; envelope conserva `blocked` accionable
- [x] F3: `analyze_fracture_kaizen` no clasifica fracturas DCC genéricas como recursión hook; test de no-regresión con traza DNS
- [x] F1: documentado como detonante de entorno; reintento DCC es operativa, no código
- [x] Argos APTO en `validacion.md` del fix
- [x] Este TODO movido a `docs/todos/done/` en la misma rama del PR
