# [ARQUITECTURA] PBI-042: Refactorización de Entidades de Dominio hacia Inyección de Dependencias (DI) Basada en Capacidades y Validación de Contratos Semánticos

## 1. Declaración de la Historia de Usuario
* **Como:** Arquitecto del Core de SddIA (Nodo de Control / Orquestador Central).
* **Quiero:** Sustituir los acoplamientos rígidos y las referencias estáticas entre las Entidades de Dominio (ED) por un modelo dinámico de Inyección de Dependencias (DI) gobernado por Capacidades Semánticas y validación matemática de esquemas JSON.
* **Para:** Garantizar la Ceguera Espacial absoluta de los agentes operativos, posibilitar la total intercambiabilidad de artefactos (procesos, acciones, skills, tools) sin alterar el genoma lógico de los consumidores, y blindar el ecosistema contra la alucinación estructural mediante la ejecución estricta del Filtro A (Lógica y Coherencia).

---

## 2. Puntos Críticos y Arquitectónicos a Contemplar

### 2.1. Rediseño Declarativo del Contrato de la ED (`spec.json`)
Cualquier Entidad de Dominio debe dejar de invocar nombres de archivos o rutas físicas específicas. Su configuración debe mutar hacia una estructura declarativa de necesidades y ofertas:
* **Bloque del Proveedor (`provides`):** El artefacto debe listar explícitamente qué capacidad o conjunto de capacidades añade al entorno y bajo qué versión de contrato opera.
* **Bloque del Consumidor (`dependencies` / `requires_capability`):** Si un proceso u otra ED depende fuertemente de una acción (por ejemplo, una etapa de "cierre documental"), no apuntará al id estático de un script, sino a la abstracción de la capacidad requerida y al esquema JSON esperado que valide dicha interacción.

### 2.2. Necesidad Obligatoria de Diccionarios y Taxonomías Semánticas (Requisito Innegociable)
Para mitigar la entropía semántica (la proliferación descontrolada de nombres redundantes o alucinados para una misma función, ej: `doc:close` vs `document:closure`), **el sistema requiere la creación de un Diccionario Universal de Capacidades SddIA**. 
* *Nota de Implementación:* Este PBI **no define** el diccionario en este bloque de tareas, pero establece como requerimiento obligatorio que ninguna ED podrá declarar una capacidad en su `spec.json` si esta no se encuentra indexada y aprobada previamente en el Códice centralizado de leyes del Core (`SddIA/norms/`).
* El inyector deberá validar las cadenas semánticas contra este glosario unificado en tiempo de enrutamiento.

### 2.3. Contratos de Interfaz basados en JSON Schema (Aduana de Datos)
La compatibilidad entre dos EDs con dependencias fuertes (como un proceso que delega en una acción de cierre) no se resolverá por compilación tradicional, sino mediante contratos I/O dinámicos.
* Cada capacidad declarada en el ecosistema debe llevar asociado un archivo de contrato (`.json` o JSON Schema).
* Si la acción inyectada en la configuración del proceso cumple con la capacidad semántica pero su payload de salida omite variables críticas que el proceso necesita aguas abajo (por ejemplo, el hash del documento verificado), el contrato se declarará roto en la fase previa a la ejecución.

### 2.4. Dinámica de Resolución en Runtime (El rol de Cúmulo y Library_Codex)
* El inyector del entorno (la interfaz CLI inerte `execute-process`) consultará a **Cúmulo** el plano de enrutamiento configurado para el entorno o laboratorio activo.
* El **Library_Codex** actuará como el mapa de asignación de recursos: si el proceso requiere `capability: "doc:closure"`, el Códice dictaminará qué artefacto físico específico de la cantera de herramientas hereda dicha jurisdicción para esa sesión concreta.

### 2.5. Intercepción y Control de Acceso por Cerbero
* Antes de que las dependencias inyectadas se empaqueten en la cápsula JSON I/O destinada al agente o Tékton, el agente de gobernanza **Cerbero** debe interceptar el payload.
* Cerbero auditará dos vectores:
    1.  Que la ED inyectada cumple matemáticamente con el esquema exigido.
    2.  Que el contexto operativo y el rol de ejecución tienen los permisos (RBAC) adecuados para consumir o activar dicha dependencia.
