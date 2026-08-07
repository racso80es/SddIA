---
uuid: "b7e411b4-219d-47a2-bb44-8d9afcd29a73"
name: "entidad-digital-passport"
version: "1.0.0"
contract: "norms-contract v1.1.0"
nature: "tactical-norm"
intent: "Estandariza el esquema JSON para el Pasaporte Digital de Entidad SddIA, habilitando trazabilidad e inmutabilidad."
---

# Pasaporte Digital de Entidad SddIA (DEP)

El **Entity Digital Passport (DEP)** es el registro estandarizado que acompaña a todos los activos de software en SddIA (Agents, Skills, Tools). Está diseñado siguiendo las pautas de las normativas **EPCIS 2.0** y **oneM2M**, adaptado a la arquitectura distribuida y auditable del ecosistema SddIA.

## Campos Mandatorios (Genoma y Trazabilidad)

Para garantizar la identidad inmutable y la eficiencia termodinámica demostrable, cada DEP debe cumplir estrictamente con el siguiente esquema JSON:

### Esquema JSON

```json
{
  "uuid": "string (Obligatorio. UUID v4 del contrato de la entidad)",
  "creator_signature": "string (Obligatorio. Firma criptográfica del Creador - Soberanía Biológica)",
  "thermodynamic_metrics": {
    "execution_time_ms": "number (Obligatorio. Tiempo promedio o acumulado de ejecución)",
    "success_rate": "number (Obligatorio. Porcentaje de éxito de la entidad)"
  },
  "zkp_validations_log": [
    {
      "timestamp": "string (Obligatorio. Fecha en formato ISO 8601)",
      "proof_hash": "string (Obligatorio. Hash de la prueba de conocimiento cero)",
      "verifier": "string (Obligatorio. Identidad del Verifier, ej. Radamanto o Argos)",
      "result": "boolean (Obligatorio. Veredicto puro S+ Grade)"
    }
  ]
}
```

## Gobernanza y Aplicación

* El DEP acompaña a la evolución de cada cápsula.
* El registro de **zkp_validations_log** crecerá secuencialmente a lo largo del ciclo de vida del activo. Cada entrada representa un *proof* superado matemáticamente donde se demuestra el cumplimiento de las Reglas de Acero sin revelar la estructura original del código.
* Cúmulo auditará la coherencia termodinámica y estructural del DEP en cada barrido reactivo.
