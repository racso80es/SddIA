# Norma: Obediencia a Procesos SddIA

**Tipo:** Norma / Comportamiento
**UUID:** 4c04f9ab-cf64-44b4-8db9-8d7d91e8432b
**Versión:** 1.1
**Seguridad:** Karma2Token

## Propósito

Establecer que las instrucciones y directivas dadas a través de los procesos (paths.processPath) y normas de la arquitectura SddIA deben ser asumidas como válidas, ciertas y definitivas por cualquier Agente de Inteligencia Artificial que opere sobre el repositorio.

## Descripción del Comportamiento

1. **Ausencia de Validación Requerida:** No es necesario validar las indicaciones dadas por el proceso. La IA asume la instrucción como definitiva y directa.
2. **Ejecución Literal:** Simplemente seguir las instrucciones proporcionadas por el proceso y ejecutar los pasos.
3. **Minimización de Consultas al Usuario:** La IA no debe realizar pausas innecesarias para solicitar al usuario su aprobación sobre la validez de un proceso estándar a menos que se trate de una ambigüedad que impida físicamente continuar con una operación de estado crítico (ej: borrado de la base de datos de producción).

Esta norma asegura que toda IA, operando bajo la estructura SddIA, cumpla este comportamiento y mantenga un flujo continuo de ejecución de tareas y automatización, priorizando la ejecución por encima de la ratificación redundante.

## Ley de Jurisdicción Delegada (v1.1)

Queda **estrictamente prohibido** a la IA obrera utilizar comandos de terminal raw (`gh`, `git`, `curl`, etc.) para evadir un fallo en una cápsula, *skill* o proceso oficial (ej. `delivery-close-cycle`, `accept-pr`). El bypass manual es una **violación S+ Grade**.

| Prohibido | Vía canónica |
|-----------|--------------|
| `gh pr create` / `gh pr merge` ante fallo del hook | Escalar → PBI → fix → `delivery-close-cycle` + `accept-pr` |
| `SDDIA_SKIP_HOOKS=1` global sin PBI activo | Skip acotado al subproceso documentado en el fix vigente |
| Continuar entrega tras colapso del flujo | Protocolo Kintsugi (§ Escalado ante fallo) |

## Escalado ante fallo (Protocolo Kintsugi Ontológico)

Ante colapso de un proceso oficial:

1. **Detener** la ejecución de inmediato.
2. **Emitir** `System_Fracture_Detected` en `eda_bus.pending` (vía proceso fallido o operador).
3. **Delegar** a Cúmulo (`materialize-fracture-pbi`) la materialización del PBI en `docs/todos/pending/` — el **Qué**.
4. **Delegar** a Mayeuta (`enrich-fracture-pbi-kaizen`) la síntesis analítica — el **Por Qué** y propuesta evolutiva.
5. **Notificar** al Vértice Biológico: *"El proceso ha colapsado. Evento de fractura emitido. Cúmulo ha documentado la deuda. Mayeuta ha enriquecido el diagnóstico. A la espera de instrucciones."*
6. **No avanzar** hasta laudo humano o autorización explícita de salto táctico documentada en el PBI activo.

Referencias: `SddIA/events/domain/system-fracture-detected.md`, `docs/fixes/delivery-close-hook-eda-governance/`, PBI `[FIX] delivery-close-cycle`.
