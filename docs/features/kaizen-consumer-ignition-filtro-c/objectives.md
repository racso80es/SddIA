---
feature_name: kaizen-consumer-ignition-filtro-c
created: "2026-08-20"
process: feature
branch_name: feat/kaizen-consumer-ignition-filtro-c
persist_ref: docs/features/kaizen-consumer-ignition-filtro-c
pbi_ref: docs/todos/pending/[KAIZEN] perfil ignición consumidor Filtro C.md
document_id: PBI-KAIZEN-CONSUMER-IGNITION-FILTRO-C
uuid: "1c70e777-9b7f-4ad3-ada5-225ab6d141c6"
execution_id: "9594b963-49a2-4ca0-8173-35ed0a986b63"
mayeuta_verdict: ok
laudo: perfil-consumidor-tripartita-via-c
---

# Objetivos — kaizen-consumer-ignition-filtro-c

## Misión

Cerrar la frontera Core ↔ Cliente: perfil **consumidor** (Filtro C) sin derrame de ingeniería; ignición sin colisión sensorial; catch-up IMAP de últimos 50 UIDs; bundle hermético con cápsulas del códice; y despliegue reproducible vía **tripartita** (norma Vía C evolucionada + proceso `instance-creator` + `ONBOARDING.md` autogenerado), con smoke post-ignición y systemd multi-cliente hermético.

## Punto objetivo

> **O-CONSUMER-IGNITION:** Una instancia consumidor se despliega e ignita en host compartido sin PIDs/locks cruzados, sin herramientas de forja activas, con sensores bajo jurisdicción única, y con verificación `success: true` orquestada —sin inventar un segundo stack de QA.

## Alcance

| Dentro | Fuera |
|--------|-------|
| Perfil consumidor en ignición / WUI / gate Fracture | Castrar genoma de ingeniería del lab |
| R-07 anti-doble-spawn | Wizard UX (`DT-CONFIG-UX-ONBOARDING`) |
| F-07 últimos 50 UIDs en `last_uid=0` | Reescribir Kalma2 completa |
| `build-release-bundle` + cápsulas del grafo (F-06) | Extirpación total `.rs`/`.py` del upstream (meta faseada) |
| Evolucionar `sddia-distribution-protocol` + UUID v4 | Crear norma duplicada |
| CREATE `instance-creator` + orquestar smoke existente | Binario inventado `sddia` |
| F-08 `%f` multi-instancia; F-09 constitución consumidor | Dominios de negocio ajenos |
| Re-despliegue Paciente 0 (opcional demostración) | Secretos en git |

## Objetivos medibles

| ID | Objetivo | Criterio |
|----|----------|----------|
| **O1** | Filtro C | Consumidor: 0 `github-bridge`; WUI sin Forjar usable; gate Fracture → 0 procesos de ingeniería |
| **O2** | R-07 | Systemd sensorial activo ⇒ script no spawnea email/telegram watchers |
| **O3** | F-07 | `last_uid=0` selecciona los 50 UIDs más recientes (no `SINCE` calendárico como criterio primario) |
| **O4** | F-06 | Bundle incluye cápsula eferente del códice verificable (`send-telegram-notification`) |
| **O5** | Tripartita | Norma evolucionada + `instance-creator` invocable vía `sddia-run` + `ONBOARDING.md` alineado al artefacto |
| **O6** | Smoke | Post-ignición: `eda-local-topology-test` / `Local_QA_Requested` → `success: true` |
| **O7** | F-08 | ≥2 instancias mismo host: WD distintos; cero PIDs/locks/credenciales cruzados |
| **O8** | F-09 | Constitución consumidor sin L2 Windows+pwsh en Linux |

## No objetivos

- Sustituir `sync-client-assets` (complementa, no absorbe).
- Exigir re-despliegue Paciente 0 como único gate de merge.
- Dualidad de norma de distribución.
- Alojamiento de secretos preprod en el repo.

## Ley aplicada

- `features-documentation-pattern` v1.2.1 / proceso `feature` v1.3.2
- `external-ai-constraints` DA-2…DA-5
- `sddia-distribution-protocol` (evolucionar; no duplicar)
- `capsule-json-io` v2.0
- Clarificaciones D0–D9 en `clarify.md` (laudo **perfil-consumidor-tripartita-via-c**)
