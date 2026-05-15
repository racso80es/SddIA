#!/usr/bin/env python3
"""One-shot migration: workspace root constitutions -> .SddIA/constitution/."""
from __future__ import annotations

import json
import shutil
import uuid
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

SUBMISSION = (
    "Este documento rige las normativas tácticas locales del nodo. "
    "En caso de colisión de directrices, la Ley Suprema Federal "
    "(SddIA/CONSTITUTION_CORE.md) prevalece siempre."
)

LOCAL_PATHS = {
    "directories": {
        "local_tools": ".SddIA/tools",
        "local_norms": ".SddIA/norms",
        "local_security": ".SddIA/security",
        "local_constitution": ".SddIA/constitution",
        "local_principles": ".SddIA/principles",
        "local_patterns": ".SddIA/patterns",
        "local_templates": ".SddIA/templates",
        "local_evolution": ".SddIA/evolution",
        "library_codexes": ".SddIA/library/codexes/",
        "library_norms": ".SddIA/library/norms/",
    },
    "paths": {
        "localLibraryCodexes": ".SddIA/library/codexes/",
        "localLibraryNorms": ".SddIA/library/norms/",
    },
    "files": {
        "interaction_triggers_override": ".SddIA/interaction-triggers.override.json",
        "local_security_contract": ".SddIA/local-security-contract.json",
        "local_evolution_log": ".SddIA/evolution/Evolution_log.md",
        "featuresDocumentationPattern": ".SddIA/library/norms/features-documentation-pattern.md",
        "features_documentation_pattern": ".SddIA/library/norms/features-documentation-pattern.md",
    },
}

EVOLUTION_README = """# Evolución del dominio local (.SddIA/)

Registro de cambios materiales en el **contexto de instancia** (constitución táctica, tools, norms, patterns, templates, security bajo `.SddIA/`).

- **Separación:** la evolución del **motor** vive en `SddIA/evolution/`; la del **producto local** aquí.
- **Formato:** fichero `{id_cambio}.md` con frontmatter (UUID v4) + entrada en `Evolution_log.md`.
- **Jerarquía:** `SddIA/CONSTITUTION_CORE.md` prevalece sobre cualquier norma táctica local.
"""

FACETS_BACKEND = """## 7. Facetas constitucionales incorporadas

Resumen absorbido del antiguo árbol `constitution/` (facetas cognitive, architect, audity, duality):

- **Cimiento arquitectónico:** código en `src/` con capas Api, Application, Domain, Infrastructure; DI explícita del stack .NET; contratos en Domain/Application.
- **Núcleo cognitivo:** persistencia vía Infrastructure/DbContext; contexto de dominio en servicios Core (no memoria Electron).
- **Auditoría:** trazabilidad vía normas SddIA del motor y agente auditor; sin Logger IPC de escritorio.
- **Dualidad operativa:** modos Boss/Calm como referencia de gobernanza; sin IPC Electron en este backend.
"""

FACETS_FRONTEND = """## 6. Facetas constitucionales incorporadas

Resumen absorbido del antiguo árbol `constitution/` (facetas cognitive, architect, audity, duality):

- **Cimiento arquitectónico:** `src/` con `app/`, `components/`, `lib/`, `contexts/`, `types/`; composición de UI; contratos HTTP en `src/lib/api/`.
- **Núcleo cognitivo:** estado y contexto vía React Context y React Query; sin cápsula de memoria persistente propia.
- **Auditoría:** alineación con normas SddIA del motor y controles de calidad en build/test.
- **Dualidad operativa:** modos Boss/Calm como marco de intervención humana vs ejecución autónoma en CI/build.
"""

EVOLUTION_SECTION_BACKEND = """## 6. Trazabilidad SddIA (motor vs instancia)

- **Motor (`./SddIA/`):** cambios normativos o estructurales auditables según `SddIA/norms/sddia-evolution-sync.md` y registro en `SddIA/evolution/`.
- **Instancia (`.SddIA/`):** cambios materiales en constitución, tools, norms u otros subárboles locales según `.SddIA/evolution/` (misma política de UUID v4 e índice).
- **Producto:** evolución funcional en `docs/evolution/` cuando aplique al software entregable.
"""

EVOLUTION_SECTION_FRONTEND = """## 5. Trazabilidad SddIA (motor vs instancia)

- **Motor (`./SddIA/`):** cambios bajo el motor inyectado según `SddIA/norms/sddia-evolution-sync.md` y `SddIA/evolution/`.
- **Instancia (`.SddIA/`):** cambios materiales locales registrados en `.SddIA/evolution/` (UUID v4 + `Evolution_log.md`).
- **Producto:** evolución funcional en `docs/evolution/` cuando aplique.
"""

WORKSPACES = [
    ("SddIA_1", "GesFer.Admin.Back", "backend"),
    ("SddIA_2", "GesFer.Admin.Front", "frontend"),
    ("SddIA_3", "GesFer.Admin.Back", "backend"),
    ("SddIA_4", "GesFer.Product.Front", "frontend"),
]


