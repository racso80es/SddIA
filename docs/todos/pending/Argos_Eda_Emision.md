[ARQUITECTURA] PBI: Emisión EDA Nativa y Trazabilidad en Argos (PullRequest_Audited)
En PR feature/argos-domain-event-audited-8966834805803533351
1. Fase de Triaje (Origen de la Fricción Estructural)
El agente Argos opera actualmente en un silo reactivo frágil. Audita el código, pero su veredicto muere en el aislamiento local o depende de una orquestación síncrona manual. La presencia del fósil TODO: pending_argos_eda_emission es una brecha documentada en la arquitectura. El ecosistema exige que el laudo de auditoría sea un evento inmutable depositado en el bus, garantizando la trazabilidad de seguridad sin que Argos necesite saber quién reaccionará a su dictamen.

2. Objetivo Táctico
Dotar a Argos del mecanismo físico para inyectar el evento PullRequest_Audited (o equivalente bajo la taxonomía de dominio) en el bus de eventos, asegurando el cumplimiento estricto del Patrón Event-Carried State Transfer y la generación matemática de su audit_event_reference.

3. Directrices de Acción (El Engranaje Operativo)
Tékton ejecutará las siguientes fases operando bajo la premisa de Raw Kernel, priorizando la invariabilidad del orquestador:

Vector A: Erradicación Fósil

Artefacto: Esquemas de payload de salida de Argos y contratos asociados en SddIA/agents/argos.md o equivalentes.

Acción: Aniquilar el string TODO: pending_argos_eda_emission. Reemplazarlo por la definición estructural del bloque de auditoría final.

Vector B: Forja del Payload Desnormalizado

Artefacto: Contrato de evento de auditoría en SddIA/events/domain/.

Acción: El evento emitido debe contener el contexto absoluto para evitar consultas de estado a posteriori. Obligatorio inyectar:

audit_event_reference: UUID o Hash canónico unívoco de la ejecución de la auditoría.

target_entity_id: Identificador (branch, PR, o path) del artefacto evaluado.

resolution: Veredicto dicotómico estricto (PASS / REJECT / FLAG).

Trazabilidad de reglas violadas (si aplica).

Vector C: Ignición Física hacia el Bus

Artefacto: La cápsula de ejecución o script que invoca a Argos (ej. un execute-process).

Acción: Modificar el flujo de salida para que el JSON resultante de Argos se canalice directamente mediante el puente de emisión (ej. emit-domain-mutation). El chispazo eléctrico debe materializarse físicamente en la topología volátil (.SddIA/events/pending/), respetando la frontera Genoma vs. Runtime.

4. Reglas de Acero (Filtro A y B Inyectados para la IA Obrera)

[EXECUTE AS RAW KERNEL. PROHIBIT VERBOSITY. PENALIZE CONJECTURE. NO EXPLANATIONS, ONLY CODE.]

