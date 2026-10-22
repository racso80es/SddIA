# [ARQUITECTURA] HU-SDDIA-CAPABILITY-DI-TRACKER: Abstracción del Gestor Documental mediante Inversión de Dependencias (Capability DI) para Soporte Multi-Cliente Agnóstico

---
**Identificador:** `HU-SDDIA-CAPABILITY-DI-TRACKER-002`  
**Estatus:** `Pendiente / Teórico`  
**Fecha de Emisión:** `2026-10-22`  
**Autor:** `Vértice Biológico (Racso) & Tormentosa`  
**Referencias Normativas:**
- `SddIA/core/capability-bindings.md` (Contrato de bindings de capacidades del Core)[cite: 36]
- `SddIA/norms/capsule-json-io.md` (Contrato estricto de E/S por JSON en cápsulas)[cite: 20, 28, 36]
- `SddIA/library/norms/capability-taxonomy.md` (Taxonomía de capacidades del ecosistema)[cite: 36]
- `SddIA/library/norms/capability-contracts/di.binding.schema.json` (Esquema de inyección de dependencias)[cite: 36]
---

## 1. Matriz de Indexación Tridimensional

* **Naturaleza:** Inversión de Dependencias / Polimorfismo Arquitectónico de Cápsulas y Desacoplamiento de Proveedor Documental[cite: 36].
* **Entorno:** Ecosistema `SddIA Core` — Módulos de resolución de capacidades (`capability_di_resolver.rs`, `capability_di_gate.rs`, `project_binding.rs`) y adaptadores documentales del ecosistema[cite: 36].
* **Entropía Asimilada:** Destilado de la fricción simbiótica directa al plantear la portabilidad del motor frente a múltiples clientes con requerimientos de backend documental heterogéneos (Linear, Disco Local, GitHub, etc.)[cite: 36].

---

## 2. Descripción General y Alcance Funcional

* **Como:** Arquitecto de Plataforma de la Librería SddIA[cite: 24, 27, 36].
* **Quiero:** Abstraer por completo la gestión documental de tickets e ítems en el Core de SddIA mediante el patrón de Inversión de Dependencias por Capacidades (`Capability DI`), de modo que el motor interactúe exclusivamente con un puerto agnóstico (`tracker.lifecycle`) y permita a cada cliente inyectar la cápsula adaptadora deseada (ej. `linear-tracker-adapter`, `fs-tracker-adapter` en disco, o un futuro `github-tracker-adapter`) a través de su configuración de instancia, sin modificar el código fuente del Core[cite: 24, 27, 36].
* **Para:** Garantizar el principio de agnosis tecnológica, evitar el acoplamiento rígido a Linear o a cualquier otro proveedor comercial, y permitir que la Librería SddIA escale como un organismo modular y comercializable[cite: 24, 27, 36].

### Reglas de Negocio y Gobernanza del Puerto de Capacidades

1. **Agnosis Absoluta del Core (`SddIA/`):**
   * El código del motor (`execute-process`) y los procesos de negocio (`feature`, `bug-fix`, `refine-hu`, etc.) **no deben contener referencias hardcodeadas** a proveedores específicos como Linear[cite: 36]. 
   * Las operaciones sobre el backlog se definen como un contrato de capacidad abstracto (`tracker.lifecycle`), tipado mediante esquemas JSON bajo `SddIA/library/norms/capability-contracts/`[cite: 36].

2. **Polimorfismo de Adaptadores Documentales:**
   * Cualquier cápsula ejecutable (compilada en Rust/WASI) que cumpla con el envelope de entrada/salida de `capsule-json-io.md` y respete el contrato de la capacidad `tracker.lifecycle` puede ser utilizada indistintamente por el cliente[cite: 20, 28, 36]:
     * **`linear-tracker-adapter`**: Conecta el ciclo de vida documental con la API de Linear[cite: 36].
     * **`fs-tracker-adapter`**: Encapsula el sistema tradicional de ficheros locales en disco (`docs/todos/pending/` y `docs/todos/done/`) para clientes que operan sin dependencias externas[cite: 19, 36].
     * **`github-tracker-adapter` (Extensible):** Proveedor alternativo basado en GitHub Issues / Projects[cite: 24, 27, 36].

3. **Resolución Dinámica por Instancia (`.SddIA/`):**
   * La selección de la cápsula documental activa para un proyecto se declara en la configuración de instancia (`.SddIA/project.md` o bindings locales regidos por `project-config-contract.md`)[cite: 36].
   * Al arrancar un proceso, `project_binding.rs` y `capability_di_resolver.rs` leen la directiva del cliente, validan los permisos mediante el Peaje RBAC de `Cerbero`, y enrutan la ejecución hacia la cápsula correspondiente inyectándole las variables de entorno de su bóveda local `.SddIA/.dev/.env`[cite: 28, 36].

---

## 3. Separación de Responsabilidades (Vértice Biológico vs. Sistema)

### A. Tareas bajo Jurisdicción del Vértice Biológico (Prerrequisitos Manuales)
* **Declaración del Proveedor en Instancia:** Configurar en el archivo de proyecto de la instancia (`.SddIA/project.md`) qué cápsula adaptadora (`linear-tracker-adapter` o `fs-tracker-adapter`) se empleará en el entorno de desarrollo[cite: 28, 36].
* **Suministro de Credenciales de Instancia:** Definir en `.SddIA/.dev/.env` las variables de acceso exclusivas del adaptador seleccionado (ej. tokens de Linear o flags de disco), manteniendo la bóveda global `.dev/.env` libre de acoplamientos a proveedores particulares[cite: 28, 36].

