package org.wickra.timemachine.examples;

import org.wickra.timemachine.TimeMachine;

/**
 * A runnable Java example: load a small recorded feed and reconstruct the market
 * snapshot at a past timestamp.
 *
 * <pre>
 *   mvn -q compile exec:java -Dexec.mainClass=org.wickra.timemachine.examples.Seek
 * </pre>
 *
 * Every language example loads the same feed and prints the same summary.
 */
public final class Seek {
    private Seek() {}

    private static final String FEED =
            "{\"ts\":10,\"symbol\":\"SYM\",\"feed\":{\"kind\":\"market\",\"type\":\"trade\","
                    + "\"symbol\":{\"base\":\"AAA\",\"quote\":\"USDT\"},\"price\":\"100\","
                    + "\"quantity\":\"1\",\"aggressor\":\"Buy\",\"timestamp\":10}}\n"
                    + "{\"ts\":20,\"symbol\":\"SYM\",\"feed\":{\"kind\":\"market\",\"type\":\"trade\","
                    + "\"symbol\":{\"base\":\"AAA\",\"quote\":\"USDT\"},\"price\":\"110\","
                    + "\"quantity\":\"2\",\"aggressor\":\"Sell\",\"timestamp\":20}}";

    /** Embed the feed as a JSON string literal inside the load command. */
    private static String loadCommand() {
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

    public static void main(String[] args) {
        try (TimeMachine tm = new TimeMachine("{}")) {
            tm.command(loadCommand());
            String snapshot = tm.command("{\"cmd\":\"seek\",\"ts\":20}");
            System.out.printf("wickra-timemachine %s%n", TimeMachine.version());
            // The snapshot JSON is byte-identical to every other binding; parse it
            // with a real JSON library in production. Here we just confirm the seek
            // reconstructed the SYM last price of 110.
            System.out.printf("SYM last present: %b%n", snapshot.contains("\"last\":110"));
        }
    }
}
