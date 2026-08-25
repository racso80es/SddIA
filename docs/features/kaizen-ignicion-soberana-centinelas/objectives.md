---
feature_name: kaizen-ignicion-soberana-centinelas
created: "2026-08-25"
process: feature
branch_name: feat/kaizen-ignicion-soberana-centinelas
persist_ref: docs/features/kaizen-ignicion-soberana-centinelas
pbi_ref: docs/todos/pending/REFACTOR - despliegue centinelas.md
document_id: PBI-KAIZEN-IGNICION-SOBERANA
uuid: "a2a69784-9dff-47ab-a0bb-aa3c576068b8"
execution_id: "7a0edc97-6a5e-4ee0-861a-894f9df6cc63"
mayeuta_verdict: ok
laudo: unidades-por-centinela-path-instance
---

# Objetivos — kaizen-ignicion-soberana-centinelas

## Misión

Transferir el ciclo de vida del bus EDA (`event-watcher`, `event-sweeper`) y de `kalma2-bridge` a **systemd --user** con el mismo patrón hermético `@%f` que ya usa `email-watcher`, de modo que un reboot con linger no deje el correo en limbo por bus muerto, y `start-sddia.sh` deje de ser el supervisor de esos procesos.

## Punto objetivo

> **O-IGNICION-SOBERANA:** `instance-creator` materializa en `{instance_root}/.SddIA/systemd/` una unidad `sddia-{daemon}@.service` por centinela base (fábrica `sddia-daemon@.service.template` + `@@DAEMON_NAME@@` + `@@SDDIA_CORE_ROOT@@` = raíz de instancia). En Linux con bus de usuario, `start-sddia.sh` instala esas unidades en `~/.config/systemd/user/`, hace `enable --now` con `systemd-escape -p "$REPO_ROOT"`, no spawnea esos binarios con `&`, y sale tras health HTTP + heartbeats. Cada centinela es una unidad independiente.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Fábrica systemd + unidades nombradas `@%f` | ExecStart directo a `target/release/` sin wrapper |
| `sddia-daemon@event-watcher` como SSOT de enable | Ese enable (rompe `%f`) |
| `start-sddia.sh` jurisdicción systemd vs script | IOTA relay como unidad |
| Lanzador `kalma2-bridge.sh` | Proceso `paciente0-undeploy` |
| Linger / `XDG_RUNTIME_DIR` como WARN | Kitchen NFT |
| Teardown Paciente 0: stop/disable de las nuevas unidades de instancia | Puerto literal 8766 en Core |
| ONBOARDING bundle / `start-sddia.md` | Segundo PR documental |

## Ley aplicada

- Ceguera espacial: `@@SDDIA_CORE_ROOT@@` = `instance_root`; `WorkingDirectory=%f`.
- Independencia: una unidad por centinela; Restart de email no toca el bus.
- Dualidad: jurisdicción systemd no spawnea los mismos ELF por consola.
- Git vía `skill:git-manager` en init; cierre documental en rama (un PR).
