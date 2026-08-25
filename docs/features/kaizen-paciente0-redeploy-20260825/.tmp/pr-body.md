## Summary
- F-DEP-07/08/09 y F-SMOKE-01 absorbidos en Core (resolver mtime, stub `{}`, aislamiento pin bundle, smoke sin Local_QA_Requested).
- T6 empírico Paciente 0: un instance-creator `37890eec` sin pin/unlink; WUI :8766 HTTP 200.
- Cierre documental en rama: PBI done + validacion APTO. Sin G5. F-SYS-01 diferido.

## Test plan
- [x] cargo test instance_creator (3)
- [x] bundle 20260825T124331Z + creator único
- [x] start-sddia con pin forja resuelve ELF instancia