"use strict";

// Determinism: seeking the same recorded feed to the same timestamp yields the
// byte-identical snapshot string. The full cross-language golden lands with the
// corpus in P-TM-5; here we pin the byte-reproducibility every binding preserves
// by forwarding the command string verbatim.

const { test } = require("node:test");
const assert = require("node:assert");
const { TimeMachine } = require("../index.js");

const FEED = [
  {
    ts: 10,
    symbol: "SYM",
    feed: {
      kind: "market",
      type: "trade",
      symbol: { base: "AAA", quote: "USDT" },
      price: "100",
      quantity: "1",
      aggressor: "Buy",
      timestamp: 10,
    },
  },
  {
    ts: 20,
    symbol: "SYM",
    feed: {
      kind: "market",
      type: "trade",
      symbol: { base: "AAA", quote: "USDT" },
      price: "105",
      quantity: "1",
      aggressor: "Sell",
      timestamp: 20,
    },
  },
]
  .map((l) => JSON.stringify(l))
  .join("\n");

function seek(ts) {
  const tm = new TimeMachine("{}");
  tm.command(JSON.stringify({ cmd: "load", data: FEED }));
  return tm.command(JSON.stringify({ cmd: "seek", ts }));
}

test("the same seek yields the byte-identical snapshot", () => {
  assert.strictEqual(seek(20), seek(20));
});

test("seek is timestamp-inclusive", () => {
  assert.ok(Math.abs(JSON.parse(seek(10)).symbols.SYM.last - 100.0) < 1e-9);
  assert.ok(Math.abs(JSON.parse(seek(20)).symbols.SYM.last - 105.0) < 1e-9);
});
