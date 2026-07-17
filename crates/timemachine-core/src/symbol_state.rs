//! Per-symbol folded state: a local order book, a bounded tape, a footprint, the
//! latest funding, and a scalar indicator set — all advanced O(1) per event.
//!
//! The building blocks (`BookState`, `TapeRing`, `Footprint`) mirror the proven
//! shapes in `wickra-terminal`'s `terminal-core`.

use std::collections::{BTreeMap, VecDeque};

use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use wickra_exchange_core::{BookDelta, BookLevel, Event, OrderBookSnapshot, OrderSide, TradePrint};

use crate::event::Feed;
use crate::indicator_set::IndicatorSet;
use crate::snapshot::{
    round_to, BookLevel as SnapLevel, BookSnapshot, FundingSnapshot, SymbolSnapshot, TradeRecord,
};
use crate::spec::TimelineSpec;

fn dec_f64(d: Decimal) -> f64 {
    d.to_f64().unwrap_or(0.0)
}

/// A locally maintained L2 order book: price → resting quantity per side.
#[derive(Debug, Default, Clone)]
pub struct BookState {
    bids: BTreeMap<Decimal, Decimal>,
    asks: BTreeMap<Decimal, Decimal>,
}

impl BookState {
    /// Replace the book with a full snapshot.
    pub fn apply_snapshot(&mut self, snap: &OrderBookSnapshot) {
        self.bids.clear();
        self.asks.clear();
        for level in &snap.bids {
            self.bids.insert(level.price, level.quantity);
        }
        for level in &snap.asks {
            self.asks.insert(level.price, level.quantity);
        }
    }

    /// Apply an incremental diff: a zero quantity removes the level.
    pub fn apply_delta(&mut self, delta: &BookDelta) {
        apply_levels(&mut self.bids, &delta.bids);
        apply_levels(&mut self.asks, &delta.asks);
    }

    /// The bid/ask spread, or `None` if either side is empty.
    #[must_use]
    pub fn spread(&self) -> Option<Decimal> {
        match (self.bids.keys().next_back(), self.asks.keys().next()) {
            (Some(bid), Some(ask)) => Some(*ask - *bid),
            _ => None,
        }
    }

    /// The top `n` bid levels, best (highest) first.
    #[must_use]
    pub fn top_bids(&self, n: usize) -> Vec<(Decimal, Decimal)> {
        self.bids
            .iter()
            .rev()
            .take(n)
            .map(|(p, q)| (*p, *q))
            .collect()
    }

    /// The top `n` ask levels, best (lowest) first.
    #[must_use]
    pub fn top_asks(&self, n: usize) -> Vec<(Decimal, Decimal)> {
        self.asks.iter().take(n).map(|(p, q)| (*p, *q)).collect()
    }
}

fn apply_levels(side: &mut BTreeMap<Decimal, Decimal>, changes: &[BookLevel]) {
    for level in changes {
        if level.quantity.is_zero() {
            side.remove(&level.price);
        } else {
            side.insert(level.price, level.quantity);
        }
    }
}

/// A bounded ring of the most recent trade prints (newest at the back).
#[derive(Debug, Clone)]
pub struct TapeRing {
    prints: VecDeque<TradePrint>,
    cap: usize,
}

impl TapeRing {
    /// A ring holding at most `cap` prints.
    #[must_use]
    pub fn new(cap: usize) -> Self {
        Self {
            prints: VecDeque::with_capacity(cap),
            cap,
        }
    }

    /// Push a print, evicting the oldest once the cap is exceeded. O(1).
    pub fn push(&mut self, print: TradePrint) {
        if self.prints.len() == self.cap {
            self.prints.pop_front();
        }
        self.prints.push_back(print);
    }

    /// The most recent `n` prints, newest first.
    #[must_use]
    pub fn recent(&self, n: usize) -> Vec<TradePrint> {
        self.prints.iter().rev().take(n).cloned().collect()
    }
}

