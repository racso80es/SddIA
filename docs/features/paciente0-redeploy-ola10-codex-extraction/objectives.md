---
feature_name: paciente0-redeploy-ola10-codex-extraction
created: "2026-09-25"
process: feature
branch_name: feat/paciente0-redeploy-ola10-codex-extraction
persist_ref: docs/features/paciente0-redeploy-ola10-codex-extraction
pbi_ref: docs/todos/pending/[OPERATIVO] Paciente 0 SddIA_AP — redeploy ola 10 post-extracción del códice de ingeniería de software.md
document_id: PBI-OPERATIVO-PACIENTE0-REDEPLOY-OLA10-CODEX-EXTRACTION
uuid: "26df818b-f3f6-4a91-b228-84b065ad49df"
execution_id: "3098fa27-fb71-48e7-8ee2-aab2bee2c468"
mayeuta_verdict: ok
phase: mayeuta-stabilization
---

# Objetivos — paciente0-redeploy-ola10-codex-extraction

## Misión

Redeploy wipe de Paciente 0 (`SddIA_AP`) como **ola 10**: instancia consumer viva con bundle post-FIX (13 ELF, Filtro C, cadena Tormentosa), perfil de dominio declarado, combustión Aiúa medible, y cristalización en forja (audit + DEUDA v1.7.0) en **un** PR.

## Punto objetivo

> **O-OLA10:** `/home/racso/Proyectos/SddIA_AP` arranca por Vía C (`build-release-bundle --profile consumer --codex codex-kalma2-assistant` + `instance-creator` v1.4.0 `skip_ignition` + `start-sddia.sh` systemd) sin parche local a `{instancia}/SddIA/`. Gates DEUDA §5 + PBI §5 APTO (o residual declarado). `POST /api/aiua/interact` → `success:true` con `provider` ∈ {`antigravity-cli-executor`,`gemini-http-infer`}. Audit `docs/audits/paciente0-deploy-{STAMP}.md` ancla ola 10. `PBI-DT-PACIENTE0-DEPLOY-PROCESS` pasa a v1.7.0.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Vault staging Filtro C + `SDDIA_GEMINI_MODEL` | Forjar `paciente0-deploy` |
| Bundle 13 ELF; G-bundle v2 / G4 v2 | `mayeuta-llm` en `CONSUMER_BINS` |
| Creator: `local.paths` ≠ `{}`; `llm-registry.json`; `active-domain-profile.json` | Plano B DLT Testnet |
| Ignición systemd `@%f`; G3b v2 LogRateLimit (email incluida) | G5 First Blood por defecto |
| G3 v2 combustión; G-eda-roots; G-authority; G-llm-registry; G-agy-host | Migración `.SddIA/projects/` |
| Audit ola 10 + matriz + veredicto | Anclaje retrospectivo ola 9 |
| DEUDA v1.7.0 (prompt, §0bis, §2, §4.2, §5, §10) | Parche `{instancia}/SddIA/` |
| Un PR + CI verde + `accept-pr` | Segundo PR documental |

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O-PREREQ** | FIX en `main` | `CONSUMER_BINS=13`; `instance-creator` v1.4.0; PBI FIX en `done/` con `validacion.md` APTO |
| **O-BUNDLE** | Artefacto hermético | 13 ELF; `library/codexes/` exacto; 0 procesos ciclo software; 0 `codex-software-engineering*`; log `[bundle] OK` |
| **O-CREATOR** | Materialización | `llm-registry.json` y `active-domain-profile.json` por creator, no a mano (salvo fricción registrada) |
| **O-TORMENTOSA** | Latido | G3 v2 `success:true` + `Thought_Persisted`; o degradado declarado si ambos oráculos fallan |
| **O-AUTHORITY** | Ceguera espacial | `./sddia-run.sh --process feature --inputs '{}'` desde instancia → fallo limpio; nunca `workspace-init` |
| **O-EDA** | Roots ausentes | Ignición sin panic/`cannot read`; `PBI_Forged` sintético → dead-letter `no_subscriber` |
| **O-OLA9** | Fricciones absorbidas | `F-BUNDLE-HERMETIC-DAEMON-RESOLVER` y `F-BUNDLE-LIBRARY-FILTRO-C` APTO en runtime |
| **O-AUDIT** | Cicatriz | `docs/audits/paciente0-deploy-{STAMP}.md` con § desacople códice y § Latido Aiúa; cero secretos |
| **O-DEUDA** | Prompt SSOT | DEUDA v1.7.0: ola 10→11, 13 ELF, gates v2, errata ola 9 |
| **O-CIERRE** | Un PR | PBI en `done/` + `validacion.md` APTO `pbi_archived: true` + CI verde + `accept-pr` |

## Ley aplicada

- `features-documentation-pattern` v1.2.1 / proceso `feature` v1.3.2
- `sddia-distribution-protocol` + `instance-creator` v1.4.0
- `external-ai-constraints` DA-2…DA-6
- Cierre documental en rama (un PR)
- Clarificaciones en `clarify.md` (Q1/Q2/Q3 + L-*)