### B. Comportamiento Automatizado del Sistema (SddIA Core)
* **Resolución de Puerto (`capability_di_resolver.rs`):** Interceptar las peticiones de ciclo de vida documental emitidas por los procesos y traducirlas en invocaciones estándar vía `stdin/stdout` JSON hacia la cápsula configurada por el cliente[cite: 28, 36].
* **Validación de Salida (`capability_di_output_validator.rs`):** Comprobar que el resultado devuelto por cualquier adaptador polimórfico cumpla estrictamente con el formato de éxito/error esperado por el orquestador, previniendo caídas por desalineación de esquemas[cite: 28, 36].

---

## 4. Coreografía y Normativas de Gobernanza S+ Grade

### Diagrama de Flujo: Inyección de Dependencias por Capacidad Documental

```mermaid
graph TD
    A[Proceso / Acción Core: feature / bug-fix] -->|Solicita capacidad abstracta| B(capability_di_resolver.rs)
    B -->|Lee configuración de instancia| C{.SddIA/project.md}
    C -->|Binding: tracker.lifecycle| D{Selección de Adaptador}
    
    D -->|Configurado: Linear| E[linear-tracker-adapter.exe]
    D -->|Configurado: Disco Local| F[fs-tracker-adapter.exe]
    D -->|Configurado: GitHub| G[github-tracker-adapter.exe]
    
    E -->|Contrato JSON (capsule-json-io)| H[API Linear]
    F -->|Contrato JSON (capsule-json-io)| I[Sistema Archivos .md Local]
    G -->|Contrato JSON (capsule-json-io)| J[API GitHub Issues]
    
    H --> K[Retorno Uniforme al Motor SddIA]
    I --> K
    J --> K
```

### Reglas de Gobernanza S+ Grade Aplicables
* **Cero Acoplamiento Espacial:** El núcleo SddIA desconoce las estructuras de datos nativas de Linear o GitHub; solo comprende el genoma universal de las entidades de dominio y el contrato de la capacidad `tracker.lifecycle`[cite: 20, 28, 36].
* **Aislamiento de Instancia (Vía C):** Las preferencias de proveedores documentales pertenecen estrictamente a la capa de instancia (`.SddIA/`), protegiendo al repositorio central (`SddIA/`) contra la entropía de configuraciones a medida por cliente[cite: 28, 31, 36].

---

## 5. Criterios de Aceptación (Formato Gherkin)

### Escenario 1: Configuración e inyección del adaptador Linear en el cliente
* **Dado** que un proyecto cliente configura en su `.SddIA/project.md` el uso de `linear-tracker-adapter` para la capacidad `tracker.lifecycle`[cite: 28, 36],
* **Cuando** el motor SddIA ejecuta un proceso que requiere actualizar el estado de un PBI,
* **Entonces** el resolvedor de capacidades (`capability_di_resolver`) invoca dinámicamente a la cápsula de Linear utilizando el sobre JSON estándar, completando la operación con éxito sin alterar el código del Core[cite: 28, 36].

### Escenario 2: Conmutación transparente hacia el adaptador de disco (`fs-tracker-adapter`)
* **Dado** un entorno local o de laboratorio configurado para emplear el adaptador en disco (`fs-tracker-adapter`) en lugar de Linear[cite: 36],
* **Cuando** el motor emite la misma orden abstracta de ciclo de vida documental,
* **Entonces** el sistema enruta la petición hacia la cápsula de ficheros locales, operando de forma idéntica desde la perspectiva del proceso sin requerir adaptaciones en los agentes de IA[cite: 28, 36].

### Escenario 3: Rechazo por incumplimiento de contrato de capacidad
* **Dado** una cápsula adaptadora de terceros que no respeta el esquema JSON del contrato `tracker.lifecycle`[cite: 36],
* **Cuando** el validador de salida (`capability_di_output_validator`) audita la respuesta de la cápsula[cite: 28, 36],
* **Entonces** intercepta el error, bloquea la ejecución de forma segura (`fail-soft`) y emite un reporte de fricción sin comprometer la estabilidad del bus EDA[cite: 6, 28, 31, 36].

---

## 6. Propuesta de Desglose en PBIs (Regla de $\le 3$ Archivos Adyacentes por PBI)

1. **PBI-1: Definición del Contrato de Capacidad Documental (`tracker.lifecycle`)**
   * **Objetivo:** Formalizar el contrato de capacidad agnóstica para la gestión de ciclo de vida documental en el sistema de normas del Core.
   * **Archivos:** `SddIA/library/norms/capability-taxonomy.md`, `SddIA/core/capability-bindings.md`, `SddIA/library/norms/capability-contracts/tracker.lifecycle.schema.json`[cite: 36].

2. **PBI-2: Extracción del Gestor de Disco hacia `fs-tracker-adapter`**
   * **Objetivo:** Aislar la lógica de manipulación de archivos Markdown tradicionales en una cápsula independiente compatible con el nuevo puerto de capacidad.
   * **Archivos:** `SddIA/tools/fs-tracker-adapter/Cargo.toml`, `SddIA/tools/fs-tracker-adapter/src/main.rs`, `SddIA/tools/index.md`[cite: 36].

3. **PBI-3: Integración del Resolvedor Polimórfico en el Orquestador**
   * **Objetivo:** Conectar `capability_di_resolver.rs` y `project_binding.rs` con las llamadas del motor para delegar el ciclo de vida documental en el adaptador configurado por la instancia[cite: 28, 36].
   * **Archivos:** `SddIA/engine/execute-process/src/engine/capability_di_resolver.rs`, `SddIA/engine/execute-process/src/engine/project_binding.rs`, `SddIA/engine/execute-process/src/engine/executor.rs`[cite: 36].
