---
feature_name: aut-76-state-map-validated
created: "2026-11-06"
process: bug-fix
phase: design
agents: dedalo
branch_name: fix/aut-76-state-map-validated
sighting_branch: fix/aut-76-state-map-validated
persist_ref: docs/fixes/aut-76-state-map-validated
pbi_ref: AUT-76
document_id: PBI-222b4def
uuid: 222b4def-325a-4645-80d3-fbb15bf7d34b
execution_id: "6c28414f-8a36-4f66-b9e3-4969dc642978"
status: dedalo_locked
init_note: "bug-fix denegado por la misma DA; salto táctico autorizado (planificación en rama)"
---

# Spec — aut-76-state-map-validated

## 1. Misión

Admitir la clave canónica `validated` en la aduana de manifiesto. Restaurar `refine-item` (y todo `./sddia-run.sh`) en Core-self. El manifiesto `.SddIA/project.md` **no** se toca.

## 2. Problema verificado

| Hecho | Evidencia |
| --- | --- |
| `refine-item` AUT-76 muere en Domain Authority | `DOMAIN_AUTHORITY_DENIED: PROJECT_CONFIG_INVALID: tracker.state_map clave desconocida 'validated'` |
| `bug-fix` de este expediente muere igual | mismo error; `fracture_deposited: true` |
| Manifiesto Core-self declara `validated: Validated` | `.SddIA/project.md` (laudo AUT-75 L-STATE) |
| El conjunto cerrado del validador no incluye `validated` | `TRACKER_STATE_KEYS` en `project_binding.rs` |
| La columna ya existe en el ciclo | `COLUMN_VALIDATED`, `lifecycle_phase_action` |
| Anti-retroceso ignora `validated` | `state_rank` en `tracker_sync_replay.rs` (rango `-1`) |
| Contrato desfasado | `project-config-contract` v1.4.0: conjunto sin `in_validation` ni `validated` |
| Auditoría 1.3.0 se contradice | encabezado nombra Validated; tabla §3.4 salta de `in_validation` (3) a `in_review` (4) |

## 3. Laudos

| Ref | Decisión |
| --- | --- |
| **L-KEEP** | `.SddIA/project.md` permanece. Quitar `validated` desharía AUT-75 L-STATE. |
| **L-KEYS** | Añadir `validated` a `TRACKER_STATE_KEYS` entre `in_validation` e `in_review`. |
| **L-RANK** | `validated` = 4; `in_review` = 5; `done` = 6; `cancelled` = 100. |
| **L-CONTRACT** | Ampliar `tracker.state_map` en `project-config-contract` con `in_validation` y `validated`. Bump de versión. Mutación vía `entity-manager` **después** de que DA acepte el manifiesto (T1 desbloquea T4). |
| **L-AUDIT** | Insertar fila `validated` en `ciclo-vida-software-sddia.md` §3.4 y correr rangos. |
| **L-EVOL** | Registrar hito en `SddIA/evolution/` ligado a este `document_id`. |
| **L-LINEAR** | Sin ficheros nuevos en `docs/todos/`. `pbi_ref: AUT-76` es el estímulo de descubrimiento. |
| **L-JUMP** | `bug-fix` no puede inicializar el workspace mientras DA rechace `validated`. Planificación sellada en rama por relevo IDE. Ejecución T1 no pasa por orquestador hasta que el binario local acepte el manifiesto. |

## 4. Criterios de aceptación

1. Manifiesto Core-self con `state_map.validated: Validated` pasa `bind` / Domain Authority.
2. Clave ajena en `state_map` sigue siendo `PROJECT_CONFIG_INVALID`.
3. `state_rank`: `in_validation` < `validated` < `in_review` < `done`. Un `in_validation` rezagado se descarta si el ítem está en `validated`; un `validated` rezagado se descarta si está en `in_review`.
4. `project-config-contract` declara el conjunto `{backlog, todo, in_progress, in_validation, validated, in_review, done, cancelled}`.
5. §3.4 de la auditoría lista `validated` con rango 4.
6. `./sddia-run.sh --process refine-item --inputs '{"item_ref":"AUT-76","project_slug":"core"}'` ya no falla por `clave desconocida 'validated'`.
7. Tests `cargo test -p execute-process --lib` cubren binding y `state_rank`.
8. `validacion.md` APTO en esta rama. Un único PR.

## 5. Fuera de alcance

- Cuerpo / DoR de AUT-76 (refino Mayeuta).
- Alta de estado de workflow Linear (operador; L-STATE AUT-75).
- Quitar o renombrar claves del manifiesto.
- Reprocesar históricos del bus.
- Ficheros nuevos en `docs/todos/`.
