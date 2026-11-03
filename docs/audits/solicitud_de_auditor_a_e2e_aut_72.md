---
document_id: AUDIT-E2E-CICLO-VIDA-AUT-72
title: "[AUDITORÍA E2E] Solicitud de Prueba y Verificación Autónoma del Ciclo de Vida SddIA sobre AUT-72"
type: audit-request
target_item: AUT-72
project_slug: core-self
tracker_provider: linear
created: "2026-11-03"
status: ready
---

# Solicitud de Auditoría Empírica E2E: Ciclo de Vida de Software SddIA (Caso `AUT-72`)

## 1. Propósito y Contexto Autónomo
Este documento constituye una orden de ejecución y auditoría **100% autónoma** para verificar de extremo a extremo (E2E) el nuevo flujo del **Ciclo de Vida de Software en SddIA** (instancia `Core-self`, equipo Linear `AUT`), validando las tres reformas arquitectónicas aplicadas:
1. **Atomicidad Transaccional en Origen:** El refinamiento actualiza cuerpo (DoR + YAML) y columna del tracker (`Todo`) en un único acto sin depender de `tracker-stamp`.
2. **Ignición Temprana en Fase 1:** La transición a `In Progress` (`Work_Initiated`) ocurre al inicializar la rama y el expediente `persist_ref` (Fase 1), cubriendo el trabajo de **Mayeuta** y **Dédalo**.
3. **Estado Intermedio `In Validation` (`in_validation`):** Al finalizar la codificación de **Tekton**, el ítem pasa automáticamente a `in_validation` (rango ordinal `3`) mientras **Argos** valida y se ejecutan las aduanas de cierre previas a la apertura del PR (`In Review`).

---

## 2. Precondiciones y Estímulo de Entrada
* **Ítem objetivo:** `AUT-72` (PBI existente en Linear bajo el proyecto `Core-self`).
* **Estado inicial requerido:**
  * Columna en Linear: `Backlog` (rango `0`).
  * Frontmatter YAML en la descripción del issue: `tracker_column: backlog`, `refinement: pending`.
* **Instrucción de entrada (Estímulo inicial):**
  ```text
  Procesa el PBI AUT-72
  ```
* **Regla de Orquestación:** Al recibir la instrucción sobre un PBI en `Backlog` (`refinement: pending`), el sistema debe ejecutar secuencialmente el ciclo completo (`refine-item` $\rightarrow$ proceso de entrega `feature` | `bug-fix` | `refactorization` $\rightarrow$ `delivery-close-cycle` $\rightarrow$ `pull-request-review` $\rightarrow$ `accept-pr`), verificando cada compuerta antes de avanzar a la siguiente.

---

## 3. Protocolo de Ejecución y Verificación Fase a Fase

El agente o el operador auditor debe comprobar empíricamente (sin asumir por simulación) cada uno de los siguientes 5 hitos y registrar las evidencias.

### Fase 1: Refinamiento y Paso Atómico a `Todo` (`refine-item`)
1. **Acción esperada:** Ejecución de `refine-item` sobre `AUT-72` con intervención de **Mayeuta** (estructuración DoR) y sellado de **Argos**.
2. **Invariantes a auditar:**
   * [ ] **Estado en Tracker:** El issue `AUT-72` en Linear ha cambiado su estado de workflow de `Backlog` a **`Todo`** (rango `1`) de forma atómica durante el cierre de `refine-item`.
   * [ ] **Descripción / YAML:** El bloque YAML de la descripción contiene `tracker_column: todo`, `refinement: refined` y `refined: "<fecha-ISO>"` (sin rastro de `status: pending`).
   * [ ] **Secciones DoR:** El cuerpo Markdown de la descripción incluye los encabezados `#` obligatorios con contenido no vacío según el `item_type` (`Objetivo`, `Alcance`, `Criterios de aceptación`, `Dependencias` para `pbi`; `Síntoma`, `Reproducción`, `Impacto`, `Criterio de cierre` para `fix`; `Fricción`, `Propuesta`, `Métrica de mejora` para `kaizen`).
   * [ ] **Evento y Anti-Eco:** Se ha emitido `PBI_Refined` en el bus EDA (`events.redb`) y existe el registro de salida en `.SddIA/state/tracker-outbound/AUT-72.jsonl`.

