---
feature_name: eda-outbound-native-remediation
created: "2026-07-13"
process: bug-fix
branch_name: fix/eda-outbound-native-remediation
persist_ref: docs/fixes/eda-outbound-native-remediation
pbi_ref: "docs/todos/pending/[FIX] EDA — remediar suscriptores IOTA y Telegram con red nativa.md"
scope: outbound-eda-delivery
---

# Especificación — remediación de entrega saliente EDA

## Hallazgo confirmado

`route_domain_core` invoca ambos tools con `prefer_wasm: false`; el resolvedor selecciona el binario nativo si existe. Por tanto, la causa actual no es que el router elija WASI: los binarios nativos de `iota-immutable-publisher` y `send-telegram-notification` son stubs que devuelven un fallo deliberado por ausencia de capacidad de red.

El cambio debe sustituir dichos stubs por implementaciones nativas operativas, sin conceder red implícita a las variantes WASI.

## Objetivo técnico

Entregar IOTA y Telegram desde las cápsulas nativas mediante contratos JSON stdin/stdout, configuración de red inyectada por la bóveda y errores normalizados para los testigos EDA.

## Límites de seguridad

- Las credenciales, tokens, URLs privadas y payloads sensibles no se escriben en eventos, dead-letters ni logs.
- La configuración procede de variables de entorno cargadas por la jerarquía de bóveda; el Core no contiene valores de instancia.
- WASI permanece sin red y debe fallar de forma explícita si se solicita una operación saliente.
- Las pruebas usan dobles HTTP/locales; el modo de laboratorio no publica IOTA ni envía Telegram real.

## Interfaces afectadas

| Componente | Cambio previsto |
|---|---|
| `route_domain_core` | Conservar la invocación nativa y enriquecer únicamente los payloads no sensibles necesarios para cada tool. |
| `send-telegram-notification` | Implementar cliente HTTP nativo, validación de configuración y respuesta JSON con resultado o error normalizado. |
| `iota-immutable-publisher` | Definir un adaptador nativo de publicación y un contrato de resultado que incluya `transaction_digest` solo tras éxito real o simulado autorizado. |
| Resolución de cápsulas | Probar que la política `prefer_wasm: false` mantiene la selección nativa para estos suscriptores. |
| Pruebas EDA | Cubrir entrega correcta, configuración ausente, indisponibilidad remota y preservación de testigos dead-letter. |

## Criterios de aceptación refinados

- Un evento de laboratorio logra respuesta de éxito mediante un doble local para Telegram e IOTA, sin llamadas externas.
- Credenciales ausentes o fallo HTTP producen un testigo dead-letter con mensaje clasificado y sin secretos.
- El router no interpreta un error de salida como éxito ni purga fuera del consenso existente.
- La ruta nativa se selecciona para ambos tools y la ruta WASI no adquiere acceso a red.
- Una reauditoría posterior no registra nuevos dead-letters por los stubs actuales de red.

## Fuera de alcance

- Purga o reescritura de dead-letters históricos.
- Emisión real por defecto contra IOTA Testnet o Telegram.
- Implementar conectividad de red en WASI.
