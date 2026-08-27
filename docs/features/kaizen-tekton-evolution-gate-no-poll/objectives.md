---
feature_name: kaizen-tekton-evolution-gate-no-poll
created: "2026-08-27"
process: feature
branch_name: feat/kaizen-tekton-evolution-gate-no-poll
persist_ref: docs/features/kaizen-tekton-evolution-gate-no-poll
pbi_ref: docs/todos/pending/[KAIZEN] Tekton — aduana local evolution y veto de polling CI.md
document_id: PBI-KAIZEN-TEKTON-EVOLUTION-GATE-NO-POLL
uuid: "07dc027a-fdb5-487a-9fea-1a5dd67d38ca"
execution_id: "96471044-003a-457a-bf59-041e94053b12"
mayeuta_verdict: ok
---

# Objetivos — kaizen-tekton-evolution-gate-no-poll

## Misión

Que un `EVOL_HASH_MISMATCH` se resuelva **en local, en una invocación, con un commit y un push**, y que el verde local sea predictivo del verde CI. Cerrar el incidente TEKTON_CI_POLL_EVOL_HASH (PR #206 / PPR #203): placeholder aceptado, asimetría WT/HEAD, recálculo ad hoc y polling de Actions.

## Punto objetivo

> **O-GATE:** Sobre árbol limpio, `sddia-qa gate-evolution --range` coincide con el job `evolution gate (delta)`. Un registro con placeholder o hash roto se re-ancla con `evolution-rehash` y pasa el gate local antes del push. Cero `sleep` / `gh pr checks` en bucle / `gh run rerun` del mismo `headSha` tras el primer log rojo.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Fidelidad `build_registry`: fm y `raw` de la misma fuente; `--range` lee blob HEAD | Cambiar el algoritmo `canonical_hash` |
| `sddia-qa evolution-rehash --id <uuid>` delegando en la cápsula | Rehab `accept-pr` / umbrales Radamanto |
| `validate_record`: `^sha256:[0-9a-f]{64}$` o `EVOL_HASH_MISMATCH` explícito | Dashboard CI / Espejo de Consciencia |
| Re-anclar 4 fósiles `pending*` + `gate-evolution --all` bloqueante post-saneamiento (mismo PR) | Reescritura amplia de `pre_commit_gate.sh` |
| DA-6 en `external-ai-constraints` v1.6.0 + extensión de `tekton-fire-and-forget.mdc` | Sellar `hash_signature` de eventos |
| `pre_push_gate.sh`: delta evolution → gate local **antes** de `route-domain-event` | Listas de rutas ad hoc en el hook |
| Tests: formato, newline/CRLF, fidelidad HEAD vs WT | |

## Objetivos medibles

| ID | Objetivo | Criterio (PBI) |
|----|----------|----------------|
| **O1** | Fidelidad local ≡ CI | K-FIDEL |
| **O2** | Re-anclaje SSOT | K-REHASH |
| **O3** | Placeholder no viaja | K-FORMAT |
| **O4** | Universo evolution canónico | K-FOSIL |
| **O5** | Push con delta evolution exige gate local | K-LOCAL |
| **O6** | Veto vigilancia remota | K-NOPOLL |
| **O7** | Cierre documental un PR | K-DOC |

## Decisiones Mayeuta (sello)

- **R1:** ningún proceso/skill emite `pending-merge` / `pending-anchor-on-merge` en `hash_integrity`. Re-anclar los cuatro; no es anclaje diferido de pipeline.
- **Nombre CLI:** `evolution-rehash`.
- **Contrato:** bump 1.1.1 → 1.1.2 **solo documental** (newline final / CRLF / strip de línea).
- **DA-6:** touchpoint existente; forja normativa vía `entity-manager`.

## No objetivos

- Invalidar hashes conformes cambiando `canonical_hash`.
- Polling de CI como método de depuración.
- Segundo PR documental post-merge.

## Ley aplicada

- `features-documentation-pattern` v1.2.1 / proceso `feature` v1.3.2
- `evolution_contract` v1.1.1 (destino 1.1.2 documental)
- `capsule-json-io` v2.0
- `external-ai-constraints` v1.5.0 → v1.6.0 (DA-5 vigente; DA-6 este ciclo)
- `CONSTITUTION_CORE` — Triaje C/A/B; Verdad Objetiva sobre depuración en CI
- DA-2/DA-3: genoma vía `entity-manager`; DA-4 topología activa; DA-5 fire-and-forget
- SSOT rutas: `SddIA/core/cumulo.paths.json` (`directories.evolution`)
