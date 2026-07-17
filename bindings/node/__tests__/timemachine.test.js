"use strict";

const { test } = require("node:test");
const assert = require("node:assert");
const { TimeMachine } = require("../index.js");

function feed() {
  const lines = [
    {
      ts: 10,
      symbol: "BTC-USDT",
      feed: {
        kind: "market",
        type: "trade",
        symbol: { base: "BTC", quote: "USDT" },
        price: "100",
        quantity: "1",
        aggressor: "Buy",
        timestamp: 10,
      },
    },
    {
      ts: 20,
      symbol: "BTC-USDT",
      feed: {
        kind: "market",
        type: "trade",
        symbol: { base: "BTC", quote: "USDT" },
        price: "110",
        quantity: "2",
        aggressor: "Sell",
        timestamp: 20,
      },
    },
    { ts: 20, symbol: "ETH-USDT", feed: { kind: "funding", rate: 0.0002, mark_price: 50.0 } },
  ];
  return lines.map((l) => JSON.stringify(l)).join("\n");
}

function loaded() {
  const tm = new TimeMachine("{}");
  tm.command(JSON.stringify({ cmd: "load", data: feed() }));
  return tm;
}

test("seek reconstructs the snapshot", () => {
  const snap = JSON.parse(loaded().command(JSON.stringify({ cmd: "seek", ts: 20 })));
  assert.strictEqual(snap.ts, 20);
  assert.ok(Math.abs(snap.symbols["BTC-USDT"].last - 110.0) < 1e-9);
  assert.ok(snap.symbols["ETH-USDT"].funding !== null);
});

test("the same seek yields byte-identical output", () => {
  const cmd = JSON.stringify({ cmd: "seek", ts: 20 });
  assert.strictEqual(loaded().command(cmd), loaded().command(cmd));
});

test("an invalid spec throws", () => {
  assert.throws(() => new TimeMachine("{ not valid json"));
});

test("version is a string", () => {
  assert.strictEqual(typeof new TimeMachine("{}").version(), "string");
});

module.exports = { feed };
