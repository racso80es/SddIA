---
uuid: b21e89f7-66a8-4235-950c-d9c9efbd6359
name: pull-request-audited
version: 1.0.0
contract: events-contract v1.2.0
category: domain
domain_type: PullRequest_Audited
origin_topology: core
description: Emitido cuando Argos finaliza la auditoría determinista de una Pull Request (o su pre-integración).
payload_schema:
  audit_event_reference: Must be documented as a strictly unique UUID or Canonical Hash.
  target_entity_id: Identifier of the evaluated artifact (branch, PR, or path).
  resolution: Strict dichotomous enum allowing only [PASS, REJECT, FLAG].
  violated_rules: Array of strings for traceability (if applicable).
---

# PullRequest_Audited (Domain Event)

## 1. Definición Ontológica
Este evento certifica que el Agente Argos ha finalizado su labor de escrutinio determinista sobre los artefactos bajo auditoría vinculados a un flujo de integración (PR o rama candidata).

## 2. Invariantes
- La resolución debe ser estrictamente `PASS`, `REJECT`, o `FLAG`.
- Cualquier otro estado invalida la emisión del evento.
- `audit_event_reference` debe ser un identificador único que correlacione la ejecución específica de Argos.

## 3. Condiciones de Emisión
- Argos ejecuta sus validaciones y consolida los resultados.
- Invocado internamente como consecuencia termodinámica de un ciclo de evaluación de Argos.
