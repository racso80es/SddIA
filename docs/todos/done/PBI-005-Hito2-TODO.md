---
document_id: PBI-005-HITO2-TODO
title: "[OPERATIVO] Objetivos TODO — Asalto 1: Motor de Acciones y Anatomía de Capas"
format: markdown
version: "1.1.0"
updated: "2026-05-20"
status: "completado"
pbi_ref: "PBI-005"
feature_ref: "docs/features/pbi-005-action-engine"
merge_ref: "caab46ed4fa116977813ab35150ee05ca0358ecb"
pr_url: "https://github.com/racso80es/SddIA/pull/8"
eda_event_merged: "d121213d-4950-4927-8aae-0a9b26d6e8fb"
---

# [OPERATIVO] Objetivos TODO — Asalto 1: Motor de Acciones y Anatomía de Capas (PBI-005 Hito 2)

Este documento técnico de control establece la lista de objetivos atómicos (**TODO**) para la ejecución del **Asalto 1**, correspondiente al **Hito 2 del PBI-005**. Su propósito es estructurar el trabajo del agente de desarrollo (Cursor/Tekton) bajo la estricta separación ontológica entre **Skills** (capacidades del dominio) y **Tools** (herramientas físicas mansas), evitando la saturación cognitiva del repositorio y el temido "cajón de sastre".

---

## 1. Arquitectura de Capas S+ Grade (Jerarquía del Dominio)

Para mitigar la proliferación descontrolada de micro-skills de infraestructura, la arquitectura queda blindada bajo la siguiente jerarquía determinista de ejecución:

```
[Acción de Dominio] ➔ [Agente Asignado] ➔ [Skill Agrupadora] ➔ [Tools Atómicas Físicas]
```

### 1.1 Definición de Fronteras Ontológicas
* **Skill: `skill:bus-operator`:** Capacidad cognitiva exclusiva del dominio. Concentra el conocimiento lógico para gobernar e interpretar el ciclo de vida de los eventos y las suscripciones en el bus de archivos planos. No ejecuta comandos directos de sistema; orquesta herramientas.
* **Tool: `tool:markdown-table-editor`:** Herramienta física atómica y mansa. Su único propósito en el universo es parsear, insertar, modificar o purgar registros en tablas de archivos Markdown (`index.md`) de forma matemática e idempotente.
* **Micro-Tools de Infraestructura (Ciegas):** Herramientas mecánicas aisladas que operan sobre el sistema de archivos local para la gestión transitoria del bus:
    * `tool:read-event-subscriptions`: Lectura e interpretación estricta del SSOT de suscripciones.
    * `tool:manage-event-receipt`: Mutación de sufijos atómicos (`.notificado`, `.procesado`, `.error`).
    * `tool:transit-event-payload`: Desplazamiento seguro de archivos JSON entre estados de ciclo de vida.

---

## 2. Matriz de Objetivos TODO

### ✅ FASE 1: Inicialización del Entorno Orgánico
* [x] **TODO 1.1:** Invocar la Puerta Física `execute-process.py` pasándole los parámetros para inicializar el proceso formal de `feature` bajo el identificador `pbi-005-hito2-action-engine`.
* [x] **TODO 1.2:** Verificar la creación física de la rama aislada de desarrollo `feat/pbi-005-action-engine` y situar el espacio de trabajo en ella.
* [x] **TODO 1.3:** Crear el directorio de seguimiento forense del hito en `docs/features/pbi-005-action-engine/`.

### ✅ FASE 2: Forja del Instrumental Mecánico (`tool:markdown-table-editor`)
* [x] **TODO 2.1:** Redactar el contrato legal de la herramienta en `SddIA/tools/markdown-table-editor.md`, detallando el frontmatter homologado, sus variables obligatorias de entrada (`file_path`, `operation`, `row_data`) y su envelope estandarizado de salida.
* [x] **TODO 2.2:** Desarrollar el script físico ejecutor en Python `SddIA/scripts/tools/markdown-table-editor/markdown_table_editor.py`. Debe incluir control de excepciones explícito para evitar la corrupción de tablas si hay fallos de concurrencia.
* [x] **TODO 2.3:** Realizar una prueba de humo en frío sobre la tool para asegurar que lee y modifica filas sin alterar las cabeceras ni el formato de los catálogos.

### ✅ FASE 3: Forja de la Capacidad de Negocio (`skill:bus-operator`)
* [x] **TODO 3.1:** Crear el contrato genómico de la capacidad en `SddIA/skills/bus-operator.md`. Esta skill agrupa el contexto lógico del procesamiento y enrutamiento avanzado.
* [x] **TODO 3.2:** Forjar los scripts mecánicos de apoyo para la manipulación ciega de los archivos del bus (`read-event-subscriptions`, `manage-event-receipt`, `transit-event-payload`) bajo el directorio de scripts de herramientas autorizadas, encapsulando las interacciones con el sistema operativo.

