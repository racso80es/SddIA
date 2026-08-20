---
feature_name: kaizen-consumer-ignition-filtro-c
created: "2026-08-20"
process: feature
base: main
scope: kaizen-consumer-ignition-filtro-c
version_spec: "1.0.0"
document_id: PBI-KAIZEN-CONSUMER-IGNITION-FILTRO-C
uuid: "1c70e777-9b7f-4ad3-ada5-225ab6d141c6"
laudo: perfil-consumidor-tripartita-via-c
---

# Especificación — kaizen-consumer-ignition-filtro-c

## 1. Laudos Dedalo

| Ref | Pregunta | Laudo | Justificación |
|-----|----------|-------|---------------|
| **L-PROFILE** | ¿Cómo se activa Filtro C? | Variable/bóveda `SDDIA_RUNTIME_PROFILE=consumer\|engineering` (default lab=`engineering`). | Proyección; no borrar genoma. |
| **L-BRIDGE** | ¿github-bridge? | Consumidor: fuera de `OPTIONAL_DAEMONS` / no spawn. | F-04 ensayo Paciente 0. |
| **L-WUI-FORGE** | ¿Forjar Proceso? | Consumidor: botón ausente o disabled + handler no-op/403. Lab intacto. | Misma proyección. |
| **L-FRACTURE** | ¿Suscriptores forja? | Gate en router/dispatch: perfil consumer ⇒ skip acciones de forja documental. JSON de suscripciones Core **no** se vacía. | Evita castrar lab; cierra derrame. |
| **L-R07** | ¿Detección jurisdicción? | `SDDIA_SENSORIAL_JURISDICTION=systemd` **o** unidad user activa con `WorkingDirectory` = `$REPO_ROOT`. Si true → no spawn email/telegram desde script. | Determinista; auditable. |
| **L-F07** | ¿Criterio IMAP `last==0`? | Buscar UIDs del mailbox; tomar los **N** mayores (`N=max_uids_per_poll`, default 50); emitir/procesar en orden. Abandonar `SINCE` calendárico como camino primario de bootstrap. | F-07; lookback días queda legado/opt-in si Dedalo documenta flag. |
| **L-BUNDLE** | ¿Qué es `build-release-bundle`? | Script/cápsula en árbol no-genoma o tool forjado: empaqueta binarios runtime + contratos/códice + cápsulas del grafo; **excluye** `target/` fuentes de ingeniería y docs de lab. Genera `ONBOARDING.md`. | F-06 + tripartita. |
| **L-NORM** | ¿Nueva norma? | **UPDATE** `sddia-distribution-protocol` vía creator; rehabilitar uuid v4; añadir secciones bundle / instance-creator / ONBOARDING / `%f`. | PBI §0. |
| **L-CREATOR** | ¿instance-creator? | CREATE proceso vía `entity-manager`. Fases lógicas: topología `.SddIA/` → secretos desde vault/plantilla → unidades systemd `%f` → ignición → smoke. | 0 hits hoy. |
| **L-SMOKE** | ¿QA nuevo? | Orquestar `eda-local-topology-test` + estímulo `Local_QA_Requested`. Gate `success:true`. | Reuso; no dualidad. |
| **L-F08** | ¿Daemon lab-fijo? | Plantillas `%f` para daemons de instancia; documentar migración de `~/.config/systemd/user/sddia-daemon@.service` WD lab. | F-08. |
| **L-F09** | ¿Constitución? | Plantilla/proyección consumidor sin L2 Windows+pwsh; instancia Paciente 0 la consume. | F-09. |
| **L-FORGE** | ¿Mutación? | Genoma (`process/`, `norms/`, …) solo `entity-manager`. Scripts raíz / daemons src / WUI / systemd templates: mutación Tekton bajo topología feature activa. | DA-2/DA-4. |

### Rechazados

- Norma nueva paralela a `sddia-distribution-protocol`.
- Binario CLI `sddia` inventado.
- Vaciar suscriptores Fracture del Core.
- Smoke QA duplicado.
- Wizard UX en este ciclo.
- Extirpación total de fuentes upstream como gate.

## 2. Circuito objetivo

```text
[perfil=consumer]
  start-sddia.sh
    → REQUIRED: event-watcher, event-sweeper
    → OPCIONAL: telegram-watcher IFF jurisdiction≠systemd
    → NUNCA: github-bridge-watcher
    → email-watcher IFF jurisdiction≠systemd
  kalma2 WUI: forge hidden
  route Fracture: skip forja

[bundle]
  build-release-bundle
    → binarios + cápsulas(códice) + contratos
    → ONBOARDING.md (paridad)
    → sin fuentes ingeniería

[deploy]
  ./sddia-run.sh --process instance-creator
    → .SddIA/ topología + vault
    → systemd @%f
    → ignición
    → eda-local-topology-test / Local_QA_Requested → success:true
```

## 3. Touchpoints (mapa)

| Área | Paths / entidades | Acción |
|------|-------------------|--------|
| Ignición | `start-sddia.sh`, `start-sddia.md` | Perfil + R-07 |
| WUI | `interfaces/kalma2/index.html`, `app.js` | Forge consumer |
| Fracture gate | router domain / subscriptions consumer overlay | Skip forja |
| IMAP | `SddIA/daemons/email-watcher` | F-07 |
| Bundle | nuevo `build-release-bundle` (+ tool si procede) | F-06 + ONBOARDING |
| Norma | `sddia-distribution-protocol` | UPDATE + uuid v4 |
| Proceso | `instance-creator` | CREATE |
| Systemd | templates `%f`; doc migración daemon@ | F-08 |
| Constitución | plantilla consumidor / sync | F-09 |
| Smoke | `eda-local-topology-test`, `Local_QA_Requested` | Orquestar |

## 4. Criterios de aceptación (Argos)

Mapa 1:1 con O1–O8 de `objectives.md` + checklist §4 del PBI. Dual-instancia F-08 obligatoria en validación. Re-despliegue Paciente 0: **recomendado**, no gate duro si O6+O7 APTO.

## 5. Riesgos

| Riesgo | Mitigación |
|--------|------------|
| Gate Fracture rompe lab | Solo bajo `profile=consumer` |
| F-07 cambia semántica IMAP | Tests unitarios + ensayo preprod opcional |
| Bundle incompleto (F-06) | Smoke verifica presencia cápsula del códice |
| Norma uuid inválido bloquea aduana | Rehabilitar v4 en misma forja UPDATE |
