---
uuid: "85750058-29d0-4217-bf23-289b4f6cf120"
name: "antigravity-cli-executor"
version: "1.0.0"
contract: "skills-contract v1.4.0"
context: "ecosystem-evolution"
capabilities:
  - "antigravity-cli-execution"
hash_signature: "sha256:0000000000000000000000000000000000000000000000000000000000000000"
inputs:
  - "request": "object; El prompt atómico inyectado de contexto y las directrices de obliteración, que será emitido al binario a través de stdin."
outputs:
  - "success": "boolean"
  - "exitCode": "integer; 0 éxito, distinto de 0 error"
  - "data": "El estado de alteración en el disco o la respuesta pura generada en el JSON de salida."
  - "error": "string de diagnóstico en caso de fallo (spawning, CLI errors o I/O)."
---

# Skill: antigravity-cli-executor

## 1. Propósito y naturaleza
Cápsula en Rust que actúa como conector puro para interactuar directamente con el binario local `agy` de Antigravity (Antigravity CLI). Está diseñada para operaciones que requieran alteración nativa del workspace o sandboxing local. Funciona con Ceguera Espacial.

## 2. Alcance y prohibidos
No toma decisiones de dominio. Requiere subprocesos por lo que se compila a binario nativo de host.
No utiliza secretos locales, lee su entorno desde `std::env::var("ANTIGRAVITY_CLI_PATH")`.

## 3. Motor de ejecución
Se compila como binario nativo en Rust bajo `SddIA/skills/antigravity-cli-executor`. Recibe peticiones vía `stdin`, las inyecta en el proceso hijo `agy`, y emite sobres JSON estandarizados en `stdout`. Previene el panicking interceptando todos los errores de Rust en sobres controlados.
