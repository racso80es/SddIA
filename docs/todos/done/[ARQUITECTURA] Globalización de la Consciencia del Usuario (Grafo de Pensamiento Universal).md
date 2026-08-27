---
document_id: PBI-ARQ-CONSCIENCIA-UNIVERSAL
uuid: "7ad2ef99-4c50-4b6d-9cc1-313d3338bb1b"
title: "[ARQUITECTURA] Globalización de la Consciencia del Usuario (Grafo de Pensamiento Universal)"
format: markdown
version: "2.0.0"
created: "2026-08-19"
updated: "2026-08-27"
refined: "2026-08-27"
status: done
pbi_archived: true
priority: alta
process: feature
scope: core-e-instancia
persist_ref: docs/features/memoria-preferencias-usuario
branch_name: feat/memoria-preferencias-usuario
validacion_ref: docs/features/memoria-preferencias-usuario/validacion.md
depends_on:
  - PBI-CORE-VECTOR-001-V3
  - PBI-CORE-GRAPH-002
related:
  - SddIA/core/cumulo.paths.json
  - SddIA/core/memory/
  - SddIA/agents/mayeuta.md
  - SddIA/agents/cumulo.md
  - SddIA/events/events-contract.md
  - SddIA/norms/capsule-json-io.md
  - docs/features/memoria-vectorial/
  - docs/features/grafo-pensamiento/
---

# [ARQUITECTURA] Memoria Global de Preferencias del Usuario

> “Grafo de Pensamiento Universal” se conserva como nombre histórico. El activo objetivo
> no es el `ThoughtGraphRepository` existente: es una memoria soberana de preferencias,
> directrices y revocaciones del Vértice Biológico, reutilizable entre canales.

## 1. Resultado del refinamiento

El propósito es válido, pero la propuesta original no era ejecutable sin violar contratos
vigentes. El alcance se estabiliza como un subsistema de **memoria global lógica dentro de
una instancia SddIA**, no como una base global compartida entre usuarios ni como
inyección indiscriminada en todas las cápsulas.

### 1.1 Hallazgos anti-alucinación

| ID | Afirmación original | Dictamen objetivo |
|----|---------------------|-------------------|
| H1 | El bus de dominio reside en `.SddIA/events/domain/`. | Incorrecto. Las instancias runtime se enrutan por `cumulo.paths.json`; la ruta fractal vigente es `./.events/domain/`. `.SddIA/events/` es personalización de instancia, no cola. |
| H2 | Los eventos nuevos se “definen en `events-contract.md`”. | Incorrecto. Cada Clase vive en `SddIA/events/domain/{name}.md`, cumple `SddIA/events/events-contract.md` y se registra en el índice de familia. |
| H3 | Una herramienta puede escribir directamente al bus. | Rechazado. La interfaz invoca una acción/proceso emisor autorizado; no escribe archivos ECST por su cuenta. |
| H4 | Mayeuta decide y ejecuta CRUD ontológico. | Excede su jurisdicción. Mayeuta estabiliza intención y puede estructurar una propuesta; no diseña arquitectura ni persiste memoria. |
| H5 | Cúmulo toca directamente LanceDB. | Inexacto. Cúmulo orquesta conocimiento y topología; la E/S física corresponde a una cápsula/adaptador mediante JSON stdin/stdout. |
| H6 | LanceDB/grafo ya ofrece una base lista para este caso. | No demostrado. `LanceDbThoughtRepo` contiene operaciones placeholder; además `ThoughtNode` representa razonamiento interno, no preferencias del usuario. |
| H7 | El orquestador debe consultar memoria antes de cualquier herramienta. | Rechazado por coste, privacidad y ruptura de contratos. Solo los procesos que declaren consumo de contexto reciben enriquecimiento. |
| H8 | “Eliminar” debe borrar físicamente una regla. | Rechazado como default. La revocación requiere tombstone y trazabilidad; purga física solo por política de privacidad explícita. |
| H9 | Un tripleto sujeto-predicado-objeto basta. | Insuficiente. Sin procedencia, ámbito, vigencia, sensibilidad y estado no se pueden resolver contradicciones ni revocaciones. |

## 2. Problema operativo

Una preferencia aprendida en un canal queda encapsulada en su lógica local. Esto genera:

