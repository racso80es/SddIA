---
feature_name: tekton-fire-and-forget
created: "2026-08-16"
process: feature
phases: [T0, T1, T2, T3, T4, T5]
branch_name: feat/tekton-fire-and-forget
persist_ref: docs/features/tekton-fire-and-forget
document_id: PBI-TEKTON-FIRE-AND-FORGET
uuid: 3ad2901a-aaf4-4631-b5df-11386b3ea997
status: blueprint_locked
laudo: L-CLI-DETACH-ALLOWLIST
agents: dedalo
---

# Plan — tekton-fire-and-forget

## Patrones aplicados

| UUID | Título | Dónde |
|------|--------|-------|
| `b6a9ed14-3a0d-4f5b-8444-d1b867335daf` | SSOT paths resolution | Depósito fractal vía `load_fractal_dirs` / `eda_fractal.orchestration`; prohibido path host |

No hay patrón de Domain Events catalogado bajo `directories.patterns` además de SSOT; no se inventa UUID.

## Tareas

| ID | Tarea | Dependencias | Notas |
|----|-------|--------------|-------|
| **T0** | `implementation.md` touchpoints | spec locked | Tekton |
| **T1** | Motor `cli_detach` en `execute-process` + flags `--detach`/`--foreground` + tests | T0 | Engine **no** es genoma DA-2. `[PATTERN-b6a9ed14-3a0d-4f5b-8444-d1b867335daf]` |
| **T2** | DA-5 en `SddIA/norms/external-ai-constraints.md` v1.5.0 + evolution | T1 | Core norms: sin `norm-creator` (L8). Bump SemVer. |
| **T3** | `entity-manager` update `tekton` (Mandato de Latencia / DA-5) | T2 | Genoma `directories.agents` |
| **T4** | Touchpoints `.cursorrules` + `.cursor/rules/` (veto post-acuse) | T3 | No genoma |
| **T5** | Smokes AC2–AC5 + `execution.md` + evolution índice | T1–T4 | Lab detach timing; no PPR GitHub |

## Orden

```text
T0 → T1 → T2 → T3 → T4 → T5
```

T1 es el corte físico de V2. T2–T4 son V1/V3. T5 cierra AC.

## Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Detach de `radamanto-batch` rompe watcher | Fuera de allowlist (L2) |
| Re-detach del hijo | `SDDIA_CLI_FOREGROUND=1` en spawn |
| `agent-creator` update destructivo | Semilla completa; si EM aborta, no forja manual — reportar |
| Working tree sucio Radamanto | No stagear |

## Handoff Tekton

Ejecutar T0–T5 en rama `feat/tekton-fire-and-forget`. Prohibido sleep/polling post-CLI en esta misma ejecución (el motor aún no detacha `feature`).
