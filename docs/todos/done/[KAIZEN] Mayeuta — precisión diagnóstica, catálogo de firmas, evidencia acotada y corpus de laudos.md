---
document_id: PBI-KAIZEN-MAYEUTA-PRECISION-DIAGNOSTICA
uuid: "9a1a11d4-67e3-4db4-854e-ad70915ed55e"
title: "[KAIZEN] Mayeuta — precisión diagnóstica, catálogo de firmas, evidencia acotada y corpus de laudos"
format: markdown
version: "1.1.0"
created: "2026-09-27"
updated: "2026-09-27"
refined: "2026-09-27"
status: "abierto"
priority: alta
process: bug-fix
type: kaizen
dispatch: false
suggested_branch: fix/kaizen-mayeuta-precision-diagnostica
persist_ref_suggested: docs/fixes/kaizen-mayeuta-precision-diagnostica
spawned_by: PBI-FIX-FRACTURE-4c01d65b972f
source_audit: "v1.0.0: lectura de analyze_fracture_kaizen y delivery_close.rs; censo sobre PBIs de fractura en docs/todos/done/. v1.1.0: contraste con HEAD — el censo mide texto congelado, no el clasificador vigente; el esquema plano no expresa árboles ni exclusiones; CA3 mezclaba diagnóstico con la supresión DCC del FIX hermano."
friction_ids:
  - F-MAYEUTA-EVIDENCE-SCOPE
  - F-MAYEUTA-SEVERITY-PRIORITY
  - F-MAYEUTA-CATCHALL-CONTRADICTORIO
  - F-MAYEUTA-FALSE-POSITIVE-MASKS-LLM
  - F-MAYEUTA-STRUCTURED-BLIND
  - F-MAYEUTA-SIGNATURE-DRIFT
  - F-MAYEUTA-OPAQUE-EVIDENCE
  - F-MAYEUTA-NO-FEEDBACK
architectural_constraints:
  - A-DETERMINISMO-ENRICH        # L-ENRICH-KINTSUGI-DETERMINISTA: sin LLM en el enrich síncrono
  - A-FIRMAS-SSOT                # una definición por id; consumidores explícitos
  - A-MIGRACION-SIN-REGRESION    # F2 no cambia matches ni veredictos
  - A-CEGUERA-NOMINAL            # catálogo resuelto vía cumulo.paths.json
  - A-CORE-AGNOSTICO             # catálogo Core = firmas Core; corpus = líneas discriminantes, no volcado de instancia
  - A-ESPERADO-ES-TRAZA          # expected del corpus sale de la traza, no de causas de otra fase
related:
  - SddIA/engine/execute-process/src/engine/enrich_fracture_pbi_kaizen.rs
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - SddIA/engine/execute-process/src/engine/materialize_fracture_pbi.rs
  - SddIA/engine/execute-process/src/engine/append_mayeuta_hypothesis.rs
  - SddIA/actions/enrich-fracture-pbi-kaizen.md
  - SddIA/agents/mayeuta.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/core/cumulo.paths.json
related_pbis:
  - id: PBI-FIX-FRACTURE-4c01d65b972f
    rol: "C-RC1 (suprimir SFD en non-fast-forward) permanece en ese FIX. Este Kaizen solo añade la firma de diagnóstico. C-RC2 queda absorbida por K2; el CA6 de aquel FIX (seguir detectando bypass léxico) se retira."
  - id: PBI-FIX-MAYEUTA-HB-KAIZEN-CLASSIFIER
    rol: "Precedente F-MAYEUTA-HB-*. Residual F3 (payload estructurado ignorado) lo cierra K3."
  - id: PBI-FEATURE-ASYNC-FRACTURE-CLARIFICATION
    rol: "Triaje LLM asíncrono. Solo ante unclassified. K5 no invoca LLM."
  - id: PBI-KAIZEN-LANCEDB-CICLO-FRICCIONES
    rol: "Cerró el falso «recursión hook» de 01c9040df256 en el código. La traza de ese sello no contiene workflow/scope; ver tabla de corpus."
