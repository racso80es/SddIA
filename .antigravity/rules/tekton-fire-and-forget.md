---
description: Veto Fire-and-Forget post-acuse CLI (DA-5), Veto CI (DA-6) y Veto post-DCC (DA-7)
alwaysApply: true
---

# DA-5 — Fire-and-Forget

Tras `./sddia-run.sh` / `execute-process` y JSON de acuse: prohibido `sleep`, `wait`, bucles de polling sobre `EventManagerPort` / `event_uri` o status, y llamadas extra (`AwaitShell`, `manage_task`, `schedule`) para vigilar centinelas.

Éxito = inyección acusada (`data.detached: true` en procesos largos). Siguiente estímulo = Racso o Kalma2.

No castrar ticks de daemons ni el wait hasta el acuse del propio CLI.

## DA-6 — Veto de Vigilancia Remota (CI)

Tras el primer log de check GitHub fallido: prohibido `sleep` de espera, `gh pr checks` en bucle y `gh run rerun` del mismo `headSha`. Un finding → parche local (`sddia-qa gate-evolution --json --range` si toca evolution) → un push. No empujar docs de cierre con check rojo conocido. Ver `external-ai-constraints.md` v1.6.2.

## DA-7 — Veto post-DCC `failed`/`blocked`

Tras acuse de `delivery-close-cycle` con fase Publicación remota `failed` o `blocked` (incl. `F-DCC-WORKFLOW-SCOPE`):
- Prohibido `git push`, `gh pr create` / `gh pr merge` raw y URL `x-access-token`.
- El rechazo GitHub `without workflow scope` es colapso de credencial (PAT de `git-manager` ≠ token `gh auth`), no de Core.
- Operador: `gh auth setup-git` (unificar helper). Siguiente estímulo = laudo del Vértice Biológico o re-inyección DCC con credencial alineada.
