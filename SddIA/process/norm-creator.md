---
uuid: "5fd5b47b-f98e-40f7-913f-b3a7c3eedf56"
name: "norm-creator"
version: "1.0.0"
contract: "process-contract v1.3.0"
context:
  - "ecosystem-evolution"
  - "knowledge-management"
hash_signature: "opcional_en_desarrollo"
inputs:
  - "norm_family": "pattern | principle | template"
  - "norm_id": "Identificador kebab-case del artefacto"
  - "norm_payload": "Frontmatter + cuerpo Markdown conforme a knowledge-contract"
  - "norm_extras": "Opcional: flags p. ej. blocking_for_pr para principles"
outputs:
  - "artifact_spec_md": "Archivo spec.md bajo directories.{patterns|principles|templates}/<norm_id>/"
  - "updated_family_index": "index.md de la familia actualizado"
phases:
  - name: "Validación contractual"
    intent: "Validar familia, metadatos y frontera knowledge vs procesos; gate Cerbero."
    delegates_to:
      - "agent:cumulo"
      - "agent:cerbero"
  - name: "Forja UUID + hash"
    intent: "Emitir UUID v4 y SHA-256 canónico del payload declarado."
    delegates_to:
      - "action:crypto-broker"
  - name: "Persistencia"
    intent: "Escribir spec.md en el directorio de familia correspondiente."
    delegates_to:
      - "skill:filesystem-manager"
  - name: "Indexación"
    intent: "Actualizar el index.md de patterns | principles | templates."
    delegates_to:
      - "agent:cumulo"
      - "skill:filesystem-manager"
phase_invocations:
  - phase_name: "Forja UUID + hash"
    invocations:
      - capsule: "action:crypto-broker"
        stdin_json:
          operation: "GENERATE_UUID"
          target_payload: null
        bind:
          "data.result": "norm_uuid"
        on_error: abort
      - capsule: "action:crypto-broker"
        stdin_spec:
          operation: "GENERATE_SHA256"
          target_type: "STRING"
          target_payload:
            type: "canonical_json_utf8"
            from_process_input: "norm_payload"
            json_dumps:
              sort_keys: true
              separators: [",", ":"]
              ensure_ascii: false
        bind:
          "data.result": "norm_payload_sha256_hex"
        on_error: abort
minteo_maximo: null
porcentaje_de_exito: null
---

# norm-creator

Proceso **creator** para la familia **knowledge**: forja `pattern`, `principle` o `template` bajo el SSOT ampliado (`cumulo.paths.json`), consumiendo explícitamente **`knowledge-contract.md`**.

## Persistencia

Tras la fase de criptografía, ensamblar cabecera con `uuid` (`norm_uuid`) y, si aplica, huella `sha256:norm_payload_sha256_hex` según política de hash del runtime. La ruta física es siempre relativa a `directories.patterns` \| `directories.principles` \| `directories.templates`.
