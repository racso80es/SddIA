---
document_id: PBI-KAIZEN-EMAIL-SIGKILL-01A
title: "[OPERATIVO] email-watcher — validación SIGKILL systemd lab"
format: markdown
version: "1.0.0"
created: "2026-08-19"
status: "abierto"
priority: baja
process: feature
source_feature: kalma2-mvp-sensorial-email
source_branch: feat/kalma2-mvp-sensorial-email
pbi_parent: PBI-KALMA2-MVP-01A
kaizen_phase: Cosecha Kaizen
correlation_id: 2XyNciPL7yiQuKGFY77qJASEEBjTP572gFt1VjK2HQVY
authored_by: agent:cumulo
---

# [OPERATIVO] email-watcher — validación SIGKILL systemd lab

## Origen

Deuda DEFER materializada en `execution.md` § T9a del PBI-KALMA2-MVP-01A (rama `feat/kalma2-mvp-sensorial-email`).

Los ítems marcados `DEFER lab` en la rama:

| Ítem | Estado rama |
|------|-------------|
| Heartbeat vivo `email-watcher` continuo | DEFER — requiere `./start-sddia.sh` en lab real |
| SIGKILL <5 s | DEFER — template systemd presente; validación formal pendiente |

## Mandato

1. En entorno lab con `SDDIA_EMAIL_IMAP_HOST` configurado, arrancar `./start-sddia.sh` y verificar latido continuo de `email-watcher` (>3 ciclos de `Daemon_Heartbeat` sin fractura).
2. Instalar template systemd (`Restart=always`, `RestartSec=5`, `WorkingDirectory=%f`) y validar SIGKILL + recuperación <5 s.
3. Registrar evidencia en `docs/features/kalma2-mvp-sensorial-email/execution.md` o en ola futura según `persist_ref` correspondiente.

## Criterio de cierre

- [ ] Latido continuo confirmado sin fractura (≥3 ciclos)
- [ ] SIGKILL <5 s validado con systemd instalado
- [ ] Este TODO movido a `docs/todos/done/`