---

# [KAIZEN] Mayeuta — precisión diagnóstica, catálogo de firmas, evidencia acotada y corpus de laudos

## 0. Qué corrige v1.1.0

1. El censo de `done/` describe el clasificador **en el momento de emitir** cada PBI. Heartbeat, DNS-vs-hook, evolution-gate-vs-hook y el subtipo DLT ya están en HEAD. No es la línea base de aceptación.
2. `{all_of, any_of, none_of}` no expresa los predicados vigentes (grupos OR, `excludes`, árbol DLT, aduanas solo por fase).
3. `F-DCC-PUSH-NON-FAST-FORWARD` con `fracture_policy: suppress` ejecutaría el C-RC1 del FIX hermano. Aquí la firma es de **diagnóstico**. La supresión sigue en `PBI-FIX-FRACTURE-4c01d65b972f`.
4. `01c9040df256` no es un especimen de workflow scope: su traza es `Head sha can't be blank`.
5. `friction_id` **ya viaja** en el payload de `emit_dcc_phase_fractures`. `dcc_friction_id` fabrica `F-DCC-{FASE}` si el report no trae uno. Tratar todo id presente como `structured` clasificaría en falso cada fractura DCC sin firma.

## 1. Falla estructural

`analyze_fracture_kaizen` acumula cubos léxicos y cierra con `verdict_priority` (`new_norm` > `refactor_tool` > `prompt_adjustment` > `process_fix`). Cada fractura mal diagnosticada ha añadido un predicado y, a menudo, su espejo en `delivery_close.rs`. Hay **9** `F-MAYEUTA-*` previos (`HB-BLIND`, `HB-TOKEN-TRAP`, `PREPUSH-EVOL-COLLISION`, `PR-METACHAR-BLIND`, `FRACTURE-HOOK-FALSE-POSITIVE`, `DCC-TOKEN-COLLISION`, `CATCHALL-FAILED`, `DLT-GENERIC`, `ORPHAN-TOKEN-TRAP`). El parche tapa el especimen y deja la clase.

### Censo histórico (`docs/todos/done/`, texto congelado, 2026-09-27)

| Métrica | Valor |
|---|---|
| PBIs de fractura cerrados | 87 |
| Con sección Mayeuta conservada | 37 |
| De esos, fallback *«no clasificada; requiere laudo humano»* | 28 (76 %) |
| Con refutación humana explícita (grep heurístico) | ≥ 22 |
| *«Bloqueo operativo sin escalado Kintsugi»* | 6 |

Ese censo justifica el Kaizen. **No** es el gap de aceptación: varios de esos 28 ya clasificarían distinto en HEAD (p. ej. `63c439de23d0` y `6c0db1296181` casan con `is_heartbeat_starvation_trace`). La línea base es F1, re-ejecutando el corpus contra HEAD.

## 2. Taxonomía

