/* A runnable C example: load a small recorded feed and reconstruct the market
 * snapshot at a past timestamp through the wickra-timemachine C ABI. Every
 * language example loads the same feed and prints the same summary. */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "wickra_timemachine.h"

/* Two trades on SYM; records separated by a newline. */
static const char *FEED =
    "{\"ts\":10,\"symbol\":\"SYM\",\"feed\":{\"kind\":\"market\",\"type\":\"trade\",\"symbol\":{\"base\":\"AAA\",\"quote\":\"USDT\"},\"price\":\"100\",\"quantity\":\"1\",\"aggressor\":\"Buy\",\"timestamp\":10}}\n"
    "{\"ts\":20,\"symbol\":\"SYM\",\"feed\":{\"kind\":\"market\",\"type\":\"trade\",\"symbol\":{\"base\":\"AAA\",\"quote\":\"USDT\"},\"price\":\"110\",\"quantity\":\"2\",\"aggressor\":\"Sell\",\"timestamp\":20}}";

/* Build {"cmd":"load","data":"<JSON-escaped feed>"} into out. */
static void build_load(char *out, size_t cap) {
    size_t n = 0;
    n += (size_t)snprintf(out + n, cap - n, "{\"cmd\":\"load\",\"data\":\"");
    for (const char *p = FEED; *p && n + 2 < cap; p++) {
        if (*p == '"') {
            out[n++] = '\\';
            out[n++] = '"';
        } else if (*p == '\n') {
            out[n++] = '\\';
            out[n++] = 'n';
        } else {
            out[n++] = *p;
        }
    }
    snprintf(out + n, cap - n, "\"}");
}

static char *run(WickraTimeMachine *tm, const char *cmd) {
    int len = wickra_timemachine_command(tm, cmd, NULL, 0);
    if (len < 0) {
        return NULL;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    if (buf) {
        wickra_timemachine_command(tm, cmd, buf, (size_t)len + 1);
    }
    return buf;
}

int main(void) {
    WickraTimeMachine *tm = wickra_timemachine_new("{}");
    if (!tm) {
        fprintf(stderr, "failed to build time machine\n");
        return 1;
    }
    char load[2048];
    build_load(load, sizeof(load));

    char *loaded = run(tm, load);
    char *snapshot = run(tm, "{\"cmd\":\"seek\",\"ts\":20}");
    if (!loaded || !snapshot) {
        fprintf(stderr, "command failed\n");
        free(loaded);
        free(snapshot);
        wickra_timemachine_free(tm);
        return 1;
    }

    printf("wickra-timemachine %s\n", wickra_timemachine_version());
    printf("snapshot bytes: %zu\n", strlen(snapshot));

    free(loaded);
    free(snapshot);
    wickra_timemachine_free(tm);
    return 0;
}
