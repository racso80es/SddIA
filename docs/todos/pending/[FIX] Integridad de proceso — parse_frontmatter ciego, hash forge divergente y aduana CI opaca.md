---
document_id: PBI-FIX-INTEGRIDAD-PROCESO-FORGE-CI
uuid: "d6387831-0e57-4bee-b402-a49f782e6837"
title: "[FIX] Integridad de proceso — parse_frontmatter ciego, hash forge divergente y aduana CI opaca"
format: markdown
version: "1.0.0"
created: "2026-08-29"
updated: "2026-08-29"
status: pending
priority: alta
process: bug-fix
type: bug-fix
dispatch: false
suggested_branch: fix/integridad-proceso-forge-ci
persist_ref_suggested: docs/fixes/integridad-proceso-forge-ci
depends_on: []
derived_from:
  - PBI-KAIZEN-CICLO-JURISDICCION-TODOS
friction_ids:
  - F-CORE-PARSE-FRONTMATTER-DELIMITADOR-CIEGO
  - F-PROCESS-FORGE-HASH-BODY-DIVERGENTE
  - F-CI-JOB-VERIFY-TOOLS-INDEX-NOMBRE-ENGANOSO
  - F-DCC-SIN-ADUANA-INTEGRIDAD-LOCAL
tech_debt_ids:
  - DT-PROCESS-MD-WORKSPACE-TEMPLATE-DELIMITADOR-COLISION
related:
  - SddIA/engine/execute-process/src/forges/common.rs
  - SddIA/engine/execute-process/src/forges/factory.rs
  - SddIA/engine/execute-process/src/core/parser.rs
  - SddIA/engine/execute-process/src/engine/verify_process_integrity.rs
  - SddIA/library/codexes/codex-software-engineering/process/delivery-close-cycle.md
  - .github/workflows/sddia-index-qa.yml
source_audit: "Ciclo PR #225 (feat/kaizen-ciclo-jurisdiccion-todos). Check verify-tools-index rojo tras push del cierre L9. Evidencia: gh run 33256495000 log-failed; sddia-qa verify-process-integrity local; git commit 76be459 (fix parcial F1/F2)."
---

# [FIX] Integridad de proceso — parse_frontmatter ciego, hash forge divergente y aduana CI opaca

Fricción emergida al cerrar `PBI-KAIZEN-CICLO-JURISDICCION-TODOS`: el check CI `verify-tools-index` falló tras el push del cierre. La causa raíz no era el índice de tools, sino un `hash_signature` desalineado en `delivery-close-cycle.md`, producto de dos defectos del motor de forja encadenados. **F1 y F2 quedaron saldadas en la misma rama del kaizen (commit `76be459`);** este PBI las documenta como incidente reproducible y escala **F3 y F4**, que siguen abiertas.

## 0. Trazabilidad fricción → sección → criterio

| `friction_id` | Sección | Estado | Criterio | Deuda ligada |
|---------------|---------|--------|----------|--------------|
| `F-CORE-PARSE-FRONTMATTER-DELIMITADOR-CIEGO` | §1 | Saldada 2026-08-29 (`76be459`) | CA1 | `DT-PROCESS-MD-WORKSPACE-TEMPLATE-DELIMITADOR-COLISION` |
| `F-PROCESS-FORGE-HASH-BODY-DIVERGENTE` | §2 | Saldada 2026-08-29 (`76be459`) | CA2 | — |
| `F-CI-JOB-VERIFY-TOOLS-INDEX-NOMBRE-ENGANOSO` | §3 | Reproducible | CA3 | — |
| `F-DCC-SIN-ADUANA-INTEGRIDAD-LOCAL` | §4 | Reproducible | CA4 | — |

## 1. `F-CORE-PARSE-FRONTMATTER-DELIMITADOR-CIEGO` — el parser de forja trunca ante `---` embebido

`forges/common.rs::parse_frontmatter` resolvía el frontmatter con `text.strip_prefix("---")` + `split_once("\n---")`. `delivery-close-cycle.md` declara:

```yaml
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/---
```

El valor **termina en `---` pegado al `/`**, y ese `---` es a la vez el delimitador de cierre del frontmatter (no hay línea `---` separada). `split_once("\n---")` busca `\n---`; como el `---` del valor no está precedido por salto de línea, **no encuentra el cierre** y devuelve `Map::new()` — frontmatter **vacío sin error**.

Consecuencia: cualquier forja `update` sobre este proceso leía `uuid`/`version`/`hash_signature` como ausentes. El `hash_refresh_only` devolvía `handoff_version: "1.0.0"` y `handoff_entity_uuid: null` (síntoma observado en runtime), sin poder sellar el hash real.

El parser Core (`core/parser.rs::parse_frontmatter`) usa `split("---")` y **sí** tolera el caso. Había dos parsers divergentes para la misma tarea.

**Saldado (`76be459`):** `forges/common.rs::parse_frontmatter` delega en el parser Core (una sola fuente de verdad).

