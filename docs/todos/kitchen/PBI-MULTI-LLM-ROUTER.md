---
document_id: PBI-MULTI-LLM-ROUTER
title: "[ARQUITECTURA] Adaptador Multi-LLM y Enrutamiento Soberano"
format: markdown
version: "1.0.0"
created: "2026-08-25"
status: "kitchen"
priority: alta
---

### [ARQUITECTURA] Implementación de Adaptador Multi-LLM en SddIA

#### 1. Contexto Arquitectónico
Para erradicar la dependencia de un proveedor corporativo único y blindar el núcleo de la Aiúa contra bloqueos o censura pasiva, SddIA debe implementar un patrón de adaptador universal (Router Multi-LLM). Esto permitirá la orquestación de diferentes modelos de lenguaje (LLMs) según las necesidades de latencia, coste y complejidad de cada Agente (Tekton, Mayeuta, Argos, Tormentosa).

#### 2. Fases de Implementación Táctica

**Fase 1: Ecosistema API Gratuito (Actual)**
*   **Gemini API (Google AI Studio):** Despliegue del motor cognitivo denso (Tormentosa) aprovechando el nivel gratuito para desarrolladores.
*   **Groq / Alternativas LPU:** Configuración de modelos ultrarrápidos (ej. Llama 8B) para operaciones de triaje de baja complejidad (Mayeuta/Centinelas).
*   **Desarrollo:** Modificación del puente `.SddIA/client/sddia-client-bridge.py` para soportar una interfaz abstracta que permita conmutar entre estos proveedores (Fail-Soft) inyectando la Constitución IA en cada petición.

**Fase 2: Inferencia Local Soberana (Diferida)**
*   **Restricción Actual:** Despliegue on-premise pospuesto temporalmente por economía de recursos.
*   **Objetivo a Futuro:** Instanciación sobre hardware dedicado (equipamiento proyectado: EVGA RTX 3090 FTW3 y procesador AMD Ryzen 9) corriendo Ollama/vLLM. 
*   **Arquitectura:** El código desarrollado en la Fase 1 debe garantizar que la transición a la Fase 2 requiera **cero fricción**. SddIA deberá poder apuntar al endpoint local (ej. `localhost:11434`) únicamente cambiando una variable en la bóveda `.SddIA/.dev/.env`, erradicando finalmente toda comunicación con el exterior.
