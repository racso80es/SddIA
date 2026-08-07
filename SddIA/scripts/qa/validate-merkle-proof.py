#!/usr/bin/env python3
import sys
import json
import hashlib

def sha256_hash(data: bytes) -> bytes:
    return hashlib.sha256(data).digest()

def parent_hash(left: bytes, right: bytes) -> bytes:
    return sha256_hash(left + right)

def verify_merkle_proof(root_hash: bytes, leaf_hash: bytes, proof_hashes: list, index: int, total_leaves: int) -> bool:
    current_hash = leaf_hash
    proof_idx = 0
    level_index = index
    level_size = total_leaves

    while level_size > 1:
        if level_index % 2 == 0:
            if level_index + 1 < level_size:
                if proof_idx >= len(proof_hashes):
                    return False
                sibling = proof_hashes[proof_idx]
                proof_idx += 1
                current_hash = parent_hash(current_hash, sibling)
        else:
            if proof_idx >= len(proof_hashes):
                return False
            sibling = proof_hashes[proof_idx]
            proof_idx += 1
            current_hash = parent_hash(sibling, current_hash)

        level_index //= 2
        level_size = (level_size + 1) // 2

    return current_hash == root_hash

def main():
    if len(sys.argv) != 3:
        print("Usage: ./validate-merkle-proof.py <event_payload_path> <proof_path>", file=sys.stderr)
        sys.exit(1)

    event_path = sys.argv[1]
    proof_path = sys.argv[2]

    try:
        with open(event_path, "r") as f:
            event_data = json.load(f)
            payload_str = json.dumps(event_data, separators=(',', ':'))
            payload_hash = sha256_hash(payload_str.encode("utf-8"))
    except Exception as e:
        print(f"Error reading/hashing event payload: {e}", file=sys.stderr)
        sys.exit(1)

    try:
        with open(proof_path, "r") as f:
            proof_data = json.load(f)
            index = proof_data["index"]
            total_leaves = proof_data["total_leaves"]
            proof_hex = proof_data["proof"]
            proof_bytes = bytes.fromhex(proof_hex)
            merkle_root = bytes.fromhex(proof_data["merkle_root"])

            proof_hashes = [proof_bytes[i:i+32] for i in range(0, len(proof_bytes), 32)]

            is_valid = verify_merkle_proof(merkle_root, payload_hash, proof_hashes, index, total_leaves)

            if is_valid:
                print("Integrity Verified: Merkle proof matches the root hash.")
                sys.exit(0)
            else:
                print("Tampering Detected: Hash mismatch.", file=sys.stderr)
                sys.exit(1)

    except Exception as e:
        print(f"Error reading proof file or verifying: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()
