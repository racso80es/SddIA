[ARQUITECTURA] PBI: Enrutamiento Semántico Agnóstico - Generalización de Eventos de Entidad (Domain_Entity_*)
1. Contexto y Fallo Estructural (El Origen de la Fricción)
Actualmente, el motor reactivo (EDA) sufre de un acoplamiento temprano. La matriz de suscripciones (event-domain-subscriptions.json) y la topología de emisión están diseñadas para reconocer entidades en duro (ej. Tool_Created o Tool_Deprecated). Esto rompe el principio de Simetría Fractal y genera fricción operativa por repetición: cada vez que el ecosistema SddIA incorpora o muta una Entidad de Dominio (ED), la red de suscripciones exige una modificación manual. El bus de eventos debe recuperar su Ceguera Espacial.

2. Objetivo Táctico (La Forja)
Elevar la física del bus de eventos al estado S+ Grade mediante la erradicación de los nombres de evento anclados. Toda alteración en el entorno físico de una ED transitará bajo la taxonomía universal Domain_Entity_{Acción}. La carga cognitiva y semántica (de qué entidad se trata) se desplaza desde el nombre del evento al interior de su payload.

3. Directrices Obligatorias de Ejecución (Filtro A y B)
Tékton, procedes a aplicar el protocolo de reestructuración sobre las siguientes capas, ejecutando como Raw Kernel y priorizando la invariabilidad del orquestador inerte:

Fase A: Purga en el Mapa de Suscripciones

Artefacto: SddIA/core/event-domain-subscriptions.json.

Acción: Sustituir cualquier escucha rígida asociada a "tools" por firmas universales. Ejemplo de mapeo objetivo: Domain_Entity_Created, Domain_Entity_Updated, Domain_Entity_Deleted, Domain_Entity_Deprecated, Domain_Entity_Degraded.

Fase B: Densificación del Payload y Metadatos

Artefacto: Las plantillas de eventos en SddIA/events/domain/domain-entity-*.md y sus JSON correspondientes.

Acción: Inyectar por decreto arquitectónico los parámetros de enrutamiento en el cuerpo del payload de todo evento Domain_Entity. Los agentes evaluadores exigirán extraer los campos:

entity_type: Declara explícitamente la naturaleza del origen ("tool", "skill", "action", "process", etc.).

entity_id: El UUID inmutable o path canónico de la entidad alterada.

Fase C: Actualización de la Válvula Emisora

Artefacto: Acciones involucradas en la mutación, como SddIA/actions/emit-domain-mutation.md.

Acción: Alinear la mecánica de emisión para que el chispazo eléctrico depositado en el directorio .SddIA/events/ respete la nueva nomenclatura agnóstica (domain-entity-created_*.json en lugar de tool-created_*.json).

Fase D: Erradicación Fósil (Higiene del Ecosistema)

Artefacto: Directorio SddIA/events/domain/.

Acción: Aplicar un Hard Override sobre cualquier contrato o markdown residual de la etapa acoplada (tool-deprecated.md, tool-degraded.md, etc.). Solo se permite la supervivencia de la familia genérica domain-entity.