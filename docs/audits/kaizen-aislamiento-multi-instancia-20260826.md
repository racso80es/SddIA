---
title: "Cicatriz — aislamiento multi-instancia (absorción 20260826)"
date: "2026-08-26"
type: audit
pbi_uuid: "b5d19318-a0fd-440b-9aac-8c6d93f775ed"
feature: kaizen-aislamiento-multi-instancia
execution_id: "3b40b62c-d048-4896-b8c1-1ee267ca7704"
entity_manager_event: "c1160a5e-aec8-404d-96a4-816d21d57d13"
antecesor_audit: docs/audits/paciente0-centinelas-email-sordo-20260826.md
---

# Auditoría de absorción — no reescribe 20260826

## Motor

`ExecStart=%f`; instance-creator v1.3.0; `_sddia_resolve_instance_root`; 0 `pkill -x`.

## User manager

| Instancia | ExecStart expandido |
|-----------|---------------------|
| `@…SddIA` | `…/SddIA/SddIA/scripts/daemons/event-watcher.sh` |
| `@…SddIA_AP` | `…/SddIA_AP/SddIA/scripts/daemons/event-watcher.sh` |

Árbol `SddIA_AP` **ausente**. Surrogate: `.SddIA/sandbox/iso-b`.

## Dos raíces (iso-b)

| | forja | iso-b |
|--|-------|--------|
| cwd event-watcher | `/home/racso/Proyectos/SddIA` | `…/sandbox/iso-b` |
| WUI | `:8765` | `:18765` |
| `/api/email-inbox` SHA | `62ce3b2b…` (2995 B) | `2b4af084…` (`items:[]`) |

Restart `sddia-event-watcher@…SddIA`: iso-b PID 56793 **vivo**.

## R-07

No Paciente 0. Lab IMAP `@…SddIA` permanece active (host=forja).