/// Volume traded at each price, split by aggressor side.
#[derive(Debug, Default, Clone)]
pub struct Footprint {
    levels: BTreeMap<Decimal, (Decimal, Decimal)>,
}

impl Footprint {
    /// Add a print's quantity to the (buy, sell) volume at its price. Saturating
    /// on `Decimal` overflow (only reachable with adversarial input).
    pub fn add(&mut self, print: &TradePrint) {
        let entry = self.levels.entry(print.price).or_default();
        let side = match print.aggressor {
            OrderSide::Buy => &mut entry.0,
            OrderSide::Sell => &mut entry.1,
        };
        *side = side.checked_add(print.quantity).unwrap_or(*side);
    }

    /// The top `n` price levels, highest price first, as `(price, buy, sell)`.
    #[must_use]
    pub fn top(&self, n: usize) -> Vec<(Decimal, Decimal, Decimal)> {
        self.levels
            .iter()
            .rev()
            .take(n)
            .map(|(price, &(buy, sell))| (*price, buy, sell))
            .collect()
    }
}

/// All folded state for a single market.
pub struct SymbolState {
    book: BookState,
    tape: TapeRing,
    footprint: Footprint,
    funding: Option<FundingSnapshot>,
    indicators: IndicatorSet,
    last: Decimal,
}

impl SymbolState {
    /// A fresh state sized by the spec (empty book/tape/footprint, no funding).
    ///
    /// # Errors
    /// Propagates [`IndicatorSet::new`] failures.
    pub fn new(spec: &TimelineSpec) -> crate::error::Result<Self> {
        Ok(Self {
            book: BookState::default(),
            tape: TapeRing::new(spec.tape_cap),
            footprint: Footprint::default(),
            funding: None,
            indicators: IndicatorSet::new(&spec.indicators)?,
            last: Decimal::ZERO,
        })
    }

    /// Fold one feed payload into the state in O(1).
    pub fn fold(&mut self, feed: &Feed) {
        match feed {
            Feed::Market(Event::Trade(print)) => {
                self.last = print.price;
                self.tape.push(print.clone());
                self.footprint.add(print);
                self.indicators.update(dec_f64(print.price));
            }
            Feed::Market(Event::Ticker(ticker)) => self.last = ticker.last,
            Feed::Market(Event::BookSnapshot(snap)) => self.book.apply_snapshot(snap),
            Feed::Market(Event::BookDelta(delta)) => self.book.apply_delta(delta),
            Feed::Funding { rate, mark_price } => {
                self.funding = Some(FundingSnapshot {
                    rate: round_to(*rate),
                    mark_price: round_to(*mark_price),
                });
            }
            // Account and lifecycle events carry no per-symbol market state.
            Feed::Market(_) => {}
        }
    }

