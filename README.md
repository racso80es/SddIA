# SddIA Core: industrialización de inteligencia descentralizada

## Librería SddIA
Ecosistema de **activos técnicos tokenizables** (NFTs lógicos: definiciones versionadas, contratos y manifiestos con identidad estable) orientados a la **industrialización de la IA**: consumo reproducible, gobernanza explícita y trazabilidad entre núcleo canónico e instancias productivas.

## Ontología de Activos

| Entidad | Finalidad | Ubicación Core | Relación operativa |
|---------|-----------|----------------|-------------------|
| **Agent** | Orquestador de consciencia y responsable de una fase específica. | `paths.directories.agents` | Posee Skills y ejecuta Acciones dentro de un **Process**. |
| **Process** | Roadmap lógico de alto nivel para un objetivo macro (p. ej. feature). | `paths.directories.process` | Orquesta el relevo (*handoff*) entre distintos **Agents**. |
| **Action** | Paso atómico, indivisible y auditable de ejecución. | `paths.directories.actions` | Invoca **Skills** o **Tools** para el trabajo técnico. |
| **Skill** | Capacidad técnica especializada definida por contrato. | `paths.directories.skills` | Ejecutada por **Cápsula** blindada (binario Rust o script Python bajo contrato). |
| **Tool** | Capacidad de infraestructura o utilidad de dominio. | `paths.directories.tools` | Servicios base a las **Actions** vía **Cápsula**. |
| **Event** | Contrato inmutable de comunicación asíncrona (Clase de Evento). | `paths.directories.events` | Definición en `{name}.md` bajo [events-contract.md](SddIA/events/events-contract.md); instancia ECST en bus runtime (`docs/events/` vía `eda_bus`). Ver también [CONSTITUTION_CORE.md](SddIA/CONSTITUTION_CORE.md) §3.1. |
| **Library_Codex** | Paquetes de normas orquestadas por dominio. | `paths.directories.library_codexes` | Agrupación de conocimiento técnico a cumplir por los **Agents**. |
| **Library_Norm** | Reglas técnicas atómicas, patrones y prohibiciones de *code-smells*. | `paths.directories.library_norms` | Cantera de la **Librería** (`SddIA/library/norms/`). **No** confundir con la normativa operativa del Core (`SddIA/norms`). |
| **Normativa de ejecución (Core)** | Contratos y normas de operación del núcleo (cápsulas, Git, triage, etc.). | `paths.directories.norms` | Árbol `SddIA/norms`; convive con la Librería; distinto alcance y clave SSOT que `library_norms`. |

### Dos canales de normativa (anti-dualidad)

| Canal | Clave en `cumulo.paths.json` | Ruta física (referencia) |
|-------|------------------------------|--------------------------|
| Operación del Core | `directories.norms` | `SddIA/norms` |
| Librería — normas atómicas | `directories.library_norms` | `SddIA/library/norms/` |

Jerarquía operativa: **Process** segmenta el objetivo en fases; cada fase asigna un **Agent** titular; el **Agent** descompone en **Actions**; las **Actions** consumen **Skills** y **Tools** materializados en cápsulas.

### Eventos: genoma, runtime e instancia

| Ruta | Rol | Contenido |
|------|-----|-----------|
| **`SddIA/events/`** | Genoma (Core) | Biblioteca de **Clases de Evento**: contratos funcionales versionados (`{name}.md`, `events-contract.md`, `index.md`). |
| **`docs/events/`** | Runtime (bus local) | Tránsito de **instancias volátiles** en colas `pending/`, `processing/`, `processed/` y `dead_letter/` (resuelto vía `eda_bus` en `cumulo.paths.json`). |
| **`.SddIA/events/`** | Instancia (proyecto) | Zona de **personalización** por repositorio productivo (Vía C); overrides y configuración táctica. **No** es cola del bus federal. |

Definición operativa de **Event**: *el contrato inmutable de comunicación asíncrona; señal con propósito (finalidad) que blinda la soberanía de las entidades conscientes, operando bajo coreografía pura y evitando el acoplamiento físico entre procesos.* Contrato de familia: `SddIA/events/events-contract.md`. Índice de clases: `SddIA/events/index.md`.

### Configuración: Jerarquía de Bóvedas

Secretos y variables de entorno del runtime se cargan desde **bóvedas locales** (no versionadas), no desde `.env` dispersos en subdirectorios de cápsulas. SSOT de rutas: clave `env_hierarchy` en [SddIA/core/cumulo.paths.json](SddIA/core/cumulo.paths.json).

| Ruta | Bóveda | Rol | Contenido |
|------|--------|-----|-----------|
| **`.dev/.env`** | Global (repo) | Valores compartidos del clone | Defaults de equipo, CI local, variables comunes al workspace |
| **`.SddIA/.dev/.env`** | Instancia (proyecto) | Overrides soberanos (Vía C) | Secretos y configuración táctica; **prevalece** sobre la bóveda global |

**Precedencia al arrancar entrypoints:**

1. **Entorno del SO** — máxima prioridad; los ficheros no sobrescriben variables ya definidas.
2. **`.dev/.env`** — rellena el diccionario intermedio.
3. **`.SddIA/.dev/.env`** — sobrescribe claves del paso anterior en el diccionario; se vuelca a `os.environ` con `setdefault`.

Si existen **ambas** bóvedas, el runtime registra en stderr: `[CONFIG] Jerarquía detectada: Aplicando SddIA/.dev/.env sobre .dev/.env`.

**Entrypoints que cargan la jerarquía** (vía `SddIA/scripts/qa/env_loader.py`) **antes** de invocar cápsulas:

