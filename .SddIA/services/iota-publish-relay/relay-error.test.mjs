import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { formatPublishFailure, serializeCause } from "./relay-error.mjs";

describe("serializeCause", () => {
  it("null si no hay cause", () => {
    assert.equal(serializeCause(new Error("fetch failed")), null);
  });

  it("extrae code/syscall/errno de TypeError+cause", () => {
    const cause = new Error("getaddrinfo ENOTFOUND api.testnet.iota.cafe");
    cause.code = "ENOTFOUND";
    cause.syscall = "getaddrinfo";
    cause.errno = -3008;
    const err = new TypeError("fetch failed");
    err.cause = cause;
    assert.deepEqual(serializeCause(err), {
      name: "Error",
      message: "getaddrinfo ENOTFOUND api.testnet.iota.cafe",
      code: "ENOTFOUND",
      syscall: "getaddrinfo",
      errno: "-3008",
    });
  });

  it("un nivel de cause anidado", () => {
    const inner = new Error("inner");
    inner.code = "ETIMEDOUT";
    const mid = new Error("mid");
    mid.cause = inner;
    const err = new TypeError("fetch failed");
    err.cause = mid;
    const out = serializeCause(err);
    assert.equal(out.message, "mid");
    assert.equal(out.cause.code, "ETIMEDOUT");
  });
});

describe("formatPublishFailure", () => {
  it("Error plano: error=feedback, sin campo cause", () => {
    const body = formatPublishFailure(new Error("config-missing: IOTA_WALLET_SECRET"));
    assert.equal(body.error, "config-missing: IOTA_WALLET_SECRET");
    assert.equal(body.feedback, body.error);
    assert.equal("cause" in body, false);
  });

  it("fetch failed + ENOTFOUND: sufijo en error/feedback y cause objeto", () => {
    const cause = new Error("getaddrinfo ENOTFOUND fullnode");
    cause.code = "ENOTFOUND";
    cause.syscall = "getaddrinfo";
    const err = new TypeError("fetch failed");
    err.cause = cause;
    const body = formatPublishFailure(err);
    assert.match(body.error, /^fetch failed \| cause:/);
    assert.match(body.error, /ENOTFOUND/);
    assert.equal(body.feedback, body.error);
    assert.equal(body.cause.code, "ENOTFOUND");
  });
});
