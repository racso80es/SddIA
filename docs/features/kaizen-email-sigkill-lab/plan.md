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
kaizen_phase: Cosecha Kaizen
correlation_id: "2XyNciPL7yiQuKGFY77qJASEEBjTP572gFt1VjK2HQVY"
status: plan_ready
ola: "1 (única)"
gates_count: 3
---

# Plan — kaizen-email-sigkill-lab

## Estructura de ola

Ola única. Sin dependencias de forja de genoma. Bloqueo externo en F-01 (entorno lab).

```
Ola-1
 ├─ G-Lab01  Heartbeat continuo (S-01)
 ├─ G-Lab02  SIGKILL systemd (S-02)
 └─ G-Lab03  Registro evidencia + cierre documental (S-03 / S-04)
```

---

## Gate G-Lab01 — Heartbeat continuo

**Criterio:** `daemon-heartbeat-audit sweep` → `fractures_emitted: []` + ≥3 heartbeats.

**Precondición:** `SDDIA_EMAIL_IMAP_HOST` en `.SddIA/.dev/.env`; `./start-sddia.sh` ejecutado.

**Pasos:**

1. Confirmar variable de entorno activa:
   ```bash
   grep SDDIA_EMAIL_IMAP_HOST .SddIA/.dev/.env
   ```
2. Arrancar daemons:
   ```bash
   ./start-sddia.sh
   ```
3. Esperar ≥3 ciclos de heartbeat (intervalo nominal del daemon: ~60 s → ~3 min de espera).
4. Ejecutar auditoría:
   ```bash
   ./sddia-run.sh --process daemon-heartbeat-audit --inputs '{"sweep":true}'
   ```
5. Capturar JSON de salida y verificar `fractures_emitted: []`.

**Artefactos a crear:** ninguno (solo lectura de salida CLI).

**Resultado esperado:** JSON con `status: sweep`, `fractures_emitted: []`.

---

## Gate G-Lab02 — SIGKILL systemd

**Criterio:** `ActiveState=active` con nuevo PID en <5 s tras `kill -SIGKILL`.

**Precondición:** G-Lab01 superado; systemd de usuario disponible en el host.

**Pasos:**

1. Instalar unit:
   ```bash
   REPO_ROOT=$(pwd)
   sed "s|@@SDDIA_CORE_ROOT@@|${REPO_ROOT}|g" \
     SddIA/templates/systemd/sddia-email-watcher@.service.template \
     > ~/.config/systemd/user/sddia-email-watcher@.service
   systemctl --user daemon-reload
   ESCAPED=$(systemd-escape --path "${REPO_ROOT}")
   systemctl --user enable --now "sddia-email-watcher@${ESCAPED}"
   ```
2. Verificar arranque:
   ```bash
   systemctl --user status "sddia-email-watcher@${ESCAPED}"
   ```
3. Obtener PID y enviar SIGKILL:
   ```bash
   EMAIL_PID=$(systemctl --user show -p MainPID --value "sddia-email-watcher@${ESCAPED}")
   T_KILL=$(date +%s)
   kill -SIGKILL "${EMAIL_PID}"
   ```
4. Verificar recuperación dentro de 5 s:
   ```bash
   sleep 5
   NEW_PID=$(systemctl --user show -p MainPID --value "sddia-email-watcher@${ESCAPED}")
   T_RECOVER=$(date +%s)
   systemctl --user status "sddia-email-watcher@${ESCAPED}"
   echo "Delta: $((T_RECOVER - T_KILL)) s | PID original: ${EMAIL_PID} | PID nuevo: ${NEW_PID}"
   ```

**Criterio APTO:** `ActiveState=active`, `${NEW_PID} != ${EMAIL_PID}`, delta ≤5 s.

---

## Gate G-Lab03 — Evidencia y cierre

**Pasos:**

1. Materializar `docs/features/kaizen-email-sigkill-lab/execution.md` con:
   - Salida JSON de `daemon-heartbeat-audit` (G-Lab01).
   - Tabla: PID original / timestamp SIGKILL / timestamp recuperación / delta / veredicto.
   - Veredicto global por ítem.
2. Si ambos gates APTO:
   - Crear `docs/features/kaizen-email-sigkill-lab/validacion.md` con `global: APTO`, `pbi_archived: true`.
   - Mover PBI: `docs/todos/pending/[OPERATIVO] email-watcher — validación SIGKILL systemd lab (kalma2-mvp-sensorial-email).md` → `docs/todos/done/`.
3. Abrir PR vía `skill:git-manager` hacia `main` con ambos cambios en el diff.

---

## Mapa de artefactos

| Artefacto | Acción | Gate |
|-----------|--------|------|
| `docs/features/kaizen-email-sigkill-lab/execution.md` | crear | G-Lab03 |
| `docs/features/kaizen-email-sigkill-lab/validacion.md` | crear | G-Lab03 |
| `docs/todos/done/[OPERATIVO] email-watcher…` | crear (move) | G-Lab03 |
| `docs/todos/pending/[OPERATIVO] email-watcher…` | eliminar | G-Lab03 |
| `~/.config/systemd/user/sddia-email-watcher@.service` | crear en lab (no versionar) | G-Lab02 |

---

## Bloqueos conocidos

| ID | Descripción | Desbloqueador |
|----|-------------|---------------|
| F-01 | `SDDIA_EMAIL_IMAP_HOST` no configurado en CI | Racso activa entorno lab |

Hasta que F-01 se resuelva, G-Lab01 y G-Lab02 no pueden ejecutarse. El blueprint está listo; la ejecución espera confirmación de Racso.
