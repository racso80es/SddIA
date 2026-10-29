---
feature_name: kaizen-runtime-binary-parity-gate
created: "2026-10-29"
process: feature
branch_name: feat/kaizen-runtime-binary-parity-gate
persist_ref: docs/features/kaizen-runtime-binary-parity-gate
pbi_ref: MAN-25
execution_id: "b4bc0b99-532a-4faa-8e57-dcd957d39b45"
---

# Objetivos — kaizen-runtime-binary-parity-gate

## Objetivo

Que el `execute-process` y el `kalma2-bridge` que atienden trabajo correspondan al fuente del árbol actual. Un merge que toca esos órganos no deja el proceso vivo con el ELF anterior, y ni el despacho ni la portada de salud aceptan ese desfase en silencio.

## Alcance

Reciclado post-merge en `accept-pr`, gate `409 STALE_RUNTIME_BINARY` en `POST /api/execute`, fila `runtime_binary_parity` en `/api/system-health`, y smoke de validación WUI (KZ-CA5 pendiente). La ignición en `start-sddia.sh` ya sella ambos órganos; solo se exige visibilidad del fallo (KZ-CA1).

## Criterios de aceptación

KZ-CA1…KZ-CA4 implementados en rama `feat/kaizen-runtime-binary-parity-gate`. KZ-CA5 documentado como seguimiento en `pull-request-review`.

Ver descripción sellada en [MAN-25](https://linear.app/oscar-perez/issue/MAN-25) y `item.refined.md` del workspace `16199e42-7e3c-455c-bb25-80ea74a9e6a3`.
