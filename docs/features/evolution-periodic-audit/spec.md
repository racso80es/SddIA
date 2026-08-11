---
feature_name: evolution-periodic-audit
created: "2026-08-11"
base: main
scope: proceso Core, SSOT de auditorías y primera ejecución
---

# Especificación — evolution-audit

## Contrato del proceso

### Entradas

- `audit_date`: fecha ISO de corte; por defecto, fecha actual.
- `mode`: `full` o `since_last`; la primera ejecución usa `full`.
- `evolution_root_ref`: clave lógica `directories.evolution`.
- `audit_root_ref`: clave lógica `paths.auditsPath`.

### Salidas

- `audit_report_path`: informe oficial versionado.
- `execution_report_path`: copia operativa en el workspace.
- `summary`: conteos por relevancia y veredicto.
- `findings`: defectos de gobernanza y desviaciones detectadas.

## Rúbrica de relevancia

| Nivel | Nombre | Criterio |
|---|---|---|
| R5 | Sistémica | Constitución, SSOT, seguridad, orquestador, EDA transversal o ruptura con impacto global. |
| R4 | Arquitectónica | Nueva capacidad Core, proceso/cápsula transversal o cambio de contrato que afecta varias entidades. |
| R3 | Funcional | Feature completa o comportamiento operativo acotado a un subsistema. |
| R2 | Correctiva | Fix localizado, deuda técnica o endurecimiento sin cambio arquitectónico amplio. |
| R1 | Documental/experimental | Documentación, análisis, propuesta, borrador o cambio sin efecto operativo demostrable. |

Se asigna el nivel más alto cuyo criterio esté probado por las rutas y el objetivo declarado.

## Reglas de validación

1. Extraer identidad, fecha y objetivo esperado del registro.
2. Resolver las rutas referenciadas contra el repositorio actual.
3. Verificar existencia, contrato, versión, índice, pruebas o evidencia funcional aplicable.
4. Emitir un único veredicto:
   - `CUMPLE`: objetivo y evidencia vigentes;
   - `CUMPLE_PARCIAL`: objetivo materializado con desviaciones;
   - `NO_CUMPLE`: evidencia contradice el objetivo;
   - `NO_VERIFICABLE`: metadatos o evidencia insuficientes.
5. No confundir antigüedad o sustitución deliberada con incumplimiento: si el registro prueba una transición histórica pero el artefacto fue reemplazado de forma trazable, se marca `CUMPLE` con estado `SUPERADO`.

## Formato del informe

Frontmatter: identidad de auditoría, fecha, proceso, rama, modo, universo, conteos y hash del inventario. Cuerpo: resumen ejecutivo, hallazgos críticos, rúbrica y tabla completa ordenada por fecha descendente.
