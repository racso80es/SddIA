---
document_id: PBI-EDA-OUTBOUND-NATIVE-REMEDIATION
title: "[FIX] EDA — remediar suscriptores IOTA y Telegram con red nativa"
format: markdown
version: "1.0.0"
created: "2026-07-13"
status: done
priority: alta
process: bug-fix
branch_name: fix/eda-outbound-native-remediation
fix_ref: docs/fixes/eda-outbound-native-remediation
validacion_ref: docs/fixes/eda-outbound-native-remediation/validacion.md
closed: "2026-07-13"
origin: "docs/todos/pending/[FIX] EDA — remediar suscriptores IOTA y Telegram con red nativa.md"
---

# PBI-EDA-OUTBOUND-NATIVE-REMEDIATION: remediar suscriptores IOTA y Telegram con red nativa

| Campo | Valor |
|-------|-------|
| **ID** | `PBI-EDA-OUTBOUND-NATIVE-REMEDIATION` |
| **Estatus** | ✅ Done |
| **Rama** | `fix/eda-outbound-native-remediation` |
| **Persist** | `docs/fixes/eda-outbound-native-remediation` |

## Entregables

| # | Entregable | Estado |
|---|------------|--------|
| 1 | Cápsula nativa `send-telegram-notification` (HTTP + lab/mock) | ✅ |
| 2 | Cápsula nativa `iota-immutable-publisher` (simulate/mock/relay) | ✅ |
| 3 | Módulo `sddia-io/outbound_lab` | ✅ |
| 4 | Rebuild `release` documentado (SSOT prioriza release) | ✅ |
| 5 | `validacion.md` APTO + PBI archivado | ✅ |

## Criterios de aceptación

- [x] `PullRequest_Presented` lab sin DL por stub WASI Telegram
- [x] IOTA sin `iota publish failed` por stub WASI en rutas corregidas
- [x] Errores `config-missing` clasificados
- [x] Tests + integración `route-domain-event` lab
- [x] Re-auditoría: `pending: 0`, `stale: 0`
