---
uuid: "b548b894-4d83-4a1e-84e1-eb10b0a8837e"
name: "antigravity-http-connector"
version: "1.0.0"
contract: "skills-contract v1.4.0"
context: "ecosystem-evolution"
capabilities:
  - "antigravity-http-inference"
hash_signature: "sha256:0000000000000000000000000000000000000000000000000000000000000000"
inputs:
  - "request": "object; El prompt atómico inyectado de contexto y las directrices de obliteración, que se enviará en el cuerpo de la petición POST HTTP a la API de Antigravity."
outputs:
  - "success": "boolean"
  - "exitCode": "integer; 0 éxito, distinto de 0 error"
  - "data": "La respuesta pura del LLM."
  - "error": "string de diagnóstico en caso de fallo (HTTP o parsing)."
---

# Skill: antigravity-http-connector

## 1. Propósito y naturaleza
Cápsula en Rust que actúa como conector puro para interactuar con la plataforma agentic Google Antigravity a través de su API HTTP (REST/gRPC). Diseñada con Ceguera Espacial absoluta.

## 2. Alcance y prohibidos
Solo interactúa con los endpoints definidos en el entorno. No toma decisiones de dominio.
No utiliza secretos hardcodeados, extrae las configuraciones de `std::env::var("ANTIGRAVITY_API_KEY")` y `std::env::var("ANTIGRAVITY_API_ENDPOINT")`.

## 3. Motor de ejecución
Se compila como binario nativo en Rust bajo `SddIA/skills/antigravity-http-connector`. Recibe peticiones vía `stdin` y emite sobres JSON estandarizados en `stdout`. Captura todo tipo de error para prevenir `panic!`.