- reglas duplicadas y divergentes;
- correcciones que no se propagan a otros canales;
- imposibilidad de revocar una preferencia desde un único punto;
- riesgo de convertir inferencias no confirmadas en mandatos del usuario.

Las herramientas permanecen como actuadores ciegos: no mantienen perfiles propios ni
deciden qué conocimiento adquiere autoridad global.

## 3. Objetivo medible

Construir un flujo gobernado que:

1. capture una corrección o directriz explícita desde un canal;
2. la transforme en una propuesta estructurada y auditable;
3. la persista o revoque en una memoria local de instancia;
4. la recupere desde un segundo consumidor autorizado sin conocer el canal de origen;
5. preserve procedencia, ámbito, privacidad y capacidad de revocación.

## 4. Alcance

### 4.1 Dentro

- Preferencias, directrices y revocaciones emitidas explícitamente por el usuario.
- Propuestas inferidas, siempre sin autoridad hasta confirmación humana.
- Persistencia local de instancia, fuera de Git.
- Consulta por ámbito y relevancia semántica.
- Integración inicial con un productor y un consumidor.
- Exportación, inspección, revocación y purga gobernada.
- Telemetría sin contenido personal.

### 4.2 Fuera

- Compartir memoria entre usuarios o instalaciones.
- Persistir conversaciones completas, correos o cuerpos de mensajes.
- Convertir el grafo interno de `ThoughtNode` en perfil del usuario.
- Inyectar contexto en todas las ejecuciones del Core.
- Anclar valores personales en IOTA/DLT.
- Crear un calendario ficticio para demostrar el caso de uso.
- Autoaprender reglas vinculantes a partir de frecuencia de uso.

## 5. Modelo conceptual mínimo

La unidad persistida es `UserPreference`, separada de `KnowledgeChunk`,
`ThoughtNode` y `EvolutionEvent`.

| Campo | Regla |
|-------|-------|
| `preference_id` | Identidad determinista de la afirmación lógica: ámbito + sujeto + predicado. |
| `revision_id` | Identidad inmutable de cada revisión. |
| `subject` | Entidad normalizada a la que aplica la preferencia. |
| `predicate` | Relación semántica de vocabulario controlado. |
| `value` | Valor estructurado; no texto libre si existe tipo canónico. |
| `scope` | `global`, `domain`, `project` o `channel`; prevalece el más específico. |
| `status` | `proposed`, `active`, `revoked` o `superseded`. |
| `authority` | `explicit_user` o `inferred`; solo la primera puede quedar activa sin confirmación adicional. |
| `provenance` | Canal, evento causal, instante y actor; nunca el cuerpo íntegro de la interacción. |
| `valid_from` / `valid_until` | Vigencia temporal opcional. |
| `sensitivity` | Clasificación usada por la política de lectura y redacción. |
| `supersedes` | `revision_id` anterior cuando exista actualización. |
| `embedding` | Índice de recuperación opcional; no fuente de autoridad. |

### 5.1 Operaciones

| Operación lógica | Efecto |
|------------------|--------|
| `IGNORE` | No crea registro; conserva solo telemetría agregada. |
| `PROPOSE` | Guarda candidato sin inyectarlo como directriz. |
| `ACTIVATE` | Activa una revisión explícita o confirmada. |
| `SUPERSEDE` | Crea nueva revisión y marca la anterior como sustituida. |
| `REVOKE` | Crea tombstone auditable; deja de devolver la regla. |
| `PURGE` | Borrado físico por solicitud/política de privacidad; no es operación ordinaria de razonamiento. |

## 6. Arquitectura estabilizada

### 6.1 Escritura

1. La interfaz recoge feedback explícito y llama a un **emisor canónico**.
2. El emisor crea una instancia ECST de dominio mediante la ruta resuelta por Cúmulo.
3. Un proceso de triaje asignado a Mayeuta transforma lenguaje natural en una
   `PreferenceProposal`; Mayeuta no escribe memoria.
4. Una fase de validación comprueba esquema, autoridad, ámbito, contradicciones y
   política de privacidad.
5. Cúmulo orquesta la persistencia, pero delega la E/S en una cápsula/repositorio
   de memoria por JSON stdin/stdout.
