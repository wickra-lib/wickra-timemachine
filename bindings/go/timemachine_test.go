package wickra

import (
	"encoding/json"
	"fmt"
	"math"
	"strings"
	"testing"
)

// feed builds a deterministic two-symbol JSONL feed: a rising then falling
// trade on SYM plus a funding tick, so seek reconstructs a known snapshot.
func feed() string {
	var b strings.Builder
	lines := []string{
		`{"ts":10,"symbol":"SYM","feed":{"kind":"market","type":"trade","symbol":{"base":"AAA","quote":"USDT"},"price":"100","quantity":"1","aggressor":"Buy","timestamp":10}}`,
		`{"ts":20,"symbol":"SYM","feed":{"kind":"market","type":"trade","symbol":{"base":"AAA","quote":"USDT"},"price":"105","quantity":"1","aggressor":"Sell","timestamp":20}}`,
	}
	for i, l := range lines {
		if i > 0 {
			b.WriteByte('\n')
		}
		b.WriteString(l)
	}
	return b.String()
}

// loadCmd builds a load command over the feed.
func loadCmd() string {
	return fmt.Sprintf(`{"cmd":"load","data":%s}`, mustJSONString(feed()))
}

func mustJSONString(s string) string {
	out, err := json.Marshal(s)
	if err != nil {
		panic(err)
	}
	return string(out)
}

func seek(t *testing.T, tm *TimeMachine, ts int) string {
	t.Helper()
	resp, err := tm.Command(fmt.Sprintf(`{"cmd":"seek","ts":%d}`, ts))
	if err != nil {
		t.Fatal(err)
	}
	return resp
}

func TestVersion(t *testing.T) {
	if Version() == "" {
		t.Fatal("empty version")
	}
}

func TestSeekReconstructsSnapshot(t *testing.T) {
	tm, err := New("{}")
	if err != nil {
		t.Fatal(err)
	}
	defer tm.Close()
	if _, err := tm.Command(loadCmd()); err != nil {
		t.Fatal(err)
	}
	var snap struct {
		Ts      int64 `json:"ts"`
		Symbols map[string]struct {
			Last float64 `json:"last"`
		} `json:"symbols"`
	}
	if err := json.Unmarshal([]byte(seek(t, tm, 20)), &snap); err != nil {
		t.Fatal(err)
	}
	if snap.Ts != 20 {
		t.Fatalf("expected ts 20, got %d", snap.Ts)
	}
	if math.Abs(snap.Symbols["SYM"].Last-105.0) > 1e-9 {
		t.Fatalf("expected last 105, got %g", snap.Symbols["SYM"].Last)
	}
}

func TestInvalidSpecIsError(t *testing.T) {
	if _, err := New("{ not valid json"); err == nil {
		t.Fatal("expected an error for an invalid spec")
	}
}
