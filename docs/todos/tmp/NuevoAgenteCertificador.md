---
status: consolidado
superseded_by: docs/todos/pending/[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade.md
document_id_unified: PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO
unified_phase: "Fase 4 (Radamanto — bucle Self-Healing)"
archived_from: docs/todos/pending/NuevoAgenteCertificador.md
---

> **Documento archivado — unificado**
> Este PBI ha sido consolidado en [`[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade`](../pending/[ARQUITECTURA] Telemetría Reactiva — Unificación EDA S+ Grade.md) (`document_id: PBI-TELEMETRIA-REACTIVA-EDA-UNIFICADO`). Corresponde a la **Fase 4** del documento unificado. No ejecutar como ítem independiente.

# PBI: Forja del Agente Certificador (Radamanto) y Bucle de Telemetría S+ Grade

Naturaleza: Arquitectura de Datos / Evolución Ontológica  
Entorno: SddIA Core / Librería SddIA  
Estatus: Pendiente de Ejecución (Bloqueado por Dependencia)  
Prioridad: Alta (Crítico para la "Física del Valor")

---

## 1. Síntesis y Propósito
El objetivo de este PBI es implementar a Radamanto (Agente Certificador/Actuario), la entidad responsable de gestionar la "Física del Valor" del ecosistema SddIA. Su propósito es procesar las estadísticas de rendimiento de herramientas y *skills*, aplicar umbrales deterministas y registrar inmutablemente los cambios de estatus en la DLT (IOTA Rebased). 

Para preservar la Ceguera Espacial absoluta y el Principio de Interceptación Central, este agente carece de capacidades de medición directa; operará exclusivamente como consumidor de los eventos inyectados en el bus estático por la Aduana Universal (CLI).

## 2. Dependencias Críticas (Hard Blocks)
* [BLOQUEO ACTIVO] PBI de Telemetría Base: La ejecución de este documento queda paralizada hasta que se materialice la lógica de la "Aduana Universal" en el CLI de SddIA. El Orquestador inerte (CLI) debe ser capaz de medir el éxito, tiempo y eficiencia de una orden, y emitir el evento en el bus .SddIA/events/ antes de que Radamanto pueda existir para consumirlo.

## 3. Arquitectura del Agente: Radamanto (Certificador)
* Genoma Determinista: Radamanto no evaluará código ni interpretará intenciones. Su red neuronal se restringe a evaluar el acumulado estadístico (batching) entregado por los eventos de telemetría (ej. porcentaje de éxito, latencia promedio).
* Jurisdicción Criptográfica: Radamanto ostentará la exclusividad absoluta sobre la ejecución de la cápsula iota-immutable-publisher.
* Acumulación Termodinámica (Batch Processing): No reaccionará ni anclará transacciones por cada evento individual. Procesará las métricas por lotes (ej. cada 10 o 50 ejecuciones) o ante caídas abruptas de umbral, para minimizar el consumo de gas en la red DLT.

## 4. El Bucle de Inmunidad Autónoma (Self-Healing)
El sistema debe ser capaz de autogestionar el ciclo de vida de un activo sin la intervención del Vértice Biológico. Se deben configurar las siguientes interacciones reactivas (EDA):

1. Cuarentena: Si Radamanto detecta que una entidad cae por debajo del umbral de rendimiento S+ Grade, emite el evento Tool_Degraded y sella la rebaja de estatus en IOTA.
2. Bloqueo Táctico: Cerbero intercepta Tool_Degraded y actualiza inmediatamente las normas locales (RBAC), revocando los permisos de uso de dicha herramienta para evitar su ejecución en producción.
3. Instanciación de Reparación: El evento Tool_Degraded funciona como "Chispa de Infraestructura" para invocar a Dédalo (Arquitecto) y Tekton (Obrero). Estos agentes inician un proceso de refactorización en un entorno aislado (sandbox).
4. Redención Automática: Una vez Argos valida el nuevo artefacto y el CLI emite telemetría exitosa tras un testeo ciego, Radamanto consolida la recuperación, emite Status_Restored, y Cerbero rehabilita el acceso.

## 5. Criterios de Aceptación (Definition of Done)
* AC1: El contrato del agente (radamanto.json o radamanto.md) está creado y estipula su exclusividad sobre las operaciones de anclaje DLT.
* AC2: El diseño prohíbe explícitamente a Radamanto invocar comandos de sistema o medir procesos por sí mismo, dependiendo íntegramente de la telemetría del CLI.
* AC3: Existen reglas documentadas de umbrales deterministas (ej. < 85% de éxito = Tool_Degraded).
* AC4: El enrutador de eventos (EDA) tiene definidas las suscripciones para que Cerbero y los procesos de refactorización reaccionen automáticamente a los eventos de estado emitidos por Radamanto.

# Ajustes al PBI de Radamanto (Actualización S+ Grade)
Se han inyectado las correcciones del Yunque Rúnico directamente en las secciones vulnerables del documento original para neutralizar el riesgo de colapso termodinámico.

[ Modificación en Sección 4: El Bucle de Inmunidad Autónoma (Self-Healing) ]

Se añade el mecanismo de "Límite de Redención" para evitar bucles infinitos:

Límite de Redención y Muerte Definitiva: Radamanto mantendrá un contador inmutable de "Intentos de Reparación" por entidad. Si una herramienta entra en estado Tool_Degraded y fracasa en su recuperación superando el límite máximo establecido (ej. 3 intentos), Radamanto emitirá el evento final Tool_Deprecated (o Tool_Burned).

Purga del Sistema: Ante el evento Tool_Deprecated, Cerbero bloquea permanentemente el acceso, y el activo/NFT asociado se marca como obsoleto o se quema en la red DLT, purgando la entropía del ecosistema sin requerir intervención del Vértice Biológico.

[ Modificación en Sección 5: Criterios de Aceptación (Definition of Done) ]

Se añade el blindaje del entorno de refactorización:

AC5 (Blindaje del Sandbox): La arquitectura de eventos define y aplica a nivel de sistema operativo un "Sandbox Estricto" para la fase de reparación. Tekton y Dédalo tendrán revocado explícitamente el permiso de escritura sobre la ruta de producción (SddIA/tools/, SddIA/skills/, etc.) y solo podrán operar en un entorno temporal aislado hasta que Argos emita el certificado de éxito.

AC6 (Límite de Intentos): Radamanto cuenta con una variable configurable de max_recovery_attempts y lógica para detonar la "Muerte Definitiva" (Tool_Deprecated).
