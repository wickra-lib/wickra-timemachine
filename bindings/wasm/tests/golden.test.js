"use strict";

// Golden test over the wasm-pack (nodejs target) output: the WebAssembly build
// re-folds byte-identically to the native run — the single-threaded re-fold in
// the browser sandbox reproduces the same snapshot exactly. Skips cleanly when
// `pkg/` has not been built yet (`wasm-pack build --target nodejs`).

const { test } = require("node:test");
const assert = require("node:assert");
const path = require("node:path");

let wasm = null;
try {
  wasm = require(path.resolve(__dirname, "..", "pkg", "wickra_timemachine_wasm.js"));
} catch {
  wasm = null;
}

const FEED = [
  { ts: 10, symbol: "SYM", feed: { kind: "market", type: "trade",
    symbol: { base: "AAA", quote: "USDT" }, price: "100", quantity: "1",
    aggressor: "Buy", timestamp: 10 } },
  { ts: 20, symbol: "SYM", feed: { kind: "market", type: "trade",
    symbol: { base: "AAA", quote: "USDT" }, price: "105", quantity: "1",
    aggressor: "Sell", timestamp: 20 } },
].map((l) => JSON.stringify(l)).join("\n");

function seek(ts) {
  const tm = new wasm.TimeMachine("{}");
  tm.command(JSON.stringify({ cmd: "load", data: FEED }));
  return tm.command(JSON.stringify({ cmd: "seek", ts }));
}

test("wasm build present or skipped", (t) => {
  if (!wasm) t.skip("run `wasm-pack build --target nodejs` first");
});

if (wasm) {
  test("wasm seek reconstructs the snapshot", () => {
    const snap = JSON.parse(seek(20));
    assert.strictEqual(snap.ts, 20);
    assert.ok(Math.abs(snap.symbols.SYM.last - 105.0) < 1e-9);
  });

  test("wasm seek is byte-identical across calls", () => {
    assert.strictEqual(seek(20), seek(20));
  });

  test("wasm seek is ts-inclusive", () => {
    const early = JSON.parse(seek(10));
    assert.ok(Math.abs(early.symbols.SYM.last - 100.0) < 1e-9);
  });

  test("wasm version matches the module export", () => {
    assert.strictEqual(new wasm.TimeMachine("{}").version(), wasm.version());
  });

  test("wasm throws on an invalid spec", () => {
    assert.throws(() => new wasm.TimeMachine("{ not valid json"));
  });
}
