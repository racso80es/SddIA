---
feature_name: inmunidad-caos-fase0
created: "2026-05-28"
base: objectives.md
scope: SddIA/ + SddIA/scripts/qa/ + .SddIA/ + cumulo.paths.json + suscripciones fractal
process: feature
---

# Especificación — Metodología de barrido Fase 0

## Objetivo técnico

Detectar acoplamientos, deudas y puntos ciegos del programa **Inmunidad / Caos S+ Grade** sobre el Core **post-Telemetría Reactiva**, sin mutar código productivo.

## Plantilla de hallazgo

| Campo | Descripción |
|-------|-------------|
| **ID** | `Hnn` secuencial |
| **Ubicación** | Ruta o artefacto SSOT |
| **Hallazgo** | Descripción factual |
| **Fase PBI** | 1–5 afectada |
| **Severidad** | bloqueante / alto / medio / informativo |
| **Gap** | (a) cubierto · (b) ampliar · (c) nueva subtarea · (d) fuera de alcance |

## Áreas obligatorias (0.A)

1. Familia ED `Suite` — SSOT, creators, entity-manager, sync-entity-index, entidades-dominio norm
2. Tools ofensivas — catálogo, contrato, RBAC, cápsulas, telemetría
3. Sandbox — `workspace_path`, `filesystem-manager`, `fix-tool-process`, `radamanto.sandbox_root`
4. Procesos audit — patrón proceso + tool + Argos; atomicidad
5. Orquestación — `invoke_subprocess_process`, workspaces anidados, timeouts
6. Eventos — genoma domain, suscripciones, emisores autorizados
7. Radamanto / DLT — jurisdicción actual vs. `System_Immunity_Certified`
8. Telemetría ↔ caos — Peaje fail-soft, compliance audit, fan-out
9. Laboratorio — handlers, tests QA, flags lab

## Herramientas de barrido

- Búsqueda estructural (`rg`) sobre `SddIA/`, `SddIA/scripts/qa/`
- Lectura SSOT: `cumulo.paths.json`, `event-*-subscriptions.json`, contratos `*-contract.md`
- Contraste con tareas PBI Fases 1–5 y axiomas transversales

## Salida canónica

`impact-analysis.md` con resumen ejecutivo, tabla H01–Hnn, matrices temáticas, decisiones D0.x y autodiagnóstico AC0.1–AC0.5.
