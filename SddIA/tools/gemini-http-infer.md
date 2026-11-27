---
uuid: "7a8da3ad-4916-4ee3-8407-aa1ecdc7ecba"
name: "gemini-http-infer"
version: "1.1.0"
contract: "tools-contract v1.2.0"
domain_origin: "SddIA"
context: "system-operations"
capabilities:
  - "gemini_http_infer"
  - "llm:infer"
provides:
  - id: "llm:infer"
    contract: "llm.infer"
    version: "1.0.0"
hash_signature: "sha256:68392e22c31186a3f2f2612af6bcf2c1c740666c6b36bfb1372e3249836ee19d"
implementation_path_ref: "SddIA/tools/gemini-http-infer"
---

# gemini-http-infer

POST generateContent (Gemini AI Studio). Request `llm.infer` + legacy. `error_code` tipado. Env GEMINI_API_KEY + SDDIA_GEMINI_API_BASE_URL opcional. Lab: SDDIA_LAB_MOCK_OUTBOUND / SDDIA_LAB_MOCK_GEMINI_URL. Sobre capsule-json-io 2.0. Sin Vertex. Sin provides llm:interact. Provides llm:infer.