| ID | Defecto | Dónde | Especimen |
|---|---|---|---|
| **F-MAYEUTA-EVIDENCE-SCOPE** | `has_any` corre sobre `blob = error_trace + attempted_action + process_name`. El stderr cita comandos (`git push --help`). | `analyze_fracture_kaizen` | `4c01d65b972f`; tests `bypass_new_norm` (el token `gh pr` está en la traza) |
| **F-MAYEUTA-SEVERITY-PRIORITY** | Gana la severidad del veredicto, no la especificidad de la firma. | `verdict_priority` | `4c01d65b972f` → `new_norm` |
| **F-MAYEUTA-CATCHALL-CONTRADICTORIO** | `timeout\|block\|abort\|colaps` sobre el blob. `block` ⊂ `blocked`. Diagnostica *«sin escalado Kintsugi»* dentro del enrich que solo corre tras `System_Fracture_Detected`. | cubo final de `analyze_fracture_kaizen` | `0c5268362b9a` (`BLOCKED — evolution gate…`) |
| **F-MAYEUTA-FALSE-POSITIVE-MASKS-LLM** | `Fracture_Clarification_Requested` solo si `root_causes` está vacío. Un falso positivo anula la hipótesis asíncrona. | `unclassified = root_causes.is_empty()` | `4c01d65b972f` |
| **F-MAYEUTA-STRUCTURED-BLIND** | DCC ya mete `friction_id` en el payload. La Clase no lo declara. Cúmulo y Mayeuta no lo leen. El fallback `dcc_friction_id` inventa un id por fase. | `emit_dcc_phase_fractures`, `system-fracture-detected.md` | Residual F3 de `PBI-FIX-MAYEUTA-HB-KAIZEN-CLASSIFIER` |
| **F-MAYEUTA-SIGNATURE-DRIFT** | Predicados duplicados y no siempre iguales. Workflow scope sí es el mismo literal. Lab-binary (DCC) es superconjunto del cubo WASM (Mayeuta). DNS y evolution-gate solo existen en DCC. | `dcc_*_trace` vs `is_*_trace` | `d0cfd5b66ff1`, `ca3d901fdc9a` |
| **F-MAYEUTA-OPAQUE-EVIDENCE** | La sección no cita el fragmento que disparó el cubo. | plantilla `section` | `4c01d65b972f` |
| **F-MAYEUTA-NO-FEEDBACK** | El laudo de `done/` no vuelve al clasificador. El cierre no exige un caso. | plantilla `materialize_fracture_pbi` | — |

## 3. Acción

De cubos ad hoc a firmas declarativas, evidencia acotada, estructura primero y corpus de laudos. Determinista: el enrich no llama a un LLM.

### K1 — Catálogo SSOT (cierra SIGNATURE-DRIFT)

Fichero `SddIA/core/fracture-signatures.json`, clave `core.fractureSignatures` en `cumulo.paths.json`. Loader único bajo `SddIA/engine/execute-process/src/core/`. Lo consumen `delivery_close.rs` (supresión) y `enrich_fracture_pbi_kaizen.rs` (diagnóstico).

Cada entrada:

- `id`
- `consumers`: subconjunto de `dcc_suppress` | `mayeuta`. Sin `diagnosis`, Mayeuta no la ve. Sin `fracture_policy: suppress`, DCC no suprime. F2 copia el dueño actual; no regala el otro consumidor.
- `scope` (opcional): `process_name`, `attempted_action`, `status`. Filtra. No aporta tokens.
- `match`: AST sobre la traza normalizada, en minúsculas.
  - `all` / `any` / `none`: literales.
  - `any_groups`: lista de `{all?, any?, none?}` en OR (hace falta para `is_symbolic_head_branch_trace`).
  - `scope_only: true` si la aduana es solo fase+status (`dcc_gate_block_suppresses_fracture`). Sin literales.
- `excludes`: ids. Si uno de ellos casó, esta firma no casa (`remote_branch` excluye symbolic head; el cubo hook excluye workflow, snapshot, wasm, symbolic).
- `refine`: ids hijos ordenados. El padre casa y el primer hijo sustituye el diagnóstico; si ninguno, vale el padre (árbol DLT: gas, object-lock, transporte, opaco).
- `fracture_policy`: `suppress` | ausente. `phase_status` y `operator_hint` solo si hay suppress.
- `diagnosis` (opcional): `root_cause`, `verdict` (`process_fix` | `refactor_tool` | `prompt_adjustment` | `new_norm`), `proposal`.

Literales migrados se copian tal cual, incluidos los anclajes en español del emisor (`omitió`, `lock huérfano`, `cápsula skill`). La independencia de locale obliga a las firmas **nuevas**, no a reescribir las de Argos.

**F2 no unifica predicados distintos.** Pares:

