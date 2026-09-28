---
document_id: PBI-ARQUITECTURA-BX-KALMA2-E2E
uuid: "f24ca269-ee5c-4ab7-b931-b66439bed985"
title: "[ARQUITECTURA] Workspace 1×N — ciclo bug-fix real sobre BarcelonaXplorer"
format: markdown
version: "1.1.0"
status: pending
priority: alta
type: arquitectura
process: bug-fix
dispatch: false
historia_ref: "docs/todos/historias/[ARQUITECTURA] Gobierno de proyectos externos desde Kalma2 — Workspace Server (MCP) 1×N.md"
historia_document_id: HU-KALMA2-PROJECT-WORKSPACE-SERVER-1xN
historia_uuid: "d05b8d36-b0b2-494a-893c-52256447266d"
created: "2026-09-28"
author: tekton
blocked_by:
  - PBI-ARQUITECTURA-WS-PILOT-REGISTRY
  - PBI-ARQUITECTURA-FS-MANAGER-PHYSICAL
  - PBI-ARQUITECTURA-WS-SERVER
  - PBI-ARQUITECTURA-AGENT-RUNTIME-MCP
  - PBI-ARQUITECTURA-KALMA2-PROJECT-SLUG
  - PBI-ARQUITECTURA-SOFTWARE-FORGE-GATE
paciente0_instance_root: /home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA
baseline_decisiones:
  - "AC-9 se demuestra desde Kalma2 de Paciente 0 (Aplicaciones), con software_forge true en esa instancia"
  - "Host APTO (WUI + bridge) es prerequisito; PBI operativo cierra G3 residual"
  - "El bug-fix muta BarcelonaXplorer; el cierre en el repo SddIA es la evidencia, no un segundo cambio de genoma"
---

# Ciclo real sobre el piloto

Historia madre: §4.H, fase F6, AC-9. Último PBI de código de la historia. No añade superficie: prueba las anteriores juntas.

## 0. Filtro A

| Afirmación | Corrección |
|------------|------------|
| AC-9 solo en la forja de lab | **Obsoleto (HU 1.3.6).** Paciente 0 = `/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA`. AC-9 desde su Kalma2 con `software_forge: true`. Requiere host APTO (`wui_http` verde en acta operativa). |
| El `bug-fix` abre un PR en el repo SddIA | El mutado es BarcelonaXplorer, según su `delivery_mode`. Este repo solo archiva evidencia (`validacion.md` + este PBI a `done/`). Si ese cierre no cabe en el PR del cambio de BX (repos distintos), el PR de SddIA es único y solo documental: no hay un segundo PR de genoma. |
| Hay que encender Antigravity o Cursor IDE | AC-9 exige los dos cerrados. El backend LLM headless (`SDDIA_AGENT_RUNTIME_COMMAND`) sigue siendo necesario. |

## 1. Intención

Un `bug-fix` elegido sobre BarcelonaXplorer, lanzado desde Kalma2 de **Paciente 0** (Aplicaciones), que modifica un fichero del piloto, pasa `qa_gates` y cierra según `delivery_mode`, con la aduana MCP como única mano sobre el árbol.

## 2. Requisitos

| ID | Requisito |
|----|-----------|
| R-E2E-1 | Perfil de Paciente 0 (`active-domain-profile.json` en Aplicaciones) con `software_forge: true` durante la prueba. Al terminar, el PBI registra si ese valor queda o se revierte. No reinstalar legado `SddIA_AP`. |
| R-E2E-2 | Estímulo desde la UI (selector `barcelonaxplorer`, Forjar Proceso, proceso `bug-fix`). El slug del evento coincide. |
| R-E2E-3 | El diff del piloto proviene de `tools/call` auditados (`Raw_Execution_Finished`). Cero escritura con `cwd` del Core o `--add-dir` al proyecto. |
| R-E2E-4 | `qa_gates` del manifiesto en verde. Cierre = `delivery_mode` del manifiesto (`branch_pr` o `trunk_direct`). |
| R-E2E-5 | Antigravity y Cursor IDE cerrados durante la ejecución. El CLI de runtime puede estar vivo. |
| R-E2E-6 | `validacion.md` del ciclo: `global: APTO`, enlaces al commit o PR de BX, `pbi_archived: true`. |

## 3. Plan

1. Los seis PBI de `blocked_by` en `done/`.
2. Init `bug-fix` con `inputs.project_slug=barcelonaxplorer` desde Kalma2, no desde el IDE.
3. Recoger evidencia. Archivar este PBI en el PR de evidencia de la forja.
4. Evolución con el uuid de la historia y el de este PBI.

## 4. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-9 | Fichero de BX modificado, `qa_gates` verdes, commit o PR según `delivery_mode`, IDEs cerrados. | `validacion.md`. |
| AC-1 (cierre) | Repetición del `rg`: ni UI ni payload de agente contienen el `project_root` literal. | Evidencia adjunta. |
| AC-4 (cierre) | El diff tiene `Raw_Execution_Finished` correlacionado. | Telemetría. |

## 5. Fuera de alcance

- Nuevas tools, cambios de contrato o de autoridad salvo el opt-in temporal de R-E2E-1.
- Ejecutar el redeploy/purga operativo (ese PBI entrega el host; este PBI consume el host APTO).

## 6. Dependencias

Los seis PBI de código de `blocked_by`. **Recomendado:** `PBI-OPERATIVO-APLICACIONES-FORGE-REDEPLOY` APTO (WUI + purga legacy) antes de declarar AC-9.
