---
document_id: AUDIT-PACIENTE0-LOCK-SESSION-CENTINELAS-20260825
uuid: "d1dfb969-2653-41d3-b80b-91a8a35ee476"
title: "Auditoría — bloqueo de sesión vs centinelas systemd Paciente 0"
created: "2026-08-25"
instance_path: /home/racso/Proyectos/SddIA_AP
bundle_manifest: "20260825T131532Z"
instance_creator_correlation_id: "bd5c7328-fc6b-4abc-a010-79cea1a1decf"
feature_name: kaizen-ignicion-soberana-centinelas
feature_persist_ref: docs/features/kaizen-ignicion-soberana-centinelas
kaizen_document_id: PBI-KAIZEN-IGNICION-SOBERANA
evolution_id: "181d6291-9735-4187-a6f7-f6e56472aa3e"
ola_deploy_audit_ref: docs/audits/paciente0-deploy-20260825T131532Z.md
forge_merge_ignicion: "44bf56fa9cc0d2e40c027dd66fce4c05ed82b30b"
verdict: GENOMA_YA_INYECTADO_LOCK_NO_TUMBA_UNIDADES
---

# Auditoría — bloqueo de sesión / linger / ignición soberana (Paciente 0)

Estímulo: al bloquear la sesión, los servicios de `SddIA_AP` se observan inactivos. Hipótesis del operador: falta teardown + redeploy de `main` actualizado + ignición soberana. Contraste empírico contra genoma `kaizen-ignicion-soberana-centinelas` y journal del host. Cero secretos.

## 0. Veredicto

**La secuencia teardown→bundle→creator→`start-sddia` no es el hueco.** El Paciente 0 **ya** corre el genoma post-merge de ignición soberana. systemd **no** paró las unidades `@%f` en los bloqueos de esta tarde. Lo que sí ocurrió, encadenado al bloqueo, fue **suspender el host** (Cinnamon + lid/idle). Linger no cubre suspender. El objetivo de la feature es supervisor systemd + reboot con linger, no «pantalla bloqueada ≡ proceso despierto» ni «sleep ACPI».

| Hipótesis sugerida | Estado empírico |
|--------------------|-----------------|
| Unidades viejas / `start-sddia` supervisor | **Falsa.** `start-sddia.sh` instancia = forja (SHA-256 idéntico). Sin proceso `start-sddia` reteniendo. PIDs bajo `user@1000`. |
| Bundle anterior al merge ignición | **Falsa.** Merge `44bf56fa` 15:09:18+02; bundle `20260825T131532Z` 15:15:32+02; ignición 15:16:38+02. |
| Linger ausente | **Falsa.** `Linger=yes`. `user@1000.service` active. |
| `WantedBy=graphical-session.target` | **Falsa.** Plantilla y unidades: `WantedBy=default.target`. `BindsTo`/`PartOf` vacíos. `graphical-session.target` del user manager: **inactive** (Cinnamon no lo usa como paraguas). |
| Falta redeploy para «envolver» linger | **No aplica.** Enable `--now` ya en `default.target.wants/` (15:16). |

## 1. Contrato de la feature vs lo observado

Misión (`objectives.md`): ciclo de vida del bus y `kalma2-bridge` a **systemd --user** `@%f`; reboot **con linger** no deja el correo sin bus; `start-sddia.sh` deja de ser supervisor.

Laudo D5 (`clarify.md`): linger / `XDG_RUNTIME_DIR` = **WARN**, no fallo de código. `validacion.md`: **AC-REBOOT = NO_APTO** (reboot host no ensayado; no gate).

Fuera de contrato explícito: bloqueo de screensaver, suspend ACPI, ecryptfs al lock, sockets IMAP/HTTP congelados.

## 2. Materialización ya presente (ola 13:15Z)

| Check | Evidencia |
|-------|-----------|
| Genoma `start-sddia.sh` | SHA `b2679357…` forja = instancia |
| Unidades instancia | `{instancia}/.SddIA/systemd/sddia-{event-watcher,event-sweeper,kalma2-bridge,email-watcher,telegram-watcher}@.service` |
| Sync user | `~/.config/systemd/user/` mismo `ExecStart` / `WorkingDirectory=%f` / `WantedBy=default.target` |
| Enable | symlinks 15:16 en `default.target.wants/` hacia plantillas `@.service` |
| Jurisdicción | log ignición: systemd; `exit 0`; sin spawn `&` de bus/WUI/email |
| CORE_ROOT | `ExecStart` bajo `/home/racso/Proyectos/SddIA_AP/SddIA/` |

