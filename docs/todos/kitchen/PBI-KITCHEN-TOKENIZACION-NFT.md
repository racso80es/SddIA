---
document_id: PBI-KITCHEN-TOKENIZACION-NFT
title: "[ARQUITECTURA] Tokenización de Cápsulas (NFTs), Smart Contracts y Lectura DLT"
format: markdown
version: "0.1.0"
created: "2026-08-24"
status: "kitchen"
priority: "incubacion"
process: "feature"
related:
  - SddIA/tools/iota-immutable-publisher.md
  - docs/features/inmunidad-caos-fase4/dlt-immunity-acta.md
---

### [ARQUITECTURA] Tokenización de Cápsulas (NFTs), Smart Contracts y Lectura DLT

#### 1. Contexto Arquitectónico (Estado de Incubación)
Este documento reside en la "Kitchen" (zona de incubación y refinamiento). Define el próximo gran salto evolutivo del Core SddIA: la transición hacia la **Librería SddIA**. 
El objetivo es transformar nuestras Entidades de Dominio (cápsulas Rust/WASI de skills, tools y agentes) en activos digitales (NFTs) regidos por Smart Contracts y verificables mediante DLT. Este documento debe ser refinado bajo el Protocolo de Acero antes de pasar a la fase de planificación táctica.

#### 2. Vectores Estructurales a Refinar

**Vector 1: Lectura DLT (El Ojo de Tormentosa)**
*   **Propósito:** Dotar al sistema de la capacidad de leer estados on-chain (IOTA Rebased) sin romper la Arquitectura Orientada a Eventos (EDA) de Estado Cero.
*   **Puntos a definir:**
    *   Forja de una nueva cápsula (ej. `iota-immutable-reader`) homóloga al publisher.
    *   ¿Qué agente asimila la lectura? (Cúmulo o Radamanto).
    *   ¿Cómo se traduce el JSON on-chain en un evento de dominio (`eda_bus`) sin generar dependencias síncronas?

**Vector 2: Forja de Smart Contracts (La Ley Inmutable)**
*   **Propósito:** Trasladar la "Física del Valor" (reglas de negocio, límites de minteo, comisiones, desbloqueo meritocrático) desde el plano teórico a código on-chain.
*   **Puntos a definir:**
    *   Elección del lenguaje y sustrato exacto (Move / EVM) compatible con nuestra red elegida.
    *   Diseño del contrato `SddIALibrary` que rija el registro y gobernanza de las cápsulas.
    *   Mecanismo de "Evolución Meritocrática": ¿Cómo actualiza el orquestador local el contrato para desbloquear límites de minteo basados en telemetría de éxito validada?

**Vector 3: Tokenización de Activos (El NFT Estructural)**
*   **Propósito:** Convertir una herramienta/skill en un NFT minteable y comercializable, garantizando al consumidor la autenticidad del código que ejecuta.
*   **Puntos a definir:**
    *   Vinculación criptográfica: El metadata del NFT debe contener obligatoriamente el Hash SHA-256 del binario `.exe` (`wasm32-wasip1`) y su documento de contrato `.md`.
    *   Proceso de Minteo: Flujo exacto desde que el "Arquitecto" finaliza la cápsula hasta que el NFT queda registrado y disponible en la Librería.

#### 3. Preguntas para el Filtro Antientrópico (Próxima Sesión)
1.  **Seguridad ZKP:** Si vamos a validar la ejecución meritocrática on-chain, ¿cómo enviamos la prueba de éxito al Smart Contract sin revelar los datos internos de la ejecución del cliente?
2.  **Inmunidad del Consumidor:** Si un consumidor "mintea" el NFT de un agente de nuestra Librería, ¿cómo verifica técnicamente su instancia de Cerbero local que el binario descargado coincide con el Hash on-chain antes de otorgarle permisos RBAC?
3.  **Despliegue del Laboratorio:** ¿Necesitamos orquestar un nodo Testnet local o validaremos directamente contra la Testnet pública de IOTA Rebased durante el desarrollo?

#### 4. Siguientes Pasos
- [ ] Aplicar el Protocolo de Acero sobre las preguntas del Filtro Antientrópico.
- [ ] Definir el stack tecnológico on-chain definitivo.
- [ ] Promover este documento de `status: "kitchen"` a `status: "pending"` en la topología de SddIA.
