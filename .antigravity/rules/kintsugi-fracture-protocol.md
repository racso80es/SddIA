---
description: Protocolo Kintsugi ante colapso de proceso (DA-Kintsugi)
alwaysApply: true
---

# Protocolo Kintsugi Ontológico (operador IA)

Ante **colapso** de un proceso oficial SddIA (`route-domain-event`, `delivery-close-cycle`, hook pre-push, cápsula indexada, etc.):

1. **Detener** la ejecución de inmediato. Prohibido continuar entrega, bypass raw (`gh`, `git`, `curl`) o recuperación manual silenciosa.
2. **Confirmar** que el runtime emitió `System_Fracture_Detected` en `eda_bus.pending` (o escalar vía proceso fallido si aplica).
3. **Delegar** al bus: Cúmulo materializa PBI (`materialize-fracture-pbi`); Mayeuta enriquece (`enrich-fracture-pbi-kaizen`).
4. **Notificar** al Vértice Biológico con el mensaje canónico: *"El proceso ha colapsado. Evento de fractura emitido. Cúmulo ha documentado la deuda. Mayeuta ha enriquecido el diagnóstico. A la espera de instrucciones."*
5. **No avanzar** hasta laudo humano o salto táctico explícito documentado en el PBI activo.

Prohibido `SDDIA_SKIP_HOOKS=1` global sin PBI activo. SSOT normativo: `SddIA/norms/obediencia-procesos.md` § Escalado ante fallo.
