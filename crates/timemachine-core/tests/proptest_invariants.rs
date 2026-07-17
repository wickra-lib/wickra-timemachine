//! Property tests over random recorded feeds: seeking never panics, the folded
//! book stays sorted and depth-capped, the tape stays within its cap, and a seek
//! is deterministic across independent instances.

use proptest::prelude::*;
use timemachine_core::TimeMachine;

const BOOK_DEPTH: usize = 5;
const TAPE_CAP: usize = 8;
const SPEC: &str = r#"{"book_depth":5,"tape_cap":8,"indicators":[{"name":"Sma","params":[3]}],"snapshot_interval":4}"#;

// A trade or a book delta on a single symbol, at a monotonically increasing ts.
#[derive(Debug, Clone)]
enum Ev {
    Trade {
        price: u32,
        qty: u32,
        buy: bool,
    },
    Delta {
        bid_price: u32,
        bid_qty: u32,
        ask_price: u32,
        ask_qty: u32,
    },
}

fn ev_strategy() -> impl Strategy<Value = Ev> {
    prop_oneof![
        (1u32..1000, 1u32..100, any::<bool>()).prop_map(|(price, qty, buy)| Ev::Trade {
            price,
            qty,
            buy
        }),
        (1u32..500, 0u32..100, 500u32..1000, 0u32..100).prop_map(|(bp, bq, ap, aq)| Ev::Delta {
            bid_price: bp,
            bid_qty: bq,
            ask_price: ap,
            ask_qty: aq
        }),
    ]
}

fn build_feed(events: &[Ev]) -> String {
    // Open with a multi-level snapshot so book invariants have something to fold.
    let mut lines = vec![String::from(
        r#"{"ts":0,"symbol":"SYM","feed":{"kind":"market","type":"book_snapshot","symbol":{"base":"SYM","quote":"USDT"},"last_update_id":1,"bids":[{"price":"90","quantity":"2"},{"price":"80","quantity":"1"}],"asks":[{"price":"110","quantity":"2"},{"price":"120","quantity":"1"}]}}"#,
    )];
    for (i, ev) in events.iter().enumerate() {
        let ts = (i64::try_from(i).unwrap() + 1) * 10;
        let line = match ev {
            Ev::Trade { price, qty, buy } => format!(
                r#"{{"ts":{ts},"symbol":"SYM","feed":{{"kind":"market","type":"trade","symbol":{{"base":"SYM","quote":"USDT"}},"price":"{price}","quantity":"{q}","aggressor":"{side}","timestamp":{ts}}}}}"#,
                q = (*qty).max(1),
                side = if *buy { "Buy" } else { "Sell" },
            ),
            Ev::Delta {
                bid_price,
                bid_qty,
                ask_price,
                ask_qty,
            } => format!(
                r#"{{"ts":{ts},"symbol":"SYM","feed":{{"kind":"market","type":"book_delta","symbol":{{"base":"SYM","quote":"USDT"}},"first_update_id":2,"final_update_id":2,"bids":[{{"price":"{bid_price}","quantity":"{bid_qty}"}}],"asks":[{{"price":"{ask_price}","quantity":"{ask_qty}"}}]}}}}"#,
            ),
        };
        lines.push(line);
    }
    lines.join("\n")
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(200))]

    #[test]
    fn seek_never_panics_and_preserves_invariants(events in prop::collection::vec(ev_strategy(), 0..40), seek_ts in 0i64..500) {
        let feed = build_feed(&events);
        let mut tm = TimeMachine::new(SPEC).unwrap();
        tm.load(&feed).unwrap();

        let snap = tm.seek(seek_ts).unwrap();
        prop_assert_eq!(snap.ts, seek_ts);

        if let Some(sym) = snap.symbols.get("SYM") {
            // Bids strictly descending by price, asks strictly ascending, both depth-capped.
            prop_assert!(sym.book.bids.len() <= BOOK_DEPTH);
            prop_assert!(sym.book.asks.len() <= BOOK_DEPTH);
            for w in sym.book.bids.windows(2) {
                prop_assert!(w[0].price > w[1].price);
            }
            for w in sym.book.asks.windows(2) {
                prop_assert!(w[0].price < w[1].price);
            }
            prop_assert!(sym.tape.len() <= TAPE_CAP);
        }

        // Idempotent, and deterministic across a fresh instance.
        let again = tm.seek(seek_ts).unwrap();
        prop_assert_eq!(serde_json::to_string(&snap).unwrap(), serde_json::to_string(&again).unwrap());

        let mut tm2 = TimeMachine::new(SPEC).unwrap();
        tm2.load(&feed).unwrap();
        let fresh = tm2.seek(seek_ts).unwrap();
        prop_assert_eq!(serde_json::to_string(&snap).unwrap(), serde_json::to_string(&fresh).unwrap());
    }
}
