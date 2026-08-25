---
feature_name: kaizen-paciente0-redeploy-fricciones
created: "2026-08-25"
process: feature
branch_name: feat/kaizen-paciente0-redeploy-fricciones
persist_ref: docs/features/kaizen-paciente0-redeploy-fricciones
pbi_ref: docs/todos/pending/[KAIZEN] Paciente 0 SddIA_AP — redeploy y fricciones operativas.md
document_id: PBI-KAIZEN-PACIENTE0-REDEPLOY-20260824
uuid: "56aff1d3-d5f6-4502-9b5b-e5a57dc718e3"
execution_id: "c95fa63f-be71-481b-a927-475e7c885fd0"
mayeuta_verdict: ok
laudo: absorber-parches-core-un-pr
phase: mayeuta-stabilization
---

# Objetivos — kaizen-paciente0-redeploy-fricciones

## Misión

Absorber en el Core las fricciones del redeploy Paciente 0 (`SddIA_AP`, 2026-08-24) para que un despliegue consumidor sea reproducible **solo** con bundle + `instance-creator` + ignición, y el correo de reunión con asunto estructural inequívoco cierre Gate G5 (`actionable` → agenda + WUI + Telegram).

Los siete parches ad-hoc aplicados en instancia **no** son SSOT. Deben desaparecer como procedimiento operador.

## Punto objetivo

> **O-PACIENTE0-REDEPLOY:** Una instancia consumidor materializada desde forja `main` arranca con `CORE_ROOT` = raíz de instancia, ELF frescos, `start-sddia.sh` bundle-safe y smoke `route-domain*` `success:true`, sin editar archivos bajo `{instancia}/`. Un correo con reunión + fecha extraíble en asunto no puede quedar `passive` por Clasificacion LLM si la extracción estructural es completa y Triaje-C no marcó `noise`.

## Alcance

| Dentro | Fuera |
|--------|-------|
| F-DEP-03 gate ELF stale / cadena `.py` en centinelas | F-TRIAGE-03 inbox WUI para `passive` (PBI UX separado) |
| F-DEP-01 `CORE_ROOT=instance_root` en systemd | Wizard `DT-CONFIG-UX-ONBOARDING` |
| F-DEP-04 starter-kit completo en Topología (no `local.paths.json` `{}`) | Castrar genoma de ingeniería del lab |
| F-DEP-02 `start-sddia.sh` bundle-safe (Core, no parche instancia) | Reescribir Kalma2 / matriz de tres vías salvo gap Dedalo |
| F-DEP-06 smoke post-ignición `route-domain*` | Dualidad de clase ECST nueva para reunión |
| F-TRIAGE-01 elevación post-LLM por extracción de asunto | Inventar `datetime` si extracción incompleta |
| F-TRIAGE-02 trazabilidad inferencia / no `passive` silencioso | Secretos en git; mutar buzón IMAP |
| F-DEP-05 inventario mínimo consumidor (documental) | Auto-merge mágico de bóvedas ajenas como producto |
| Redeploy smoke Paciente 0 + Gate G5 | Segundo PR documental post-merge |
| Auditoría empírica al cierre (`auditsPath`) | Consolidar parches 2026-08-24 como runbook canónico |

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O1** | Bundle fresco | `build-release-bundle --skip-build` rechazado si cicatriz SHA-256 diverge o testigo ausente, o si `strings` de centinelas contiene `execute-process.py` / cadena `.py` de orquestador. Oráculo = hash de fuentes, **no** mtime. |
| **O2** | Systemd instancia | `ExecStart` de `sddia-email-watcher@%f` resuelve bajo `{instance_root}/SddIA/` sin intervención operador |
| **O3** | Topología | Fase Topología materializa starter-kit; `local.paths.json` no queda `{}` |
| **O4** | Ignición bundle | `./start-sddia.sh` en instancia hermética no invoca `cargo build` si ELF resuelto o `MANIFEST.json` presente / `Cargo.toml` ausente |
| **O5** | Smoke EDA | Post-ignición: ≥1 evento domain de laboratorio enruta `route-domain*` con `success:true` |
| **O6** | Extracción post-LLM | LLM `{"verdict":"passive"}` + asunto reunión con datetime extraíble → `actionable`; `noise` C-* no se eleva |
| **O7** | Test UID 104579 | Caso unitario: patrón asunto `Reunión … dd/mm/yyyy … hh:mm` + mock LLM `passive` → `actionable` |
| **O8** | Inferencia | Proof con `tokens_in/out` documentados si LLM activo, o flag `classification-degraded` si inferencia omitida; `SDDIA_LLM_REQUIRE_INFER=1` no emite `passive` silencioso |
| **O9** | Redeploy | Paciente 0 re-desplegado sin parche local a `{instancia}/start-sddia.sh`; Gate G5 reunión |
| **O10** | Auditoría | Documento bajo `docs/audits` con bitácora empírica Paciente 0 + Kaizen absorbido |
| **O11** | Cierre | Un PR: PBI en `docs/todos/done/` + `validacion.md` `global: APTO` `pbi_archived: true` |

## Prioridad de veredicto correo (invariante)

```text
Triaje-C noise  >  extracción estructural completa de asunto  >  veredicto LLM ambiguo/passive
```

Alineado a `email-triage-matrix` §1 desempate y §2 (C-* concluye). Prohibido que Clasificacion **degrade** una señal estructural inequívoca (reunión + `datetime` extraíble).

## No objetivos

- Superficie WUI de historial `passive` (F-TRIAGE-03).
- Wizard de onboarding de bóveda.
- Rehabilitar o reabrir el PBI de ignición consumidor Filtro C salvo deuda residual `AGENT_RUNTIME_*` en `.dev/` raíz (fuera de este ciclo salvo mención documental).
- Tratar los siete pasos manuales del redeploy 2026-08-24 como procedimiento canónico.

## Ley aplicada

- `features-documentation-pattern` v1.2.1 / proceso `feature` v1.3.2
- `external-ai-constraints` DA-2…DA-5
- `sddia-distribution-protocol` v1.1.0 (Vía C; evolucionar si Dedalo demuestra gap)
- `email-triage-matrix` v1.0.0 (consumir; mutar solo si Dedalo prueba hueco normativo)
- `capsule-json-io` v2.0
- Cierre documental en rama (un PR)
- Clarificaciones D0–D11 en `clarify.md` (laudo **absorber-parches-core-un-pr**)

## Orden de forja (semilla PBI §7 — Dedalo no lo invierte)

```text
(1) F-DEP-03 gate bundle + rebuild centinelas
(2) F-DEP-01 + F-DEP-04 instance-creator
(3) F-DEP-02 start-sddia bundle-safe
(4) F-TRIAGE-01 extracción asunto post-LLM
(5) F-TRIAGE-02 auditoría mayeuta-llm + REQUIRE_INFER
(6) Redeploy smoke Paciente 0 + Gate G5
(7) F-TRIAGE-03 UX — fuera de este persist_ref
```

Mutación de genoma (`directories.process` / normas) vía `entity-manager`. Handler `instance_creator.rs` y `email_triage.rs` viven en `SddIA/engine/` (fuera de DA-2). Scripts de bundle/ignición en `SddIA/scripts/` y raíz: Dedalo fija el locus.

## Entregable de cierre adicional (PBI §9)

Auditoría empírica de despliegue Paciente 0: fricciones Kaizen, métricas del snapshot 2026-08-24, cadena causal UID 104579, y qué quedó absorbido en Core. Ruta lógica: `paths.auditsPath` (`docs/audits`).
