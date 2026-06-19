---
feature_name: kalma2-mayeuta-llm-router
branch: feat/kalma2-mayeuta-llm-router
global: APTO
pbi_archived: false
created: "2026-06-19"
process: feature
checks:
  O7_skill: "APTO — cargo build -p mayeuta-llm"
  O8_fallback: "APTO — sin SDDIA_LLM_CLI_COMMAND → síntesis determinista"
  O10_async: "APTO — fix prompt emite Kalma2_Process_Requested + acuse"
  O12_paridad: "APTO — telegram-fallback-responder sin cambios en handler"
  O14_eda: "APTO — suscriptor task-queue-manager + rama dispatcher + ECST"
  O13_cerbero: "PARCIAL — contexto local-subprocess declarado; gate CI no verificado"
---

# Validación — kalma2-mayeuta-llm-router

**Veredicto global: APTO** (con deudas registradas en PBI)

| ID | Criterio | Estado | Evidencia |
|----|----------|--------|-----------|
| A | Skill mayeuta-llm | ✅ | `cargo build -p mayeuta-llm` |
| B | SYNTHESIZE CLI | ✅ | tests echo mock; fallback sin env |
| C | CLASSIFY_INTENT | ✅ | heurística [FIX] + parse JSON |
| D | Handler kalma2 | ✅ | `cargo test -p execute-process kalma2` |
| E | Emisión evento | ✅ | `.events/domain/*.json` + `emitted: true` |
| E2 | Lazo EDA P1-P3 | ✅ | ECST Python + suscripción + dispatcher |
| O12 | Paridad telegram | ✅ | handler telegram-fallback intacto |

## Comandos (2026-06-19)

```bash
cd SddIA && CARGO_TARGET_DIR=target cargo build -p mayeuta-llm -p execute-process
CARGO_TARGET_DIR=target cargo test -p mayeuta-llm -p execute-process kalma2

# Fallback determinista
unset SDDIA_LLM_CLI_COMMAND
./sddia-run.sh --process kalma2-interact --inputs '{"prompt":"hola"}'

# Enrutamiento fix → evento
./sddia-run.sh --process kalma2-interact --inputs \
  '{"prompt":"inicia fix docs/todos/pending/[FIX] event-sweeper — fractura sistémica (8b1ed140e48d).md"}'
ls .events/domain/

# ECST
python3 -c "from pathlib import Path; import sys; sys.path.insert(0,'SddIA/scripts/qa'); \
from ecst_validation import load_event_class_schemas, validate_ecst_instance; \
s=load_event_class_schemas(Path('.')); \
ev={'event_id':'x','event_type':'Kalma2_Process_Requested','payload':{'process':'bug-fix','raw_text':'t'},'delivery_state':{}}; \
print(validate_ecst_instance(ev, s['Kalma2_Process_Requested']))"
```

## Deuda aceptada

Ver PBI `docs/todos/pending/[FEATURE] kalma2-mayeuta-llm-router…` sección Deudas.

## Operador post-merge

```bash
cp .dev/.env.example .dev/.env   # configurar SDDIA_LLM_CLI_COMMAND
cd SddIA && cargo build -p mayeuta-llm -p execute-process -p kalma2-bridge
./start-sddia.sh
```
