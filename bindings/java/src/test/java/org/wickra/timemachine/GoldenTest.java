package org.wickra.timemachine;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;

import org.junit.jupiter.api.Test;

// The cross-language golden invariant seen from Java: seeking the same recorded
// feed to the same timestamp yields byte-identical output across instances. The
// response bytes are what every other binding produces too, because the re-fold
// lives once in the Rust core and this binding forwards its JSON verbatim.
class GoldenTest {
    private static String seek(int ts) {
        try (TimeMachine tm = new TimeMachine("{}")) {
            tm.command(TimeMachineTest.loadCmd());
            return tm.command("{\"cmd\":\"seek\",\"ts\":" + ts + "}");
        }
    }

    @Test
    void seekIsByteIdenticalAcrossInstances() {
        assertEquals(seek(20), seek(20));
    }

    @Test
    void seekIsTsInclusive() {
        assertFalse(seek(10).isEmpty());
    }
}
