package wickra

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

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"testing"
)

const (
	playFrom = 1700000000
	playTo   = 1700000400
	playStep = 100
)

func goldenDir() string {
	return filepath.Join("..", "..", "golden")
}

func playMachine(t *testing.T) *TimeMachine {
	t.Helper()
	spec, err := os.ReadFile(filepath.Join(goldenDir(), "specs", "mini.json"))
	if err != nil {
		t.Skipf("golden fixtures absent: %v", err)
	}
	feed, err := os.ReadFile(filepath.Join(goldenDir(), "data", "mini", "events.jsonl"))
	if err != nil {
		t.Skipf("golden fixtures absent: %v", err)
	}
	tm, err := New(string(spec))
	if err != nil {
		t.Fatalf("new time machine: %v", err)
	}
	payload, err := json.Marshal(map[string]string{"cmd": "load", "data": string(feed)})
	if err != nil {
		t.Fatalf("marshal load: %v", err)
	}
	if _, err := tm.Command(string(payload)); err != nil {
		t.Fatalf("load: %v", err)
	}
	return tm
}

func TestPlayEqualsRepeatedStateAt(t *testing.T) {
	tm := playMachine(t)
	defer tm.Close()

	out, err := tm.Command(fmt.Sprintf(
		`{"cmd":"play","from":%d,"to":%d,"step":%d}`, playFrom, playTo, playStep))
	if err != nil {
		t.Fatalf("play: %v", err)
	}
	var played []json.RawMessage
	if err := json.Unmarshal([]byte(out), &played); err != nil {
		t.Fatalf("parse play: %v", err)
	}

	var oneByOne []json.RawMessage
	for ts := playFrom; ts <= playTo; ts += playStep {
		frame, err := tm.Command(fmt.Sprintf(`{"cmd":"state_at","ts":%d}`, ts))
		if err != nil {
			t.Fatalf("state_at %d: %v", ts, err)
		}
		oneByOne = append(oneByOne, json.RawMessage(frame))
	}

	if len(played) != len(oneByOne) {
		t.Fatalf("frame counts differ: %d vs %d", len(played), len(oneByOne))
	}
	for i := range played {
		if string(played[i]) != string(oneByOne[i]) {
			t.Errorf("frame %d differs\n play: %s\n seek: %s", i, played[i], oneByOne[i])
		}
	}
}

func TestAReSeekToTheSameInstantIsByteIdentical(t *testing.T) {
	tm := playMachine(t)
	defer tm.Close()

	cmd := fmt.Sprintf(`{"cmd":"state_at","ts":%d}`, playTo)
	first, err := tm.Command(cmd)
	if err != nil {
		t.Fatalf("state_at: %v", err)
	}
	// Seek somewhere else and back: a re-fold must not depend on where it came
	// from, which is what makes scrubbing a timeline meaningful.
	if _, err := tm.Command(fmt.Sprintf(`{"cmd":"state_at","ts":%d}`, playFrom)); err != nil {
		t.Fatalf("state_at: %v", err)
	}
	again, err := tm.Command(cmd)
	if err != nil {
		t.Fatalf("state_at: %v", err)
	}
	if again != first {
		t.Errorf("a re-seek to the same instant differed")
	}
}
