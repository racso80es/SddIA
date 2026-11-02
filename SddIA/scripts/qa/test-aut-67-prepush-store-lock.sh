#!/usr/bin/env bash
# AUT-67: reintento pre-push ante StoreError::Locked; mensaje STORE_LOCK_CONTENTION.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
# shellcheck source=/dev/null
source "$ROOT/SddIA/scripts/qa/git-hooks/hook_common.sh"

fail() { echo "FAIL: $*" >&2; exit 1; }

REPO="$ROOT"
tmpdir=$(mktemp -d)
trap 'rm -rf "$tmpdir"' EXIT

lock_err='StoreError::Locked: Database already open. Cannot acquire lock.'
echo "$lock_err" > "$tmpdir/lock.stderr"
hook_stderr_is_store_lock_contentions "$tmpdir/lock.stderr" || fail "lock stderr must classify"
echo "not a lock" > "$tmpdir/other.stderr"
hook_stderr_is_store_lock_contentions "$tmpdir/other.stderr" && fail "non-lock must not classify"

mock_bin="$tmpdir/mock-execute-process"
state="$tmpdir/calls"
echo 0 > "$state"
cat > "$mock_bin" <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail
n=$(cat "$MOCK_STATE_FILE")
echo $((n + 1)) > "$MOCK_STATE_FILE"
if [[ "$n" -lt 1 ]]; then
  echo "StoreError::Locked: Database already open. Cannot acquire lock." >&2
  exit 1
fi
printf '{"success":true}\n'
exit 0
MOCK
chmod +x "$mock_bin"
export MOCK_STATE_FILE="$state"
export SDDIA_EXECUTE_PROCESS_BIN="$mock_bin"
export SDDIA_PREPUSH_STORE_LOCK_RETRY_MS=5000
export SDDIA_PREPUSH_STORE_LOCK_RETRY_SLEEP_MS=50

payload='{"event_type":"Local_QA_Requested","blocking":true,"emitter_agent":"lab","payload":{"branch":"feat/lab","qa_profile":"full"}}'
invoke_local_qa_route_domain_event "$payload" || fail "retry after lock should succeed"
[[ "$(cat "$state")" -ge 2 ]] || fail "expected at least two invoke attempts"

echo 0 > "$state"
cat > "$mock_bin" <<'MOCK2'
#!/usr/bin/env bash
echo "StoreError::Locked: Database already open. Cannot acquire lock." >&2
exit 1
MOCK2
chmod +x "$mock_bin"
export SDDIA_PREPUSH_STORE_LOCK_RETRY_MS=80
export SDDIA_PREPUSH_STORE_LOCK_RETRY_SLEEP_MS=40
set +e
out=$(invoke_local_qa_route_domain_event "$payload" 2>&1)
rc=$?
set -e
[[ "$rc" -eq 2 ]] || fail "expected exit 2 on budget exhausted, got $rc"
echo "$out" | grep -q 'STORE_LOCK_CONTENTION' || fail "missing STORE_LOCK_CONTENTION in: $out"

grep -q 'DEFAULT_OPEN_BUDGET_MS: u64 = 10_000' "$ROOT/SddIA/sddia-event-store/src/lib.rs" \
  || fail "DEFAULT_OPEN_BUDGET_MS changed unexpectedly"

echo "OK test-aut-67-prepush-store-lock"