| Entrypoint | Punto de carga |
|------------|----------------|
| `SddIA/scripts/qa/execute-process.py` | Tras resolver raíz del repo |
| `SddIA/scripts/qa/execute_process_capsules.py` | Inicio de `run_process()` |
| `SddIA/scripts/qa/execute-action.py` | Inicio de `main()` |
| `SddIA/scripts/daemons/event-watcher.py` | Inicio de `main()` |

Las cápsulas (p. ej. `iota-immutable-publisher`) **consumen** `process.env` / `os.environ` ya inyectado; **prohibido** `dotenv` local en el directorio del tool.

**Plantillas:** `SddIA/scripts/starter-kit/.SddIA/.dev/.env.example` (instancia). Copiar a `.SddIA/.dev/.env` en la raíz del workspace. Variables habituales: `IOTA_WALLET_SECRET`, `IOTA_ANCHOR_PACKAGE_ID`, `SDDIA_LAB_SIMULATE_IOTA`, `SDDIA_IOTA_TIMEOUT_SECONDS`.

**Migración desde legacy:** mover contenido de `SddIA/scripts/tools/iota-immutable-publisher/.env` → `.SddIA/.dev/.env` y eliminar el fichero local de la cápsula.

Documentación de feature: [docs/features/ampliacion-configuracion-entornos/](docs/features/ampliacion-configuracion-entornos/).

## Agentes del Core (resumen)

Catálogo canónico (UUID, `allowed_policies`, versiones): `{paths.directories.agents}` según el SSOT [SddIA/core/cumulo.paths.json](SddIA/core/cumulo.paths.json) (`cumulo.paths.json`); tabla e índice en [SddIA/agents/index.md](SddIA/agents/index.md). Cada definición vive en `{name}.md` junto al contrato de familia `agents-contract.md`.

| Agente | Rol (una línea) |
|--------|------------------|
| **Cerbero** | Peaje RBAC: autoriza o bloquea invocaciones según contexto y políticas. |
| **Cúmulo** | SSOT: topología, índices y coherencia documental del Core. |
| **Tekton** | Ejecución: materializa procesos delegando en cápsulas, sin terminal cruda. |
| **Mayeuta** | Clarificación: estabiliza el *qué* y el *por qué*; no diseña procesos ni código. |
| **Dedalo** | Planificación: norm pack + blueprint de **Process** alineado a contrato y RBAC del ejecutor. |
| **Argos** | Verificación: orquesta linters/tests/SAST vía procesos; juicio por evidencia, no por “vibes”. |

**Flujo típico:** Mayeuta → Dedalo → Tekton → Argos. Cerbero actúa en cada paso de delegación a cápsulas; Cúmulo gobierna rutas y catálogos.

## Orquestación multi-agente y relevo por artefactos
La colaboración entre agentes (p. ej. Tekton y un agente de seguridad) no es mensajería efímera: es **línea de montaje** gobernada por el **Process**.

1. El **Process** fija qué **Agent** tiene el mando en cada fase.
2. El traspaso de estado e información usa la **Carpeta de Tarea** referenciada por `persist_ref` (persistencia explícita, no contexto volátil).
3. El agente emisor deposita **artefactos** (código, informes, validaciones, binarios de prueba). El agente receptor **audita** esos artefactos antes de asumir la fase siguiente.

Sin carpeta de tarea materializada y artefactos versionables, no hay handoff válido bajo este modelo.

## Estándar de entidades de dominio SddIA
**Toda** familia de entidades de dominio debe cumplir el siguiente rigor arquitectónico:
1. **Ubicación SSOT:** Dispone de su ubicación según las indicaciones del agente Cúmulo (`{paths.directories...}`).
2. **Contrato Legal:** En dicha ubicación ha de existir, innegociablemente, un contrato para la implementación de entidades `{entidad}-contract.md`.
3. **Índice de Trazabilidad:** En dicha ubicación ha de existir un índice (`index.md`) con los items correspondientes a las implementaciones de entidades existentes. El agente Cúmulo tiene la responsabilidad de la coherencia de datos indicados por cada implementación.

## Cicatriz digital y estándar atómico
**Toda** entidad de dominio SddIA nace con **Cicatriz Digital**: un único archivo de definición `{name}.md` que incluye:
- Cabecera **YAML** obligatoria (contrato de la entidad).
- **Versión SemVer** y **UUID v4 inmutable** asignado en creación (identidad estable para trazabilidad y catálogo).
- Cuerpo en Markdown con propósito y límites del activo.
Ese paquete es el prerequisito para integración en la **Librería SddIA** y su modelo de activo direccionable (capa NFT-ready).

## Desacoplamiento Core / instancia
Definición de núcleo en este repositorio; especialización, constitución táctica y **secretos** en instancia local bajo `.SddIA/` (constitución, eventos, **bóveda `.SddIA/.dev/.env`**). La bóveda global `.dev/.env` complementa valores compartidos del clone. Ver [Jerarquía de Bóvedas](#configuración-jerarquía-de-bóvedas). Sin lógica de negocio dispersa fuera de **Actions** orquestadas y **Cápsulas**; sin `.env` operativos en subdirectorios de tools.

## Estándar de ejecución
Lógica crítica en **Cápsulas** (preferente binario Rust; Python permitido cuando esté explicitado y bajo contrato). Contrato de E/S: JSON por stdin/stdout según `SddIA/norms/capsule-json-io.md`. Los agentes orquestan; no sustituyen a la cápsula en cómputo ni en contrato de I/O.
