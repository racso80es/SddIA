# Puntos de interacción con la IA (touchpoints)

**Objetivo:** Mantener los distintos gestores de IA (Cursor, Jules, etc.) y artefactos (.cursor/rules, .github) alineados con las normas definidas en **SddIA** (AGENTS.md, SddIA/norms/). SddIA es la única fuente de verdad; los touchpoints son **difusión** de esas normas.

Acción que orquesta la revisión y actualización: **paths.actionsPath/sddia-difusion/ (SddIA/actions/sddia-difusion/)**.

---

## Touchpoints actuales

| Touchpoint | Ubicación | Descripción | Cómo mantener |
|------------|-----------|-------------|----------------|
| **Cursor** | `.cursor/rules/*.mdc` + `.cursorrules` | Reglas que Cursor aplica al asistente (disparadores #Skill, #Action, #Process, subir; SSOT). `.cursorrules` §8 difunde **Blindaje IA Obrera** (`external-ai-constraints.md`). | Al cambiar procesos, acciones o listados en SddIA, ejecutar la acción sddia-difusion y actualizar los .mdc. La regla `sddia-ssot.mdc` declara que SddIA prevalece. Tras cambios en blindaje, sincronizar `.cursorrules` §8 con la norma motor SSOT. |
| **AGENTS.md** | Raíz del repo | Protocolo maestro del sistema multiagente. Referenciado por Cursor y por documentación. | Modificar solo según acuerdo con SddIA; no duplicar lógica en .cursor que contradiga AGENTS.md. |
| **AGENTS.norms.md** | Raíz del repo | Tabla de disparadores; enlaza a SddIA/norms/interaction-triggers.md. | Mantener sincronizado con interaction-triggers.md cuando se añadan disparadores (ej. #Tool). |
| **.github** | `.github/` | PR template, README (difusión). Workflows e issue templates si se añaden. | No duplicar normas; referenciar AGENTS.md y SddIA. PR template pide rama feat/fix, proceso y doc de tarea. Revisar con acción sddia-difusion. |

## Touchpoints futuros o opcionales

| Touchpoint | Ubicación | Descripción | Cómo mantener |
|------------|-----------|-------------|----------------|
| **Jules / Windsurf** | Configuración del gestor; opcional `.windsurfrules` | Prompt de sistema, reglas, contexto. Mismo blindaje que Cursor: referenciar `SddIA/norms/external-ai-constraints.md` o copiar resumen de `.cursorrules` §8. | Al desplegar el gestor, inyectar DA-1..3 y prefijo creator; no duplicar norma completa. Si existe `.windsurfrules`, debe enlazar la norma SSOT, no contradecirla. |
| **Otros IDEs/gestores** | Variable | Cualquier otro cliente que use el repo para asistencia IA. | Misma regla: SddIA es SSOT; documentar en esta tabla el touchpoint y el procedimiento de difusión. |

---

## Principios

1. **Una sola fuente de verdad:** Las normas de comportamiento están en SddIA (y AGENTS.md); los touchpoints las difunden, no las reemplazan.
2. **Rutas vía Cúmulo:** En reglas y documentación de comportamiento no usar rutas literales; referenciar `paths.workspacesRoot`, `paths.featurePath`, `paths.actionsPath`, `paths.skillCapsules`, etc. (SddIA/norms/paths-via-cumulo.md). En orquestación multi-agente, operar solo bajo `workspace_path` inyectado por el CLI; eventos tácticos ECST incluyen esa coordenada en payload (`Process_Execution_Completed`, orquestación Fase 3+).
3. **Inocuidad del Caos (`chaos-engineering`):** Contexto RBAC definido en `execution-contexts.md` §2.9. Toda tool con `context: chaos-engineering` debe invocar `assert_workspace_bound` (`SddIA/scripts/qa/chaos_workspace_utils.py`) **antes** de cualquier lectura o escritura en filesystem. Prohibido I/O fuera del `workspace_path` inyectado. Las **campañas de Caos** se declaran como ED **Suite** bajo `paths.directories.suites` y se orquestan vía `process:execute-suite` — no improvisar scripts de stress fuera del patrón Suite.
4. **Revisión al cambiar SddIA:** Si se añade un proceso (p. ej. refactorization), una acción (p. ej. sddia-difusion) o un disparador, actualizar los touchpoints afectados (acción sddia-difusion).

---
## Directriz registrada: Eficiencia Operativa y Gestión de Tokens (EO/GT)

**Objetivo:** Reducir coste operativo y tokens sin degradar la corrección, manteniendo trazabilidad (Karma2Token) y consistencia con **AGENTS.md** y SddIA.

### Reglas (obligatorias en todos los gestores / touchpoints)

1. **Prohibición de Entropía Social**
   - El agente debe **eliminar cortesías, prefacios y confirmaciones verbales** en todas las interacciones.
   - Prohibido: “Entendido”, “Claro”, “Gracias”, “Perfecto”, “Déjame…”, “A continuación…”, “Buena pregunta…”, “Con gusto…”.
   - Permitido: comunicación estrictamente necesaria para ejecutar (riesgos, bloqueos, pasos, resultados).

2. **Densidad Técnica S+ (Filtro A: Lógica)**
   - Respuestas **lacónicas** y centradas en **Lógica/decisión/acción** (Filtro A).
   - Prioridad de salida: **código/cambios verificables** > comandos vía skill/tool/acción/proceso > prosa.
   - Evitar redundancia: no repetir lo que el usuario ya sabe; no “narrar” el proceso.

3. **Pre-verificación de Suposiciones (antes de ejecutar)**
   - Antes de aplicar cambios, el agente debe **validar el contexto y dependencias existentes** para evitar retrabajo:
     - estado de archivos/artefactos canónicos (AGENTS.md, SddIA/norms, contratos relevantes),
     - presencia/ausencia de touchpoints (p. ej. reglas en `.cursor/rules`, configuración de Jules si existe),
     - coherencia con restricciones de ejecución (comandos solo vía skill/tool/acción/proceso).
   - La verificación es **técnica y documental**, no una solicitud de confirmación al usuario.

### Coherencia constitucional y auditoría de desviaciones

- **Coherencia con CONSTITUTION.md**: esta directriz regula **eficiencia de interacción** (no lógica de negocio). Si un gestor o regla local contradice SddIA/AGENTS, **prevalece SddIA** (SSOT).
- **Desviaciones**: cualquier excepción o desviación detectada debe registrarse como cambio auditable bajo el flujo **Feature** o **Refactorization** correspondiente (según alcance), con trazabilidad SddIA (incl. evolution cuando aplique a `./SddIA/`).

### Mecanismo de difusión (obligatorio al registrar o modificar esta directriz)

1. **Actualizar `.cursor/rules`** con una regla de aplicación global que refleje EO/GT (difusión para Cursor).
2. **Actualizar touchpoint Jules** (si existe): incorporar esta directriz en su prompt/reglas operativas para que aplique las mismas restricciones que Cursor.
3. **Verificar** que los touchpoints no introducen contradicciones con AGENTS.md, SddIA/norms y el contrato Token (Karma2Token).

---
*Referencia: paths.actionsPath/sddia-difusion/ (SddIA/actions/sddia-difusion/). Actualizar esta tabla al añadir o quitar gestores.*
