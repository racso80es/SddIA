---
feature_name: kaizen-ignicion-soberana-centinelas
created: "2026-08-25"
process: feature
purpose: Estabilización Mayeuta — PBI-KAIZEN-IGNICION-SOBERANA
branch_name: feat/kaizen-ignicion-soberana-centinelas
persist_ref: docs/features/kaizen-ignicion-soberana-centinelas
pbi_ref: docs/todos/pending/REFACTOR - despliegue centinelas.md
document_id: PBI-KAIZEN-IGNICION-SOBERANA
uuid: "a2a69784-9dff-47ab-a0bb-aa3c576068b8"
execution_id: "7a0edc97-6a5e-4ee0-861a-894f9df6cc63"
mayeuta_verdict: ok
laudo: unidades-por-centinela-path-instance
---

# Clarificación — kaizen-ignicion-soberana-centinelas

Transcript Mayeuta (2026-08-25). Semilla PBI v1.0.0 marcada **pendiente de refinar**. Filtro A contra genoma vigente. No se implementa el texto literal.

---

## D0 — Apertura formal

| Pregunta | Decisión |
|----------|----------|
| Proceso | `feature` v1.3.2 (Kaizen ignición; el filename PBI dice REFACTOR, el frontmatter dice `feature`) |
| `feature_name` | `kaizen-ignicion-soberana-centinelas` |
| Rama | `feat/kaizen-ignicion-soberana-centinelas` |
| `persist_ref` | `docs/features/kaizen-ignicion-soberana-centinelas` |
| `document_id` | `PBI-KAIZEN-IGNICION-SOBERANA` |
| Init lab | `./sddia-run.sh --process feature` + skips archive/delivery + `SDDIA_AGENT_RUNTIME_COMMAND=` |
| `execution_id` | `7a0edc97-6a5e-4ee0-861a-894f9df6cc63` |
| Antecesor | `kaizen-consumer-ignition-filtro-c` (F-08 `%f`) + R-07 email systemd |

**Toll:** un `persist_ref`, un PR.

---

## D1 — «instance-creator solo emite email-watcher»

| Semilla | Filtro A | Laudo |
|---------|----------|-------|
| Fase Systemd instancia únicamente `sddia-email-watcher@.service.template` | **Falso.** `install_systemd_templates` copia **todos** los `.template` de `SddIA/templates/systemd/` | El hueco no es el copiado: es que `sddia-daemon@.service` usa `%i` como nombre de binario **y** `%f` como cwd. Un solo especificador systemd no puede ser a la vez `event-watcher` y path de instancia |
| «Generar `.service` sustituyendo `@@DAEMON_NAME@@`» | Coherente como **fábrica** | **Dentro.** Renderizar `sddia-{daemon}@.service` con `WorkingDirectory=%f` y `ExecStart=…/scripts/daemons/@@DAEMON_NAME@@.sh`. Enable: `sddia-{daemon}@$(systemd-escape -p "$INSTANCE_ROOT").service` — mismo patrón que email |

---

## D2 — ExecStart a `target/release/`

| Semilla | Filtro A | Laudo |
|---------|----------|-------|
| Plantilla debe apuntar a `SddIA/target/release/` | **Inexacto.** Vigente: `SddIA/scripts/daemons/%i.sh` → `_run_daemon.sh` → `SddIA/daemons/{name}.sh` + locks CEN-01 | **Fuera** sustituir wrappers por ELF crudo. Pierde lock/bóveda. Ceguera espacial = `@@SDDIA_CORE_ROOT@@` + `%f`, no hardcode de forja |
| `kalma2-bridge` | No hay `scripts/daemons/kalma2-bridge.sh` | **Dentro.** Lanzador nuevo que resuelve ELF nativo y `exec` |

---

## D3 — Comandos `sddia-daemon@event-watcher`

| Semilla | Filtro A | Laudo |
|---------|----------|-------|
| `systemctl --user enable --now sddia-daemon@event-watcher.service` | Rompe F-08: `%i=event-watcher` deja `%f` vacío | **Prohibido** como SSOT. Unidades nombradas `sddia-event-watcher@%f` |

---

## D4 — Puerto 8766 y «todos» los centinelas

| Semilla | Filtro A | Laudo |
|---------|----------|-------|
| WUI en `:8766` | Default Core `SDDIA_CLIENT_PORT` = **8765**. `8766` es bóveda Paciente 0 | Criterio de aceptación = HTTP en `$SDDIA_CLIENT_PORT`, no literal 8766 |
| Erradicar `&` y PIDs | `start-sddia.sh` spawnea REQUIRED + opcionales + bridge + IOTA relay | **Dentro** para bus + bridge + email (si IMAP). IOTA relay **fuera** de systemd este ciclo (residual documentado) |
| Telegram/github | PBI no los lista; R-07 ya omite spawn sensorial | Fábrica emite unidades; enable sigue la política de spawn vigente (telegram si token; github si no consumer) |

---

## D5 — Dualidad ejecutores

| Semilla | Laudo |
|---------|-------|
| No consola si systemd gestiona | Jurisdicción `SDDIA_DAEMON_JURISDICTION=systemd\|script`. Default: `systemd` si `systemctl --user` habla con el bus; si no, `script` (lab sin user session) |
| Linger / `XDG_RUNTIME_DIR` | Advertir; no fallar laudo si el operador ignora linger (AC de reboot queda NO_APTO operativo, no de código) |

---

## D6 — Docs deuda / kitchen

| Path | Laudo |
|------|-------|
| `docs/todos/DeudaTecnica/[DEUDA] Paciente 0 — prompt de teardown.md` | **Dentro.** Tabla de unidades: stop/disable de `sddia-{daemon}@%f` de la instancia |
| `docs/todos/kitchen/PBI-KITCHEN-TOKENIZACION-NFT.md` | **Fuera.** Cero mención a ignición/systemd |

---

## Fuera de alcance

- Forjar proceso `paciente0-undeploy` (DA-2).
- Unidad systemd para IOTA relay.
- Castrar `start-sddia.sh` en Windows / ausencia de `systemctl --user`.
