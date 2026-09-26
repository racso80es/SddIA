---
document_id: PBI-ARQUITECTURA-INSTALLER-V3-UX-EJECUTABLES
uuid: "8cb95b4a-52f0-4030-91c1-8b937a425589"
title: "[ARQUITECTURA] Installer v3 — UX de los ejecutables Deploy y Eliminar Cliente"
format: markdown
version: "1.0.0"
status: pending
priority: alta
type: arquitectura
process: feature
dispatch: false
feature_name: sddia-installer-v3-ux-executables
historia_ref: docs/todos/historias/[ARQUITECTURA] Installer v3 — contrato de entrada-salida por comando y UX de Deploy - Eliminar Cliente.md
historia_document_id: HU-INSTALLER-V3-IO-CONTRACT-UX
historia_uuid: "489f5b85-fa2f-4f0e-ae52-7278e039dff1"
created: "2026-09-26"
author: tekton
blocked_by:
  - PBI-ARQUITECTURA-INSTALLER-V3-IO-CONTRACT
installer_contract_ref: SddIA/library/norms/sddia-installer-contract.md
installer_contract_uuid: "b1327ef3-5f07-4fba-9073-a72c5fdf97e2"
installer_contract_version_actual: "1.2.0"
installer_contract_version_objetivo: "1.3.0"
pbi_antecesores:
  - document_id: PBI-ARQUITECTURA-INSTALLER-V3-IO-CONTRACT
    uuid: "cb5483e7-4e39-4fb6-9c11-4a8285b957b7"
related:
  - SddIA/scripts/sddia-installer.sh
  - sddia-installer.sh
baseline_decisiones:
  - "D6: confirmación por la palabra ELIMINAR en TTY; --yes la omite; el motor sigue exigiendo --force"
  - "Norma 1.3.0 solo para presentador y atajos; 1.2.0 la publica el PBI de contrato"
---

# Installer v3 — UX de Deploy y Eliminar Cliente

Historia madre: `HU-INSTALLER-V3-IO-CONTRACT-UX` §4.5–§4.6, fases F4–F5, AC-9..AC-12. El envelope, los pasos y el canal de progreso los entrega `PBI-ARQUITECTURA-INSTALLER-V3-IO-CONTRACT`. Este PBI no arranca hasta que ese PBI esté en `done/`.

## 0. Hechos de partida

| Hecho | Evidencia |
|-------|-----------|
| `~/Aplicaciones/SddIA/SddIA_Deploy.sh` y `SddIA_Eliminar_Cliente.sh` están fuera de git. Hacen `exec` al installer de la forja. | lectura directa 2026-09-26 |
| Sin argumentos, Eliminar ejecuta `teardown --force`: `rm -rf` del root por defecto, sin confirmación. | `SddIA_Eliminar_Cliente.sh` |
| La documentación de features previas cita `SddIA_Teardown.sh`. El fichero real ya se llama `SddIA_Eliminar_Cliente.sh`. | `docs/features/sddia-deterministic-teardown/` |
| El motor no puede pedir la tecla ni la confirmación: I-UX-NOPROMPT-MOTOR queda sellado en la norma 1.2.0 por el PBI de contrato. | ese PBI, R-NORM-1 |

## 1. Intención

Que el doble clic en Deploy o en Eliminar Cliente muestre el paso en curso (`k/N — título`), termine con un resumen (resultado, puerto, servicios, acta, log, duración) y no cierre la ventana hasta pulsar una tecla. Eliminar Cliente pide la palabra `ELIMINAR` antes de borrar. Sin TTY, el presentador no pinta nada y deja pasar el envelope.

## 2. Baseline de decisiones (HU §7)

| ID | Baseline de este PBI |
|----|----------------------|
| D6 | En TTY, antes de invocar al motor: destino, `esc` y si el destino está vivo; hay que escribir `ELIMINAR`. Otra entrada → exit 3, `error.code=TEARDOWN_REQUIRES_FORCE`, motor no invocado. `--yes` omite la pregunta e invoca con `--force`. |
| Norma | Este PBI publica **1.3.0** (presentador, hold, atajos). No reabre 1.2.0. |

## 3. Requisitos

### 3.1 Presentador

| ID | Requisito |
|----|-----------|
| R-UX-1 | Script `SddIA/scripts/installer/sddia-installer-ui.sh`. Invoca la fachada `./sddia-installer.sh` y lee el canal de progreso (fd 3). No reimplementa deploy ni teardown. |
| R-UX-2 | Con TTY: una línea por paso, `k/N`, título de HU §4.4, estado `ok` / `failed` / `skipped`. Sin salida de `cargo`, `systemctl` ni JSON. En fallo: el paso, `error.code`, `message` y `result.log_ref`. |
| R-UX-3 | Resumen final, en este orden: Resultado, Puerto WUI, Servicios (activos y omitidos, solo nombres de unidad), Acta (`result.verify.audit_ref`), Log (`result.log_ref`), Duración. |
| R-UX-4 | Tras el resumen, éxito o fallo: `read -rsn1` si stdin es TTY y no hay `--no-hold` ni `SDDIA_INSTALLER_HOLD=0`. Tope `read -t 600`. Sin TTY: no espera y no pinta la UI; stdout = envelope de la fachada; `exitCode` propagado. |
| R-UX-5 | Si el presentador falla antes de poder pintar, imprime el envelope crudo y termina. No deja la ventana bloqueada sin el tope de R-UX-4. |
| R-UX-6 | Color solo si el terminal lo admite y `NO_COLOR` no está definido. |