Fósil no causal: `sddia-daemons.target` (jun-16) sigue en `default.target.wants`; **inactive/dead**; no tiene `ExecStart`; no para las unidades `@%f`.

## 3. Timeline host (journal)

| UTC+2 | Hecho |
|-------|--------|
| 15:16:38–39 | `Started` las cinco unidades AP. PIDs actuales nacen aquí. `NRestarts=0`. |
| 15:20 / 15:22 / 15:24 | `cinnamon-screensaver` (lock). `pam_ecryptfs: seteuid error` en unlock. |
| 15:25:18 | NetworkManager **sleep**; `Reached target sleep.target`; `systemd-sleep` **suspend**. |
| 17:15:56 | Resume. NM wake. Screensaver PAM otra vez. |
| 15:16 → 17:20 | **Cero** líneas `Stopped`/`Failed` en unidades `sddia-*@…SddIA_AP`. |

PIDs 81239/81266/81293/81312/81324 vivos ~2 h (incl. el sleep). WUI post-auditoría: HTTP 200 `:8766`.

Conclusión de journal: **bloqueo ≠ `systemctl stop`**. El huso «inactivo» coincide con **máquina suspendida** y/o **red ASLEEP**, no con unidades `inactive` de systemd.

## 4. Por qué el lock *parece* tumbar el escudo

1. **Cinnamon:** `lock-enabled=true`; idle AC `sleep-inactive-ac-type=suspend`. Bloquear y cerrar tapa / idle 1800 s encadena screensaver → **suspend**. Linger no impide `systemd-sleep`.
2. **Durante suspend** los ELF no corren; HTTP/IMAP no contestan. Es indistinguible de «servicio caído» si no se mira `systemctl --user is-active`.
3. **ecryptfs** en `/home/racso`. PAM del screensaver registra `pam_ecryptfs: seteuid error`. Procesos con fd abiertos siguen; accesos nuevos a `$HOME` pueden fallar. No hay stop systemd asociado.
4. Feature **no** declara `After=sleep.target` / `ExecStop` / restart-on-resume. `Restart=always` no dispara si el proceso no muere (freeze ACPI).

## 5. Residual (no causa del síntoma lock)

- Launchers: `kalma2-bridge.sh` prueba `target/debug` **antes** que `release`. Procesos actuales resuelven `…/SddIA_AP/SddIA/target/debug/…` pese a bundle consumer en `release/`. Analogía F-DEP-07 en centinelas, no en el orquestador de ignición.
- Instancia contiene árbol `target/debug/` (no Filtro C de binario github; sí ruido de perfil).
- AC-REBOOT sigue sin ensayo de **reboot** (distinto de lock y de suspend).

## 6. Qué haría y qué no haría teardown→redeploy

Re-ejecutar el prompt de deuda (stop/disable `@%f` AP, matar hijos de `start-sddia`, wipe, bundle, creator, `start-sddia` `env -u`) **reinstala lo mismo** respecto a lock/suspend/linger. Útil solo si se quiere: limpiar `sddia-daemons.target` legado, forzar ELF release, o repetir AC-REBOOT con reboot real.

No forjar genoma en este estímulo. No abrir Kaizen de lock salvo que se declare alcance nuevo: sobrevivir screensaver, o `HandleLidSwitch`, o restart post-`sleep.target` (eso **no** está en `PBI-KAIZEN-IGNICION-SOBERANA`).

## 7. Criterio de repetición empírica (operador)

Para no confundir lock con suspend:

1. `systemctl --user is-active sddia-kalma2-bridge@$(systemd-escape -p /home/racso/Proyectos/SddIA_AP).service` **antes** del lock.
2. Bloquear **sin** cerrar tapa y **sin** idle-suspend (o `systemctl mask sleep.target` de laboratorio).
3. Desde otra TTY/SSH con linger: el mismo `is-active` debe seguir `active`; `journalctl --user -u …` sin `Stopped`.
4. Lid/suspend es otro ensayo (ACPI); no es el AC de la feature.
