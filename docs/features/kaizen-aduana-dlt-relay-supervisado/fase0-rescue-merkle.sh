#!/usr/bin/env bash
# Fase 0 — Rescate Merkle (L-RESCUE). One-shot lab bajo persist_ref.
# Invoca cápsula iota-immutable-publisher; NO reinyecta a pending/.
# Criterio de parada: fallo del lote ⇒ exit ≠ 0 (no acta parcial).
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$REPO_ROOT"
# shellcheck source=SddIA/scripts/common/sddia_shell_lib.sh
source "$REPO_ROOT/SddIA/scripts/common/sddia_shell_lib.sh"
_sddia_load_vault "$REPO_ROOT"

WINDOW_START="${RESCUE_WINDOW_START:-2026-08-25}"
WINDOW_END="${RESCUE_WINDOW_END:-2026-08-27}"
DL_SUB="${REPO_ROOT}/.events/dead-letter/subscribers"
PROOFS="${REPO_ROOT}/.SddIA/proofs"
PROCESSED="${REPO_ROOT}/.events/processed"
ACTA="${PROOFS}/merkle-acta-dlt-backfill-20260827.json"
HEALTH_URL="${IOTA_PUBLISH_RELAY_URL%/v1/publish}/health"

echo "[Fase0] health=${HEALTH_URL}"
if ! curl -sf --max-time 3 "$HEALTH_URL" >/dev/null; then
  echo "[Fase0] BLOCKED: relay /health no responde. Arranque iota-publish-relay antes del rescate." >&2
  exit 2
fi

mkdir -p "$PROOFS" "$PROCESSED"
INVENTORY="$(mktemp)"
PAYLOADS_JSON="$(mktemp)"
UUIDS_JSON="$(mktemp)"
trap 'rm -f "$INVENTORY" "$PAYLOADS_JSON" "$UUIDS_JSON"' EXIT

python3 - "$DL_SUB" "$WINDOW_START" "$WINDOW_END" "$INVENTORY" "$PAYLOADS_JSON" "$UUIDS_JSON" <<'PY'
import json, sys, re
from pathlib import Path
from datetime import datetime

dl, w0, w1, inv_path, pay_path, uuid_path = sys.argv[1:7]
start = datetime.fromisoformat(w0 + "T00:00:00")
end = datetime.fromisoformat(w1 + "T23:59:59")
pat = re.compile(r"^(.+)\.cumulo\.iota-immutable-publisher\.json$")
rows = []
payloads = []
uuids = []
for p in sorted(Path(dl).glob("*.cumulo.iota-immutable-publisher.json")):
    m = pat.match(p.name)
    if not m:
        continue
    try:
        body = json.loads(p.read_text(encoding="utf-8"))
    except Exception:
        continue
    trace = (body.get("error_trace") or body.get("error") or "")
    if "batch-missing-merkle-anchor" not in str(trace) and "batch-anchor-failed" not in str(trace):
        # incluir también la traza opaca histórica
        if "merkle" not in str(trace).lower() and "anchor" not in str(trace).lower():
            continue
    # mtime como proxy de ventana si timestamp ausente
    ts = body.get("timestamp") or body.get("failed_at")
    try:
        if ts:
            dt = datetime.fromisoformat(str(ts).replace("Z", "+00:00")).replace(tzinfo=None)
        else:
            dt = datetime.utcfromtimestamp(p.stat().st_mtime)
    except Exception:
        dt = datetime.utcfromtimestamp(p.stat().st_mtime)
    if dt < start or dt > end:
        continue
    uuid = m.group(1)
    # envelope: preferir event embebido / path
    event = body.get("event") or body.get("original_event") or {}
    payload = event.get("payload")
    if payload is None:
        # dead-letter subscriber a menudo solo referencia uuid — buscar processed/dl raíz
        for cand in [
            Path(dl).parent / f"{uuid}.json",
            Path(dl).parent.parent / "processed" / f"{uuid}.json",
            Path(dl).parent.parent / "dead-letter" / f"{uuid}.json",
        ]:
            if cand.is_file():
                try:
                    ev = json.loads(cand.read_text(encoding="utf-8"))
                    payload = ev.get("payload")
                    event = ev
                    break
                except Exception:
                    pass
    if payload is None:
        continue
    payloads.append(json.dumps(payload, separators=(",", ":"), ensure_ascii=False))
    uuids.append(uuid)
    rows.append({"uuid": uuid, "dl": str(p), "ts": dt.isoformat()})