| Id | F2 | F3 |
|---|---|---|
| `F-DCC-WORKFLOW-SCOPE` | Un id, dos consumidores. Los literales ya coinciden. | — |
| `F-DCC-LAB-BINARY-MISSING` | Dos entradas: la de `dcc_lab_binary_missing_trace` (`dcc_suppress`) y la de `is_shell_executor_wasm_fallback_trace` (`mayeuta`). | Colapsar al predicado DCC (superconjunto) y añadir `mayeuta`. Única fusión permitida. |
| `F-DCC-DNS-UNRESOLVED`, `F-DCC-HOOK-EVOL-OVERESCALATION` | Solo `dcc_suppress`, mismos literales que hoy. | Añadir `diagnosis` + consumidor `mayeuta`. |
| `F-DCC-PUSH-NON-FAST-FORWARD` | No existe. No se inventa en F2. | Firma nueva, **solo** `mayeuta` + `diagnosis`. Sin `fracture_policy`. |

Orden del array = orden de causa primaria (K4). En F2 el consumidor Mayeuta sigue acumulando matches y aplicando `verdict_priority`. El orden es dato, todavía no política.

Instancias: catálogo de extensión opcional, ruta en el `cumulo.paths.json` de la instancia. El Core no lleva firmas de cliente.

### K2 — Higiene de evidencia (cierra EVIDENCE-SCOPE)

Vista de match = `error_trace` sin:

- líneas cuyo trim casa `^(hint|ayuda|help|sugerencia)\s*:`
- líneas que empiezan por `SddIA pre-push: SKIPPED`

La traza guardada en el PBI no se toca. `process_name` y `attempted_action` solo entran por `scope`.

Se elimina el cubo «Violación de jurisdicción delegada». El enrich no observa comandos del operador. El único test positivo, `analyze_fracture_kaizen_bypass_new_norm`, mete `gh pr` en `error_trace`: es el canal contaminado, no un bypass real. No se crea marcador `bypass_detected` en este PBI. Si una aduana lo emite más adelante, será otra firma con `scope` sobre ese campo.

### K3 — Estructura primero (cierra STRUCTURED-BLIND)

- Declarar `friction_id` OPTIONAL en `System_Fracture_Detected` vía `entity-manager`. Propagarlo a los inputs de `enrich-fracture-pbi-kaizen` y `materialize-fracture-pbi`.
- `classification: structured` solo si el id **está en el catálogo**. Un id desconocido, incluido el sintético `F-DCC-{FASE}` de `dcc_friction_id`, cae al léxico.
- Cúmulo escribe `friction_id` en el frontmatter solo cuando el valor vino en el payload y no es el sintético.

### K4 — Veredicto por especificidad (cierra SEVERITY-PRIORITY)

A partir de F3: una causa primaria (structured, si no la primera firma `mayeuta` que casa, en orden de catálogo). Su `verdict` es el del PBI. El resto de firmas que casen son señales secundarias y no mueven el veredicto. Se borra `verdict_priority`.

### K5 — Sin catch-all (cierra CATCHALL-CONTRADICTORIO y FALSE-POSITIVE-MASKS-LLM)

Se eliminan el catch-all `timeout|block|abort|colaps` y cualquier cubo sin firma. Lo que no casa es `unclassified` y emite `Fracture_Clarification_Requested`, igual que hoy. Eso no llama a un LLM.

Prohibido un texto de diagnóstico que contradiga el circuito que lo invoca (*«sin escalado Kintsugi»* dentro del enrich de Kintsugi).

### K6 — Evidencia citada (cierra OPAQUE-EVIDENCE)

```markdown
- **Clasificación:** `structured` | `signature` | `unclassified`
- **Firma:** `F-DCC-PUSH-NON-FAST-FORWARD`
- **Evidencia:** `! [rejected] … (non-fast-forward)` (≤ 2 líneas, subcadena literal de la vista normalizada)
- **Señales secundarias:** `F-…` (si hay)
```

`unclassified` no lleva firma ni evidencia.

### K7 — Corpus (cierra NO-FEEDBACK)

