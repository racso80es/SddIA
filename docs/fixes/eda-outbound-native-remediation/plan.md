---
feature_name: eda-outbound-native-remediation
created: "2026-07-13"
process: bug-fix
branch_name: fix/eda-outbound-native-remediation
persist_ref: docs/fixes/eda-outbound-native-remediation
phases:
  - contract-and-configuration
  - native-transport-implementation
  - router-and-dead-letter-integration
  - verification
---

# Plan — remediación de entrega saliente EDA

## Decisión de partida

No se cambiará la preferencia del router a WASI: ya usa `prefer_wasm: false` y resuelve los binarios nativos. La corrección consiste en reemplazar las respuestas stub de esos binarios por transportes nativos con red explícita y configurada.

## Fase 1 — Contrato y configuración

1. Inventariar las variables de bóveda ya disponibles para IOTA y Telegram, sin exponer sus valores.
2. Definir el contrato de entrada/salida de ambos tools: campos requeridos, errores clasificados y forma de `transaction_digest`.
3. Documentar el modo laboratorio: doble local explícito, sin salida a redes públicas.
4. Definir qué errores son recuperables y qué detalle mínimo puede pasar a un testigo dead-letter.

**Salida:** contrato de transporte y matriz de configuración para pruebas.

## Fase 2 — Implementaciones nativas

1. Implementar el transporte HTTP nativo de Telegram con timeout, validación de token/configuración y normalización JSON.
2. Seleccionar e integrar el adaptador nativo de IOTA compatible con el runtime Rust actual; aislarlo detrás del contrato de publicación.
3. Mantener el artefacto WASI sin red y con fallo explícito para operaciones salientes.
4. Garantizar que stdout conserva exclusivamente el sobre JSON y que los diagnósticos no contienen secretos.

**Salida:** tools nativos funcionales y compilables.

## Fase 3 — Integración EDA

1. Confirmar con pruebas que `route_domain_core` resuelve las dos cápsulas como `native` cuando `prefer_wasm: false`.
2. Ajustar los payloads de invocación solo si el contrato de Fase 1 lo exige; no incluir credenciales en ellos.
3. Preservar el protocolo actual de `route-domain-event`: éxito sella entrega; fallo genera testigo dead-letter y conserva la trazabilidad Kaizen.
4. Añadir mensajes de error distingibles para configuración ausente, timeout y rechazo remoto.

**Salida:** fan-out EDA compatible con los transportes nativos.

## Fase 4 — Verificación

1. Pruebas unitarias de ambos tools: entrada inválida, configuración ausente, éxito simulado y error remoto.
2. Pruebas de integración de `route-domain-event` con dobles locales para `PullRequest_Presented` y un evento de dominio anclable.
3. Verificar que el binario WASI sigue sin red y no se selecciona en el flujo saliente.
4. Ejecutar `event-bus-audit` tras los smokes y comprobar ausencia de nuevos dead-letters atribuibles a los stubs.
5. Registrar resultados en `implementation.md`, `execution.md` y `validacion.md` únicamente durante la fase de ejecución posterior.

**Salida:** evidencia reproducible para los criterios de aceptación.

## Criterio de parada de esta sesión

La planificación queda completada con `objectives.md`, `spec.md` y este `plan.md`. No se implementan cambios de código, no se modifica el PBI ni se inicia cierre documental o entrega remota.
