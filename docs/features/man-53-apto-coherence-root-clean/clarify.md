---
feature_name: man-53-apto-coherence-root-clean
created: "2026-11-22"
process: feature
branch_name: feat/man-53-apto-coherence-root-clean
persist_ref: docs/features/man-53-apto-coherence-root-clean
tracker_ref: MAN-53
purpose: "Clarificaciones y decisiones de diseño para H-NEW-06 / KZ-EST-04 (MAN-53)"
---
# Clarificaciones — man-53-apto-coherence-root-clean

## Decisiones estabilizadas

1. **Detección de matriz AC/CA (C5):**
   - Se inspeccionan `execution.md` y `validacion.md` (cuerpo Markdown tras frontmatter) en filas de tabla (`| AC-x | ... |` o `| CA-x | ... |`) y listas con prefijo `AC-x` / `CA-x`.
   - Para evitar falsos positivos en notas explicativas (p. ej. `**PASS** — sin pendientes`), se evalúa el token de estado de las celdas de estado (limpiando negritas/backticks) frente al conjunto `{PARCIAL, FAIL, NO_APTO, ABIERTO, PENDIENTE}` (incluyendo `PENDIENTE-CI` por límite de palabra en `PENDIENTE`).
   - Fricción emitida: `apto_coherence_ac_partial`.
2. **Checks troncales no degradables:**
   - En `validacion.md` frontmatter `checks:`, cualquier clave en `{F2_DOC_GATE, F3_TECH_GATE, F4_RBAC_GATE, ROOT_TOPOLOGY_CLEAN}` o con prefijo `DOC_HONESTY_` cuyo valor sea `NO_APTO` (case-insensitive) invalida `global: APTO` con fricción `apto_coherence_trunk_gate`.
   - `pull-request-review` aplica `validate_apto_coherence` antes de emitir `verdict: aprobado`, escribir atestación QA (`try_write_from_ppr`) o ejecutar `handoff_accept_pr`.
3. **Ausencia física de directorios efímeros en raíz:**
   - Lista canónica: `[".events", "orchestration", "telemetry", "pending", "domain", "dead-letter", "progress", "_lab"]`.
   - Se verifica existencia física en disco (`repo.join(dir).is_dir()`), independiente de `git status --porcelain`.
   - Se integra en `delivery-close-cycle` (`capsule_gate_pre_pr_qa`) y `accept-pr` (`Auditoría Genómica` / `Fusión Soberana`), emitiendo fricción accionable `root_topology_dirty` sin `System_Fracture_Detected`.
