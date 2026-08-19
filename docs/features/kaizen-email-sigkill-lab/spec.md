---
feature_name: kaizen-email-sigkill-lab
created: "2026-08-19"
process: feature
phase: Diseño de Blueprint
agents: dedalo
branch_name: feat/kaizen-email-sigkill-lab
persist_ref: docs/features/kaizen-email-sigkill-lab
document_id: PBI-KAIZEN-EMAIL-SIGKILL-01A
uuid: "a3f7c812-1e45-4b09-95d1-6e820f4dc301"
version: "1.0.0"
type: feature
parent_pbi: PBI-KALMA2-MVP-01A
parent_persist_ref: docs/features/kalma2-mvp-sensorial-email
kaizen_phase: Cosecha Kaizen
correlation_id: "2XyNciPL7yiQuKGFY77qJASEEBjTP572gFt1VjK2HQVY"
status: spec_ready
dedalo_verdict: ok
---

# Spec — kaizen-email-sigkill-lab

## Resumen

Kaizen operativo de una sola ola. Cierra dos ítems DEFER de T9a en `PBI-KALMA2-MVP-01A`:

| Ítem | Gate |
|------|------|
| Heartbeat vivo continuo `email-watcher` | ≥3 ciclos `Daemon_Heartbeat` sin `fractures_emitted` |
| SIGKILL + recuperación systemd | Servicio activo de nuevo en <5 s tras `kill -SIGKILL` |

**Prerequisito de ejecución (F-01):** `SDDIA_EMAIL_IMAP_HOST` configurado en `.SddIA/.dev/.env` del host de lab. Sin esa variable el spec es válido pero la ejecución está bloqueada hasta que Racso confirme el entorno.

---

## S-01 — Lab Heartbeat (`Lab-01`)

### Comportamiento esperado

Al ejecutar `./start-sddia.sh` con `SDDIA_EMAIL_IMAP_HOST` activo, el daemon `email-watcher` emite eventos `Daemon_Heartbeat` de forma continua. El proceso `daemon-heartbeat-audit` (sweep) debe reportar:

- `fractures_emitted: []`
- Al menos 3 entradas en el historial de heartbeat del workspace activo.

### Contrato de verificación

```bash
./sddia-run.sh --process daemon-heartbeat-audit --inputs '{"sweep":true}'
```

Criterio APTO: `status: sweep`, `fractures_emitted: []`, conteo de heartbeat ≥3.

### Artefactos implicados

| Artefacto | Rol | Mutación |
|-----------|-----|----------|
| `SddIA/daemons/email-watcher.sh` | Emite heartbeat | ninguna (caja negra) |
| `./start-sddia.sh` | Orquesta arranque de daemons | ninguna |
| `daemon-heartbeat-audit` (proceso Core) | Auditor sweep | ninguna |

---

## S-02 — Lab SIGKILL systemd (`Lab-02`)

### Comportamiento esperado

Con el template `sddia-email-watcher@.service.template` instalado como unit systemd de usuario, al enviar `SIGKILL` al proceso principal el campo `Restart=always` + `RestartSec=5` de systemd debe recuperar el servicio en menos de 5 segundos.

### Instalación del template

```bash
# Sustituir @@SDDIA_CORE_ROOT@@ por la ruta absoluta al repo
REPO_ROOT=$(pwd)
sed "s|@@SDDIA_CORE_ROOT@@|${REPO_ROOT}|g" \
  SddIA/templates/systemd/sddia-email-watcher@.service.template \
  > ~/.config/systemd/user/sddia-email-watcher@.service

systemctl --user daemon-reload
systemctl --user enable --now "sddia-email-watcher@$(systemd-escape --path "${REPO_ROOT}")"
```

### Secuencia de validación

```bash
# 1. Obtener PID del email-watcher
EMAIL_PID=$(systemctl --user show -p MainPID --value "sddia-email-watcher@$(systemd-escape --path "$(pwd)")")

# 2. SIGKILL
kill -SIGKILL "${EMAIL_PID}"

# 3. Esperar y verificar recuperación (window 5s)
sleep 5
systemctl --user status "sddia-email-watcher@$(systemd-escape --path "$(pwd)")"
```

Criterio APTO: `ActiveState=active` y nuevo `MainPID` distinto del original, dentro del ventana de 5 s.

### Artefactos implicados

| Artefacto | Rol | Mutación |
|-----------|-----|----------|
| `SddIA/templates/systemd/sddia-email-watcher@.service.template` | Fuente del unit | ninguna (lectura) |
| `~/.config/systemd/user/sddia-email-watcher@.service` | Unit instalada (instancia local) | creada en lab |
| `systemd --user` | Supervisor de recuperación | estado de sistema |

---

## S-03 — Registro de evidencia (`Lab-03`)

Los resultados de Lab-01 y Lab-02 se registran en `docs/features/kaizen-email-sigkill-lab/execution.md` con:

- Salida cruda del CLI `daemon-heartbeat-audit` (sweep).
- PID original, timestamp SIGKILL, timestamp de recuperación, delta en segundos.
- Veredicto `APTO` o `BLOQUEADO` por ítem.

---

## S-04 — Cierre documental

Al concluir Lab-01, Lab-02 y Lab-03:

1. `validacion.md` en este `persist_ref` con `global: APTO`, `pbi_archived: true`.
2. PBI movido de `docs/todos/pending/` → `docs/todos/done/`.
3. PR único hacia `main` que incluya ambos cambios.

---

## Restricciones

- **Cero forja de genoma:** no crear ni modificar entidades en `SddIA/tools/`, `skills/`, `actions/`, `process/`, `agents/`, `events/`, `norms/`, `library/`.
- **Git vía skill:git-manager** exclusivamente.
- **DA-5:** tras acuse CLI, prohibido polling posterior.
- El template systemd se instala solo en el host de lab del usuario; no se versiona la unit instalada.
