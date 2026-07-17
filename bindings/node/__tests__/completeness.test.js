"use strict";

const { test } = require("node:test");
const assert = require("node:assert");
const { TimeMachine } = require("../index.js");

test("the TimeMachine surface exposes command and version", () => {
  const darwin = new TimeMachine("{}");
  assert.strictEqual(typeof darwin.command, "function");
  assert.strictEqual(typeof darwin.version, "function");
});
