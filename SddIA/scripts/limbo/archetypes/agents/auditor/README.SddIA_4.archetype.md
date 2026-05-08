# Agentes Auditor (SddIA/agents/auditor/)

**Carpeta unificada:** Los agentes de auditoría residen en `SddIA/agents/auditor/`.

| Archivo | Descripción |
|---------|-------------|
| **auditor.json** | Auditor frontend: TypeScript/Next.js (deuda técnica, imports, layer leakage), accesibilidad (WCAG), path mapping (@/ alias) y registro de interacciones entre entidades. Contexto: GesFer.Product.Front. |
| **process-interaction.json** | Auditor de procesos e interacciones: Karma2Token, Git Hooks, skills mandatorios; consumidor de interaction_audit del contrato de Token. |

Referencia en protocolo: AGENTS.md (rol [AUDITOR] y [AUDITOR-PROCESS]). Contrato de Token: SddIA/tokens/tokens-contract.json (consumers).
