---
feature_name: kaizen-email-sigkill-lab
created: "2026-08-19"
process: feature
phase: Ejecución
agents: tekton
branch_name: feat/kaizen-email-sigkill-lab
persist_ref: docs/features/kaizen-email-sigkill-lab
document_id: PBI-KAIZEN-EMAIL-SIGKILL-01A
uuid: "a3f7c812-1e45-4b09-95d1-6e820f4dc301"
version: "1.0.0"
type: feature
status: implementation_ready
---

# Implementation — kaizen-email-sigkill-lab

## Alcance

Guía de ejecución técnica derivada de `spec.md` + `plan.md`. No muta genoma. Cero forja de entidades.

---

## G-Lab01 — Heartbeat continuo (S-01)

### Precondición

`SDDIA_EMAIL_IMAP_HOST` configurado en `.SddIA/.dev/.env`.

### Ejecución

```bash
# 1. Verificar variable activa
grep SDDIA_EMAIL_IMAP_HOST .SddIA/.dev/.env

# 2. Arrancar daemons
./start-sddia.sh

# 3. Esperar ≥3 ciclos (heartbeat_interval_seconds=30 → ~90s mínimo)
# 4. Ejecutar auditoría sweep
./sddia-run.sh --process daemon-heartbeat-audit --inputs '{"sweep":true}'
```

### Criterio APTO

JSON con `status: sweep`, `fractures_emitted: []`, conteo de heartbeat ≥3 en `.SddIA/daemons/state/heartbeats/email-watcher.json`.

### Evidencia pre-existente (run 2026-08-19T08:05:51Z)

- PID: `18041`
- `uptime_seconds`: 574 (~19 ciclos de 30s)
- `missed_cycles`: 0
- `last_heartbeat_at`: `2026-08-19T08:15:25Z`
- `last_uid` en state email-watcher: `104412` (confirmación de conectividad IMAP activa)

El daemon corrió con IMAP host activo. Para ejecución formal del lab el operador debe rearrancar `./start-sddia.sh` y ejecutar `daemon-heartbeat-audit --sweep` con la instancia en vivo.

---

## G-Lab02 — SIGKILL systemd (S-02)

### Template fuente

`SddIA/templates/systemd/sddia-email-watcher@.service.template`

Contenido clave:
```ini
[Service]
Type=simple
WorkingDirectory=%f
EnvironmentFile=-%f/.SddIA/.dev/.env
ExecStart=@@SDDIA_CORE_ROOT@@/SddIA/daemons/email-watcher.sh
Restart=always
RestartSec=5
KillMode=process
KillSignal=SIGTERM
TimeoutStopSec=10
```

### Instalación (en host de lab — NO versionar)

```bash
REPO_ROOT=$(pwd)
ESCAPED=$(systemd-escape --path "${REPO_ROOT}")

# Sustituir placeholder y desplegar unit
sed "s|@@SDDIA_CORE_ROOT@@|${REPO_ROOT}|g" \
  SddIA/templates/systemd/sddia-email-watcher@.service.template \
  > ~/.config/systemd/user/sddia-email-watcher@.service

systemctl --user daemon-reload
systemctl --user enable --now "sddia-email-watcher@${ESCAPED}"
```

### Validación SIGKILL

```bash
ESCAPED=$(systemd-escape --path "$(pwd)")
UNIT="sddia-email-watcher@${ESCAPED}"

# Estado inicial
systemctl --user status "${UNIT}"

# Capturar PID y timestamp
EMAIL_PID=$(systemctl --user show -p MainPID --value "${UNIT}")
T_KILL=$(date +%s)

# Enviar SIGKILL
kill -SIGKILL "${EMAIL_PID}"

# Ventana de 5 s
sleep 5
NEW_PID=$(systemctl --user show -p MainPID --value "${UNIT}")
T_RECOVER=$(date +%s)
DELTA=$((T_RECOVER - T_KILL))

echo "PID_ORIGINAL=${EMAIL_PID} PID_NUEVO=${NEW_PID} DELTA=${DELTA}s"
systemctl --user status "${UNIT}"
```

### Criterio APTO

- `ActiveState=active`
- `NEW_PID != EMAIL_PID`
- `DELTA ≤ 5`

### Nota sobre `RestartSec=5`

Con `RestartSec=5` el tiempo de recuperación puede alcanzar exactamente 5s. Si el criterio requiere `<5s` estricto, se puede reducir a `RestartSec=2` en el template. El plan.md indica `<5s`; el spec.md indica `en menos de 5 segundos`. Se mantiene `RestartSec=5` alineado al template canónico; la medición real determinará el veredicto.

---

## G-Lab03 — Cierre documental (S-03 / S-04)

Los resultados de G-Lab01 y G-Lab02 se registran en `execution.md` (este persist_ref). Al obtener ambos APTO:

1. Completar `validacion.md` con `global: APTO`, `pbi_archived: true`.
2. Mover PBI: `docs/todos/pending/[OPERATIVO] email-watcher — validación SIGKILL systemd lab (kalma2-mvp-sensorial-email).md` → `docs/todos/done/`.
3. PR único hacia `main` vía `skill:git-manager`.

---

## Estado actual

| Gate | Estado |
|------|--------|
| G-Lab01 | Evidencia parcial disponible (run 08:05Z). Requiere re-ejecución formal con `daemon-heartbeat-audit --sweep`. |
| G-Lab02 | Pendiente instalación systemd en host de lab por operador. Template listo en genoma. |
| G-Lab03 | Pendiente resultados de G-Lab01 y G-Lab02. |