### 3.2 Confirmación de borrado (G7)

| ID | Requisito |
|----|-----------|
| R-UX-7 | `teardown` con TTY y sin `--yes`: muestra destino y `esc`, pide `ELIMINAR`. Confirmado → fachada con `--force`. No confirmado → exit 3, envelope `TEARDOWN_REQUIRES_FORCE`, cero llamadas al motor. |
| R-UX-8 | El motor y la fachada siguen sin `read`. La palabra se pide solo en el presentador. I-TEAR-FORCE no cambia: el motor sigue rechazando teardown sin `--force`. |

### 3.3 Atajos (G6)

| ID | Requisito |
|----|-----------|
| R-UX-9 | Plantillas en `SddIA/scripts/installer/shortcuts/`: `SddIA_Deploy.sh`, `SddIA_Eliminar_Cliente.sh`. Opcional `SddIA_Deploy.desktop` y `SddIA_Eliminar_Cliente.desktop` con `Terminal=true`. Invocan el presentador `deploy` o `teardown`. La ruta de la forja se inyecta al generar; la plantilla no la lleva escrita. |
| R-UX-10 | `./sddia-installer.sh shortcuts --dest DIR` materializa los scripts, ejecutables. Segunda ejecución sin cambios de plantilla = no-op. Plantilla con `sha256` distinto → sobrescritura. Mismo criterio que `enable_units` con las plantillas systemd. |
| R-UX-11 | Documentar en la norma 1.3.0 el comando y que `~/Aplicaciones/SddIA/` deja de ser fuente. La regeneración en ese directorio del host es paso de operador post-merge, no gate del PR. |

## 4. Plan de ejecución

1. Confirmar `PBI-ARQUITECTURA-INSTALLER-V3-IO-CONTRACT` en `done/` y norma 1.2.0 publicada. Si no, no iniciar.
2. Init `feature` `sddia-installer-v3-ux-executables` con este PBI como `pbi_ref`.
3. Presentador (R-UX-1..8) y smoke sin TTY.
4. Plantillas y comando `shortcuts` (R-UX-9..10).
5. Norma **1.3.0** vía `entity-manager` / `norm-creator`: presentador, hold, palabra `ELIMINAR`, atajos generados. Hash por la cadena.
6. Cierre documental en la misma rama. Evolución con el `uuid` de este PBI y el de la historia.

## 5. Criterios de aceptación

| ID | Criterio | Verificación |
|----|----------|--------------|
| AC-9 | Con TTY: línea `k/N — título` por paso; resumen con Resultado, Puerto, Servicios, Acta, Log y Duración; no termina hasta `read -rsn1`. `--no-hold` termina solo. | `script` / `expect` |
| AC-10 | Sin TTY (`\| cat`): sin UI y sin espera; stdout = envelope de la fachada; `exitCode` propagado. | smoke |
| AC-11 | Eliminar con TTY exige `ELIMINAR`. Otra entrada → exit 3, `TEARDOWN_REQUIRES_FORCE`, motor no invocado. `--yes` invoca la fachada con `--force`. | `expect` |
| AC-12 | `shortcuts --dest /tmp/atajos` genera los dos `.sh` ejecutables. Segunda ejecución sin cambios = no-op. Plantilla modificada → sobrescritura. | smoke |
| AC-13 | Sigue en cero: `read`, `select`, `zenity`, `whiptail` en motor y fachada. El `read` vive solo en `sddia-installer-ui.sh`. | grep gate |
| AC-14b | Norma 1.3.0 publicada por la cadena, con hold, `ELIMINAR` y `shortcuts`. 1.2.0 no se reescribe. | index-integrity |
| AC-15 | `test-sddia-installer.sh` y `sddia-installer-smoke` verdes, incluidos los casos sin TTY y `shortcuts`. | CI |

## 6. Fuera de alcance

- Cambiar el envelope, los ids de paso, los códigos 1–7 o el canal de progreso (PBI de contrato).
- Wizard de bóveda (`DT-CONFIG-UX-ONBOARDING`).
- UI gráfica (zenity, GTK).
- Ejecutar el `shortcuts` real sobre `~/Aplicaciones/SddIA/` dentro del PR. Queda como paso de operador tras el merge.
- Forjar `paciente0-deploy` / `paciente0-undeploy`.

## 7. Riesgos

| Riesgo | Mitigación |
|--------|-----------|
| `read` bloquea la ventana si el presentador no llega al resumen. | Tope 600 s (R-UX-4) y fallback al envelope crudo (R-UX-5). |
| La palabra `ELIMINAR` estorba en uso repetido. | `--yes`. El motor no cambia. |
| Los atajos del host vuelven a divergir. | Generados desde plantilla; `shortcuts` idempotente; la norma 1.3.0 lo fija. El directorio `~/Aplicaciones/SddIA/` no entra en el diff. |

## 8. Dependencias

- Bloqueado por `PBI-ARQUITECTURA-INSTALLER-V3-IO-CONTRACT` (envelope, pasos, fd 3, norma 1.2.0).
- No bloquea a otros PBI. La regeneración de los atajos del escritorio es posterior al merge y la pide el operador.
