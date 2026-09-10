---
document_id: PBI-FIX-KALMA2-AGY-NETWORK-SANITIZE
uuid: "a8634b4b-49c8-4905-a824-ae0c251ca900"
title: "[FIX] Kalma2 WUI — red agy sanitizada en epidermis (sin retry)"
format: markdown
version: "1.0.0"
created: "2026-09-10"
updated: "2026-09-10"
status: "cerrado"
closed: "2026-09-10"
fix_ref: docs/fixes/kalma2-wui-agy-network-sanitize
refinement_status: refinado
refined: true
priority: media
type: operativo
process: bug-fix
dispatch: false
suggested_branch: fix/kalma2-wui-agy-network-sanitize
persist_ref: docs/fixes/kalma2-wui-agy-network-sanitize
persist_ref_suggested: docs/fixes/kalma2-wui-agy-network-sanitize
spawned_by: PBI-NUCLEO-AIUA-ANTIGRAVITY-CLI-VECTOR
incident_ref: "Auditoría empírica WUI 2026-09-10 — agy exit=1 network issue connecting, blob recortado a 240"
architectural_constraints:
  - A-NO-RETRY-BACKOFF-DA5
  - A-EPIDERMAL-SANITIZATION-BRIDGE
  - A-INTACT-GENOME-SKILL-DA2
  - A-INTACT-FRONTEND-APPJS
  - A-NO-MULTI-LLM-FAILOVER-SCOPE
  - A-NO-FRACTURE-OVERLOAD-UPSTREAM
  - A-HERMETIC-TESTS-NO-LIVE
  - A-PREFIX-NETWORK-ISSUE-SUFFICIENT
related:
  - docs/todos/done/[OPERATIVO] Kalma2 WUI — 503 Gemini sanitizado en epidermis (sin retry).md
  - docs/todos/done/[NÚCLEO] Aiúa — combustión Tormentosa vía antigravity-cli.md
  - SddIA/interfaces/kalma2-bridge/src/main.rs
  - SddIA/skills/antigravity-cli-executor/src/main.rs
  - interfaces/kalma2/app.js
---

# [FIX] Kalma2 WUI — red agy sanitizada en epidermis (sin retry)

## Síntoma

`#aiua-pulse` → `[error] agy exit=1; stderr=error: There was a network issue connecting to the server, please try again.; stdout={"conversation_id":"…","status":"ERROR",…` recortado a 240. Hay `conversation_id` (sesión viva). No es auth ni `response_text obligatorio`.

## Causa

Skill formatea `agy exit=N; stderr=…; stdout=…` si `map_agy_result` no mapea SUCCESS. `sanitize_bridge_message` clasifica 503 HTTP, `agy-timeout` y auth; **no** `network issue connecting` ni `dial tcp`. El copy «please try again» del CLI no autoriza retry (DA-5).

Host: IPv6 sin ruta; `agy` (Go) dial AAAA. Ajuste de instancia **fuera de este PR** (`nmcli ipv6.method disabled` en la WiFi). Este PBI es solo epidermis.

## Corrección

En `sanitize_bridge_message`, antes del recorte 240:

- Señal: substring case-insensitive `network issue connecting` **o** `dial tcp` (Go; cubre elegibilidad IPv6 previa).
- Canónico castellano, distinto de 503 y de timeout/auth.
- Cero skill, cero `app.js`, cero retry.

## Fuera

Mutar `antigravity-cli-executor`. Failover kitchen. `sysctl`/`gai.conf` en el repo. Fractura Kintsugi. Live `agy` como gate CI.

## CA

| ID | Criterio |
|----|----------|
| AGY-NET-CA1 | Blob empírico `network issue connecting` → mensaje canónico, sin `{` ni stdout JSON |
| AGY-NET-CA2 | `dial tcp` (elegibilidad) → el mismo canónico |
| AGY-NET-CA3 | No enmascarar auth, `agy-timeout`, 503, `timeout motor`, `agy-failed` |
| AGY-NET-CA4 | Cero sleep/retry/backoff nuevos |
| AGY-NET-CA5 | Tests herméticos `--bin kalma2-bridge` |
| AGY-NET-CA6 | Un PR: puente + tests + persist_ref + PBI `done/`; cero diffs skill/`app.js` |
| AGY-NET-CA-CI | Checks GitHub verdes con `run_id` antes de `global: APTO` y `accept-pr` |
