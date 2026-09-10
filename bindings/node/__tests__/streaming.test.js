"use strict";

// `play` streams frames; `state_at` re-folds each instant independently.
//
// `timemachine-core` proves the two agree in Rust, but that says nothing about the
// boundary this binding crosses: a binding that dropped a frame from the played
// array, or mis-serialised one, would hand back a sequence that looks plausible and
// is not the engine's.
//
// So the same instants are asked for both ways through this binding and the two
// must be identical. That equality is the whole claim of the engine -- a seek is a
// deterministic re-fold, not an interpolation -- so a binding that breaks it breaks
// the product, not a detail.

const { test } = require("node:test");
const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const { TimeMachine } = require("../index.js");

const GOLDEN = path.resolve(__dirname, "..", "..", "..", "golden");
const SPEC_PATH = path.join(GOLDEN, "specs", "mini.json");
const HAVE_GOLDEN = fs.existsSync(SPEC_PATH);

const FROM = 1700000000;
const TO = 1700000400;
const STEP = 100;

function machine() {
  const spec = fs.readFileSync(SPEC_PATH, "utf8");
  const feed = fs.readFileSync(path.join(GOLDEN, "data", "mini", "events.jsonl"), "utf8");
  const tm = new TimeMachine(spec);
  tm.command(JSON.stringify({ cmd: "load", data: feed }));
  return tm;
}

test("play equals repeated state_at", { skip: !HAVE_GOLDEN }, () => {
  const tm = machine();
  const played = JSON.parse(
    tm.command(JSON.stringify({ cmd: "play", from: FROM, to: TO, step: STEP })),
  );
  const oneByOne = [];
  for (let ts = FROM; ts <= TO; ts += STEP) {
    oneByOne.push(JSON.parse(tm.command(JSON.stringify({ cmd: "state_at", ts }))));
  }
  assert.deepStrictEqual(played, oneByOne);
  assert.strictEqual(played.length, 5);
});

test("a re-seek to the same instant is byte-identical", { skip: !HAVE_GOLDEN }, () => {
  const tm = machine();
  const cmd = JSON.stringify({ cmd: "state_at", ts: TO });
  const first = tm.command(cmd);
  // Seek somewhere else and back: a re-fold must not depend on where it came
  // from, which is what makes scrubbing a timeline meaningful.
  tm.command(JSON.stringify({ cmd: "state_at", ts: FROM }));
  assert.strictEqual(tm.command(cmd), first);
});
