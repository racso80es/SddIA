---
feature_name: sddia-deterministic-installer
created: "2026-09-25"
process: feature
branch_name: feat/sddia-deterministic-installer
persist_ref: docs/features/sddia-deterministic-installer
pbi_ref: docs/todos/pending/PBI-ARQUITECTURA-DEPLOY-DETERMINISTA.md
execution_id: "7b22f932-c162-4104-b38d-b1c9c6068414"
document_id: PBI-ARQUITECTURA-DEPLOY-DETERMINISTA
pbi_uuid: "c154bea0-c4c1-457e-b070-f7dfd3bc5f1b"
pbi_version: "1.0.0"
---

# Objetivos — sddia-deterministic-installer

## Misión

Erradicar la dependencia de la ventana de contexto de un LLM para el ciclo de vida de una instancia SddIA. Un orquestador determinista (Ceguera de Ejecución) lee configuración, materializa el bundle Full Node y enciende o destruye el entorno sin validaciones interpretativas.

## Alcance

1. Fachada `./sddia-installer.sh` + motor `SddIA/scripts/sddia-installer.sh` (`deploy` / `teardown`).
2. Perfil `full-node` en `build-release-bundle.sh` (todas las cápsulas nativas; sin Filtro C).
3. Cadena deploy: validar host → bundle `--out ROOT` → `instance-creator` → `systemctl --user enable --now` `@%f`.
4. Cadena teardown: stop/disable `@${ESC}` → `daemon-reload` → residual → `rm -rf ROOT`.
5. Abort fail-closed si destino vivo sin `--force`. Smoke QA en `/tmp`.

## Fuera de gate

Deploy live a `/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA`. Proceso `paciente0-undeploy`. Mutación de genoma (`instance-creator`, normas). Vaults `*.deploy-vault`.

## Ley aplicada

- PBI v1.0.0 Filtro A (laudos L-* en `clarify.md`).
- `features-documentation-pattern` v1.2.1: un PR; `validacion.md` APTO solo con CI verde (`run_id`).
- README / Constitución: default de ruta inyectable; last-resort PBI solo Zero-Touch.
- DA-2: scripts ≠ genoma. Git vía `git-manager` en DCC. `accept-pr` solo post-CI verde.

## Criterios

| ID | Criterio |
|----|----------|
| CA-ATOMIC | `./sddia-installer.sh deploy` (sin flags) materializa + enciende en ROOT resuelto; cero prompts. |
| CA-PAYLOAD | Bundle `full-node` contiene todas las cápsulas compilables; incluye al menos `CONSUMER_BINS`. |
| CA-TEARDOWN | Tras `teardown`, cero unidades `@${ESC}` active, cero procesos bajo ROOT, directorio ausente. |
| CA-LIVE | Destino vivo sin `--force` → abort ≠ 0; con `--force` → wipe previo + redeploy. |
| CA-FORGE | Teardown no toca forja ni bóveda. ROOT=forja → abort. |
| CA-CI | Checks GitHub del PR verdes (`run_id`) antes de `global: APTO` y `accept-pr`. |