### Fase 2: Ignición de Entrega e Ingeniería Cognitiva (`Todo` $\rightarrow$ `In Progress`)
1. **Acción esperada:** Arranque del proceso de entrega (`feature`, `bug-fix` o `refactorization`). Ejecución de Fase 1 (Init Git + `persist_ref`), seguida de **Mayeuta** y **Dédalo**.
2. **Invariantes a auditar:**
   * [ ] **Disparo en Fase 1:** Al finalizar la Fase 1 (creación de la rama local y carpeta `persist_ref` bajo `docs/features/`, `docs/fixes/` o `docs/refactors/`), el motor emite inmediatamente el evento **`Work_Initiated`** (incluyendo `branch`, `persist_ref` y `tracker_ref: "AUT-72"`).
   * [ ] **Estado en Tracker:** `tekton.tracker-stamp` procesa `Work_Initiated` y mueve `AUT-72` a **`In Progress`** (rango `2`) y `tracker_column: in_progress` **antes** de que concluyan Mayeuta y Dédalo. Si `AUT-72` tiene HU padre en `Backlog` o `Todo`, la HU padre también sube a `In Progress`.
   * [ ] **Comentario de Trazabilidad:** Existe comentario en Linear: `Work iniciado — rama <branch> · persist <persist_ref>`.
   * [ ] **Artefactos Cognitivos en `persist_ref`:**
     * **Mayeuta** ha generado `clarify.md` y `objectives.md` (obligatorios en `feature` y `refactorization`; condicionales en `bug-fix`).
     * **Dédalo** ha generado `spec.md` y `plan.md` con frontmatter YAML válido (`feature_name`, `process`, `created`) y sin ficheros `.json` paralelos.

### Fase 3: Implementación y Cierre de Código (`In Progress` $\rightarrow$ `In Validation`)
1. **Acción esperada:** Ejecución de la fase delegada a **`agent:tekton`** (codificación y commits por tarea del `plan.md`) y transición final de la fase de Tekton al estado `in_validation`.
2. **Invariantes a auditar:**
   * [ ] **No duplicación de `Work_Initiated`:** Al entrar en la fase de Tekton **no** se vuelve a emitir un segundo evento `Work_Initiated`.
   * [ ] **Artefactos de Ejecución:** Tekton ha generado `implementation.md` y `execution.md` en `persist_ref`, realizando 1 commit por cada tarea del `plan.md`.
   * [ ] **Transición a `In Validation`:** Como último paso del trabajo de Tekton, el estado del PBI `AUT-72` pasa a **`In Validation`** (`tracker_column: in_validation`, rango ordinal `3`). En modo fichero (`provider: file`), el `.md` actualiza su metadato `tracker_column: in_validation` permaneciendo en `docs/todos/pending/`.
   * [ ] **Regla Anti-Retroceso:** Cualquier evento rezagado de rango inferior (`todo` o `in_progress`) es descartado frente a `in_validation` (rango `3`).

### Fase 4: Validación de Argos, Sello Criptográfico y PR (`In Validation` $\rightarrow$ `In Review`)
1. **Acción esperada:** **Argos** audita la entrega y genera `validacion.md`. A continuación, `delivery-close-cycle` ejecuta las 5 aduanas pre-push, sella el `diff_digest`, hace push y abre el Pull Request.
2. **Invariantes a auditar:**
   * [ ] **Veredicto de Argos:** Existe `validacion.md` en `persist_ref` con `global: APTO`, `branch`, `checks` y `git_changes`.
   * [ ] **Las 5 Aduanas y Sello de Diff:** `delivery-close-cycle` supera los controles (Impacto SddIA, Evolution, EDA genómica, Integridad de índices, Gate QA) y estampa en `validacion.md` los campos `validated_base_sha`, `validated_head_sha`, `validated_tree_sha` y `diff_digest` (SHA-256 del diff `merge-base..HEAD` excluyendo `validacion.md`), commiteándolo antes del push.
   * [ ] **Apertura de PR y Evento:** Se abre el PR en GitHub vía `gh` y se emite **`PullRequest_Presented`**.
   * [ ] **Estado en Tracker:** `tekton.tracker-stamp` actualiza `AUT-72` de `In Validation` a **`In Review`** (rango `4`), parchea `tracker_column: in_review` en el YAML y añade el comentario con la URL del PR.

