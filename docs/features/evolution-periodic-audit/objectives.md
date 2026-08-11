---
feature_name: evolution-periodic-audit
created: "2026-08-11"
process: feature
branch: feat/evolution-periodic-audit
base_branch: main
---

# Objetivos — auditoría periódica de evolution

## Objetivo

Crear y ejecutar un proceso oficial, repetible y trazable que audite los registros resueltos por `directories.evolution`, ordene las evoluciones por fecha de implementación descendente, las clasifique en cinco niveles de relevancia y contraste cada resultado esperado con el estado actual del repositorio.

## Criterios de aceptación

1. Existe un proceso atómico `evolution-audit` creado mediante `entity-manager`, registrado en el índice Core y sellado por EDA.
2. El proceso define una rúbrica determinista de relevancia de cinco niveles.
3. Cada registro evolution queda inventariado y recibe un veredicto `CUMPLE`, `CUMPLE_PARCIAL`, `NO_CUMPLE` o `NO_VERIFICABLE`, con evidencia trazable.
4. El informe oficial se persiste bajo una ruta de auditorías resuelta por Cúmulo y queda ordenado por fecha descendente.
5. La primera ejecución del proceso genera el informe base y un resumen cuantitativo.
6. La ejecución puede repetirse periódicamente sin depender de rutas absolutas ni alterar registros históricos.

## Restricciones

- No mezclar el trabajo con la rama activa de reparación de Centinelas.
- No tratar borradores o archivos sin identidad contractual como evoluciones oficiales; deben señalarse como entropía.
- No declarar cumplimiento sin evidencia física vigente.
- La mutación del proceso transita por `entity-manager`; cualquier remediación manual posterior a la forja queda limitada a carencias demostrables del creator y se resella por la misma vía.
