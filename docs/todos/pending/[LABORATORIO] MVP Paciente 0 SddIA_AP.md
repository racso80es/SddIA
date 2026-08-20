---
document_id: PBI-LAB-PACIENTE0-SDDIA-AP
title: "[LABORATORIO] Ensayo Clínico: Despliegue de Paciente 0 (SddIA_AP)"
status: abierto
type: feature
priority: alta
assigned_to: Mayeuta, Tormentosa (Auditoría)
---

## 0. Pendiente de refinar. Tener en cuenta posibles incongruencias, inexatitudes y alucinaciones.

## 1. Especificación y Contexto (El Umbral)

El objetivo de este PBI es registrar la instanciación física y la auditoría del primer entorno funcional puro orientado al consumidor: **SddIA_AP (Asistente Personal)**. 

Aplicando la Ley de Jurisdicción Dividida, este ensayo clínico demostrará la capacidad de SddIA para operar como un motor inerte (Core) sobre el cual se inyecta un genoma de dominio específico (Códice). Se aplicará una poda estricta sobre cualquier entropía de ingeniería de software (Git, forja de código, Tékton) para entregar valor directo y silencioso mediante el triaje y gestión de correos electrónicos.

## 2. Objetivos de Arquitectura (S+ Grade)

* **Aislamiento Funcional (Filtro C):** Garantizar que el Paciente 0 opere sin dependencias del `codex-software-engineering`. Ningún hook de repositorio o proceso de despliegue debe contaminar su ciclo.
* **Soberanía Cognitiva (Mayeuta):** Validar la capacidad de la entidad Mayeuta para asumir la responsabilidad exclusiva del triaje semántico de la bandeja de entrada, guiado únicamente por la Constitución local del consumidor.
* **Trazabilidad Sensorial:** Confirmar que los receptores periféricos (`email-watcher`, `telegram-watcher`) y el Sistema Nervioso (EDA) laten sin colisionar con los daemons de otros laboratorios o del propio Core.

## 3. Plan de Ejecución (Las Fases de la Forja)

### Fase 1: El Trasplante Físico (Instanciación)
- [ ] Desplegar la topología base utilizando el `starter-kit` en un entorno/directorio completamente aislado para SddIA_AP.
- [ ] Configurar el `local.paths.json` para enrutar el bus de eventos y la memoria local sin fricción.
- [ ] Redactar la `CONSTITUTION.md` local, estableciendo las reglas de negocio exclusivas del consumidor (ej. directrices de triaje, priorización de remitentes, purga de ruido promocional).

### Fase 2: Inyección del Genoma (Jurisdicción Dividida)
- [ ] **Despliegue del Motor (Core):** Instanciar el CLI inerte, el bus de eventos (`.SddIA/events/`), el daemon `event-sweeper`, y la infraestructura base de `Cerbero` (RBAC) y `Cúmulo` (logs locales).
- [ ] **Inyección del Códice (Customer):** Activar el `codex-kalma2-assistant` (o equivalente) y habilitar a **Mayeuta** como la única entidad cognitiva en el entorno.
- [ ] **Poda Estricta:** Validar la ausencia total de hooks de Git, procesos de desarrollo (`feature`, `bug-fix`, `pull-request-review`), y asegurar que **Tékton** permanece desconectado de este paciente.

### Fase 3: Ignición de Receptores Sensoriales
- [ ] Configurar y arrancar el daemon `email-watcher` con las credenciales IMAP/SMTP del consumidor.
- [ ] Configurar y arrancar el daemon `telegram-watcher` para establecer el puente asíncrono de notificación y respuesta humana.
- [ ] Verificar la ausencia de colisiones en los procesos en segundo plano.

### Fase 4: Prueba de Fuego Causal (First Blood)
- [ ] Provocar un evento físico inyectando un correo real en la bandeja monitorizada.
- [ ] Auditar la trazabilidad del Sistema Nervioso: Detección (`email-watcher`) -> Emisión de evento (`email-received`) -> Evaluación Semántica (Mayeuta) -> Ejecución de acción resolutiva o Notificación (Telegram).

## 4. Validación y Cierre Documental
- [ ] SddIA_AP es capaz de recibir, triar y gestionar un correo de extremo a extremo en su propio entorno local.
- [ ] La telemetría de la Aduana (CLI/Argos) demuestra un éxito operativo (`success: true`) sin intentar invocar procesos de ingeniería.
- [ ] Las fricciones descubiertas durante la instanciación han sido destiladas y el conocimiento se ha inyectado como Sabiduría Estratégica en la documentación del motor central de SddIA.
