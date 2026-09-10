package org.wickra.timemachine;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.Test;

/**
 * {@code play} streams frames; {@code seek} re-folds each instant independently.
 *
 * <p>{@code timemachine-core} proves the two agree in Rust, but that says
 * nothing about the boundary this binding crosses: a binding that dropped a
 * frame from the played array, or mis-serialised one, would hand back a sequence
 * that looks plausible and is not the engine's.
 *
 * <p>That equality is the whole claim of the engine — a seek is a deterministic
 * re-fold, not an interpolation — so a binding that breaks it breaks the
 * product, not a detail.
 */
class StreamingTest {
    private static final long FROM = 10L;
    private static final long TO = 30L;
    private static final long STEP = 10L;

    private static String seek(TimeMachine tm, long ts) {
        return tm.command("{\"cmd\":\"seek\",\"ts\":" + ts + "}");
    }

    @Test
    void playEqualsRepeatedSeek() {
        try (TimeMachine tm = TimeMachineTest.loaded()) {
            String played = tm.command("{\"cmd\":\"play\",\"from\":" + FROM
                    + ",\"to\":" + TO + ",\"step\":" + STEP + "}");

            StringBuilder oneByOne = new StringBuilder("[");
            for (long ts = FROM; ts <= TO; ts += STEP) {
                if (ts > FROM) {
                    oneByOne.append(',');
                }
                oneByOne.append(seek(tm, ts));
            }
            oneByOne.append(']');

            assertEquals(oneByOne.toString(), played,
                    "play must equal the same instants sought one at a time");
        }
    }

    @Test
    void aReSeekToTheSameInstantIsByteIdentical() {
        try (TimeMachine tm = TimeMachineTest.loaded()) {
            String first = seek(tm, TO);
            // Seek somewhere else and back: a re-fold must not depend on where
            // it came from, which is what makes scrubbing a timeline meaningful.
            seek(tm, FROM);
            assertEquals(first, seek(tm, TO));
        }
    }
}
