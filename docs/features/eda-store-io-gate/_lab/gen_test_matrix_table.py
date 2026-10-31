#!/usr/bin/env python3
"""Genera § Cobertura actual en spec-test-matrix.md (P4). Solo documental."""
from __future__ import annotations

import json
import re
import sys
from datetime import date
from pathlib import Path

REPO = Path(__file__).resolve().parents[4]
CORE = REPO / "SddIA/core"
FEATURE = REPO / "docs/features/eda-store-io-gate"
SPEC = FEATURE / "spec-test-matrix.md"

DUAL_SAMPLE = {
    "Domain_Entity_Created",
    "System_Fracture_Detected",
    "PullRequest_Presented",
    "Kaizen_Alert_Required",
    "Thought_Persisted",
    "Work_Initiated",
}
DOMAIN_FRACTAL_SAMPLE = {
    "Domain_Entity_Telemetry_Captured",
    "Fracture_Pbi_Materialized",
    "Local_QA_Requested",
    "Kalma2_Process_Requested",
}
FS_TELEMETRY = {"Raw_Execution_Finished", "Daemon_Heartbeat"}
EXTRA_TELEMETRY = {"System_Vitality_Probed"}
EXTRA_ORCH = {"TQM_Dispatch_Discarded", "Local_QA_Requested"}


def load_map(name: str) -> dict:
    return json.loads((CORE / name).read_text(encoding="utf-8"))


def subscriber_count(maps: list[dict], event_type: str) -> int:
    for m in maps:
        if event_type in m:
            subs = m[event_type]
            if isinstance(subs, list):
                return len(subs)
    return 0


def registries_for(etype: str, reg: dict[str, dict]) -> str:
    files = [f for f, m in reg.items() if etype in m]
    return ", ".join(f.replace("event-", "").replace(".json", "") for f in files) or "—"


def main() -> int:
    reg = {
        "event-subscriptions.json": load_map("event-subscriptions.json"),
        "event-domain-subscriptions.json": load_map("event-domain-subscriptions.json"),
        "event-telemetry-subscriptions.json": load_map("event-telemetry-subscriptions.json"),
        "event-orchestration-subscriptions.json": load_map(
            "event-orchestration-subscriptions.json"
        ),
    }
    pending_bus = sorted(
        set(reg["event-subscriptions.json"]) | set(reg["event-domain-subscriptions.json"])
    )
    telemetry = sorted(set(reg["event-telemetry-subscriptions.json"]) | EXTRA_TELEMETRY)
    orchestration = sorted(
        set(reg["event-orchestration-subscriptions.json"]) | EXTRA_ORCH
    )
    all_types = sorted(set(pending_bus) | set(telemetry) | set(orchestration))

    rows: list[str] = []
    for et in all_types:
        subs = subscriber_count(
            [
                reg["event-subscriptions.json"],
                reg["event-domain-subscriptions.json"],
                reg["event-telemetry-subscriptions.json"],
                reg["event-orchestration-subscriptions.json"],
            ],
            et,
        )
        tm1 = "APTO" if et in pending_bus else "—"
        tm2 = "APTO" if et in telemetry else "—"
        tm3 = "APTO" if et in orchestration else "—"
        if et in DOMAIN_FRACTAL_SAMPLE:
            tm4 = "APTO"
        elif et in reg["event-domain-subscriptions.json"]:
            tm4 = "muestra"
        else:
            tm4 = "—"
        tm5_parts = []
        if et in pending_bus:
            tm5_parts.append("pending FS")
        if et in FS_TELEMETRY:
            tm5_parts.append("telemetry FS")
        tm5 = "APTO" if tm5_parts else "—"
        tm6 = "APTO" if et in DUAL_SAMPLE else ("—" if et not in pending_bus else "gap")
        tm7 = "APTO" if subs == 0 else "fan-out"
        if subs == 0:
            tm8 = "N/A"
        elif et == "PullRequest_Presented":
            tm8 = "APTO"
        else:
            tm8 = "parcial"

        rows.append(
            f"| `{et}` | {registries_for(et, reg)} | {tm1} | {tm2} | {tm3} | {tm4} | {tm5} | {tm6} | {tm7} | {tm8} |"
        )

    generated = f"""<!-- MATRIX-COVERAGE:START — generado {date.today().isoformat()}; no editar a mano -->
## Cobertura actual (generada)

Comando: `./docs/features/eda-store-io-gate/gen-test-matrix-table.sh`

Resumen: **{len(all_types)}** `event_type` distintos · pending_bus **{len(pending_bus)}** · telemetría **{len(telemetry)}** · orquestación **{len(orchestration)}**

Gaps PTC explícitos (fuera matriz ECST): side-channel latidos; `progress_traces` — ver `objectives-test-matrix.md`.

| `event_type` | Registro(s) | TM-1 | TM-2 | TM-3 | TM-4 | TM-5 | TM-6 | TM-7 | TM-8 |
|--------------|-------------|------|------|------|------|------|------|------|------|
{chr(10).join(rows)}

Leyenda: **APTO** = cubierto por test de integración en rama; **muestra** = subconjunto fractal domain; **parcial** = patrón no exhaustivo; **fan-out** = tiene suscriptores; **N/A** = sin suscriptores (TM-7).
<!-- MATRIX-COVERAGE:END -->"""

    if not SPEC.is_file():
        print(f"missing {SPEC}", file=sys.stderr)
        return 1

    text = SPEC.read_text(encoding="utf-8")
    pattern = re.compile(
        r"<!-- MATRIX-COVERAGE:START.*?<!-- MATRIX-COVERAGE:END -->",
        re.DOTALL,
    )
    if pattern.search(text):
        new_text = pattern.sub(generated, text)
    else:
        old_block = re.search(
            r"## Cobertura actual \(rama\)\n\n.*?(?=\n## Definition of Done)",
            text,
            re.DOTALL,
        )
        if not old_block:
            print("no insertion anchor in spec-test-matrix.md", file=sys.stderr)
            return 1
        new_text = text[: old_block.start()] + generated + "\n\n" + text[old_block.end() :]

    SPEC.write_text(new_text, encoding="utf-8")
    print(f"updated {SPEC.relative_to(REPO)} ({len(rows)} rows)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
