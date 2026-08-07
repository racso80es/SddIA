# **\[PBI\] Arquitectura Ocean DPP: Pasaporte Digital de Entidad y Merkle Batching V2**

**Estado:** Backlog (Reserva táctica para futura iteración) | **Entidades Afectadas:** Cúmulo, iota-immutable-publisher, Radamanto, Argos, Tékton

## **1\. Especificación Técnica Profunda (Spec)**

Integración del modelo *Ocean DPP* en la Librería SddIA para resolver el cuello de botella termodinámico del anclaje de eventos (reducción transaccional del 99% en IOTA Rebased) y habilitar la confidencialidad de la auditoría mediante Pruebas de Conocimiento Cero (ZKP Groth16).

### **1.1. Motor de Inmutabilidad y Merkle Batching**

> * **Agrupación Táctica (Batching EDA):** El agente Cúmulo abandonará el enrutamiento unitario hacia la cápsula de inmutabilidad. Escuchará de forma reactiva la topología .SddIA/events/ hasta alcanzar un umbral de saturación térmico (ej. 50 eventos) o un *timeout* configurado.  
> * **Cápsula de Acero (Rust \- iota-immutable-publisher):** Refactorización del binario WASI. La entrada (*stdin*) mutará para aceptar un array de *payloads* JSON. Se integrará una librería nativa en Rust (como rs-merkle) para computar el árbol de Merkle utilizando el algoritmo de hash SHA-256.  
> * **Locked Notarization:** El orquestador ejecutará la transacción a través del IOTA SDK hacia la Testnet anclando única y exclusivamente el *Merkle Root* (32 bytes).  
> * **Persistencia de Evidencias (Ceguera Espacial):** Por cada evento del lote, la cápsula devolverá su *Merkle Proof* correspondiente. Cúmulo almacenará estas pruebas de forma aislada en la nueva topología .SddIA/proofs/\<uuid\_evento\>.json.

### **1.2. Pruebas de Conocimiento Cero (ZKP \- Groth16)**

> * Se diseñarán circuitos lógicos (mediante Circom o implementaciones nativas en Rust) donde el "Prover" (ejecutor) demuestra matemáticamente que un *snippet* de código ha cumplido las **Reglas de Acero** sin revelar la estructura original del código.  
> * El "Verifier" (Radamanto o Argos) consumirá este *proof* y emitirá un veredicto booleano puro S+ Grade. Este veredicto se transforma en un nuevo evento de dominio que, al pasar por Cúmulo, quedará anclado en la DLT.

### **1.3. Pasaporte Digital de Entidad SddIA (DEP)**

> * Se implementará un esquema JSON estandarizado basado en las pautas EPCIS 2.0 y oneM2M, adaptado a nuestros activos de software (Agents, Skills, Tools).  
> * **Genoma y Trazabilidad:** El DEP incluirá el *UUID* del contrato, la firma criptográfica del Creador (Soberanía Biológica), métricas de eficiencia termodinámica (tiempo de ejecución/éxito) y un registro encadenado de las validaciones ZKP superadas a lo largo de su ciclo de vida.

## **2\. Fronteras de Dominio y Gobernanza (Clarify)**

| Capa / Componente | Jurisdicción y Restricción Estructural |
| :---- | :---- |
| Bus de Eventos (.SddIA/events/) | Almacenamiento temporal desnormalizado. Estado inerte; no aplica lógica de negocio. |
| Cúmulo (Nodo de Control) | Responsabilidad ciega sobre la recolección y filtrado de eventos con estado success. Invoca al CLI de ejecución. |
| iota-immutable-publisher (Rust) | Cálculo de criptografía pesada y E/S de red (Testnet IOTA). Ignora la semántica de los eventos; solo opera hashes. |

## **3\. Plan de Ejecución Táctica (Plan)**

> 1. **Mutación de Contratos I/O:** Refinar capsule-json-io.md y actions-contract.md para habilitar el procesamiento en bloque (*arrays*) dentro de las interfaces de comunicación con los módulos WASI.  
> 2. **Desarrollo del Árbol de Merkle (Crate Rust):**  
   * Inyectar el gestor de dependencias criptográficas en el Cargo.toml.  
   * Adaptar la llamada a Locked Notarization del IOTA SDK para que ancle exclusivamente el hash de la raíz.  
   * Formatear el *stdout* del módulo Rust para entregar el *Merkle Proof* de cada rama al proceso invocador.  
> 3. **Adaptación del Motor EDA:** Enseñar a Cúmulo a realizar barridos sobre .SddIA/events/, aplicando la Táctica de Inmunidad para excluir eventos que ya posean confirmación de anclaje, previniendo el doble gasto de transacciones.  
> 4. **Forja Normativa S+:** Crear y sellar el documento constitucional SddIA/norms/entidad-digital-passport.md, estableciendo los campos mandatorios que definirán la tokenización de activos de la librería.

## **4\. Definition of Done (DoD)**

> * El binario de Rust compila de forma inmaculada (cargo build \--release) tras la integración de los algoritmos de *hashing*, sin *warnings* lógicos.  
> * Al empujar 50 *mock events* al sistema, el explorador de la Testnet IOTA Rebased refleja la creación de **una única transacción** (el Merkle Root).  
> * Existe un script validador de prueba que puede confirmar matemáticamente cualquier proof almacenado en .SddIA/proofs/ contra el hash anclado en la DLT.  
> * El encapsulamiento se mantiene: las cápsulas no exigen la instalación de dependencias dinámicas como PyYAML; el entorno sigue siendo hermético.