---
document_id: PBI-DT-PACIENTE0-UNDEPLOY-PROCESS
uuid: "dddee1ff-aeac-400b-85df-70374d37c45d"
title: "[DEUDA] Paciente 0 — prompt de teardown y proceso futuro"
format: markdown
version: "1.0.0"
status: deuda_tecnica
type: deuda
priority: alta
process: null
process_candidate: paciente0-undeploy
process_candidate_class: process
created: "2026-08-25"
updated: "2026-08-25"
instance_name_default: SddIA_AP
instance_parent: /home/racso/Proyectos
config_source: /home/racso/Proyectos/.dev/.env
companion_deploy_ref: docs/todos/DeudaTecnica/[DEUDA] Paciente 0 — prompt y proceso de despliegue.md
companion_deploy_document_id: PBI-DT-PACIENTE0-DEPLOY-PROCESS
tech_debt_ids:
  - DT-PACIENTE0-UNDEPLOY-PROCESS
  - DT-SYSTEMD-USER-ENABLE
derived_from:
  - PBI-DT-PACIENTE0-DEPLOY-PROCESS
  - PBI-LAB-PACIENTE0-SDDIA-AP
---

# [DEUDA] Paciente 0 — prompt de teardown y candidato a proceso

## 0. Propósito

Documento **operativo + semilla de proceso**. Prompt para **apagar y borrar** un despliegue Paciente 0 (por defecto `SddIA_AP`) sin tocar la forja ni la bóveda `/home/racso/Proyectos/.dev`. Complementa `PBI-DT-PACIENTE0-DEPLOY-PROCESS`.

**Done:** unidad systemd de la instancia `inactive`/`disabled`; cero procesos cuyo binario o cwd sea `{INSTANCE_ROOT}`; carpeta `{INSTANCE_ROOT}` inexistente; forja `SddIA` y `.dev` intactos.

El proceso Core `paciente0-undeploy` **no** está forjado (DA-2).

---

## 1. Prompt (copiar al Vértice Productivo)

```text
Limpia / elimina el despliegue Paciente 0 según PBI-DT-PACIENTE0-UNDEPLOY-PROCESS.

Constantes:
- Nombre por defecto: SddIA_AP
- INSTANCE_ROOT: /home/racso/Proyectos/SddIA_AP  (salvo override)
- No borrar: forja /home/racso/Proyectos/SddIA , bóveda /home/racso/Proyectos/.dev
- No borrar la plantilla systemd ~/.config/systemd/user/sddia-email-watcher@.service (compartida)
- No stop/disable unidades cuyo %f sea la FORJA (lab), solo la instancia
- Orden: SIGTERM a start-sddia de la instancia → stop/disable email-watcher@%f instancia
         → pkill residual bajo INSTANCE_ROOT → rm -rf INSTANCE_ROOT
- Verificar: unit inactive; pgrep sin INSTANCE_ROOT; directorio ausente; :8766 sin kalma2 de esa instancia
- Vaults *.deploy-vault / *.preprod-vault: NO borrar salvo petición explícita (siguientes redeploys)
- Cero secretos en logs. No git rm de .dev.
```

---

## 2. Constantes y alcance

| Clave | Valor por defecto |
|-------|-------------------|
| `INSTANCE_NAME` | `SddIA_AP` |
| `INSTANCE_ROOT` | `/home/racso/Proyectos/${INSTANCE_NAME}` |
| `UNIT_AP` | `sddia-email-watcher@$(systemd-escape -p "$INSTANCE_ROOT").service` |
| `UNIT_LAB` | `sddia-email-watcher@$(systemd-escape -p /home/racso/Proyectos/SddIA).service` |

### En alcance

- `start-sddia.sh` y centinelas spawneados desde la instancia (`event-watcher`, `event-sweeper`, `kalma2-bridge`, `telegram-watcher` si el script los levantó).
- systemd `email-watcher@%f` **de esa instancia** (R-07).
- ELF bajo `{INSTANCE_ROOT}/SddIA/target/`.
- Árbol `{INSTANCE_ROOT}` (bundle, `.SddIA/`, `.events/`, logs).

### Fuera de alcance (prohibido)

| Recurso | Motivo |
|---------|--------|
| `/home/racso/Proyectos/SddIA` | Forja |
| `/home/racso/Proyectos/.dev` | SSOT de configuración |
| `UNIT_LAB` si está `active` | Ensayo lab; no es Paciente 0 |
| Plantilla `sddia-email-watcher@.service` | Instancia-agnóstica (`%f`) |
| `{INSTANCE_NAME}.deploy-vault` / `.preprod-vault` | Default: conservar |

