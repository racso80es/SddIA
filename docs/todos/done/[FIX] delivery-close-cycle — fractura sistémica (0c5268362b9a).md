---
document_id: PBI-FIX-FRACTURE-0c5268362b9a
uuid: "a99f5958-0d35-437f-8b38-9635ca6a14d5"
title: "[FIX] delivery-close-cycle — pre-push evolution gate sobreescalado a fractura"
format: markdown
version: "1.2.0"
created: "2026-08-30"
updated: "2026-08-31"
closed: "2026-08-31"
status: "cerrado"
refinement_status: "refinado"
resolution_ref: docs/fixes/dcc-hook-evol-overescalation-0c5268362b9a/
priority: alta
process: bug-fix
fracture_hash: 0c5268362b9a
fracture_process: delivery-close-cycle
friction_id: F-DCC-HOOK-EVOL-OVERESCALATION
friction_ids:
  - F-DCC-HOOK-EVOL-OVERESCALATION
  - F-MAYEUTA-PREPUSH-EVOL-COLLISION
  - F-DCC-OPERATOR-PUSH-NO-GUARD
incident_ref: "System_Fracture_Detected — 0c5268362b9a"
specimen_cycle: "fix/mayeuta-heartbeat-kaizen-classifier (PR #236; PBI residual fuera de ese PR)"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
  - SddIA/scripts/qa/git-hooks/pre_push_gate.sh
  - SddIA/scripts/qa/git-hooks/hook_common.sh
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - docs/todos/done/[FIX] delivery-close-cycle — fractura sistémica (d0cfd5b66ff1).md
  - docs/todos/done/[FIX] delivery-close-cycle — barrera de fase simulated (c51acf014c0f).md
  - docs/todos/done/[FIX] delivery-close-cycle — fractura sistémica (c339de406e29).md
  - docs/todos/done/[KAIZEN] AEL-CA9 — fase gate-evolution SSOT en delivery-close-cycle.md
  - docs/todos/done/[FIX] Mayeuta — clasificador Kaizen ciego a latido de centinelas.md
---

# [FIX] delivery-close-cycle — pre-push evolution gate sobreescalado a fractura

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `delivery-close-cycle` |
| Emisor | `execute-process` |
| Acción intentada | `Publicación remota` |
| Fase física | `capsule_delivery_remote_push` (`phase_capsules.rs`) |
| Hook | `pre_push_gate.sh` → `run_evolution_gate` (`--range --if-touched --sync-base`) |
| Clasificación | `F-DCC-HOOK-EVOL-OVERESCALATION` (deuda) + `F-MAYEUTA-PREPUSH-EVOL-COLLISION` (síntesis falsa) |

## Traza de error

```
SddIA pre-push: BLOCKED — evolution gate (--range --if-touched) failed
error: falló el empuje de algunas referencias a 'https://github.com/racso80es/SddIA.git'
```

Literal de `pre_push_gate.sh` (`run_evolution_gate`) + mensaje genérico de git cuando el hook sale ≠ 0. **No** es DNS, **no** es auth, **no** es `gh`, **no** es re-entrada DCC (`BLOCKED — delivery-close-cycle failed`).

## Contexto de reproducción (especimen)

