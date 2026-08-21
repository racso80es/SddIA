---
document_id: PBI-LAB-PACIENTE0-DLT-TELEMETRY-MVP
uuid: "a1b2c3d4-e5f6-4789-a012-3456789dlt1"
title: "[LABORATORIO] MVP Paciente 0: Anclaje de Telemetría DLT con Billetera Local"
format: markdown
version: "1.0.0"
status: abierto
type: laboratorio
priority: alta
process: null
assigned_to: Tekton, Tormentosa (auditoría)
created: "2026-08-21"
updated: "2026-08-21"
derived_from: PBI-LAB-PACIENTE0-SDDIA-AP
---

# Pendiente de refinamiento; especial atención a posibles alucinaciones, incoherencias o inexactitudes.

# [LABORATORIO] MVP Paciente 0: Anclaje de Telemetría DLT con Billetera Local

## 1. Propósito y Abstracción del Problema

Pausar la implementación de arquitecturas económicas (Gas Station / Transacciones Patrocinadas) para priorizar la validación física del anclaje de datos. 

Este MVP tiene un objetivo dual y atómico:
1. **Estructurar la Verdad Objetiva:** Definir exactamente qué métricas de las cápsulas y Entidades de Dominio (ED) se registrarán en la DLT para que, en un futuro, el contrato inteligente pueda extraer la "Física del Valor" (estadísticas de éxito, uso, latencia).
2. **Puente Físico Asíncrono:** Lograr que la instancia de consumo (`SddIA_AP`) firme y emita estas métricas hacia IOTA Rebased utilizando una billetera local pre-fondeada por el usuario (o el laboratorio), sin que el tiempo de ejecución afecte la experiencia del consumidor.

## 2. Objetivos de Arquitectura (S+ Grade)

### 2.1 La Estructura del Dato de Valor (Payload DLT)
El registro a inyectar en la DLT no será ruido genérico. El orquestador debe empaquetar un JSON/Move Struct estandarizado que contenga:
- `capsule_id` / `entity_uuid`: El identificador inmutable de la herramienta o skill ejecutada[cite: 35].
- `execution_id`: El rastro de la ejecución local para auditorías cruzadas.
- `thermodynamic_cost` / `duration_ms`: El tiempo de ejecución real consumido.
- `success_status`: Booleano (1/0) o `exit_code` que certifica si la cápsula cumplió su contrato.

### 2.2 Soberanía Criptográfica Mínima Viable
- Inyectar las variables `IOTA_LOCAL_PRIVATE_KEY` y `IOTA_NETWORK` en la bóveda de la instancia (`.SddIA/.dev/.env`).
- Asumimos que esta billetera cuenta con gas suficiente para el ensayo. El cliente actúa como su propio patrocinador en este MVP.

### 2.3 Asincronía Termodinámica (Tacto Inerte)
- Queda dogmáticamente prohibido que la ejecución del `email-triage-gateway` o cualquier cápsula espere la confirmación de la red DLT.
- La responsabilidad del anclaje recae exclusivamente en el flujo diferido: Emisión en la Aduana -> Captura en `.events/pending/` -> Barrido por `event-sweeper` -> Despacho a `iota-immutable-publisher`[cite: 35].

## 3. Plan de Ejecución (Línea de Montaje)

### Fase 1: Adecuación de la Aduana (CLI)
- [ ] Refactorizar `execute-process` (la Aduana Universal) para que, al concluir la ejecución de cualquier herramienta (ej. `mayeuta-llm`), genere un sobre estadístico con la estructura de valor.
- [ ] Emitir el evento de contrato ECST `Domain_Entity_Telemetry_Captured` hacia el bus fractal (`./.events/domain/`).

### Fase 2: Configuración del Cliente
- [ ] Dotar al `.SddIA/.dev/.env` del Paciente 0 de las credenciales criptográficas necesarias para interactuar con IOTA Rebased (Mainnet/Testnet).
- [ ] Modificar el archivo `event-domain-subscriptions.json` local del paciente para enrutar el evento `Domain_Entity_Telemetry_Captured` hacia la herramienta `iota-immutable-publisher`.

### Fase 3: Forja del Publicador
- [ ] Adaptar el binario en Rust de `iota-immutable-publisher` para que:
  1. Lea el evento entrante desde el `event-sweeper`[cite: 35].
  2. Firme la transacción con la clave privada de la bóveda local.
  3. Ejecute el anclaje del payload estructurado en la red MoveVM / IOTA.
  4. Devuelva `success: true` para que el sweeper mueva el evento a `processed/`[cite: 35].

## 4. Criterios de Aceptación (Protocolo de Acero)
- [ ] Al triar un correo en `SddIA_AP`, se genera un archivo de telemetría en `./.events/domain/`.
- [ ] El `event-sweeper` local procesa el evento en segundo plano[cite: 35].
- [ ] Un explorador de bloques (IOTA Explorer) confirma la transacción generada desde la billetera del cliente, mostrando el payload estructurado con el ID de la cápsula y su resultado.
- [ ] La latencia de la WUI Kalma2 o el tiempo total de procesamiento del correo no sufre incremento debido al anclaje DLT.
