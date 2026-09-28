---
document_id: PBI-OPERATIVO-APLICACIONES-FORGE-REDEPLOY
uuid: "1c463a76-c874-4774-bdfa-8be23ab8c877"
title: "[OPERATIVO] Workspace 1×N — redeploy de la instancia forjadora en Aplicaciones"
format: markdown
version: "1.2.0"
status: done
closed: "2026-09-28"
priority: alta
type: operativo
process: null
dispatch: false
historia_ref: "docs/todos/historias/[ARQUITECTURA] Gobierno de proyectos externos desde Kalma2 — Workspace Server (MCP) 1×N.md"
historia_document_id: HU-KALMA2-PROJECT-WORKSPACE-SERVER-1xN
historia_uuid: "d05b8d36-b0b2-494a-893c-52256447266d"
created: "2026-09-28"
author: tekton
blocked_by:
  - PBI-ARQUITECTURA-SOFTWARE-FORGE-GATE
audit_ref: docs/audits/instance-deploy-home-racso-Aplicaciones-Asistencia_Tormentosa_SddIA-20260928T184255Z.md
audit_ref_first_attempt: docs/audits/instance-deploy-home-racso-Aplicaciones-Asistencia_Tormentosa_SddIA-20260928T064112Z.md
purge_audit_ref: docs/audits/purge-legacy-SddIA_AP-20260928T181500Z.md
audit_ref_baseline: docs/audits/installer-deploy-aplicaciones-20260926T113533Z.md
instance_root: /home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA
paciente0_ssot: /home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA
legacy_purge_root: /home/racso/Proyectos/SddIA_AP
requires_human_laudo: true
baseline_decisiones:
  - "El PBI installer v2/v3 está en done/. Este ítem es el laudo de host que aquel PBI dejó fuera del PR."
  - "Paciente 0 = instance_root (Aplicaciones). SddIA_AP descatalogado → purga AC-OP-6."
  - "AC-9 desde Kalma2 de Paciente 0 (PBI-ARQUITECTURA-BX-KALMA2-E2E) cuando host APTO."
---

# Redeploy de Asistencia_Tormentosa_SddIA

Historia madre: G3, §4.F (laudo), fase F5 (host). No es un PR de genoma.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| Hay que esperar a `PBI-ARQUITECTURA-INSTALLER-V2-DESPLIEGUE-LIMPIO` | Ese PBI está en `docs/todos/done/` (`status: done`). Su propio texto deja el redeploy real **fuera del PR**, con laudo del Vértice. |
| `R-PROF-1` deja la instancia con `software_forge: true` | No escribe ese campo. Hace falta el opt-in del PBI `PBI-ARQUITECTURA-SOFTWARE-FORGE-GATE` ya mergeado. |
| Paciente 0 = `SddIA_AP` en Proyectos | **Obsoleto.** Paciente 0 = `Asistencia_Tormentosa_SddIA` en Aplicaciones. `SddIA_AP` descatalogado → purgar (AC-OP-6). |
| `/home/racso/Aplicaciones/SddIA/` es la instancia | Ahí solo hay atajos. La instancia Paciente 0 es `Asistencia_Tormentosa_SddIA`. |

## 1. Intención

Dejar la instancia candidata operativa como forjadora de proyectos externos: WUI con puerto propio, bóveda que no es copia de la forja, unidades que no están en crash loop, perfil explícito con `software_forge: true`.

## 2. Hecho medido (baseline 2026-09-26 — `audit_ref_baseline`)

- `kalma2-bridge` en bucle por `bind` del puerto 8765 ocupado por el bridge de la forja.
- `telegram-watcher` e `iota-publish-relay` en crash loop.
- Bóveda copiada de la forja (secretos duplicados, sin `SDDIA_CLIENT_PORT`).
- Sin `active-domain-profile.json` explícito. Autoridad actual por regla legado, no por declaración.

## 2.1 Post-redeploy (2026-09-28)

- Laudo otorgado; deploy `deploy-20260928T063940Z`.
- Primera verify (`audit_ref_first_attempt`): **NO-APTO** por probe HTTP obsoleto (`/api/status` sin `event_id`).
- Purga legado `SddIA_AP` (`purge_audit_ref`) → AC-OP-6 APTO.
- Re-verify (`audit_ref`): **APTO** — bridge `:8766`, `NRestarts=0`, perfil `software_forge: true`.

## 2.2 Laudo

| Campo | Valor |
|-------|-------|
| Estado | **Otorgado** (Vértice Biológico, 2026-09-28) |
| Alcance | Redeploy/saneo de Paciente 0 (`instance_root`); **purga** de `/home/racso/Proyectos/SddIA_AP` (legado); excluye genoma Core de la forja |
| Disparo | Relanzamiento del proceso de despliegue (2026-09-28 ~08:41 UTC+2) |

## 3. Condiciones para ejecutar

1. `PBI-ARQUITECTURA-SOFTWARE-FORGE-GATE` en `done/` (el installer sabe persistir `software_forge`).
2. Laudo explícito del Vértice Biológico en este PBI (fecha y alcance). Sin ese laudo no hay `teardown` ni `deploy`.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-OP-1 | Laudo escrito en este PBI antes de cualquier comando sobre la instancia. | Frontmatter o sección de laudo. |
| AC-OP-2 | WUI de la instancia escucha en un puerto distinto del de la forja. Cero reinicios por `bind` de 8765. | Acta en `docs/audits/`. |
| AC-OP-3 | `.SddIA/active-domain-profile.json` de la instancia tiene `software_forge: true` y `codex_slug: codex-software-engineering`. | Lectura del fichero. |
| AC-OP-4 | La bóveda de la instancia no es copia de la bóveda global de la forja. `SDDIA_CLIENT_PORT` definido. Cero secretos en el acta. | Diff de claves (nombres, no valores). |
| AC-OP-5 | El plan de deploy de Paciente 0 no reinstala ni muta el árbol legacy `SddIA_AP`; solo Aplicaciones. | Acta. |
| AC-OP-6 | Legado `/home/racso/Proyectos/SddIA_AP` **purgado**: unidades `@…SddIA_AP` inactive/disabled; directorio ausente; sin proceso ni bind de bridge de esa instancia. Prompt: `PBI-DT-PACIENTE0-UNDEPLOY-PROCESS` con `INSTANCE_ROOT` explícito. | Acta + `pgrep`/`systemctl`. |

## 5. Fuera de alcance

- Cambios de genoma. Si el redeploy destapa un defecto, se abre otro PBI; no se parchea el Core desde aquí.
- El ciclo `bug-fix` sobre BarcelonaXplorer (PBI de cierre). Este PBI deja el host listo para repetirlo.

## 6. Dependencias

- `PBI-ARQUITECTURA-SOFTWARE-FORGE-GATE`.
- No bloquea `PBI-ARQUITECTURA-BX-KALMA2-E2E`.
