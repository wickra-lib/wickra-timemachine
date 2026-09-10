"use strict";

// `play` streams frames; `seek` re-folds each instant independently.
//
// The golden test here proves the WASM build reproduces a blessed seek. This
// proves the two halves agree when a timeline is scrubbed live, which is the
// path a browser consumer actually takes: a binding that dropped a frame from
// the played array would hand back a sequence that looks plausible and is not
// the engine's.
//
// Skips cleanly when `pkg/` has not been built yet
// (`wasm-pack build --target nodejs`).

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
  '{"ts":10,"symbol":"SYM","feed":{"kind":"market","type":"trade","symbol":{"base":"SYM","quote":"USDT"},"price":"100.0","quantity":"1.0","aggressor":"Buy","timestamp":10}}',
  '{"ts":20,"symbol":"SYM","feed":{"kind":"market","type":"trade","symbol":{"base":"SYM","quote":"USDT"},"price":"105.0","quantity":"2.0","aggressor":"Sell","timestamp":20}}',
  '{"ts":30,"symbol":"SYM","feed":{"kind":"market","type":"trade","symbol":{"base":"SYM","quote":"USDT"},"price":"110.0","quantity":"1.5","aggressor":"Buy","timestamp":30}}',
].join("\n");

const FROM = 10;
const TO = 30;
const STEP = 10;

function loaded() {
  const tm = new wasm.TimeMachine("{}");
  tm.command(JSON.stringify({ cmd: "load", data: FEED }));
  return tm;
}

test("play equals repeated seek", { skip: wasm === null }, () => {
  const tm = loaded();
  const played = JSON.parse(
    tm.command(JSON.stringify({ cmd: "play", from: FROM, to: TO, step: STEP })),
  );
  const oneByOne = [];
  for (let ts = FROM; ts <= TO; ts += STEP) {
    oneByOne.push(JSON.parse(tm.command(JSON.stringify({ cmd: "seek", ts }))));
  }
  assert.deepStrictEqual(played, oneByOne);
  assert.strictEqual(played.length, 3);
});

test("a re-seek to the same instant is byte-identical", { skip: wasm === null }, () => {
  const tm = loaded();
  const cmd = JSON.stringify({ cmd: "seek", ts: TO });
  const first = tm.command(cmd);
  // A re-fold must not depend on where the previous seek left off, which is
  // what makes scrubbing a timeline meaningful.
  tm.command(JSON.stringify({ cmd: "seek", ts: FROM }));
  assert.strictEqual(tm.command(cmd), first);
});
