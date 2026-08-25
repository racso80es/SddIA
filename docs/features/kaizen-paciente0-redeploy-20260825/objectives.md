---
feature_name: kaizen-paciente0-redeploy-20260825
created: "2026-08-25"
process: feature
branch_name: feat/kaizen-paciente0-redeploy-20260825
persist_ref: docs/features/kaizen-paciente0-redeploy-20260825
pbi_ref: docs/todos/pending/[KAIZEN] Paciente 0 SddIA_AP — redeploy 20260825 y fricciones.md
document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260825
uuid: "d4f13e9a-5d91-4ab8-a2f5-be2e6b8c4815"
execution_id: "7fd0a353-d2fe-4895-8abe-d7f5b34f652c"
mayeuta_verdict: ok
laudo: absorber-fricciones-post-absorcion-un-pr
phase: mayeuta-stabilization
---

# Objetivos — kaizen-paciente0-redeploy-20260825

## Misión

Absorber en el Core las fricciones **nuevas** del redeploy Paciente 0 (`SddIA_AP`, 2026-08-25) tras el Kaizen T6 mergeado, de modo que un despliegue consumidor sea reproducible con **un** `instance-creator` (sin unlink de overlay ni pin ELF) más ignición que resuelva el orquestador del **bundle de instancia**, no el de la forja.

Las correcciones ad-hoc del 2026-08-25 (pin `SDDIA_EXECUTE_PROCESS_BIN`, `unlink` de `{}`, herencia de env de sesión) **no** son SSOT.

## Punto objetivo

> **O-PACIENTE0-REDEPLOY-25:** Con ELF = genoma actual, `_sddia_resolve_orchestrator` no elige debug stale frente a release fresco; `instance-creator` sustituye `local.paths.json` vacío; `start-sddia.sh` en instancia no hereda pin de forja; smoke no dead-lettera `Local_QA_Requested` por `payload.branch` ausente (o no emite esa clase). Un redeploy Paciente 0 con un solo creator deja `ExecStart` bajo `{instancia}/` y overlay no vacío.

## Alcance

| Dentro | Fuera |
|--------|-------|
| F-DEP-07 resolución debug vs release / cicatriz | F-TRIAGE-01/02/03 |
| F-DEP-08 stub `{}` no es overlay válido | Gate G5 correo reunión (T6) |
| F-DEP-09 aislamiento `SDDIA_EXECUTE_PROCESS_BIN` en ignición instancia | Wizard `DT-CONFIG-UX-ONBOARDING` |
| F-SMOKE-01 `Local_QA_Requested` ECST-completo o no emitir | Auto-merge de bóvedas (laudo D2 antecesor) |
| F-SYS-01 opcional `install_user_unit` (no bloquea Done) | Reabrir F-DEP-01…04 en handler release |
| Redeploy smoke Paciente 0 — un creator | Consolidar parches 2026-08-25 como runbook |
| Auditoría empírica **nueva** (`auditsPath`) | Reescribir audit T6 `kaizen-paciente0-redeploy-20260825.md` |

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O1** | Resolver orquestador | `_sddia_resolve_orchestrator` no selecciona debug más viejo que release / cicatriz vigente (F-DEP-07) |
| **O2** | Overlay | `instance-creator` reemplaza `local.paths.json` cuyo contenido es `{}` / vacío (F-DEP-08) |
| **O3** | Creator único | Redeploy Paciente 0 con **un** `instance-creator` (sin unlink ni pin ELF) → `ExecStart` bajo `{instance_root}/` y overlay no vacío |
| **O4** | Ignición | `start-sddia.sh` en instancia resuelve ELF del bundle aunque el entorno operador tenga `SDDIA_EXECUTE_PROCESS_BIN` de la forja (F-DEP-09) |
| **O5** | Smoke ECST | Creator: no dead-letter por `payload.branch` ausente en `Local_QA_Requested`, o no emite esa clase (F-SMOKE-01) |
| **O6** | Systemd user | Opcional: `install_user_unit=true` copia/enable unidad user. Ausencia no impide APTO |
| **O7** | Operador bóveda | Huecos Telegram + extras IMAP + perfil consumer/systemd en `.dev` — **ya cerrado** (PBI Fase E) |
| **O8** | Auditoría | Documento nuevo bajo `docs/audits` (no duplicar T6) |
| **O9** | Cierre | Un PR: PBI en `docs/todos/done/` + `validacion.md` `global: APTO` `pbi_archived: true` |

## Invariante

F-DEP-01…04 absorbidos en el **handler release** no regresionan si el ELF ejecutado es el genoma actual. La 1.ª pasada 2026-08-25 falló por **debug stale**, no por reversión de T2.

## No objetivos

- Re-auditar G5 / UID reunión.
- Inbox WUI para `passive`.
- Wizard de onboarding.
- Purgar `AGENT_RUNTIME_*` de `/home/racso/Proyectos/.dev` (bóveda operador; Filtro C: se poda solo en bóveda instancia).
- Tratar los seis pasos manuales del PBI §1.2 como procedimiento canónico.

## Ley aplicada

- `features-documentation-pattern` v1.2.1 / proceso `feature` v1.3.2
- `external-ai-constraints` DA-2…DA-5
- `sddia-distribution-protocol` v1.2.0 (Vía C)
- `events-contract` + clase `local-qa-requested` (`payload.branch` REQUIRED)
- `capsule-json-io` v2.0
- Cierre documental en rama (un PR)
- Clarificaciones D0–D12 en `clarify.md` (laudo **absorber-fricciones-post-absorcion-un-pr**)

## Orden de forja (semilla PBI §5 — Dedalo no lo invierte)

```text
(1) F-DEP-07 resolver orquestador
(2) F-DEP-08 overlay {} no es no-op
(3) F-DEP-09 aislamiento env ignición
(4) F-SMOKE-01 ECST Local_QA
(5) F-SYS-01 opcional
(6) Redeploy smoke Paciente 0 — un instance-creator
```

Mutación de genoma (`directories.process`) vía `entity-manager`. Handler `instance_creator.rs` vive en `SddIA/engine/` (fuera DA-2). Scripts `sddia_shell_lib.sh` / `start-sddia.sh`: Dedalo fija el locus.

## Entregable de cierre adicional (PBI §7)

Auditoría empírica del redeploy 2026-08-25: F-DEP-07…, métricas snapshot, qué quedó absorbido en Core. Ruta lógica: `paths.auditsPath` (`docs/audits`). Nombre distinto al audit T6.
