---
document_id: PBI-FIX-FRACTURE-c51acf014c0f
uuid: "c51acf01-4c0f-4000-8000-000000000001"
title: "[FIX] delivery-close-cycle — apertura en forja sobre fix sin diseño (barrera simulated)"
format: markdown
version: "1.0.0"
created: "2026-08-29"
updated: "2026-08-29"
status: "abierto"
priority: alta
process: bug-fix
fracture_hash: c51acf014c0f
fracture_process: delivery-close-cycle
friction_id: F-DCC-APERTURA-EN-FORJA
incident_ref: "System_Fracture_Detected — c51acf014c0f"
correlation_hash_source: "no se pudo resolver pr_url desde gh"
related:
  - SddIA/norms/obediencia-procesos.md
  - SddIA/events/domain/system-fracture-detected.md
  - SddIA/engine/execute-process/src/engine/executor.rs
  - SddIA/engine/execute-process/src/engine/phase_capsules.rs
  - SddIA/library/codexes/codex-software-engineering/process/bug-fix.md
  - SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
  - docs/todos/pending/[FIX] route-domain-event — fractura sistémica (b3a715381787).md
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

## Mandato

Corregir la causa raíz del colapso. **Prohibido bypass raw** (`gh`, `git`, `curl`) hasta cierre documentado. Prohibido abrir el PR a mano o `rerun` de `delivery-close-cycle`: eso trataría el síntoma (F3) e ignoraría F1.

## Delimitación (ruido descartado)

| PBI | `error_trace` | Por qué **no** es esta fractura |
|-----|---------------|--------------------------------|
| `c339de406e29` (`delivery-close-cycle`) | `diff material sin evolution correlacionada` | Otra fase (`Aduana evolution`), otro hash. |
| `1d4115c57471` (`bug-fix` / workspace-init) | `dirty-worktree: …` | Otro worktree/sesión; hoy `pbi_ref` dejó pasar el PBI. |
| `b3a715381787` (`route-domain-event`) | `iota-relay-unreachable: … 500` | Fractura **origen** que motivó la sesión; permanece abierta e independiente. |

## Conclusión Analítica y Propuesta Evolutiva

*(Síntesis Tekton — Kintsugi, pendiente enriquecimiento Mayeuta)*

### Diagnóstico de causa raíz

- **Defecto de orquestación**, no de `gh`: `delivery-close-cycle` se ejecuta sobre un ciclo `bug-fix` cuyas fases de agente quedaron `simulated`. El `pr_url` inalcanzable es el punto donde la ausencia de diseño/validación se hace visible, no su causa.

### Veredicto evolutivo

**Corrección de proceso oficial + observabilidad** (`process_fix` + `observability`).

### Propuestas

- **Barrera de relevo IDE:** en `bug-fix`/`feature`/`refactorization`, si las fases de agente resuelven `simulated` (relevo IDE) y no existe `validacion.md`, **no** encadenar `delivery-close-cycle`; retornar estado `awaiting_agents`/`detached` con acuse limpio (respeta el mandato "detente tras el plan"). Evaluar tratar `simulated` como barrera cuando el proceso exige cascada documental mínima.
- **Skip documental con señal:** que `Cierre documental en rama` skipeado por `validacion.md ausente` corte el pipeline (o emita evento explícito), en vez de dejar avanzar el cierre remoto.
- **Telemetría de forja:** propagar `gh_stdout`/`gh_stderr` (truncados) al envelope de error de `capsule_delivery_gh_pr` para diagnosticar el `pr_url` ausente sin re-ejecutar.

> Kintsugi transforma la fractura en deuda accionable; el Vértice Biológico valida antes de ejecutar.

## Criterio de cierre

- [ ] F1 resuelta: `bug-fix` con fases de agente `simulated` **no** dispara `delivery-close-cycle` (o corta antes de `Apertura en forja`)
- [ ] F2 resuelta: skip de `Cierre documental en rama` por `validacion.md` ausente emite señal/corta pipeline
- [ ] F3 mitigada: envelope de error de forja incluye `gh_stdout`/`gh_stderr`
- [ ] Rama remota `fix/route-domain-event-fracture-b3a715381787` higienizada vía proceso (no raw)
- [ ] Argos APTO en `validacion.md` del fix
- [ ] Este TODO movido a `docs/todos/done/`