### ✅ FASE 4: Construcción del Motor Universal de Acciones
* [x] **TODO 4.1:** Desarrollar el intérprete universal de nivel Core `SddIA/scripts/qa/execute-action.py` (análogo a nuestro `execute-process.py`).
* [x] **TODO 4.2:** Programar el cargador de contratos: el script debe leer el `.md` de la acción solicitada por CLI (`--action`), validar que los `--inputs` proporcionados cumplen con el esquema de entrada del dominio y determinar qué agente y herramientas se necesitan activar.
* [x] **TODO 4.3:** Enlazar la acción `sync-entity-index` a este motor, asegurando que utiliza a `agent:cumulo` invocando a `skill:bus-operator` y `tool:markdown-table-editor` de forma transparente.

### ✅ FASE 5: Desacoplamiento de la Infraestructura y Purga Rúnica
* [x] **TODO 5.1:** Modificar el demonio detector físico `event-watcher.py` (o el orquestador síncrono del runtime actual). Remover toda importación directa del antiguo script `sync-entity-index.py`.
* [x] **TODO 5.2:** Reconfigurar el despachador físico del daemon para que ejecute una llamada limpia por línea de comandos hacia la nueva puerta oficial: `python SddIA/scripts/qa/execute-action.py --action sync-entity-index --inputs "..."`.
* [x] **TODO 5.3:** **Purga de Herejías:** Eliminar físicamente del repositorio el script rígido ad-hoc `SddIA/scripts/qa/sync-entity-index.py`. El sistema nervioso queda oficialmente libre de automatizaciones legacy inertes.

### ✅ FASE 6: Verificación en Caliente (Auditoría Argos)
* [x] **TODO 6.1:** Levantar el demonio modificado en modo depuración.
* [x] **TODO 6.2:** Forzar la emisión manual de un evento `Domain_Entity_Created` simulado en la carpeta de tránsito activa.
* [x] **TODO 6.3:** Validar que el daemon se mantiene ciego, invoca a `execute-action.py`, y que el índice se actualiza perfectamente mediante la coreografía pura de capas.
* [x] **TODO 6.4:** Redactar el informe definitivo de conformidad técnica en `docs/features/pbi-005-action-engine/validacion.md` con veredicto **APTO**.

### ✅ FASE 7: Entrega y cierre operativo *(post-Argos)*

* [x] **TODO 7.1:** Commits atómicos en `feat/pbi-005-action-engine` (`f717a5d`, `f02b795`, `89bb001`).
* [x] **TODO 7.2:** Merge squash a `main` — PR #8 → `caab46e`.
* [x] **TODO 7.3:** Emisión `PullRequest_Merged` (`d121213d-4950-4927-8aae-0a9b26d6e8fb`) y procesamiento vía `event-watcher.py --once`.
* [x] **TODO 7.4:** Actualización del manifiesto PBI padre (`docs/todos/[OPERATIVO] Planificación de Backlog_…`) v1.3.0 y `execution.md` forense.

---

## 3. Definition of Done (DoD) - Criterios de Cierre

Un ítem de esta matriz solo se considerará completado cuando cumpla con los tres pilares de acero de nuestra constitución:

| Pilar | Estado | Evidencia |
|-------|--------|-----------|
| **Ausencia de Alucinación** | ✅ | Rutas vía `cumulo.paths.json`; tools reciben `file_path` parametrizado |
| **Idempotencia Estricta** | ✅ | `delete_row` / tránsitos bus / sufijos receipt documentados como no-op si destino existe |
| **Trazabilidad Semántica** | ✅ | 3 commits feature + squash PR #8; forense en `docs/features/pbi-005-action-engine/` |

**Veredicto DoD:** **CUMPLIDO** — Asalto 1 (Hito 2) cerrado en `main`.

---

## 4. Registro de cierre (2026-05-20)

| Campo | Valor |
|-------|--------|
| **Rama entrega** | `feat/pbi-005-action-engine` (eliminada post-merge) |
| **PR** | https://github.com/racso80es/SddIA/pull/8 — **MERGED** (squash) |
| **Merge commit** | `caab46ed4fa116977813ab35150ee05ca0358ecb` |
| **Cierre documental** | `43ac435` — manifiesto PBI v1.3.0 + EDA |
| **Validación** | `docs/features/pbi-005-action-engine/validacion.md` — **APTO** |
| **Handler feature laboratorio** | `execute-process.py` fase 1 viva (deuda TODO arquitectura cerrada) |
| **Entrega previa relacionada** | PR #7 (`dbf606b`) — base motor + purga `sync-entity-index.py` |
