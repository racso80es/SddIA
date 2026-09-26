---
feature_name: destilacion-sddia-installer
created: "2026-09-26"
process: feature
branch_name: feat/destilacion-sddia-installer
persist_ref: docs/features/destilacion-sddia-installer
execution_id: "4b9f0aaa-67f9-4213-8031-6dd9fc98dbcb"
document_id: PBI-ARQUITECTURA-DESTILACION-DEPLOY-DETERMINISTA
---

# Spec — destilacion-sddia-installer

## 1. Artefacto normativo

| Campo | Valor |
|-------|-------|
| `entity_name` | `sddia-installer-contract` |
| Ruta | `SddIA/library/norms/sddia-installer-contract.md` |
| `scope` | `infrastructure` |
| `category` | `architecture` |
| Versión inicial | `1.0.0` |

## 2. Contenido obligatorio (Directriz Core)

- Motor único: `SddIA/scripts/sddia-installer.sh` + fachada `./sddia-installer.sh`.
- Capítulo **Deploy**: tablas I-DEP-* (PBI §3.1).
- Capítulo **Teardown**: tablas I-TEAR-* (PBI §3.2).
- **Códigos de salida**: 1 path/forja; 2 live-gate deploy; 3 force teardown.
- **Cuatro jurisdicciones** (installer default, Paciente 0, atajos Deploy/Teardown).
- **Perfiles**: `full-node`/`engineering` (installer deploy) vs `consumer`/`SddIA_AP`.
- **Simetría**: `resolve_root` / `ESC`; `deploy --force` → `do_teardown` previo.
- **Ceguera**: G3/G-heartbeat/G-dlt no son fases CLI; teardown sin curl/ss.

## 3. Restricciones duras

- Prohibido segundo motor `sddia-undeploy`.
- Prohibido default teardown = `SddIA_AP` sin `--root`.
- Prohibido `rm` plantillas `sddia-*@.service` en teardown.
- Prohibido `pkill -f` global en teardown.
- Prohibido archivar deudas Paciente 0 en este ciclo.

## 4. Deuda (opcional)

Frontmatter en:

- `docs/todos/pending/[DEUDA] Paciente 0 — prompt y proceso de despliegue.md`
- `docs/todos/pending/[DEUDA] Paciente 0 — prompt de teardown.md`

Clave: `installer_contract_ref: SddIA/library/norms/sddia-installer-contract.md`

## 5. Verificación

- Smoke: `SddIA/scripts/qa/test-sddia-installer.sh` (sin cambio obligatorio).
- `sddia-qa audit-eda-coverage` post-forja norma.
