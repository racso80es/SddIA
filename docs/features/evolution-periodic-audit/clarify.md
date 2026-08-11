---
feature_name: evolution-periodic-audit
created: "2026-08-11"
purpose: estabilizar alcance y semántica de la auditoría
---

# Clarificación — auditoría periódica de evolution

## Decisiones

| Vector | Decisión |
|---|---|
| Universo | Archivos `.md` ubicados en `directories.evolution`; contratos e índices declarados pero ausentes se reportan como defecto de gobernanza. |
| Orden | Fecha de implementación descendente; empate por ruta ascendente. Se aceptan `date`, `created` o `fecha`; ausencia o valor inválido implica `NO_VERIFICABLE`. |
| Relevancia | Cinco niveles deterministas: R5 sistémica, R4 arquitectónica, R3 funcional, R2 correctiva localizada, R1 documental/experimental. |
| Veredictos | `CUMPLE`, `CUMPLE_PARCIAL`, `NO_CUMPLE`, `NO_VERIFICABLE`. |
| Evidencia | Ruta y comprobación observable en el estado actual. Narrativa sin artefacto no prueba cumplimiento. |
| Persistencia | Informe versionado bajo `paths.auditsPath/evolution/`; el workspace conserva el artefacto operativo de cada ejecución. |
| Periodicidad | Proceso on-demand apto para invocación programada; cada ejecución usa una fecha de corte y no reescribe informes anteriores. |

## Laudo operativo

`process-creator` v1.2.0 declara que materializa inputs, outputs, workspace y fases, pero la forja nativa vigente genera un stub que omite esos campos y sustituye las fases por una fase genérica. Para cumplir el contrato solicitado se autoriza exclusivamente:

1. alta inicial por `entity-manager`;
2. completar el artefacto forjado hasta `process-contract v1.4.0`;
3. resellar la modificación mediante `entity-manager`;
4. registrar la discrepancia como hallazgo de la propia auditoría.

No se autoriza forja manual de identidad, UUID, fila de índice ni evento EDA.