`SddIA/engine/execute-process/tests/fixtures/fracture-corpus/{id}.json`:

`{process_name, attempted_action, agent_emitter, error_trace, friction_id?, expected: {classification, signature_id?, verdict}}`

La traza es el mínimo discriminante (las líneas que el laudo usa). Sin URL de instancia ni volcado del PBI. `expected` sale de **esta** traza (A-ESPERADO-ES-TRAZA). El texto Mayeuta archivado no es la expectativa.

| Fixture | Esperado | Veredicto | Papel |
|---|---|---|---|
| `4c01d65b972f` | `signature` / `F-DCC-PUSH-NON-FAST-FORWARD` | `process_fix` | Firma nueva. F1 falla. Sin *jurisdicción* ni `new_norm`. |
| `d0cfd5b66ff1` | `signature` / `F-DCC-DNS-UNRESOLVED` | `process_fix` | Hoy el test solo niega recursión. F3 exige diagnóstico positivo. |
| `0c5268362b9a` | `signature` / `F-DCC-HOOK-EVOL-OVERESCALATION` | `process_fix` | `BLOCKED` alimenta el catch-all. Exigir la firma, no solo la ausencia de recursión. |
| `01c9040df256` | `signature` / `F-DCC-REMOTE-BRANCH-ABSENT` | `process_fix` | Traza = Head sha blank. Prohibido `F-DCC-WORKFLOW-SCOPE` y prohibido recursión. El laudo de dos causas usa la fase anterior, que esta traza no contiene. |
| `6c0db1296181` | `signature` / `F-ARGOS-HEARTBEAT-STARVATION` | `refactor_tool` | Candado. HEAD ya casa. F1 debe acertar. |
| `63c439de23d0` | `signature` / `F-ARGOS-HEARTBEAT-STARVATION` | `refactor_tool` | El unclassified del fichero es histórico. Candado. |
| `60db1db67e49` | `signature` / `F-DLT-GAS-VERSION` (hijo de `F-DLT-PUBLISH-ERROR`) | `process_fix` | No transporte, no `prompt_adjustment`. |
| `ca3d901fdc9a-ola1` | `signature` / `F-DCC-LAB-BINARY-MISSING` | `refactor_tool` | Traza Ola 1: `cápsula skill 'git-manager' no encontrada…`. El hash está repetido en Ola 2 y 3; un fixture, una traza. Pasa cuando F3 fusiona al predicado ancho. |

Dos modos de test:

- `fracture_corpus_baseline` (F1): no falla por desvío. Escribe por stderr `precisión = aciertos / clasificados` y `cobertura = clasificados / total`.
- `fracture_corpus_regression` (desde F3): falla si hay desvío.

Plantilla de Cúmulo, `## Criterio de cierre`: casilla **«Caso añadido a `fracture-corpus` (firma esperada o `unclassified` justificado)»**.

## 4. Contrato con `PBI-FIX-FRACTURE-4c01d65b972f`

| | Este Kaizen | El FIX |
|---|---|---|
| Non-fast-forward | Diagnóstico Mayeuta, sin suppress | `fracture_policy: suppress` + estampa en DCC, sobre el id ya catalogado. Sin predicado Rust nuevo. |
| Bypass / C-RC2 | K2 elimina el cubo | CA6 de aquel PBI queda retirado |
| Orden | Indistinto para el diagnóstico | Si el FIX entra antes, F2 migra también sus predicados nuevos y F3 les quita el cuerpo Rust |

## 5. Fuera de alcance

- LLM en el enrich, o como juez del veredicto.
- Reescribir PBIs de `done/`.
- Umbral CI de precisión. Primero F1; el gate va en un PBI posterior.
- Suprimir el SFD de `4c01d65b972f` (C-RC1).
- Inventar el marcador de bypass de la aduana.
- Unificar predicados que no sean el par lab-binary / shell-wasm.

## 6. Fases

