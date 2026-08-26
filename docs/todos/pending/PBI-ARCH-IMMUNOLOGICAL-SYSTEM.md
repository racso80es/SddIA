[ARQUITECTURA] Sistema Inmunológico Autónomo: Triaje y Auto-Poda de Ruido Sistémico
YAML
document_id: PBI-ARCH-IMMUNOLOGICAL-SYSTEM
uuid: "GENERATED-UPON-MATERIALIZATION"
title: "[ARQUITECTURA] Sistema Inmunológico Autónomo: Triaje y Auto-Poda de Ruido Sistémico"
format: markdown
version: "1.0.0"
created: "2026-08-26"
status: "pending-refinement"
priority: "alta"
process: feature
related:
  - docs/audits/AUDIT-CENTINELAS-FRACTURAS-EVENTOS-PENDING-20260826.md
  - SddIA/norms/events-contract.md
  - SddIA/agents/radamanto.md
  - SddIA/agents/argos.md

0. Pendiente de refinar, teniendo especial atención en posibles alucinaciones, incongruencias o inexactitudes.
1. Contexto y Fricción Evolutiva
Actualmente, el sistema EDA de SddIA opera con una reactividad hiper-sensible sin capa de triaje. La caída temporal de un centinela (por ejemplo, debido a la suspensión del host o interrupciones de red) dispara mecánicamente un evento de System_Fracture_Detected. Esto provoca que Cúmulo materialice de inmediato un PBI en la carpeta pending/. Esta inercia viola el Filtro C (Eficiencia), delegando la carga de auditar falsos positivos transitorios (ruido) directamente sobre el Vértice Biológico.

Este PBI define la arquitectura para dotar a SddIA de un Sistema Inmunológico Autónomo capaz de discernir entre la muerte térmica (fractura real) y el letargo físico (falso positivo), fagocitando su propio ruido documental.

2. Objetivos Estratégicos (S+ Grade)
Abolición de la Reactividad Ciega: Desvincular la detección del síntoma físico de la materialización documental del error.

Triaje Autónomo: Implementar una "Cuarentena de Eventos" donde el sistema se auto-audita empíricamente antes de escalar.

Poda Ontológica Dinámica: Si el sistema confirma la auto-recuperación térmica de un nodo, el síntoma se registra en la bitácora evolutiva y se descarta su elevación a PBI sin requerir intervención humana.

3. Especificación del Flujo de Defensa (Línea de Montaje)
El nuevo metabolismo del error se dividirá en las siguientes fases atómicas:

Fase 1: Interceptación del Síntoma (Ruido de Nivel 1)

La falta de latido (o cualquier timeout de tool) ya no genera un System_Fracture_Detected directo. Genera un evento Anomaly_Detected (Telemetría).

Fase 2: Cuarentena y Búfer Térmico

La anomalía entra en un estado de retención transitoria. Se define un margen de cooldown (ej. 2 ciclos de evaluación) para permitir la recuperación natural de la inercia del sistema operativo.

Fase 3: Auditoría Macrófaga (Verificación Ciega)

Transcurrida la cuarentena, se invoca a un agente verificador (Argos/Radamanto) para testear el estado físico actual del proceso o servicio reportado.

Fase 4: Resolución y Veredicto

Falla Controlada (Laudo B Automático): Si el agente empírico confirma que el PID está vivo y el latido se ha restablecido, el evento se clasifica como "latencia del host". Se purga de la cola de alertas y se documenta en el log histórico como ruido fagocitado.

Fractura Confirmada (Laudo A): Solo si el sistema sigue sin latido o responde con error estructural tras la cuarentena, se emite el System_Fracture_Detected al bus de dominio para que Cúmulo materialice el PBI destinado al Vértice Biológico.

4. Criterios de Aceptación (Protocolo de Acero)
[ ] Definición de responsabilidades exactas: Especificar qué agente asume el rol del "Macrófago" (¿Radamanto en su barrido de telemetría o Argos mediante ejecución dirigida?).

[ ] Creación/modificación de los contratos de eventos pertinentes en SddIA/events/.

[ ] Integración limpia con el bus EDA, garantizando que los eventos en cuarentena no bloqueen el procesamiento de orquestación principal (inmunidad anti-bloqueos).

[ ] Ningún apagón de host inferior a N horas debe resultar en un PBI abierto pendiente de revisión humana.