6. Tras persistencia exitosa se emite una señal de cambio que contiene identificadores
   y metadatos no sensibles, no el valor personal.

### 6.2 Lectura

1. El proceso consumidor declara explícitamente que requiere contexto de usuario.
2. El runtime resuelve una capacidad de lectura registrada en la taxonomía; queda
   prohibido inventar su `capability_id` durante la implementación.
3. El consumidor envía claves de consulta, ámbito y presupuesto máximo de resultados.
4. El repositorio filtra primero por autoridad, estado, sensibilidad y ámbito; KNN
   solo ordena candidatos permitidos.
5. La respuesta se inyecta en un bloque versionado del envelope de entrada. Si no
   hay binding o la consulta falla, se aplica la política fail-open/fail-closed
   declarada por el proceso, nunca un default global silencioso.

### 6.3 Precedencia y contradicciones

Orden mínimo de precedencia:

1. revocación vigente;
2. ámbito más específico;
3. autoridad explícita sobre inferida;
4. revisión más reciente dentro de la misma identidad lógica.

Una contradicción no resuelta bloquea la activación automática y genera una solicitud
de confirmación; no se decide mediante similitud vectorial.

## 7. Eventos candidatos

Los nombres se consolidarán durante el ciclo `feature`; no se añaden al contrato
maestro como una lista informal.

| Clase candidata | Finalidad | Restricción de payload |
|-----------------|-----------|------------------------|
| `User_Preference_Change_Requested` | Solicitar propuesta/activación/revocación desde una interacción autorizada. | Referencia causal y fragmento mínimo; prohibido cuerpo completo de correo/conversación. |
| `User_Preference_Changed` | Notificar una revisión ya persistida. | Solo IDs, operación, ámbito y clasificación; prohibido `value` sensible. |

Cada Clase deberá forjarse como `{name}.md` bajo la familia `domain`, sincronizar
su índice y declarar emisores/suscriptores. No tendrá suscriptor DLT en el MVP.

## 8. Persistencia y soberanía

- El almacén es local a la instancia y está fuera de Git.
- La ubicación física se obtiene por configuración inyectada; no se cablea en agentes.
- Puede reutilizar la infraestructura hexagonal de `SddIA/core/memory/`, pero requiere
  puerto y modelo propios.
- LanceDB es una opción de adaptador, no una condición asumida como implementada.
- Debe existir un adaptador durable funcional y pruebas de reapertura antes de declarar
  persistencia real; un placeholder que devuelve `Ok(())` no satisface el PBI.
- El refinamiento de feature debe resolver mediante amenaza documentada: cifrado en
  reposo, retención, backup, exportación y purga.

## 9. Responsabilidades

| Componente | Responsabilidad | Prohibición |
|------------|-----------------|------------|
| Interfaz/canal | Capturar feedback y llamar al emisor. | Mantener perfil local o escribir el bus directamente. |
| Mayeuta | Clarificar intención y producir propuesta estructurada. | Persistir, borrar o convertir inferencia en mandato. |
| Cúmulo | Resolver topología, validar identidad documental y orquestar memoria. | Ejecutar E/S física embebida en el agente. |
| Cápsula de memoria | Persistir, consultar, revocar y purgar bajo contrato. | Razonar sobre intención biológica. |
| Cerbero | Autorizar lectura/escritura según política y sensibilidad. | Conceder acceso global por defecto. |
| Orquestador | Resolver DI e inyectar contexto solo a consumidores declarados. | Consultar memoria antes de cada ejecución. |
| Argos | Verificar contratos, privacidad, persistencia y pruebas cruzadas. | Validar por narrativa sin evidencia. |

## 10. Plan incremental

### Fase 0 — Especificación y amenaza

- Materializar feature documental y mapa de datos.
- Separar formalmente `UserPreference` de `ThoughtNode`.
- Resolver política de consentimiento, sensibilidad, retención, exportación y purga.
- Decidir nombres ECST y capacidades sin violar la taxonomía vigente.

### Fase 1 — Dominio y puertos

- Crear modelo, invariantes y contrato de repositorio en `core/memory`.
- Definir esquemas JSON de escritura y lectura.
- Implementar repositorio durable con reapertura e idempotencia.

### Fase 2 — Flujo de escritura

