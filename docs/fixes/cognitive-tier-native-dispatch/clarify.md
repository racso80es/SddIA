---
feature_name: cognitive-tier-native-dispatch
created: "2026-10-31"
purpose: decisiones de alcance AUT-26
process: bug-fix
branch_name: Fix_Events_Multiple
tracker_ref: AUT-26
execution_id: "73d65b19-25cb-489c-94e5-abffe60b1e46"
---

# Clarificación — AUT-26

## Decisiones

1. El síntoma de AUT-8 (Argos, `notify-humanized-pr-merged`, Telegram ausente) queda como origen. El predicado actual ya trata `action` y `tool` como nativos. El hueco que sigue el relato es `process`.
2. El cierre exige los tres: `process`, `action` y `tool`. El test de `action` fija que el corte no vuelva.
3. `AGENT_PHASE` solo si el suscriptor no declara cápsula. El test `tier_none_does_not_spawn_and_cognitive_receives_ecst_and_workspace` cubre al agente solo y se mantiene.
4. El `success` de `notify-humanized-pr-merged` ya sale de `NotifyOutcome::Sent`. `SkippedEmpty` ya es `skipped-empty-message`. No se reescribe el handler. Se impide que el runtime lo suplante.
5. Cúmulo (`tier: none`) no cambia de camino.
6. Rama de trabajo: `Fix_Events_Multiple`. Sin checkout a `fix/cognitive-tier-native-dispatch`.
