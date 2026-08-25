---
document_id: PBI-KAIZEN-IGNICION-SOBERANA
title: "[ARQUITECTURA] Ignición Soberana y Persistencia de Centinelas (Systemd Completo)"
format: markdown
version: "1.0.0"
status: cerrado
type: kaizen
priority: alta
process: feature
---

# [ARQUITECTURA] Ignición Soberana y Persistencia de Centinelas (Systemd Completo)

# pendiente de refinar. Prestar especial antención a posibles incohrencias, alucinaciones o inexactitudes.

## 0. Contexto y Fractura Termodinámica
El despliegue de **Paciente 0** sufre actualmente de "Ignición Híbrida". Mientras el `email-watcher` opera bajo la jurisdicción de `systemd` como servicio de usuario, el núcleo del bus de eventos (`event-watcher`, `event-sweeper`) y la interfaz (`kalma2-bridge`) dependen de la ejecución del script `start-sddia.sh` atado a la sesión de terminal. 

Esta arquitectura es frágil: un reinicio del equipo rompe el bus de eventos y el correo entra en un limbo (falla en el enrutamiento).

Este PBI erradica el script manual como motor de arranque y transfiere el control absoluto del ciclo de vida de **todos** los centinelas operativos a `systemd`.

## 1. Superficie de Refactorización

### 1.1. Modificación de `instance-creator` (Motor Core)
El archivo `SddIA/engine/execute-process/src/engine/handlers/instance_creator.rs` (o el submódulo encargado de la fase `Systemd`) debe ser refactorizado:
*   **Comportamiento Actual:** Instancia únicamente la plantilla `sddia-email-watcher@.service.template`.
*   **Nuevo Comportamiento:** Debe materializar las unidades para el resto de centinelas base (`event-watcher`, `event-sweeper`, `kalma2-bridge`) utilizando la plantilla genérica existente `SddIA/templates/systemd/sddia-daemon@.service.template`.
*   **Enrutamiento de Nombres:** Debe generar los archivos `.service` en el `instance_root` (o directorio de instalación) sustituyendo las variables de la plantilla (`@@DAEMON_NAME@@` o similar) por el nombre exacto de cada ejecutable.

### 1.2. Castración del Script `start-sddia.sh`
El script de ignición (presente en las plantillas del bundle/starter-kit) pierde su autoridad para levantar procesos en segundo plano.
*   **Poda Lógica:** Eliminar los comandos que ejecutan binarios con `&` y guardan PIDs.
*   **Transmutación a Diagnóstico/Control:** El script debe limitarse a invocar a `systemd`. En lugar de ejecutar los binarios, ejecutará:
    `systemctl --user enable --now sddia-daemon@event-watcher.service`
    `systemctl --user enable --now sddia-daemon@event-sweeper.service`
    `systemctl --user enable --now sddia-daemon@kalma2-bridge.service`
    `systemctl --user enable --now sddia-email-watcher@...service`
*   Debe verificar si la variable `$XDG_RUNTIME_DIR` está accesible y advertir si el usuario no ha activado `loginctl enable-linger`.

### 1.3. Ajuste de Plantillas Systemd
Revisar `SddIA/templates/systemd/sddia-daemon@.service.template` para asegurar que el `ExecStart` apunte correctamente al binario compilado de `SddIA/target/release/` dentro del `CORE_ROOT` de la instancia, garantizando la Ceguera Espacial (no hardcodear la ruta de la forja).

### 1.4. Ajuste en documentos de despliegue

Revisar y adecuar docs/todos/DeudaTecnica/[DEUDA] Paciente 0 — prompt de teardown.md y docs/todos/kitchen/PBI-KITCHEN-TOKENIZACION-NFT.md si es necesario contenido afectado por los cambios aplicados en el PBI

## 2. Restricciones Duras (Protocolo de Acero)
*   **No duplicación de ejecutores:** Ningún centinela debe poder ser arrancado por consola si ya está siendo gestionado por `systemd`. El diseño debe prevenir colisiones de puertos (en el caso de `kalma2-bridge`) o bloqueos de archivos en `.events/`.
*   **Independencia de Componentes:** Cada centinela tiene su propio `.service`. Si `email-watcher` cae, `systemd` lo reinicia sin afectar a `event-watcher`. No anidar servicios dependientes mediante scripts monolíticos.

## 3. Criterios de Aceptación (Gate de Salida)
*   [ ] `instance-creator` emite limpiamente los archivos `.service` para el núcleo del bus de eventos.
*   [ ] Ejecutar `start-sddia.sh` no deja procesos hijos huérfanos en la terminal, sino que activa las unidades de `systemctl --user`.
*   [ ] Un reinicio de la máquina host levanta automáticamente el bus de eventos y la interfaz web en el puerto `8766`, sin necesidad de hacer login en el entorno gráfico (gracias al Linger y a las unidades *enabled*).