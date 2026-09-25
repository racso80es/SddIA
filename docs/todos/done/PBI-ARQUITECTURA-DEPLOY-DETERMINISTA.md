---
document_id: PBI-ARQUITECTURA-DEPLOY-DETERMINISTA
uuid: "c154bea0-c4c1-457e-b070-f7dfd3bc5f1b"
title: "[ARQUITECTURA] Orquestador Físico de Despliegue y Borrado (SddIA Installer)"
format: markdown
version: "1.0.0"
status: done
priority: alta
type: arquitectura
process: feature
feature_name: sddia-deterministic-installer
branch_name: feat/sddia-deterministic-installer
persist_ref: docs/features/sddia-deterministic-installer
related:
  - SddIA/norms/sddia-distribution-protocol.md
  - SddIA/process/instance-creator.md
---

# [ARQUITECTURA] Orquestador Físico de Despliegue y Borrado (SddIA Installer)

## 1. Intención Estratégica
Erradicar la dependencia de la ventana de contexto de un LLM para las operaciones de infraestructura. Se diseña una herramienta determinista y autocontenida que asume la responsabilidad absoluta del ciclo de vida de una instancia SddIA (Despliegue y Borrado). La herramienta operará bajo el principio de "Ceguera de Ejecución": leerá una configuración, materializará los activos y encenderá o destruirá el entorno sin requerir validaciones interpretativas.

## 2. Especificaciones de Configuración y Entorno
El orquestador debe ser capaz de arrancar sin que el Vértice Biológico le inyecte parámetros explícitos, garantizando un despliegue operativo "Zero-Touch".

*   **Destino de Materialización (Default):** `/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA`. Si no se provee un flag o variable de entorno de sobreescritura, la herramienta asumirá esta coordenada espacial de forma innegociable.
*   **Payload de Cápsulas (Full Node):** Se elimina el límite restrictivo de los 9 ELF del perfil `consumer` clásico. Por defecto, el empaquetador acoplará **todas las cápsulas disponibles** (incluyendo herramientas de orquestación avanzadas, router Multi-LLM, adaptadores de inferencia y scripts de mantenimiento), dotando a la instancia de la capacidad operativa completa (S+ Grade).
*   **Configuración Base:** La herramienta heredará la bóveda local estándar (`.dev/.env`) si no se indica un archivo de configuración explícito para la instancia.

## 3. Fases del Flujo de Despliegue (Deploy)
La ejecución del comando de instalación (ej. `./sddia-installer.sh deploy`) desencadenará atómicamente la siguiente línea de montaje:
1.  **Validación de Entorno:** Comprobación de dependencias del host (Rust/Cargo si requiere compilación, `systemctl --user`, etc.).
2.  **Forja de Bundle:** Invocación interna a `build-release-bundle.sh` inyectando la directiva de empaquetado total de cápsulas.
3.  **Inyección de Creador:** Ejecución de `instance-creator` con la ruta de destino asignada.
4.  **Ignición Sensorial (Systemd):** Renderizado, copiado a `~/.config/systemd/user/` y habilitación (`enable --now`) de todos los centinelas correspondientes al ecosistema.

## 4. Fases del Flujo de Borrado (Teardown)
La ejecución del comando de destrucción (ej. `./sddia-installer.sh teardown`) aplicará el protocolo de tierra quemada sobre la instancia, asegurando que no queden procesos fantasma:
1.  **Parada Sensorial:** Identificación y parada (`stop` + `disable`) de todos los servicios systemd de usuario asociados a la ruta del despliegue (escapada vía `systemd-escape`).
2.  **Purga de Unidades:** Eliminación de los archivos `.service` en `~/.config/systemd/user/` vinculados a la instancia, seguido de un `daemon-reload`.
3.  **Wipe Físico:** Borrado recursivo (`rm -rf`) del directorio de la instancia (`/home/racso/Aplicaciones/Asistencia_Tormentosa_SddIA`).

## 5. Criterios de Aceptación (Protocolo de Acero)
*   [x] **Despliegue Atómico:** `./sddia-installer.sh deploy` sin prompts; cadena bundle `full-node` + `instance-creator` + systemd `@%f`. Smoke dry-run + CI `sddia-installer-smoke`. Deploy live ruta PBI = fuera de gate.
*   [x] **Payload Íntegro:** `--profile full-node --list-capsules` ⊇ CONSUMER_BINS + crate extra.
*   [x] **Teardown Limpio:** `stop`+`disable` `@${ESC}`; locks PID; `rm -rf ROOT`. Exige `--force`. Dry-run esc coherente.
*   [x] **Resiliencia de Sobreescritura:** destino vivo sin `--force` → exit 2.
