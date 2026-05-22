[ARQUITECTURA] Blindaje Ontológico — Restricciones Duras y Enrutamiento Forzado para IA Obrera (Cursor/Jules)

0. ✅ Analizado e implementado — feature `docs/features/ia-obrera-blindaje/` (Fases A + B)

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

4. Fase C — La Aduana Física (Argos Gatekeeper) — **✅ COMPLETA**

Acción: Ejecutar aduana EDA e integridad de procesos como gate de commit y ciclo PR.

| Entrega | Estado | Evidencia |
|---------|--------|-----------|
| `pre-commit` — VPI + Existencia en Bus | ✅ | PR #12 — `pre_commit_gate.py` |
| Hooks `pre-push` / `post-merge` | ✅ | PR #13 — Ola B CA-3 |
| Norma `external-ai-constraints.md` | ✅ | Fase A — feature `ia-obrera-blindaje` |