---

## 3. Procedimiento (pasos)

Si `INSTANCE_ROOT` no existe: ejecutar igual stop/disable de `UNIT_AP` (unidad huérfana) y declarar APTO de teardown.

### 1 — Señalar `start-sddia` de la instancia

El script atrapa `SIGTERM` y apaga jobs/centinelas. Preferir eso a `pkill -9` ciego.

```bash
INST="${INSTANCE_ROOT:-/home/racso/Proyectos/SddIA_AP}"
# PIDs cuyo cwd o cmdline contiene INST/start-sddia.sh
pkill -TERM -f "${INST}/start-sddia.sh" 2>/dev/null || true
```

Espera breve del trap (~1 s). Si el proceso sigue: `pkill -KILL -f "${INST}/start-sddia.sh"`.

### 2 — systemd sensorial de la instancia

```bash
UNIT_AP="sddia-email-watcher@$(systemd-escape -p "$INST").service"
systemctl --user stop "$UNIT_AP" 2>/dev/null || true
systemctl --user disable "$UNIT_AP" 2>/dev/null || true
systemctl --user reset-failed "$UNIT_AP" 2>/dev/null || true
```

**No** `disable` de `sddia-email-watcher@.service` (plantilla). **No** tocar `UNIT_LAB`.

Otras unidades `sddia-daemon@*` cuyo `WorkingDirectory` sea `$INST`: `stop` + `disable` igual.

### 3 — Residuales ELF / puerto

```bash
pkill -TERM -f "${INST}/SddIA/target/" 2>/dev/null || true
# si quedan:
pkill -KILL -f "${INST}/SddIA/target/" 2>/dev/null || true
```

Comprobar que ningún proceso `kalma2-bridge` tenga cwd `$INST`. No matar un bridge de la forja (`:8765`).

### 4 — Borrar carpeta

```bash
# Abortar si INST está vacío, es "/", o es la forja
test -n "$INST" && test "$INST" != "/" && test "$INST" != "/home/racso/Proyectos/SddIA"
rm -rf "$INST"
```

### 5 — Verificación

| Check | APTO |
|-------|------|
| `test ! -e "$INST"` | directorio ausente |
| `systemctl --user is-active "$UNIT_AP"` | `inactive` o `unknown` |
| `pgrep -af "$INST"` | vacío (salvo el propio grep) |
| `ss`/`curl` `:8766` | no responde **o** no es el bridge de esa instancia |
| Forja | `test -d /home/racso/Proyectos/SddIA` |
| `.dev` | `test -f /home/racso/Proyectos/.dev/.env` |

---

## 4. Invariantes

| Regla | Motivo |
|-------|--------|
| Borrar solo `INSTANCE_ROOT` | Ceguera espacial; no wipe de Proyectos/ |
| SIGTERM antes que KILL | `start-sddia` cleanup (locks, hijos) |
| Plantilla `@.service` intacta | Siguiente `paciente0-deploy` reusa `%f` |
| Lab IMAP intacto | R-07 inverso: no castrar forja |
| Secretos no en git | Teardown no commitea `.env` |

---

## 5. Candidato a proceso Core (`paciente0-undeploy`)

**No forjar ahora.** Pareja de `paciente0-deploy`.

| Campo | Valor |
|-------|--------|
| `id` | `paciente0-undeploy` |
| `type` | `process` |
| `inputs` | `instance_name` (default `SddIA_AP`), `instance_parent`, `wipe_vaults` (default false) |
| `outputs` | `instance_root`, `unit_stopped`, `dir_removed` |

Fases: SignalScript → StopSystemd → ReapPids → RmTree → Verify. Prohibido `rm` fuera de `instance_root` resuelto.

---

## 6. Referencias

| Ref | Uso |
|-----|-----|
| `docs/todos/DeudaTecnica/[DEUDA] Paciente 0 — prompt y proceso de despliegue.md` | Simétrico deploy |
| `start-sddia.sh` | trap SIGTERM / cleanup |
| `SddIA/templates/systemd/sddia-email-watcher@.service.template` | Unidad `%f` |
| `SddIA/norms/sddia-distribution-protocol.md` | Multi-cliente hermético |