Primer `delivery-close-cycle` del ciclo `fix/mayeuta-heartbeat-kaizen-classifier` (2026-08-30). Correlato evolution del bump `enrich-fracture-pbi-kaizen` 1.2.0 **no** listaba `SddIA/actions/index.md` (ni `SddIA/core/eda-coverage.json`). El operador documentó este PBI como residual **fuera** de [PR #236](https://github.com/racso80es/SddIA/pull/236). Segundo DCC del mismo ciclo, con correlato completo, publicó y selló `PullRequest_Presented`.

El detonante de **ese** push (`EVOL_*` por material sin correlato) quedó remediado en #236. **No** es la deuda de este PBI.

Envelope de fase de la 1.ª DCC **no** conservado. Inferencias de ruta se marcan como tales.

## Errata Mayeuta (síntesis auto-generada v1.0.0)

El enriquecimiento Kaizen es **falso positivo**. `analyze_fracture_kaizen` dispara el cubo hook si `error_trace|attempted_action` contiene `pre-push` / `hook` / `recurs` / `re-entrada` (`enrich_fracture_pbi_kaizen.rs`). Esta traza contiene `pre-push` → veredicto `refactor_tool` + texto de recursión, **con independencia** de que el hook haya bloqueado por aduana evolution.

La guarda propuesta (`SDDIA_HOOK_DELIVERY_CLOSE` + `SDDIA_SKIP_HOOKS=1` acotado) **ya existe** (`in_delivery_close_cycle`, Ola B / `delivery-close-hook-eda-governance`, genoma `delivery-close-cycle` v1.4.0 § Anti-recursión). Misma clase de errata que `d0cfd5b66ff1` (F3, token `delivery-close`) y `c339de406e29` (propuesta moot). **Prohibido** reimplementarla.

El test `analyze_fracture_kaizen_recursion_verdict` **congela** el falso positivo: exige que `"pre-push hook blocked"` clasifique recursión. Cualquier BLOCKED de pre-push (incluida aduana evolution) hereda ese veredicto.

## Cadena de fricciones

El síntoma emitido (`System_Fracture_Detected`) es el **último eslabón**, no la causa raíz del push.

| # | Friction | Naturaleza | Emite evento |
|---|----------|-----------|:---:|
| F0 | Material sin correlato evolution (`actions/index.md`) | Detonante del especimen; cerrado en #236 | No (stderr `sddia-qa`) |
| F1 | Hook pre-push ejecuta `gate-evolution` **durante** Publicación remota de DCC operador | Hueco de guarda / topología AEL-CA9 | No (stderr hook) |
| F2 | `failed` de Publicación remota por gate evolution → colapso sistémico | Sobre-escalado (hueco F4b) | Sí (`0c5268362b9a`) |
| F3 | Mayeuta: token `pre-push` ≡ recursión hook | Ceguera clasificadora | Contamina el PBI |

### F0 — Detonante del especimen (no es deuda de este PBI)

`entity-manager` actualizó `SddIA/actions/index.md`. El registro evolution `5eae5eb6-…` no lo listaba en `relacionado` en el primer intento de cierre. `gate-evolution --range` debe bloquear con `EVOL_MATERIAL_UNREGISTERED`. Comportamiento de aduana **correcto**. Remediación: correlato + rehash en el mismo ciclo (#236). Reintento DCC con correlato íntegro **no** es causa raíz accionable aquí.

### F1 — Aduana evolution del hook alcanza el push de DCC operador

Hechos de código (no inferidos):

1. `capsule_delivery_remote_push` (Rust) invoca `git-manager push` **sin** `SDDIA_HOOK_DELIVERY_CLOSE` ni `SDDIA_SKIP_HOOKS`. El skip documentado (`source_process == git-hook-pre-push`) existía en el handler Python; el nativo no lo replica. En DCC invocado por operador, el hook **corre**.
2. `in_delivery_close_cycle` solo es verdadero si el **hook** exportó `SDDIA_HOOK_DELIVERY_CLOSE=1` al invocar DCC. DCC operador no auto-exporta esa guarda → no hay skip temprano.
3. El literal de la traza **solo** lo emite `run_evolution_gate`, y `main` solo lo llama si `pre_push_hook_runs_evolution_gate` (`#branches == 0`). AEL-CA9 CA-2 reserva ese camino a PR OPEN / skip presentación, **no** a presentación de rama nueva (CA-1: una sola vez, fase DCC).

**Hipótesis de ruta (código, envelope no conservado):** `is_delete_push` testa `$remote_sha` contra ceros. En el protocolo pre-push, SHA remoto cero = **ref nueva**, no delete (delete = SHA **local** cero). Un primer push de rama nueva vacía `branches[]` → el hook toma el camino F4c (`gate-evolution --if-touched`) en lugar de delegar en DCC. Eso viola AEL-CA9 CA-1 y explica por qué la fase DCC «Aduana evolution» no es la `attempted_action` del evento: el colapso ocurre **después**, en Publicación remota.

**No afirmado:** que la fase «Aduana evolution» del 1.er DCC hubiera pasado o se hubiera skipeado (`SDDIA_LAB_SKIP_EVOLUTION_GATE`). Lo único seguro por el payload Cúmulo: el fallo materializado es de `Publicación remota`, no de `Aduana evolution`. Si la fase DCC hubiera bloqueado, F4b habría **suprimido** `System_Fracture_Detected` y este PBI no existiría.

### F2 — Gate accionable escala a Kintsugi (deuda)

F4b (`c51acf014c0f` / `c339de406e29`) silencia fractura solo si `phase_name ∈ {Aduana evolution, Aduana EDA genómica}` **y** `status == blocked`. `dcc_net_block_suppresses_fracture` cubre DNS/red en Publicación remota / Apertura en forja.

`Publicación remota` `failed` por hook `evolution gate … failed` **no** entra en ningún predicado → `emit_dcc_phase_fractures` emite `System_Fracture_Detected` + `friction_id` derivado `F-DCC-PUBLICACIÓN-REMOTA`. Misma clase que F4b: aduana determinista ≠ colapso de runtime.

### F3 — Colisión de cubo Kaizen (deuda)

Cuarto incidente de la familia «Mayeuta proyecta recursión hook sobre DCC»:

| Hash | Traza real | Cubo disparado |
|------|------------|----------------|
| `c339de406e29` | `diff material sin evolution correlacionada` | token `delivery-close` en `process_name` (pre-F3 DNS) |
| `c51acf014c0f` | `no se pudo resolver pr_url desde gh` | idem |
| `d0cfd5b66ff1` | `Could not resolve host` | idem; parcheó blob hook (sin `process_name`) |
| **`0c5268362b9a`** | `pre-push: BLOCKED — evolution gate` | token `pre-push` en `error_trace` (post-F3 DNS) |

El parche F3 de `d0cfd5b66ff1` **no** cubre este caso: el token vive en la traza, no en `process_name`.

## Mandato

Corregir la causa raíz **sistémica** (F1/F2/F3). **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado. Prohibido reimplementar `SDDIA_HOOK_DELIVERY_CLOSE`. Prohibido reabrir F0/#236 como alcance de este PBI.

## Delimitación (ruido descartado)

| Ítem | Relación |
|------|----------|
| Recursión hook ↔ DCC | **Descartada.** No hay `BLOCKED — delivery-close-cycle failed` ni evidencia de re-entrada. Guarda ya desplegada. |
| DNS / `Could not resolve host` | **Otro** incidente (`d0cfd5b66ff1`, cerrado). Esta traza no es red. |
| F4b Aduana evolution → fractura | **Cerrado** en `c51acf014c0f`. Este PBI es el **hueco simétrico**: mismo gate, fase distinta (`Publicación remota` vía hook). |
| PR #236 / cubo latido Mayeuta | Ciclo **origen** del especimen; no es el defecto a parchear aquí. |
| `analyze_fracture_kaizen_dns_not_hook_recursion` | Sigue válido. No protege trazas con `pre-push`. |

## Conclusión Analítica y Propuesta Evolutiva

*(Síntesis Tekton — Kintsugi; sustituye la auto-generada de Mayeuta)*

### Diagnóstico de causa raíz

- **Sobre-escalado de aduana**, no recursión de hook: el pre-push bloqueó `gate-evolution --range --if-touched` durante `Publicación remota`; DCC tradujo ese `failed` a `System_Fracture_Detected`. Mayeuta leyó `pre-push` y proyectó una guarda ya existente.

### Veredicto evolutivo

**Kaizen de discriminación** (`refactor_tool` sobre clasificador + suppress de fractura; `process_fix` menor en topología hook si F1 se confirma).

### Propuestas

- **F2 — Extender F4b a Publicación remota bloqueada por evolution gate:** si `error_trace` coincide con el literal `evolution gate (--range --if-touched) failed` (o `reason_codes` `EVOL_*` en envelope), `emit_dcc_phase_fractures` **no** emite `System_Fracture_Detected`. Conservar `status: failed|blocked` y friction accionable (`F-DCC-HOOK-EVOL-OVERESCALATION` / reutilizar `F-DCC-EVOLUTION-GATE`). No tragarse fallos de red ni de `git-manager` ajenos al gate.
- **F3 — Cubo hook más estricto:** no clasificar recursión por el solo token `pre-push`. Exigir re-entrada real (p. ej. `delivery-close-cycle failed for` / `HOOK_DELIVERY_CLOSE` ausente **y** invocación anidada). Traza `evolution gate … failed` → no «Recursión o re-entrada»; no proponer reimplementar la guarda. Actualizar `analyze_fracture_kaizen_recursion_verdict` para que deje de usar `"pre-push hook blocked"` como proxy de recursión; añadir no-regresión con la traza canónica de este PBI.
- **F1 — Topología hook (si laudo confirma hipótesis SHA cero):** `is_delete_push` debe inspeccionar SHA **local**, no remoto. Push de ref nueva (`remote_sha` cero) permanece en `branches[]` → no corre `run_evolution_gate`; DCC es SSOT (AEL-CA9 CA-1). Alternativa/complemento: DCC operador exporta `SDDIA_HOOK_DELIVERY_CLOSE=1` en el subproceso `git-manager` (paridad con invocación desde hook) para que el push no re-aduane. Elegir **una** SSOT; no duplicar gate.

> Kintsugi transforma la fractura en deuda accionable; el Vértice Biológico valida antes de ejecutar.

## Criterio de cierre

- [x] Diagnóstico v1.0.0 («recursión hook» + reimplementar `SDDIA_HOOK_DELIVERY_CLOSE`) marcado erróneo en este PBI
- [x] F2: `emit_dcc_phase_fractures` no escala Publicación remota fallida por evolution gate de pre-push a `System_Fracture_Detected`
- [x] F3: `analyze_fracture_kaizen` no clasifica la traza canónica de este incidente como recursión hook; test de no-regresión verde; test de recursión real sigue cubierto
- [x] F1: `is_delete_push` alineado al protocolo git **y** guarda de ciclo en push DCC operador (laudo spec: complementarios, SSOT DCC); AEL-CA9 CA-1 intacto
- [x] F0 documentado como detonante de especimen (cerrado en #236); fuera de alcance de código de este fix
- [x] Argos APTO en `validacion.md` del fix
- [x] Este TODO movido a `docs/todos/done/` en la misma rama del PR
