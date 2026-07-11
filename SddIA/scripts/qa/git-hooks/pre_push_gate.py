#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Puerta pre-push Ola B: guarda main (O3), idempotencia PR (O1), delivery-close-cycle."""

from __future__ import annotations

import sys

from hook_common import (
    MAIN_GUARD_MSG,
    in_delivery_close_cycle,
    is_delete_push,
    is_main_ref,
    parse_pre_push_stdin,
    ref_to_branch,
    resolve_persist_ref,
    should_skip_pre_push_present,
    skip_hooks,
    invoke_process,
    branch_slug,
)


def main() -> int:
    if skip_hooks():
        print("SddIA pre-push: SKIPPED (SDDIA_SKIP_HOOKS=1)", file=sys.stderr)
        return 0

    if in_delivery_close_cycle():
        print("SddIA pre-push: SKIPPED (delivery-close-cycle guard)", file=sys.stderr)
        return 0

    stdin_text = sys.stdin.read()
    refs = parse_pre_push_stdin(stdin_text)
    if not refs:
        return 0

    branches_to_present: list[str] = []

    for ref in refs:
        if is_delete_push(ref["remote_sha"]):
            continue
        local_ref = ref["local_ref"]
        if is_main_ref(local_ref):
            print(MAIN_GUARD_MSG, file=sys.stderr)
            return 1
        branch = ref_to_branch(local_ref)
        if not branch or branch == "main":
            print(MAIN_GUARD_MSG, file=sys.stderr)
            return 1
        if should_skip_pre_push_present(branch):
            continue
        branches_to_present.append(branch)

    if not branches_to_present:
        return 0

    exit_code = 0
    for branch in branches_to_present:
        qa_payload = {
            "event_type": "Local_QA_Requested",
            "blocking": True,
            "emitter_agent": "git-hook-pre-push",
            "payload": {"branch": branch},
        }
        qa_code = invoke_process("route-domain-event", qa_payload)
        if qa_code != 0:
            print(
                f"SddIA pre-push: BLOCKED — Local_QA_Requested failed for {branch}",
                file=sys.stderr,
            )
            exit_code = qa_code
            continue

        persist_ref = resolve_persist_ref(branch)
        slug = branch_slug(branch)
        payload: dict = {
            "source_process": "git-hook-pre-push",
            "branch_name": branch,
            "pr_title": f"feat: {slug or branch}",
            "pr_body": "Presentación automática vía hook pre-push (PBI-005 Ola B).",
            "target_branch": "main",
        }
        if persist_ref:
            payload["persist_ref"] = persist_ref
        else:
            payload["persist_ref"] = None

        code = invoke_process("delivery-close-cycle", payload)
        if code != 0:
            print(f"SddIA pre-push: BLOCKED — delivery-close-cycle failed for {branch}", file=sys.stderr)
            exit_code = code

    return exit_code


if __name__ == "__main__":
    sys.exit(main())
