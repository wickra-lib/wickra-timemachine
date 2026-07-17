// A runnable Node.js example: load a small recorded feed and reconstruct the
// market snapshot at a past timestamp.
//
//   npm install
//   node examples/node/seek.js
//
// Every language example loads the same feed and seeks the same timestamp, and
// they all print the same summary.
"use strict";

const { TimeMachine } = require("wickra-timemachine");

const FEED = [
  { ts: 10, symbol: "SYM", feed: { kind: "market", type: "trade", symbol: { base: "AAA", quote: "USDT" }, price: "100", quantity: "1", aggressor: "Buy", timestamp: 10 } },
  { ts: 20, symbol: "SYM", feed: { kind: "market", type: "trade", symbol: { base: "AAA", quote: "USDT" }, price: "110", quantity: "2", aggressor: "Sell", timestamp: 20 } },
].map((l) => JSON.stringify(l)).join("\n");

const tm = new TimeMachine("{}");
tm.command(JSON.stringify({ cmd: "load", data: FEED }));
const snapshot = JSON.parse(tm.command(JSON.stringify({ cmd: "seek", ts: 20 })));

console.log(`wickra-timemachine ${tm.version()}`);
console.log(`snapshot ts: ${snapshot.ts}`);
console.log(`symbols: ${Object.keys(snapshot.symbols).length}`);
console.log(`SYM last: ${snapshot.symbols.SYM.last}`);
