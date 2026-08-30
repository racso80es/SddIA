---
document_id: PBI-FIX-FRACTURE-c51acf014c0f
uuid: "c51acf01-4c0f-4000-8000-000000000001"
title: "[FIX] delivery-close-cycle — apertura en forja sobre fix sin diseño (barrera simulated)"
format: markdown
version: "1.2.0"
created: "2026-08-29"
updated: "2026-08-30"
status: "cerrado"
priority: alta
process: bug-fix
fracture_hash: c51acf014c0f
fracture_process: delivery-close-cycle
friction_id: F-DCC-APERTURA-EN-FORJA
friction_ids:
  - F-DCC-APERTURA-EN-FORJA
  - F1-SIMULATED-SIN-BARRERA
  - F2-SKIP-DOCUMENTAL-MUDO
  - F4-CI-WASI-RECURRENTE
incident_ref: "System_Fracture_Detected — c51acf014c0f"
correlation_hash_source: "no se pudo resolver pr_url desde gh"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/engine/execute-process/src/engine/executor.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/engine/execute-process/src/engine/delivery_close.rs
  - SddIA/library/codexes/codex-software-engineering/process/bug-fix.md
  - SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
  - SddIA/scripts/qa/git-hooks/pre_push_gate.sh
  - SddIA/tools/sddia-qa/src/gate_evolution.rs
  - SddIA/tools/sddia-qa/src/verify_hooks.rs
  - .github/workflows/sddia-index-qa.yml
  - docs/todos/pending/[FIX] route-domain-event — fractura sistémica (b3a715381787).md
  - docs/todos/pending/[FIX] delivery-close-cycle — fractura sistémica (c339de406e29).md
  - docs/todos/done/[KAIZEN] Aduana evolution local inexistente — hooks sin instalar, --if-touched invertido y fase de impacto stub.md
---

# [FIX] delivery-close-cycle — apertura en forja sobre fix sin diseño (barrera simulated)

## Incidente (auto-generado por Cúmulo)

| Campo | Valor |
|-------|--------|
| Proceso | `delivery-close-cycle` (encadenado desde `bug-fix`) |
| Emisor | `execute-process` |
| Acción intentada | `Apertura en forja` |
| Fase física | `capsule_delivery_gh_pr` (`phase_capsules.rs`) |
| Clasificación | `F-DCC-APERTURA-EN-FORJA` |

## Traza de error

```
no se pudo resolver pr_url desde gh
```

## Contexto de reproducción

Invocación: `./sddia-run.sh --process bug-fix` para materializar el fix de la fractura DLT `b3a715381787`, con mandato explícito **"forja plan y detente tras commit"**. Perfil laboratorio, relevo IDE (`SDDIA_AGENT_RELAY_IDE=1`).

Resultado: `execution_id` `b27989e4-…`; workspace-init `executed` (rama `fix/route-domain-event-fracture-b3a715381787`); fases Dedalo/Tekton/Argos `simulated`; **cierre encadenado sin corte**; colapso en `Apertura en forja`. Rama empujada a origin **sin PR**; worktree limpio; plan no forjado.

## Cadena de fricciones detectadas

Tres defectos encadenados. El síntoma emitido (`F-DCC-APERTURA-EN-FORJA`) es el **último eslabón**, no la causa raíz.

| # | Friction | Naturaleza | Emite evento |
|---|----------|-----------|:---:|
| F1 | **Barrera `simulated` inexistente** | Causa raíz | No |
| F2 | **Skip documental mudo** | Ceguera de señal | No |
| F3 | **`F-DCC-APERTURA-EN-FORJA`** | Síntoma / colapso | Sí (`c51acf014c0f`) |
| F4 | **Recurrencia CI `wasi-runtime-smoke`** | Consecuencia sistémica de F1 | Sí (por ciclo, en remoto) |

### F1 — `simulated` no barre el pipeline (causa raíz)