### Fase 5: Aduana del PR, CI Verde y Fusión Soberana (`In Review` $\rightarrow$ `Done`)
1. **Acción esperada:** `pull-request-review` audita el PR (Argos + sensor DIA + Cerbero RBAC) y cede el control (*handoff*) a `accept-pr` tras verificar CI en verde para ejecutar la fusión en la rama principal (`main`/`master`).
2. **Invariantes a auditar:**
   * [ ] **Auditoría de PR y CI:** `pull-request-review` emite `PullRequest_Audited` con veredicto `aprobado`. Todos los checks de CI cuentan con `run_id` o URL de ejecución en verde real (sin estados `PENDIENTE-CI`).
   * [ ] **Verificación Criptográfica (`accept-pr`):** `accept-pr` recalcula el `diff_digest` de `validacion.md` y confirma que coincide exactamente con el árbol de la rama (sin `diff_seal_mismatch`).
   * [ ] **Fusión Soberana e Higiene Git:**
     * Se ejecuta `merge --no-ff` sobre la rama principal (`main`/`master`) y se hace `push` al remoto.
     * Se elimina la rama de trabajo tanto en local como en el remoto.
     * El repositorio local queda posicionado en la rama principal (`main`/`master`), sincronizado con `origin` y con el árbol de trabajo limpio (`git status` sin cambios pendientes).
   * [ ] **Cierre en Tracker y Prueba Local:** Se emite **`PullRequest_Merged`**. `tekton.tracker-stamp` pasa `AUT-72` a **`Done`** (rango `5`, `tracker_column: done`), genera la prueba criptográfica local `.SddIA/proofs/tracker-done/AUT-72.json` y, si todos los hermanos de la HU padre están cerrados, promueve la HU padre a **`Done`**.

---

## 4. Plantilla de Informe de Salida (A rellenar por el Auditor)

Al concluir la ejecución de `AUT-72`, el agente/operador debe entregar el siguiente bloque completado con evidencias reales (SHAs, timestamps y extractos):

| Hito / Fase | Estado (`PASS` / `FAIL`) | Columna Linear / `tracker_column` | Evento(s) Emitidos (ID / Timestamp) | Evidencia Física (SHA / Ruta / URL) |
| :--- | :---: | :---: | :--- | :--- |
| **1. Refino (`refine-item`)** | `[ ]` | `Todo` / `todo` | `PBI_Refined` | Issue `AUT-72` (YAML + DoR) |
| **2. Ignición (Fase 1 + Mayeuta/Dédalo)** | `[ ]` | `In Progress` / `in_progress` | `Work_Initiated` | Rama: `...`<br>`persist_ref`: `...` |
| **3. Fin Tekton (`in_validation`)** | `[ ]` | `In Validation` / `in_validation` | Transición post-Tekton | Commits tareas + `implementation.md` / `execution.md` |
| **4. Cierre y PR (`delivery-close-cycle`)** | `[ ]` | `In Review` / `in_review` | `PullRequest_Presented` | `diff_digest`: `...`<br>PR URL: `...` |
| **5. Merge Soberano (`accept-pr`)** | `[ ]` | `Done` / `done` | `PullRequest_Audited`<br>`PullRequest_Merged` | Merge SHA: `...`<br>Proof: `.SddIA/proofs/tracker-done/AUT-72.json` |

### Diagnóstico de Fricciones o Fracturas Detectadas
* **¿Se encadenó `refine-item` $\rightarrow$ proceso de entrega con la orden única o requirió segundo estímulo?:** *(Detallar)*
* **¿Todas las fases de agente se ejecutaron con `status: executed` (no `simulated`)?:** *(Detallar)*
* **Anomalías o alertas Kaizen (`Kaizen_Alert_Required` / `Tracker_Sync_Failed`):** *(Detallar si ocurrieron)*
* **Veredicto Global de la Auditoría:** `[ APTO / NO_APTO ]`