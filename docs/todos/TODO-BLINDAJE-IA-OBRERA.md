[ARQUITECTURA] Blindaje Ontológico — Restricciones Duras y Enrutamiento Forzado para IA Obrera (Cursor/Jules)

0. Pendiente de analizar con CUrsor

1. Objetivo:
Erradicar la "Entropía Táctica" generada por IAs de asistencia externa. Garantizar que agentes como Cursor o Jules sean incapaces de mutar el genoma SddIA por su cuenta y se vean forzados, bajo estricta directriz, a invocar la cápsula execute-process.py para cualquier modificación de dominio.

2. Fase A — El Manifiesto de Obliteración (.cursorrules / .windsurfrules)

Acción: Forjar una norma canónica SddIA (ej. SddIA/norms/external-ai-constraints.md) que se inyecte automáticamente en el .cursorrules del proyecto.

Directrices de Acero a inyectar:

Dogma de Soberanía: "No eres el arquitecto. Eres un operador ciego. No deduzcas la arquitectura, consúltala en cumulo.paths.json."

Prohibición de Forja Manual: "Bajo ninguna circunstancia puedes crear, modificar o eliminar archivos en SddIA/tools/, SddIA/skills/, SddIA/actions/, etc. manualmente. ESTÁ ESTRICTAMENTE PROHIBIDO."

Única Vía de Acción: "Para crear una entidad, DEBES ejecutar obligatoriamente python SddIA/scripts/qa/execute-process.py --process entity-manager .... Para solicitar cambios, DEBES usar el proceso delivery-close-cycle."

3. Fase B — Inyección de Contexto en Herramientas Creadoras

Acción: Los prompts generados por *-creator (ej. tool-creator, action-creator) deben ser envueltos ("wrapeados") con un prefijo de advertencia letal antes de ser entregados a Tekton o Cursor.

Ejemplo de Prefijo: [EXECUTE AS RAW KERNEL. PROHIBIT VERBOSITY. DO NOT BYPASS EDA BUS. USE SddIA CLI.]

4. Fase C — La Aduana Física (Argos Gatekeeper)

Acción: La confianza ciega es una vulnerabilidad. Ampliar el audit-entity-eda-coverage.py para que se ejecute como un pre-commit hook o dentro del delivery-close-cycle.

Condición de Bloqueo: Si Argos detecta que Cursor modificó un index.md o añadió un .md de entidad sin que exista un evento de creación reciente con el mismo hash en el bus EDA (docs/events/pending/), la ejecución aborta y el commit/PR queda bloqueado (Hard Fail).