Las fases de agente sin runtime (`SDDIA_AGENT_RUNTIME_COMMAND` ausente o relevo IDE) retornan `status: simulated`. El guardia de barrera solo bloquea estados terminales de agente:

```47:51:SddIA/engine/execute-process/src/engine/executor.rs
fn agent_phase_blocks_downstream(status: &str) -> bool {
    matches!(
        status,
        "failed" | "blocked" | "awaiting_agents" | "awaiting"
    )
}
```

`simulated` es neutro → el orquestador trata "el agente **no** ejecutó" como "fase OK" y **encadena `delivery-close-cycle`**. Consecuencia: cierre de entrega (push + PR + sello ECST) sobre un `bug-fix` **sin `spec.md` / `plan.md` / `implementation.md` / `validacion.md`**. El mandato "detente tras el plan" es irrepresentable: no existe punto de corte entre Diseño y Cierre cuando las fases de agente quedan en relevo IDE.

### F2 — Skip documental mudo

`Cierre documental en rama` resuelve `status: executed` + `skipped: true`, `reason: "validacion.md ausente"`. Guarda correcta (no archiva PBI sin validación), pero **no emite fractura ni detiene el pipeline**: el cierre remoto prosigue igualmente. Señal perdida.

### F3 — `Apertura en forja` sin `pr_url` (síntoma)

`capsule_delivery_gh_pr`: la fase **Publicación remota** empujó la rama nueva a origin (GitHub responde "create a pull request … visiting …"), pero `gh pr create` + fallback `gh pr view --json url` **no devuelven URL parseable** → `ok_or("no se pudo resolver pr_url desde gh")`:

```746:753:SddIA/engine/execute-process/src/engine/phase_capsules.rs
    pr_url = view
        .get("stdout")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string);
    }
    let pr_url = pr_url.ok_or("no se pudo resolver pr_url desde gh")?;
```

Aguas abajo, `capsule_delivery_emit_presented` exige `pr_url` para el sello ECST (`PullRequest_Presented` incompleto → DLT Argos/IOTA). El envelope de error **no propaga `gh_stdout/stderr`**: telemetría opaca (mismo antipatrón que el 500 DLT de `b3a715381787`).

### F4 — Recurrencia de `wasi-runtime-smoke` (consecuencia sistémica de F1)

Auditoría de recurrencia (2026-08-29). El job CI `wasi-runtime-smoke` no falla por WASI: falla en el step **`evolution gate (delta)`** con `EVOL_MATERIAL_UNREGISTERED` («diff material sin evolution correlacionada»). Recurrente en PR #230 (`workspace_init.rs`), #209 (18 findings), #203, #194. F1 lo convierte en **determinista**, no aleatorio:

1. **Orden invertido (F1):** en relevo IDE las fases de agente quedan `simulated` → el orquestador encadena `delivery-close-cycle` → **push + PR con snapshot incompleto** (en #230, `a183a8a` = solo `objectives.md` + PBI, sin código ni correlato evolution). CI arranca sobre estado que, por construcción, carece de material+evolution → rojo garantizado en el primer run del ciclo.
2. **El push del orquestador se auto-exime de la aduana local:**

```21:24:SddIA/scripts/qa/git-hooks/pre_push_gate.sh
  if in_delivery_close_cycle; then
    echo "SddIA pre-push: SKIPPED (delivery-close-cycle guard)" >&2
    exit 0
  fi
```

3. **La aduana local está construida pero dormida:** el blindaje de `PBI-KAIZEN-ADUANA-EVOLUTION-LOCAL` (`6d64bcc7`, done) implementó `verify_hooks.rs`, `range_touches_material` y `resolve_base`, y `start-sddia.sh` fija `core.hooksPath`. Pero es config **local por-clon, no versionada**: en una sesión IDE sin bootstrap, `core.hooksPath = None` (verificado hoy: `.git/hooks/` solo `*.sample`; `verify-hooks → success:false`). Los commits de contenido posteriores también saltan el gate.

→ La **primera** capa que aplica el contrato evolution es CI. En lote, en remoto, y consumiendo el único intento barato que permite DA-6.

#### F4b — `c339de406e29` unificada (Aduana evolution escala a colapso)

`c339de406e29` (`delivery-close-cycle` / fase «Aduana evolution», traza `diff material sin evolution correlacionada`) queda **fusionada aquí**. Su mecanismo real (auditado, no el auto-diagnóstico):

- La fase «Aduana evolution» de `delivery-close-cycle` **bloquea correctamente** con `EVOL_MATERIAL_UNREGISTERED` (`status: blocked`; test `evolution_phase_blocks_unregistered_material_ca12`). Comportamiento **correcto**.
- Pero `emit_dcc_phase_fractures` escala **todo** `blocked`/`failed` a `System_Fracture_Detected` → materializa PBI Kintsugi de ruido:

```271:298:SddIA/engine/execute-process/src/engine/delivery_close.rs
        if status != "blocked" && status != "failed" {
            continue;
        }
        // …
        let _ = materialize_pending_domain_event(
            repo,
            "System_Fracture_Detected",
            "execute-process",
            payload,
        );
```

- **Mismo antipatrón que `F-DIRTY-WORKTREE` (`1d4115c57471`, ya resuelto):** un guard determinista (aduana) escalado a colapso sistémico. Higiene/gate ≠ fractura de runtime.
- Su propuesta auto-generada («recursión hook» → guarda `SDDIA_HOOK_DELIVERY_CLOSE` + `SDDIA_SKIP_HOOKS=1` acotado) es **moot**: esa guarda **ya existe** (`in_delivery_close_cycle`, `6d64bcc7` §3.7) y es precisamente la que exime el push del orquestador (F4, punto 2) — no la causa.

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado. Prohibido abrir el PR a mano o `rerun` de `delivery-close-cycle`: eso trataría el síntoma (F3) e ignoraría F1.

## Delimitación (ruido descartado)

| PBI | `error_trace` | Relación con esta fractura |
|-----|---------------|--------------------------------|
| `c339de406e29` (`delivery-close-cycle`) | `diff material sin evolution correlacionada` | **Correlacionada (no ruido):** cara interna «Aduana evolution» de F4. Mismo bucle; su diagnóstico auto-generado es erróneo. Candidata a fusión/cierre bajo la remediación de F1+F4. |
| `1d4115c57471` (`bug-fix` / workspace-init) | `dirty-worktree: …` | Otro worktree/sesión; cerrada (`done/`). Su fix disparó el rojo #230 que evidenció F4. |
| `b3a715381787` (`route-domain-event`) | `iota-relay-unreachable: … 500` | Fractura **origen** que motivó la sesión; independiente. |

## Conclusión Analítica y Propuesta Evolutiva

*(Síntesis Tekton — Kintsugi, pendiente enriquecimiento Mayeuta)*

### Diagnóstico de causa raíz

- **Defecto de orquestación**, no de `gh`: `delivery-close-cycle` se ejecuta sobre un ciclo `bug-fix` cuyas fases de agente quedaron `simulated`. El `pr_url` inalcanzable es el punto donde la ausencia de diseño/validación se hace visible, no su causa.

### Veredicto evolutivo

**Corrección de proceso oficial + observabilidad** (`process_fix` + `observability`). F4 eleva el alcance: la barrera de F1 es también la palanca que **corta la recurrencia de CI**, no solo el colapso de forja.

### Propuestas

- **Barrera de relevo IDE (corta F1 y F4):** en `bug-fix`/`feature`/`refactorization`, si las fases de agente resuelven `simulated` (relevo IDE) y no existe `validacion.md`, **no** encadenar `delivery-close-cycle`; retornar estado `awaiting_agents`/`detached` con acuse limpio (respeta el mandato "detente tras el plan"). Sin push prematuro, CI deja de correr sobre estados incompletos → fin del rojo estructural de `wasi-runtime-smoke`.
- **Skip documental con señal:** que `Cierre documental en rama` skipeado por `validacion.md ausente` corte el pipeline (o emita evento explícito), en vez de dejar avanzar el cierre remoto.
- **Telemetría de forja:** propagar `gh_stdout`/`gh_stderr` (truncados) al envelope de error de `capsule_delivery_gh_pr` para diagnosticar el `pr_url` ausente sin re-ejecutar.
- **Aduana evolution alcanzable en la ruta del orquestador (F4):** el push de `delivery-close-cycle` se auto-exime del gate local (`in_delivery_close_cycle`). Declarar `gate-evolution` como **fase del proceso** `delivery-close-cycle` (AEL-CA9 de `6d64bcc7`, aún pendiente) — vía `entity-manager` + recálculo `hash_signature` — para que **ninguna** ruta de entrega quede exenta, incluida la del propio motor. El hook delega; no duplica.
- **Activación verificada de `core.hooksPath` (F4):** que el arranque de escritura (o `verify-hooks`) **fije** `core.hooksPath` idempotente en vez de solo reportarlo; elimina el punto único de fallo por-clon (`6d64bcc7` §5.1, incumplido en la práctica).
- **Discriminar gate-block de colapso en `emit_dcc_phase_fractures` (F4b / absorbida de `c339de406e29`):** un `blocked` determinista de aduana (`Aduana evolution` → `EVOL_MATERIAL_UNREGISTERED`; `Aduana EDA` → orphans) es **resultado de gate accionable**, no colapso sistémico → **no** debe emitir `System_Fracture_Detected` ni materializar PBI Kintsugi. Restringir la emisión a fallos no-deterministas/de runtime, o degradar el bloque de aduana a señal `telemetry` (mismo criterio aplicado en `1d4115c57471`). Conserva el `status: blocked` y el veredicto accionable.

> Kintsugi transforma la fractura en deuda accionable; el Vértice Biológico valida antes de ejecutar.

## Criterio de cierre

- [x] F1 resuelta: `bug-fix` con fases de agente `simulated` **no** dispara `delivery-close-cycle` (o corta antes de `Apertura en forja`)
- [x] F2 resuelta: skip de `Cierre documental en rama` por `validacion.md` ausente emite señal/corta pipeline
- [x] F3 mitigada: envelope de error de forja incluye `gh_stdout`/`gh_stderr`
- [x] F4 resuelta: sin push prematuro, un ciclo `bug-fix` en relevo IDE **no** produce rojo de `wasi-runtime-smoke` por `EVOL_MATERIAL_UNREGISTERED` (smoke reproducible)
- [x] F4a (refinado v1.2.0): dispatchers con `+x` versionado; `verify-hooks --fix` arma `core.hooksPath` + `chmod` y comprueba bit ejecutable — la aduana local deja de estar dormida por-clon (CA-9/CA-10/CA-11)
- [x] F4c (refinado v1.2.0): `pre_push_gate.sh` corre `gate-evolution --range` antes de la rama DCC; hash evolution inválido → BLOCKED local, no en CI (CA-12)
- [ ] AEL-CA9 (diferido): `gate-evolution` como fase del genoma `delivery-close-cycle` vía `entity-manager`
- [x] F4b: `emit_dcc_phase_fractures` no escala `blocked` de aduana determinista (`Aduana evolution`/`Aduana EDA`) a `System_Fracture_Detected`; conserva veredicto accionable (absorbe `c339de406e29`)
- [x] `c339de406e29` unificada en este PBI (stub `done/` apuntando aquí; diagnóstico erróneo corregido)
- [ ] Rama remota `fix/route-domain-event-fracture-b3a715381787` higienizada vía proceso (no raw)
- [x] Argos APTO en `validacion.md` del fix
- [x] Este TODO movido a `docs/todos/done/`
