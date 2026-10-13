---



feature_name: kaizen-telegram-pec-silence-value-es
created: "2026-10-13"
process: bug-fix
branch_name: fix/kaizen-telegram-pec-silence-value-es
persist_ref: docs/fixes/kaizen-telegram-pec-silence-value-es
pbi_ref: docs/todos/done/[KAIZEN] Telegram — silencio PEC accept-pr y PPR, síntesis de valor en castellano.md
document_id: PBI-KAIZEN-TELEGRAM-PEC-SILENCE-VALUE-ES
execution_id: "19a1785f-ae5a-49af-8e21-22cacbba7b34"
---
# Objetivos — kaizen-telegram-pec-silence-value-es

## Misión

Dejar de notificar el PEC `success` de `accept-pr` y el PEC `failed` de `pull-request-review`. En el aviso de PR fusionado, el apartado Síntesis de Valor va en castellano cuando hay inferencia, y declara la omisión cuando no la hay.

## Alcance

- Silencio solo por `process_name` y `status` del payload, en el humanizador.
- Prompt de síntesis en castellano. Input de idioma vehicular descartado (TG-CA7).
- Rama de trabajo `fix/kaizen-telegram-pec-silence-value-es` antes de cerrar la entrega.
