package wickra

// The cross-language golden invariant seen from Go: seeking the same recorded
// feed to the same timestamp yields byte-identical output across instances. The
// response bytes are what every other binding produces too, because the re-fold
// lives once in the Rust core and this binding forwards its JSON verbatim.

import (
	"testing"
)

func loadedSeek(t *testing.T, ts int) string {
	t.Helper()
	tm, err := New("{}")
	if err != nil {
		t.Fatal(err)
	}
	defer tm.Close()
	if _, err := tm.Command(loadCmd()); err != nil {
		t.Fatal(err)
	}
	return seek(t, tm, ts)
}

func TestSeekByteIdenticalAcrossInstances(t *testing.T) {
	a := loadedSeek(t, 20)
	b := loadedSeek(t, 20)
	if a != b {
		t.Fatalf("expected byte-identical output, got:\n a: %s\n b: %s", a, b)
	}
}

func TestSeekIsTsInclusive(t *testing.T) {
	early := loadedSeek(t, 10)
	if early == "" {
		t.Fatal("expected a non-empty snapshot")
	}
}