- Forjar Clases ECST y emisor mediante `entity-manager`/creators.
- Implementar triaje de propuesta y validación de autoridad.
- Implementar `ACTIVATE`, `SUPERSEDE`, `REVOKE` y `PURGE`.

### Fase 3 — Flujo de lectura opt-in

- Registrar capacidades y contratos DI mediante mutación gobernada.
- Añadir declaración de consumo al proceso piloto.
- Inyectar bloque de contexto versionado, acotado y redactado.

### Fase 4 — Validación cruzada

- Productor A registra una preferencia explícita.
- Consumidor B, de otro canal/dominio y sin conocer A, recupera la regla.
- Revocación desde A impide su recuperación posterior en B.
- Ejecutar pruebas de contradicción, fuga de sensibilidad, duplicación e indisponibilidad.

## 11. Criterios de aceptación

| ID | Criterio verificable |
|----|----------------------|
| CA-01 | Existe `UserPreference` con esquema versionado, procedencia, ámbito, autoridad, estado y vigencia. |
| CA-02 | No se reutiliza `ThoughtNode` para representar preferencias del usuario. |
| CA-03 | Ningún canal mantiene una segunda SSOT de preferencias ni escribe archivos del bus directamente. |
| CA-04 | Una inferencia queda `proposed` hasta confirmación; nunca se activa solo por confianza/KNN. |
| CA-05 | Actualizar crea una nueva revisión y `supersedes`; revocar crea tombstone. |
| CA-06 | La purga física elimina valor, embedding y derivados conforme a una política probada. |
| CA-07 | El repositorio persiste de verdad: escritura, reinicio y lectura conservan la revisión. |
| CA-08 | Un productor y un consumidor de dominios distintos demuestran transferencia sin dependencia directa. |
| CA-09 | Solo procesos opt-in reciben contexto; un proceso no autorizado recibe cero preferencias. |
| CA-10 | Precedencia y contradicciones producen resultados deterministas y cubiertos por tests. |
| CA-11 | Eventos, logs, telemetría y DLT no contienen valores personales ni cuerpos de mensajes. |
| CA-12 | Todas las cápsulas cumplen `capsule-json-io` y las capacidades usadas existen en la taxonomía. |
| CA-13 | Rutas runtime y persistencia se resuelven por topología/configuración, sin hardcode en agentes. |
| CA-14 | La validación incluye fallo del store y política explícita fail-open/fail-closed del consumidor. |
| CA-15 | Existe exportación legible y mecanismo de inspección/revocación bajo control del usuario. |

## 12. Riesgos que bloquean cierre

| Riesgo | Mitigación obligatoria |
|--------|------------------------|
| Secuestro semántico | Autoridad explícita, estado `proposed`, confirmación y trazabilidad. |
| Fuga de datos personales | Payload mínimo, redacción, sin DLT, store local y política de acceso. |
| Dualidad de memoria | Puerto único; canales sin persistencia propia. |
| Context poisoning | Filtros previos a KNN, presupuesto de resultados y esquema tipado. |
| Dependencia de placeholder | Prueba de reapertura y lectura sobre adaptador real. |
| Coste universal | Consumo opt-in, límites de consulta y cero búsqueda cuando no se declara. |
| Revocación incompleta | Tombstone, invalidación de caché y prueba end-to-end posterior. |

## 13. Definition of Ready

El PBI puede entrar al proceso `feature` cuando:

- el Vértice Biológico apruebe el alcance local de instancia y el consentimiento;
- se elijan productor y consumidor piloto reales;
- Dedalo resuelva capacidades, contratos y política de fallo;
- exista amenaza de privacidad documentada;
- el plan no presuponga que el adaptador LanceDB placeholder ya es funcional.

## 14. Cierre (feature memoria-preferencias-usuario)

| Campo | Valor |
|-------|--------|
| Rama | `feat/memoria-preferencias-usuario` |
| `persist_ref` | `docs/features/memoria-preferencias-usuario/` |
| `validacion.md` | `global: APTO`, `pbi_archived: true` |
| Evolution | `SddIA/evolution/7ad2ef99-4c50-4b6d-9cc1-313d3338bb1b.md` |

Deuda post-MVP: smoke runtime Telegram (operador); sellos `entity-manager`; test contradicción CA-10; cápsula WASI.
