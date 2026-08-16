---
feature_name: tekton-fire-and-forget
created: "2026-08-16"
process: feature
branch_name: feat/tekton-fire-and-forget
persist_ref: docs/features/tekton-fire-and-forget
pbi_ref: docs/todos/pending/ARQUITECTURA] Erradicación de esperas síncronas en Tekton (Patrón Fire-and-Forget).md
document_id: PBI-TEKTON-FIRE-AND-FORGET
uuid: 3ad2901a-aaf4-4631-b5df-11386b3ea997
status: blueprint_locked
mayeuta_verdict: ok
dedalo_verdict: ok
laudo: L-CLI-DETACH-ALLOWLIST
execution_id: 57dc7e51-9a48-4b98-a717-191da9070903
---

# Objetivos — tekton-fire-and-forget

## Misión

Erradicar la supervisión síncrona de la IA obrera (Tekton/Cursor) sobre procesos asíncronos del Core. Tras inyectar intención en la Aduana (`./sddia-run.sh` / `execute-process`), el hilo obrero recibe un acuse y se libera; el bus fractal y los centinelas continúan. Patrón **Fire-and-Forget**.

## Punto objetivo

> **O-TEKTON-FAF:** Dada una invocación de proceso SddIA (incluido uno de larga duración), la consola del operador recupera el control en el instante en que el evento queda depositado en el bus fractal (`eda_fractal` / `event_bus` vía Cúmulo: `./.events/{family}/`), con JSON de confirmación y `exitCode` 0; Tekton no inyecta temporizadores ni polling post-acuse; Racso o Kalma2 dictan el siguiente estímulo.

## Alcance estabilizado

| Dentro | Fuera |
|--------|-------|
| Veto normativo a `sleep` / `wait` / bucles de comprobación / `AwaitShell` post-acuse (perímetro D3 de `clarify.md`) | Pasarela HTTP Kalma2 (PBI-044) |
| Contratos: `external-ai-constraints.md`, `agents/tekton.md`; difusión `.cursorrules` / `.cursor/rules/` | Canal PTC/SSE de progreso |
| Acuse CLI al depositar en `./.events/…`, sin join a `event-watcher` ni a la carga larga | Tratar `.SddIA/events/` como cola |
| Relevo de testigos: no encadenar respuestas de procesos **largos** en el mismo hilo | Ticks/backoff internos de daemons y cápsulas |
| Prueba de fricción: `pull-request-review` o `radamanto-batch` | Relevo IDE de fases `simulated` del ciclo `feature` activo |
| Cierre documental en rama (patrón features v1.2.x) | Suciedad de working tree ajena al PBI |

## Requisitos termodinámicos (inyección Dedalo)

1. **V1 — Blindaje ontológico.** La norma motor y el contrato de Tekton declaran de forma explícita e innegociable el veto D3 y el mandato de latencia (éxito = inyección acusada, no = trabajo remoto terminado).
2. **V2 — Corte de retorno en la Aduana.** El CLI operador confirma al depositar el evento en el bus fractal; no espera a centinelas ni a completar `pull-request-review` / `radamanto-batch` (u homólogo largo) en el mismo proceso de primer plano.
3. **V3 — Relevo de testigos.** Tras el acuse, el siguiente paso lo dicta el Vértice Biológico o una interfaz externa (Kalma2). Tekton no vigila el tiempo.
4. **V4 — Bus y centinelas.** Los watchers asumen la carga en background; los artefactos aparecen en destino sin dependencia del hilo de la IA.

## Ley aplicada

- SSOT espacial: `SddIA/core/cumulo.paths.json` (`event_bus`, `eda_fractal`, `directories.agents`, `directories.norms`, `paths.featurePath`).
- Genoma (`agents/`, `norms/`): solo `entity-manager`.
- Touchpoints IDE: difunden `external-ai-constraints.md`; no la contradicen.
- Constitución: Triaje C/A/B; Verdad Objetiva sobre complacencia del LLM que «espera a ver si acabó».
- Códice activo: `codex-software-engineering` / `features-documentation-pattern`.

## Criterios de aceptación (heredados del PBI, laudo D6)

- [ ] Normas y contrato Tekton incluyen veto + Fire-and-Forget.
- [ ] Proceso largo invocado vía Tekton: consola inmediata; Tekton cierra sin esperas post-acuse.
- [ ] Centinelas procesan en background; artefactos independientes del hilo IA.
- [ ] PBI en `docs/todos/done/` y `validacion.md` APTO (`pbi_archived: true`) en el mismo PR.