## 2. `F-PROCESS-FORGE-HASH-BODY-DIVERGENTE` — el forge sella un hash que la aduana no valida

`run_process_forge` en modo `update` con `markdown_body_replacements` (vía `patch_artifact_body_replacements`) calculaba `hash_signature` con `canonical_artifact_hash` — hash del **artefacto completo** (frontmatter + body).

Pero `verify-process-integrity` (`engine/verify_process_integrity.rs`) valida contra `sha256_phases_integrity(phases)` — hash **solo del array `phases`**. Dos algoritmos distintos para el mismo campo.

Al documentar la excepción `backfill-manifest.json` (CA3b del kaizen) vía `entity-manager` `markdown_body_replacements`, el process quedó sellado con `sha256:7610a5b4…` (artefacto) mientras la aduana esperaba `sha256:b26d16f7…` (fases). Verde en local naïve, **rojo en CI**.

**Saldado (`76be459`):** tras aplicar body replacements, el process forge llama `refresh_process_hash` para sellar el hash de fases canónico.

## 3. `F-CI-JOB-VERIFY-TOOLS-INDEX-NOMBRE-ENGANOSO` — el nombre del job oculta lo que falla

El job CI `verify-tools-index` (`.github/workflows/sddia-index-qa.yml:19`) ejecuta **tres** steps:

```yaml
- name: verify-tools-index
- name: verify-process-integrity
- name: evolution-register unit tests
```

El fallo se reporta como `verify-tools-index` (nombre del job), pero la causa real fue el step `verify-process-integrity`. El diagnóstico exigió abrir `gh run --log-failed` para descubrir que el verificador de tools estaba verde y el de procesos rojo. El nombre del job miente sobre su alcance.

## 4. `F-DCC-SIN-ADUANA-INTEGRIDAD-LOCAL` — el cierre no verifica integridad antes del push

`delivery-close-cycle` tiene aduanas de **evolution** y **EDA genómica**, pero **no** ejecuta `verify-process-integrity` ni `verify-tools-index` antes de la publicación remota. Un `hash_signature` corrupto atraviesa el cierre local y solo se detecta en CI remoto — precisamente el bucle de vigilancia remota que DA-6 prohíbe alimentar.

Simetría rota: si la mutación de genoma (procesos/tools) puede desalinear estos índices, el gate que los valida debería vivir en la aduana local del cierre, no exclusivamente en el workflow de GitHub.

## 5. Criterios de aceptación

- **CA1** — `forges/common.rs::parse_frontmatter` tolera valores con `---` embebido (delega en parser Core). Test unitario con fixture cuyo frontmatter contenga `workspace_template: …/---`; aserción de que `uuid`/`hash_signature` se leen. ✅ `76be459`; **resta el test dedicado**.
- **CA2** — El process forge `update` con `markdown_body_replacements` sella `hash_signature` = `sha256_phases_integrity(phases)`, aceptado por `verify-process-integrity`. Test unitario que forje un body replacement y verifique paridad de hash. ✅ `76be459`; **resta el test dedicado**.
- **CA3** — El workflow `sddia-index-qa.yml` expone cada verificador con nombre propio: o bien jobs separados (`verify-tools-index`, `verify-process-integrity`), o un job renombrado (`sddia-index-integrity`) cuyo nombre no reclame solo el índice de tools.
- **CA4** — `delivery-close-cycle` gana una fase/aduana local que ejecuta `verify-process-integrity` + `verify-tools-index` antes de la Publicación remota, con veredicto `block` ante mismatch. Verificable: sellar un hash corrupto a mano y confirmar que el DCC bloquea localmente sin llegar al push.
- **CA5** *(opcional, deuda)* — Sanear `DT-PROCESS-MD-WORKSPACE-TEMPLATE-DELIMITADOR-COLISION`: `workspace_template` de `delivery-close-cycle.md` no debe terminar en `---` (colisión con delimitador). Vía `entity-manager` update; el arreglo del parser (CA1) ya lo blinda, pero el artefacto sigue siendo frágil como precedente.

## 6. Invariantes

| Regla | Motivo |
|-------|--------|
| Un solo parser de frontmatter en el motor | Dos implementaciones divergentes reproducen el defecto |
| `hash_signature` de process = hash de `phases` | Es lo único que `verify-process-integrity` valida |
| Verificar integridad **antes** del push, no después | DA-6: no alimentar el bucle de vigilancia remota |
| Genoma solo vía `entity-manager` / `--forge` | DA-2; prohibido editar `delivery-close-cycle.md` a mano |

## 7. Riesgos

| Riesgo | Mitigación |
|--------|-----------|
| Añadir aduana de integridad al DCC ralentiza el cierre | Los verificadores son sub-segundo en local; net-positivo frente al round-trip CI |
| Renombrar el job CI rompe required checks configurados en el repo | Coordinar con la config de branch protection; documentar el rename en el PR |
| Sanear `workspace_template` cambia el hash del proceso | El propio CA2 sella el hash nuevo; re-forjar solo `delivery-close-cycle` |