Path(inv_path).write_text(json.dumps(rows, indent=2), encoding="utf-8")
Path(pay_path).write_text(json.dumps(payloads), encoding="utf-8")
Path(uuid_path).write_text(json.dumps(uuids), encoding="utf-8")
print(f"[Fase0] censo={len(uuids)} (acta=inventario real, no semilla 28)")
if not uuids:
    sys.exit(3)
PY

CENSUS="$(python3 -c "import json; print(len(json.load(open('$UUIDS_JSON'))))")"
echo "[Fase0] inventario=$CENSUS → un lote Merkle"

INPUTS="$(python3 - "$PAYLOADS_JSON" <<'PY'
import json, sys
payloads = json.load(open(sys.argv[1]))
print(json.dumps({
  "action": "publish_immutable_data",
  "network": "testnet",
  "payload": payloads,
}, ensure_ascii=False))
PY
)"

echo "[Fase0] invocando iota-immutable-publisher (lote)..."
OUT="$(mktemp)"
if ! ./sddia-run.sh --tool iota-immutable-publisher --inputs "$INPUTS" >"$OUT" 2>&1; then
  echo "[Fase0] STOP: cápsula falló. Sin acta parcial." >&2
  cat "$OUT" >&2 || true
  exit 4
fi

python3 - "$OUT" "$UUIDS_JSON" "$INVENTORY" "$PROOFS" "$PROCESSED" "$ACTA" "$DL_SUB" <<'PY'
import json, sys, shutil
from pathlib import Path
from datetime import datetime, timezone

out_path, uuids_path, inv_path, proofs, processed, acta_path, dl_sub = sys.argv[1:8]
raw = Path(out_path).read_text(encoding="utf-8")
# acuse CLI: última línea JSON
body = None
for line in reversed(raw.splitlines()):
    line = line.strip()
    if line.startswith("{"):
        try:
            body = json.loads(line)
            break
        except Exception:
            pass
if not body or not body.get("success"):
    print("[Fase0] STOP: success!=true", body, file=sys.stderr)
    sys.exit(4)
result = body.get("result") or body.get("data") or {}
if isinstance(result, dict) and "result" in result:
    result = result["result"]
digest = result.get("transaction_digest")
root = result.get("merkle_root")
proofs_arr = result.get("merkle_proofs") or []
if not digest or digest == "batched-digest":
    print("[Fase0] STOP: digest inválido", digest, file=sys.stderr)
    sys.exit(4)

uuids = json.loads(Path(uuids_path).read_text(encoding="utf-8"))
inv = {r["uuid"]: r for r in json.loads(Path(inv_path).read_text(encoding="utf-8"))}
now = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
proofs_dir = Path(proofs)
processed_dir = Path(processed)
dl = Path(dl_sub)

for i, uuid in enumerate(uuids):
    if i < len(proofs_arr):
        (proofs_dir / f"{uuid}.json").write_text(
            json.dumps(proofs_arr[i], indent=2, ensure_ascii=False) + "\n", encoding="utf-8"
        )
    # localizar envelope y sellar
    candidates = [
        processed_dir / f"{uuid}.json",
        dl.parent / f"{uuid}.json",
        Path(".events/dead-letter") / f"{uuid}.json",
    ]
    for cand in candidates:
        if not cand.is_file():
            continue
        ev = json.loads(cand.read_text(encoding="utf-8"))
        ds = ev.setdefault("delivery_state", {})
        ds["merkle_anchored"] = True
        ds["transaction_digest"] = digest
        if root:
            ds["merkle_root"] = root
        ds["anchored_retroactively"] = True
        ds["anchored_at"] = now
        ds.pop("last_batch_anchor_error", None)
        cand.write_text(json.dumps(ev, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        if cand.parent != processed_dir:
            dest = processed_dir / cand.name
            shutil.move(str(cand), str(dest))
        break
    # mover artefacto subscriber DL → processed/
    sub = dl / f"{uuid}.cumulo.iota-immutable-publisher.json"
    if sub.is_file():
        dest = processed_dir / sub.name
        shutil.move(str(sub), str(dest))

acta = {
    "acta": "merkle-acta-dlt-backfill-20260827",
    "retroactive": True,
    "window": {"start": "2026-08-25", "end": "2026-08-27"},
    "transaction_digest": digest,
    "merkle_root": root,
    "census": uuids,
    "census_count": len(uuids),
    "anchored_at": now,
    "anchored_retroactively": True,
}
Path(acta_path).write_text(json.dumps(acta, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
print(f"[Fase0] OK acta={acta_path} digest={digest} n={len(uuids)}")
PY