* Cualquier inconsistencia abortará la secuencia inmediatamente, derivando el estímulo fallido hacia la cola de errores aislados (Dead-Letter Queue / DLQ) bajo la Táctica de Inmunidad.

### 2.6. Propagación Desacoplada mediante Eventos de Dominio (EDA)
* Ninguna inyección de dependencias generará una llamada síncrona imperativa que bloquee el hilo de procesamiento termodinámico.
* Cuando la ED inyectada termine de operar de forma aislada (recibiendo sus dependencias estrictas por `stdin`), notificará su finalización arrojando un Evento de Dominio al bus físico (`.SddIA/events/`).
* Los procesos aguas arriba reaccionarán de forma asíncrona al evento transportado (`Event-Carried State Transfer`), garantizando latencia cero y preservando el estado de inmutabilidad.

---

## 3. Criterios de Aceptación Técnicos (Definition of Done)

### Escenario 1: Validación Semántica Correcta durante la Inyección
* **Dado** un Proceso de negocio que requiere una dependencia fuerte denominada `cierre_documental`.
* **Y** que dicho requerimiento está configurado mediante la abstracción de la capacidad `capability: "doc:closure"` y un esquema JSON estricto.
* **Cuando** el orquestador (`execute-process`) inicializa el proceso e inyecta una Acción concreta asignada por el `Library_Codex`.
* **Entonces** el sistema debe verificar que la Acción declara explícitamente proveer `doc:closure` en su metadato y que su estructura I/O pasa satisfactoriamente la validación del JSON Schema del contrato sin lanzar excepciones.

### Escenario 2: Bloqueo por Incumplimiento de Contrato (Filtro A Fails)
* **Dado** un Proceso que demanda la capacidad `capability: "doc:closure"`.
* **Cuando** se intenta configurar o inyectar un artefacto alternativo cuyas salidas (Outputs) no satisfacen la totalidad de la firma de datos o campos obligatorios requeridos por el esquema del proceso.
* **Entonces** el validador táctico de la Aduana de Fricción debe interceptar la carga útil, bloquear la invocación física de la cápsula, marcar el estado de entrega como fallido y emitir un log de fricción ontológica hacia la DLQ, impidiendo la alucinación del proceso.

### Escenario 3: Bloqueo por Capacidad no Indexada (Control de Entropía Semántica)
* **Dado** una nueva Acción en desarrollo cuyo archivo `spec.json` declara una capacidad inventada o no homologada (ej. `documentos:cerrar_archivo`).
* **Cuando** el sistema procesa el mapa de dependencias o valida el artefacto durante su fase previa a la aduana.
* **Entonces** la ejecución debe abortarse de forma limpia indicando explícitamente que la capacidad no pertenece a la **Taxonomía Universal de Capacidades SddIA**, forzando al desarrollador o a la IA obrera a dar de alta el término en el diccionario central antes de su materialización.

---

## 4. Matriz de Tareas Clave a Contemplar en el Sprint Técnico

1.  [ ] **Estandarización del Schema de Metadatos:** Modificar el validador de los `spec.json` base del Core para asimilar las propiedades `provides` y `requires_capability`.
2.  [ ] **Aislamiento de Interfaces (JSON I/O):** Desarrollar el wrapper de la CLI que inyecta dinámicamente las rutas físicas resueltas de las dependencias dentro del bloque de contexto enviado a la cápsula obrera por `stdin`.
3.  [ ] **Pipeline de Verificación Estricta:** Implementar el motor de validación en la aduana (vía Cerbero) para contrastar los esquemas de entrada/salida entre el consumidor y el proveedor inyectado en tiempo de ejecución.
4.  [ ] **Mecanismo de Alerta en la DLQ:** Asegurar que los fallos contractuales de inyección muten el estado del evento de forma atómica y queden anclados para posterior auditoría por Argos.
