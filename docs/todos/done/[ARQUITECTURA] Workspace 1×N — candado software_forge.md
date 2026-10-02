---
document_id: PBI-ARQUITECTURA-SOFTWARE-FORGE-GATE
uuid: "23cfb176-8394-4d98-8ec8-8dbfc4874f16"
title: "[ARQUITECTURA] Workspace 1×N — candado software_forge"
format: markdown
version: "1.0.0"
status: done
priority: alta
type: arquitectura
process: feature
dispatch: false
feature_name: software-forge-authority
historia_ref: "docs/todos/historias/[ARQUITECTURA] Gobierno de proyectos externos desde Kalma2 — Workspace Server (MCP) 1×N.md"
historia_document_id: HU-KALMA2-PROJECT-WORKSPACE-SERVER-1xN
historia_uuid: "d05b8d36-b0b2-494a-893c-52256447266d"
created: "2026-09-28"
author: tekton
unblocks:
  - PBI-OPERATIVO-APLICACIONES-FORGE-REDEPLOY
  - PBI-ARQUITECTURA-BX-KALMA2-E2E
installer_contract_ref: SddIA/library/norms/sddia-installer-contract.md
installer_contract_version_actual: "1.3.0"
installer_contract_version_objetivo: "1.4.0"
baseline_decisiones:
  - "D3: allow con project_slug solo si project.codex_slug es codex-software-engineering Y software_forge es true Y el perfil de instancia no es consumer"
  - "software_forge vive en active-domain-profile.json; default false; la bóveda no concede autoridad"
  - "Perfil engineering NO implica software_forge true"
  - "Sin project_slug se conserva has_software_authority (comentario interno del .rs dice D4; no es la D4 de la historia)"
---

# Candado software_forge

Historia madre: §4.F (solo el código), fase F5 menos el laudo de host, D3, AC-8, AC-11. El redeploy de `Asistencia_Tormentosa_SddIA` es `PBI-OPERATIVO-APLICACIONES-FORGE-REDEPLOY`.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| `R-PROF-1` ya escribe `software_forge` | `R-PROF-1` está entregado (installer v2, PBI en `done/`). Escribe `codex_slug` y `git_required`. `rg software_forge` en el repo solo da la historia. El campo no existe. |
| El installer vigente es el de aquel PBI | `sddia-installer-contract` está en **1.3.0** (v3 de E/S y UX ya mergeado). El bump de este PBI sale de 1.3.0. |
| Perfil `engineering` debe nacer con `software_forge: true` | No. Eso daría forja sobre proyectos ajenos a toda instancia engineering. Default **false**. Opt-in explícito. |
| «Regla D4 legado» | Es `has_software_authority` en `domain_authority.rs`. El comentario del fuente la llama D4. La D4 de la historia es la bóveda. No mezclar. |
| Reinstalar `SddIA_AP` consumer para forjar | Prohibido. Paciente 0 = Aplicaciones; legado `SddIA_AP` descatalogado. Perfil `codex-kalma2-assistant` sigue en `DOMAIN_AUTHORITY_DENIED`. |

## 1. Intención

Con `inputs.project_slug`, el proceso software solo corre si el proyecto es de códice software **y** la instancia declara que puede forjar proyectos externos. Sin slug, la autoridad de hoy no se mueve. El installer aprende a materializar el candado; no redeploya ninguna instancia viva.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-AUTH-1 | Con `project_slug`: `allow` ⇔ `project.codex_slug == codex-software-engineering` **y** `software_forge == true` **y** `codex_slug` de instancia distinto de `codex-kalma2-assistant` (y de cualquier slug no software). Si no, `DOMAIN_AUTHORITY_DENIED`. |
| R-AUTH-2 | Campo ausente o `false` ≡ denegado para proyecto externo. |
| R-AUTH-3 | Sin `project_slug`: `has_software_authority` intacta (`git_required` legado o slug software del propio Core). |
| R-AUTH-4 | Instancia `codex-kalma2-assistant` denegada siempre para procesos de `process_membership` software, haya o no `project_slug`. |
| R-INST-1 | `instance-creator` persiste `software_forge` en `.SddIA/active-domain-profile.json`. Default `false` en consumer y en engineering. `true` solo con opt-in de inputs (`software_forge: true`). No lee la bóveda para este campo. |
| R-NORM-1 | `sddia-installer-contract` 1.3.0 → **1.4.0** por cadena de entidad. Documenta el opt-in y que engineering no lo activa solo. |

## 3. Plan

1. Init `feature` `software-forge-authority`. Sin espera de otros PBI de esta historia: `codex_slug` ya es obligatorio en el contrato 1.0.0.
2. Tests de matriz en `domain_authority.rs` antes de tocar el installer.
3. `instance-creator` + bump de norma.
4. Cierre documental en la misma rama. Deploy/teardown de Paciente 0 (Aplicaciones) y purga de `SddIA_AP` quedan en el PBI operativo de la HU 1×N.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-11 | `software_forge: true` + `project.codex_slug` software → allow. `software_forge` false o ausente → deny. `project.codex_slug` ≠ software → deny aunque la instancia sea forjadora. | Matriz 2×2×2. |
| AC-8 | Perfil `codex-kalma2-assistant` → `DOMAIN_AUTHORITY_DENIED`. | Test de perfil. |
| AC-7 (autoridad) | Sin `project_slug`, los tests legado (`git_required`, slug explícito, slug ajeno) siguen en verde. | Tests actuales de `domain_authority.rs`. |
| AC-F5-1 | Dry-run de installer: engineering sin opt-in escribe `software_forge: false`. Con opt-in, `true`. Consumer ignora el opt-in y queda `false`. | Smoke installer. |

## 5. Fuera de alcance

- Parar, redeployar o tocar la bóveda de `/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA`.
- Reinstalar o mutar el legado `SddIA_AP` (descatalogado).
- Servidor MCP, UI, runtime.
- Poner `software_forge: true` en el perfil de la forja de laboratorio: lo hace el PBI de ciclo real si hace falta para AC-9, no este.

## 6. Dependencias

- Ningún PBI de esta historia. Desbloquea el redeploy operativo y el ciclo real.