    /// Materialize the folded state into a serializable snapshot.
    #[must_use]
    pub fn snapshot(&self, spec: &TimelineSpec) -> SymbolSnapshot {
        let level = |(p, q): &(Decimal, Decimal)| SnapLevel {
            price: round_to(dec_f64(*p)),
            qty: round_to(dec_f64(*q)),
        };
        let book = BookSnapshot {
            bids: self
                .book
                .top_bids(spec.book_depth)
                .iter()
                .map(level)
                .collect(),
            asks: self
                .book
                .top_asks(spec.book_depth)
                .iter()
                .map(level)
                .collect(),
            spread: self.book.spread().map(|d| round_to(dec_f64(d))),
        };
        let tape = self
            .tape
            .recent(spec.tape_cap)
            .iter()
            .map(|p| TradeRecord {
                price: round_to(dec_f64(p.price)),
                qty: round_to(dec_f64(p.quantity)),
                side: match p.aggressor {
                    OrderSide::Buy => "buy".to_string(),
                    OrderSide::Sell => "sell".to_string(),
                },
                ts: p.timestamp,
            })
            .collect();
        let footprint = self
            .footprint
            .top(spec.book_depth)
            .iter()
            .map(|(p, b, s)| {
                (
                    round_to(dec_f64(*p)),
                    round_to(dec_f64(*b)),
                    round_to(dec_f64(*s)),
                )
            })
            .collect();
        let indicators = self
            .indicators
            .values()
            .into_iter()
            .map(|(k, v)| (k, v.map(round_to)))
            .collect();
        SymbolSnapshot {
            last: round_to(dec_f64(self.last)),
            book,
            tape,
            footprint,
            funding: self.funding.clone(),
            indicators,
        }
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::{BookState, Footprint, SymbolState, TapeRing};
    use crate::event::Feed;
    use crate::spec::TimelineSpec;
    use rust_decimal_macros::dec;
    use wickra_exchange_core::{
        BookDelta, BookLevel, Event, OrderBookSnapshot, OrderSide, Symbol, TradePrint,
    };

    fn trade(price: rust_decimal::Decimal, side: OrderSide) -> Feed {
        Feed::Market(Event::Trade(TradePrint {
            symbol: Symbol::new("BTC", "USDT"),
            price,
            quantity: dec!(2),
            aggressor: side,
            timestamp: 0,
        }))
    }

    #[test]
    fn fold_trade_updates_last_tape_footprint() {
        let spec = TimelineSpec::default();
        let mut st = SymbolState::new(&spec).unwrap();
        st.fold(&trade(dec!(100), OrderSide::Buy));
        st.fold(&trade(dec!(101), OrderSide::Sell));
        let snap = st.snapshot(&spec);
        assert!((snap.last - 101.0).abs() < 1e-9);
        assert_eq!(snap.tape.len(), 2);
        assert_eq!(snap.tape[0].price, 101.0);
    }

    #[test]
    fn book_delta_zero_qty_removes_level() {
        let mut book = BookState::default();
        book.apply_snapshot(&OrderBookSnapshot {
            symbol: Symbol::new("BTC", "USDT"),
            last_update_id: 1,
            bids: vec![
                BookLevel::new(dec!(100), dec!(1)),
                BookLevel::new(dec!(99), dec!(2)),
            ],
            asks: vec![BookLevel::new(dec!(101), dec!(1))],
        });
        assert_eq!(book.spread(), Some(dec!(1)));
        book.apply_delta(&BookDelta {
            symbol: Symbol::new("BTC", "USDT"),
            first_update_id: 2,
            final_update_id: 2,
            bids: vec![BookLevel::new(dec!(100), dec!(0))],
            asks: vec![],
        });
        assert_eq!(book.top_bids(1), vec![(dec!(99), dec!(2))]);
    }

    #[test]
    fn tape_ring_respects_cap() {
        let mut ring = TapeRing::new(2);
        for i in 0..4 {
            ring.push(TradePrint {
                symbol: Symbol::new("BTC", "USDT"),
                price: rust_decimal::Decimal::from(i),
                quantity: dec!(1),
                aggressor: OrderSide::Buy,
                timestamp: i,
            });
        }
        assert_eq!(ring.recent(9).len(), 2);
        assert_eq!(ring.recent(1)[0].price, dec!(3));
    }

    #[test]
    fn funding_folds_into_snapshot() {
        let spec = TimelineSpec::default();
        let mut st = SymbolState::new(&spec).unwrap();
        st.fold(&Feed::Funding {
            rate: 0.0001,
            mark_price: 100.5,
        });
        let snap = st.snapshot(&spec);
        let funding = snap.funding.unwrap();
        assert!((funding.rate - 0.0001).abs() < 1e-9);
        assert!((funding.mark_price - 100.5).abs() < 1e-9);
    }

    #[test]
    fn footprint_splits_by_side() {
        let mut fp = Footprint::default();
        if let Feed::Market(Event::Trade(p)) = trade(dec!(100), OrderSide::Buy) {
            fp.add(&p);
        }
        assert_eq!(fp.top(1)[0], (dec!(100), dec!(2), dec!(0)));
    }
}
