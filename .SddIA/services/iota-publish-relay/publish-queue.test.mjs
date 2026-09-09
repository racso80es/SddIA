import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { createSerialQueue } from "./publish-queue.mjs";

describe("createSerialQueue", () => {
  it("no solapa dos trabajos: el segundo arranca tras el primero", async () => {
    const enqueue = createSerialQueue();
    const order = [];
    let firstEntered = false;

    const p1 = enqueue(async () => {
      firstEntered = true;
      order.push("a-start");
      await new Promise((r) => setTimeout(r, 30));
      order.push("a-end");
      return "a";
    });
    const p2 = enqueue(async () => {
      assert.equal(firstEntered, true);
      assert.equal(order.includes("a-end"), true);
      order.push("b-start");
      order.push("b-end");
      return "b";
    });

    const [ra, rb] = await Promise.all([p1, p2]);
    assert.equal(ra, "a");
    assert.equal(rb, "b");
    assert.deepEqual(order, ["a-start", "a-end", "b-start", "b-end"]);
  });

  it("el fallo del primero no impide el segundo", async () => {
    const enqueue = createSerialQueue();
    const p1 = enqueue(async () => {
      throw new Error("boom");
    });
    const p2 = enqueue(async () => "ok");
    await assert.rejects(p1, /boom/);
    assert.equal(await p2, "ok");
  });
});