| Fase | Contenido | Salida |
|---|---|---|
| **F0** | Ciclo `bug-fix` vía `execute-process`, rama `fix/kaizen-mayeuta-precision-diagnostica` | `persist_ref` |
| **F1** | Corpus semilla contra HEAD, modo baseline | Cifras en `validacion.md`. No es gate. |
| **F2** | Catálogo + loader. Migración 1:1 con `consumers` actuales. Mayeuta sigue en multi-match + `verdict_priority`. | Tests actuales de matcher y DCC verdes, **sin editar aserciones**. |
| **F3** | K2, K4, K5, fusión lab-binary, diagnósticos Mayeuta de DNS / evolution-gate / non-fast-forward. Sustituir `analyze_fracture_kaizen_bypass_new_norm`. | `fracture_corpus_regression` verde. El resto de tests del matcher, verdes sin tocar. |
| **F4** | K3: OPTIONAL en la Clase, inputs, frontmatter. Id sintético ≠ structured. | Test `structured` + test negativo del id `F-DCC-{FASE}`. |
| **F5** | K6 + casilla de corpus. Bump de `enrich-fracture-pbi-kaizen.md` y `materialize-fracture-pbi.md` vía `entity-manager`. | Snapshot de sección con Firma y Evidencia. |
| **F6** | `SddIA/evolution/` (UUID `enrich-fracture-pbi-kaizen` `c4d5e6f7-…`, Mayeuta `db1acdb5-…`, este PBI) + `gate-evolution --range` | Gate verde |

## 7. Criterios de aceptación

- [ ] **CA1:** `rg "fn (dcc_.*_trace|is_.*_trace)"` sobre `engine/` no deja predicados de firma. Pueden quedar `dcc_friction_id`, `stamp_*`, `dcc_post_push_phase` y `resolve_symbolic_head_branch`.
- [ ] **CA2:** Tras F2, los tests actuales de `enrich_fracture_pbi_kaizen` y `delivery_close` pasan sin editar aserciones. Tras F3, el único test de ese fichero reescrito es `analyze_fracture_kaizen_bypass_new_norm`.
- [ ] **CA3:** Traza de `4c01d65b972f` → `signature` / `F-DCC-PUSH-NON-FAST-FORWARD` / `process_fix`, sin *«Violación de jurisdicción»* ni `new_norm`. `emit_dcc_phase_fractures` **sigue** emitiendo SFD para esa traza.
- [ ] **CA4:** Traza cuya única línea con `git push` es `ayuda:`/`hint:` → `unclassified` y emite `Fracture_Clarification_Requested`. La entrada antigua de `bypass_new_norm` también queda `unclassified`.
- [ ] **CA5:** `friction_id` catalogado → `structured`, aunque la traza contradiga. Id ausente del catálogo, incluido `F-DCC-PUBLICACIÓN-REMOTA`, → léxico.
- [ ] **CA6:** Dos firmas → veredicto de la primera en orden de catálogo; la segunda es señal secundaria.
- [ ] **CA7:** Ninguna sección generada contiene *«sin escalado Kintsugi»*.
- [ ] **CA8:** Toda sección `signature`/`structured` incluye Firma y Evidencia (≤ 2 líneas, subcadena de la vista normalizada).
- [ ] **CA9:** `fracture_corpus_regression` verde con los 8 fixtures. `validacion.md` guarda precisión y cobertura de F1 y del cierre.
- [ ] **CA10:** PBI nuevo de Cúmulo trae la casilla de corpus y, si el payload traía un id no sintético, `friction_id` en el frontmatter.
- [ ] **CA11:** `cargo test -p execute-process`, `sddia-qa gate-evolution --json --range --sync-base`, `verify-process-integrity` y `verify-tools-index` verdes.

## 8. Criterio de cierre

- [ ] CA1–CA11
- [ ] Argos APTO en `validacion.md` (`pbi_archived: true`)
- [ ] Este TODO en `docs/todos/done/` en la misma rama del PR
