---

feature_name: man-53-apto-coherence-root-clean
created: "2026-11-22"
process: feature
branch_name: feat/man-53-apto-coherence-root-clean
persist_ref: docs/features/man-53-apto-coherence-root-clean
tracker_ref: MAN-53
parent_ref: MAN-43
document_id: PBI-90403a29
execution_id: "a3971880-ee8b-49c3-97d4-40f61477aa47"
---
# Objetivos — man-53-apto-coherence-root-clean

## Misión

Endurecer la aduana de coherencia documental (`apto_coherence`), `pull-request-review`, `delivery-close-cycle` y `accept-pr` para impedir que se selle `global: APTO` o se cierre ciclo con matrices de ACs parciales/abiertas, checks troncales en `NO_APTO` (sin degradación a `non_blocking_findings`) o directorios efímeros de topología de bus en la raíz del repositorio (incluso vacíos, invisibles a `git status --porcelain`).

## Alcance y Criterios de Aceptación

- **AC-1 (`KA-AC-PARTIAL-1` — Regla C5):** `apto_coherence` rechaza `global: APTO` si alguna fila de la matriz de ACs/CAs en `execution.md` o `validacion.md` está en `PARCIAL`, `FAIL`, `NO_APTO`, `ABIERTO` o `PENDIENTE` (`apto_coherence_ac_partial`).
- **AC-2 (`KA-GATE-TRUNK-2` — Checks troncales no degradables):** `apto_coherence` y `pull-request-review` rechazan `global: APTO` si cualquiera de `F2_DOC_GATE`, `F3_TECH_GATE`, `F4_RBAC_GATE`, `ROOT_TOPOLOGY_CLEAN` o `DOC_HONESTY_*` figura como `NO_APTO` en `checks:` — sin reclasificarlos a `non_blocking_findings`.
- **AC-3 (`KA-ROOT-PHYS-3` — Ausencia física de topología efímera en raíz):** `delivery-close-cycle` (Gate pre-PR QA) y `accept-pr` verifican que en la raíz del repositorio no existan, ni vacíos, los directorios `.events`, `orchestration`, `telemetry`, `pending`, `domain`, `dead-letter`, `progress`, `_lab`; el rechazo es fricción accionable (sin `System_Fracture_Detected`).
- **AC-4 (`KA-CLOSE-4` — Contrafactual MAN-50):** Contrafactual verificado por test y documentado: con C5 + veto troncal + check físico de raíz activos, el cierre de MAN-50 no habría sellado `global: APTO` bajo las condiciones de H-NEW-06.
- **AC-5 (Regresión y CI):** `cargo test` y `cargo clippy -- -D warnings` en `execute-process` verdes; CI de GitHub en verde.

## Fuera de alcance

- Remediación de residuos `.events` en código/contrato (MAN-51).
- Paridad runtime/daemons (MAN-52).
- Ergonomía `entity-manager` (MAN-54).
- Prompts/oráculos de agentes (MAN-55).