def migrate_workspace(ws: str, product: str, profile: str) -> str:
    base = ROOT / ws
    legacy_md = base / "CONSTITUTION.md"
    legacy_json = base / "constitution.json"
    if not legacy_md.exists():
        raise FileNotFoundError(legacy_md)
    body = legacy_md.read_text(encoding="utf-8")
    for marker in (
        "## 6. Trazabilidad del protocolo SddIA (evolution)",
        "## 5. Trazabilidad del protocolo SddIA (evolution)",
    ):
        if marker in body:
            body = body.split(marker)[0].rstrip() + "\n"
    if profile == "backend":
        body += "\n" + EVOLUTION_SECTION_BACKEND + "\n" + FACETS_BACKEND + "\n"
    else:
        body += "\n" + EVOLUTION_SECTION_FRONTEND + "\n" + FACETS_FRONTEND + "\n"
    frontmatter = (
        "---\n"
        f"entity_type: local-constitution\n"
        f"workspace_id: {ws}\n"
        f"product: {product}\n"
        "jurisdiction: tactical\n"
        "federal_constitution_ref: SddIA/CONSTITUTION_CORE.md\n"
        "local_paths_ref: .SddIA/local.paths.json\n"
        f"submission_clause: {json.dumps(SUBMISSION, ensure_ascii=False)}\n"
        "---\n\n"
        f"> {SUBMISSION}\n\n"
    )
    const_dir = base / ".SddIA" / "constitution"
    evo_dir = base / ".SddIA" / "evolution"
    const_dir.mkdir(parents=True, exist_ok=True)
    evo_dir.mkdir(parents=True, exist_ok=True)
    (const_dir / "CONSTITUTION.md").write_text(frontmatter + body.lstrip(), encoding="utf-8")
    (base / ".SddIA" / "local.paths.json").write_text(
        json.dumps(LOCAL_PATHS, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )
    data = json.loads(legacy_json.read_text(encoding="utf-8"))
    data.setdefault("meta", {})["nature"] = "product"
    data["meta"]["last_updated"] = "2026-05-11"
    data["configuration"]["paths_ref"] = (
        "Cumulo: SddIA/core/cumulo.paths.json (universal) + .SddIA/local.paths.json (instancia)."
    )
    laws = data.get("universal_laws", [])
    if not any(law.get("id") == "L9_LOCAL_EVOLUTION" for law in laws):
        laws.append(
            {
                "id": "L9_LOCAL_EVOLUTION",
                "rule": "Local instance evolution",
                "description": (
                    "Any material change under .SddIA/ (constitution, tools, norms, patterns, "
                    "templates, security) must be recorded in .SddIA/evolution/ with UUID v4 "
                    "detail file and Evolution_log.md update."
                ),
                "severity": "HIGH",
            }
        )
    data["universal_laws"] = laws
    (const_dir / "constitution.json").write_text(
        json.dumps(data, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )
    (const_dir / "README.md").write_text(
        "# Constitución táctica (`.SddIA/constitution/`)\n\n"
        "Bylaw local del laboratorio. El motor federal está en `SddIA/CONSTITUTION_CORE.md`.\n",
        encoding="utf-8",
    )
    (evo_dir / "README.md").write_text(EVOLUTION_README, encoding="utf-8")
    evo_id = str(uuid.uuid4())
    evo_path = evo_dir / f"{evo_id}.md"
    evo_body = (
        "---\n"
        f"id_cambio: {evo_id}\n"
        "fecha: 2026-05-11T00:00:00+00:00\n"
        f"workspace: {ws}\n"
        "tipo_operacion: modificacion\n"
        "descripcion_breve: Migración constitución táctica a .SddIA/constitution/ y purga legacy en raíz\n"
        "---\n\n"
        f"Migración constitucional del laboratorio **{ws}** ({product}). "
        "Origen: `CONSTITUTION.md`, `constitution.json` y `constitution/` en raíz. "
        "Destino: `.SddIA/constitution/` y registro en `.SddIA/evolution/`.\n"
    )
    evo_path.write_text(evo_body, encoding="utf-8")
    (evo_dir / "Evolution_log.md").write_text(
        "# Evolution log (instancia local)\n\n"
        "| id_cambio | fecha | resumen |\n"
        "|-----------|-------|--------|\n"
        f"| {evo_id} | 2026-05-11 | Migración constitución táctica a .SddIA/constitution/ |\n",
        encoding="utf-8",
    )
    legacy_md.unlink()
    legacy_json.unlink()
    legacy_const = base / "constitution"
    if legacy_const.exists():
        shutil.rmtree(legacy_const)
    return evo_id


def main() -> None:
    for ws, product, profile in WORKSPACES:
        evo_id = migrate_workspace(ws, product, profile)
        print(f"{ws} {evo_id}")


if __name__ == "__main__":
    main()
