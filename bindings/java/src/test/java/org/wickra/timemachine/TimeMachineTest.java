package org.wickra.timemachine;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.Test;

class TimeMachineTest {
    // A deterministic two-record feed on SYM: a rising then falling trade.
    static final String FEED =
            "{\"ts\":10,\"symbol\":\"SYM\",\"feed\":{\"kind\":\"market\",\"type\":\"trade\","
                    + "\"symbol\":{\"base\":\"AAA\",\"quote\":\"USDT\"},\"price\":\"100\","
                    + "\"quantity\":\"1\",\"aggressor\":\"Buy\",\"timestamp\":10}}\n"
                    + "{\"ts\":20,\"symbol\":\"SYM\",\"feed\":{\"kind\":\"market\",\"type\":\"trade\","
                    + "\"symbol\":{\"base\":\"AAA\",\"quote\":\"USDT\"},\"price\":\"105\","
                    + "\"quantity\":\"1\",\"aggressor\":\"Sell\",\"timestamp\":20}}";

    // The feed embedded as a JSON string literal for the load command.
    static String loadCmd() {
        StringBuilder sb = new StringBuilder("{\"cmd\":\"load\",\"data\":\"");
        for (int i = 0; i < FEED.length(); i++) {
            char c = FEED.charAt(i);
            if (c == '"') {
                sb.append("\\\"");
            } else if (c == '\n') {
                sb.append("\\n");
            } else {
                sb.append(c);
            }
        }
        sb.append("\"}");
        return sb.toString();
    }

    static TimeMachine loaded() {
        TimeMachine tm = new TimeMachine("{}");
        tm.command(loadCmd());
        return tm;
    }

    @Test
    void versionIsNonEmpty() {
        assertFalse(TimeMachine.version().isEmpty());
    }

    @Test
    void seekReconstructsSnapshot() {
        try (TimeMachine tm = loaded()) {
            String out = tm.command("{\"cmd\":\"seek\",\"ts\":20}");
            assertTrue(out.contains("\"ts\":20"), out);
            assertTrue(out.contains("\"last\":105"), out);
        }
    }

    @Test
    void invalidSpecThrows() {
        assertThrows(IllegalArgumentException.class, () -> new TimeMachine("{ not valid json"));
    }
}
