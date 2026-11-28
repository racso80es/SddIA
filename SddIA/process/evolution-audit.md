---
uuid: 8f4b09da-e277-4fc2-9890-8a363fa8a96f
name: evolution-audit
version: 1.0.0
contract: process-contract v1.4.0
workspace_template: .SddIA/workspaces/{process_name}/{execution_id}/
context:
- quality-assurance
- filesystem-ops
hash_signature: "sha256:97520b5140bccc2de551c9dfce2144d9374a6858088d31cdf0188b7866641192"
inputs:
- audit_date: Fecha ISO de corte de la auditoría
- mode: 'Enum estricto: full | since_last'
- evolution_root_ref: Clave Cúmulo; debe ser directories.evolution
- audit_root_ref: Clave Cúmulo; debe ser paths.auditsPath
outputs:
- audit_report_path: Informe oficial versionado bajo audit_root_ref/evolution
- execution_report_path: Copia operativa del informe en workspace_path
- summary: Conteos por relevancia y veredicto
- findings: Desviaciones de gobernanza y cumplimiento
phases:
- name: Inventario normalizado
  intent: 'Resolver evolution_root_ref vía Cúmulo, separar registros oficiales de entropía y ordenar por fecha de implementación descendente; salida: universo completo o aborto documentado.'
  delegates_to:
  - agent:cumulo
  - agent:argos
  id: inventario_normalizado
- name: Clasificación de relevancia
  intent: 'Asignar R1-R5 mediante la rúbrica contractual y evidencia explícita; salida: todo registro oficial clasificado.'
  delegates_to:
  - agent:argos
  id: clasificacion_de_relevancia
- name: Validación material
  intent: 'Contrastar cada resultado esperado con código, contratos, índices y pruebas vigentes; salida: CUMPLE, CUMPLE_PARCIAL, NO_CUMPLE o NO_VERIFICABLE con evidencia por item.'
  delegates_to:
  - agent:argos
  id: validacion_material
- name: Persistencia oficial
  intent: 'Generar informe Markdown inmutable en workspace y persistir copia versionada bajo audit_root_ref/evolution; salida: ambas rutas, resumen y hallazgos.'
  requires_capability:
  - id: fs:persist
    contract: fs.persist
    version: '>=1.0.0'
  id: persistencia_oficial
- name: Protocolo de Acero
  intent: 'Revisar contradicciones, cobertura total y hallazgos críticos antes del cierre; salida: ejecución APTO o NO_APTO.'
  delegates_to:
  - agent:argos
  id: protocolo_de_acero
---

# evolution-audit

## Propósito

Auditoría periódica del registro evolution: inventario, relevancia, validación contra evidencia vigente y persistencia oficial.

## Operación

Invocación vía `./sddia-run.sh --process <name>`; fases en frontmatter YAML.

## Rúbrica R1–R5

| Nivel | Alcance |
|---|---|
| R5 | Sistémico: Constitución, seguridad, SSOT, orquestador o EDA transversal. |
| R4 | Arquitectónico: capacidad Core o contrato que afecta varias entidades. |
| R3 | Funcional: comportamiento completo acotado a un subsistema. |
| R2 | Correctivo: fix localizado o endurecimiento sin cambio transversal. |
| R1 | Documental/experimental: análisis, propuesta, borrador o cambio sin efecto operativo demostrado. |

## Reglas de evidencia

1. `date`, `created` o `fecha` determinan el orden; valor ausente o inválido produce `NO_VERIFICABLE`.
2. La narrativa del registro no prueba cumplimiento: se exige evidencia física actual.
3. Una evolución sustituida de forma trazable puede ser `CUMPLE` con estado `SUPERADO`.
4. Archivos sin UUID v4 o fuera del formato contractual se catalogan como entropía y no se silencian.
5. El informe oficial no reescribe auditorías previas; `audit_date` forma parte de su identidad.

## Ejecución

```bash
./sddia-run.sh --process evolution-audit --inputs \
  '{"audit_date":"2026-08-11","mode":"full","evolution_root_ref":"directories.evolution","audit_root_ref":"paths.auditsPath"}'
```
