/* `play` streams frames; `seek` re-folds each instant independently.
 *
 * `wickra-timemachine-core` proves the two agree in Rust, but that says nothing about
 * the boundary a C caller crosses. Every reach behind this ABI asks for the
 * response length first and reads it second, so a command that is not a pure
 * function of its payload runs twice per call. `load` is exactly that — it
 * ingests the feed — and a double-applied load would fold every event in twice,
 * which a snapshot of the *last* state cannot reveal but a mid-timeline seek can.
 *
 * That equality is the whole claim of the engine: a seek is a deterministic
 * re-fold, not an interpolation. A binding that breaks it breaks the product.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "wickra_timemachine.h"

#define FROM 10
#define TO 30
#define STEP 10

/* Three trades, ten seconds apart, so a seek lands between them as well as on
 * them. The quantities differ so a double-applied load changes the tape. */
static const char *FEED =
    "{\"ts\":10,\"symbol\":\"SYM\",\"feed\":{\"kind\":\"market\",\"type\":\"trade\","
    "\"symbol\":{\"base\":\"SYM\",\"quote\":\"USDT\"},\"price\":\"100.0\","
    "\"quantity\":\"1.0\",\"aggressor\":\"Buy\",\"timestamp\":10}}\n"
    "{\"ts\":20,\"symbol\":\"SYM\",\"feed\":{\"kind\":\"market\",\"type\":\"trade\","
    "\"symbol\":{\"base\":\"SYM\",\"quote\":\"USDT\"},\"price\":\"105.0\","
    "\"quantity\":\"2.0\",\"aggressor\":\"Sell\",\"timestamp\":20}}\n"
    "{\"ts\":30,\"symbol\":\"SYM\",\"feed\":{\"kind\":\"market\",\"type\":\"trade\","
    "\"symbol\":{\"base\":\"SYM\",\"quote\":\"USDT\"},\"price\":\"110.0\","
    "\"quantity\":\"1.5\",\"aggressor\":\"Buy\",\"timestamp\":30}}";

/* The feed as a JSON string literal: the quotes inside each record and the
 * newlines between them both have to be escaped to cross the boundary. Caller
 * frees. */
static char *as_json_string(const char *raw) {
    size_t cap = strlen(raw) * 2 + 3;
    char *out = (char *)malloc(cap);
    if (!out) {
        return NULL;
    }
    size_t j = 0;
    out[j++] = '"';
    for (size_t i = 0; raw[i] != '\0'; i++) {
        char c = raw[i];
        if (c == '\n') {
            out[j++] = '\\';
            out[j++] = 'n';
        } else if (c == '\r') {
            continue;
        } else if (c == '"' || c == '\\') {
            out[j++] = '\\';
            out[j++] = c;
        } else {
            out[j++] = c;
        }
    }
    out[j++] = '"';
    out[j] = '\0';
    return out;
}

/* Run one command through the documented two-call idiom. Caller frees. */
static char *run(WickraTimeMachine *tm, const char *cmd) {
    int32_t len = wickra_timemachine_command(tm, cmd, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed: code %d\n", (int)len);
        return NULL;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    if (!buf) {
        return NULL;
    }
    int32_t written = wickra_timemachine_command(tm, cmd, buf, (size_t)len + 1);
    if (written != len) {
        fprintf(stderr, "second call returned %d, first said %d\n", (int)written, (int)len);
        free(buf);
        return NULL;
    }
    return buf;
}

/* A loaded machine, or NULL. */
static WickraTimeMachine *loaded(void) {
    WickraTimeMachine *tm = wickra_timemachine_new("{}");
    if (!tm) {
        return NULL;
    }
    char *quoted = as_json_string(FEED);
    if (!quoted) {
        wickra_timemachine_free(tm);
        return NULL;
    }
    size_t cap = strlen(quoted) + 32;
    char *load = (char *)malloc(cap);
    if (!load) {
        free(quoted);
        wickra_timemachine_free(tm);
        return NULL;
    }
    snprintf(load, cap, "{\"cmd\":\"load\",\"data\":%s}", quoted);
    free(quoted);
    char *ack = run(tm, load);
    free(load);
    if (!ack) {
        wickra_timemachine_free(tm);
        return NULL;
    }
    free(ack);
    return tm;
}

static char *seek_at(WickraTimeMachine *tm, int ts) {
    char cmd[64];
    snprintf(cmd, sizeof(cmd), "{\"cmd\":\"seek\",\"ts\":%d}", ts);
    return run(tm, cmd);
}

int main(void) {
    WickraTimeMachine *tm = loaded();
    if (!tm) {
        fprintf(stderr, "could not load the feed\n");
        return 1;
    }

    int failures = 0;

    char play_cmd[96];
    snprintf(play_cmd, sizeof(play_cmd),
             "{\"cmd\":\"play\",\"from\":%d,\"to\":%d,\"step\":%d}", FROM, TO, STEP);
    char *played = run(tm, play_cmd);

    /* Build the same sequence one seek at a time: `[frame,frame,frame]`. */
    size_t cap = 8192;
    char *one_by_one = (char *)malloc(cap);
    if (!played || !one_by_one) {
        fprintf(stderr, "could not drive the timeline\n");
        free(played);
        free(one_by_one);
        wickra_timemachine_free(tm);
        return 1;
    }
    size_t used = 0;
    used += (size_t)snprintf(one_by_one + used, cap - used, "[");
    for (int ts = FROM; ts <= TO; ts += STEP) {
        char *frame = seek_at(tm, ts);
        if (!frame) {
            failures++;
            break;
        }
        used += (size_t)snprintf(one_by_one + used, cap - used, "%s%s",
                                 ts == FROM ? "" : ",", frame);
        free(frame);
    }
    snprintf(one_by_one + used, cap - used, "]");

    if (failures == 0 && strcmp(played, one_by_one) != 0) {
        fprintf(stderr, "play != the same instants sought one at a time\n  play: %s\n  seek: %s\n",
                played, one_by_one);
        failures++;
    }

    /* A re-fold must not depend on where the previous seek left off. */
    char *first = seek_at(tm, TO);
    char *away = seek_at(tm, FROM);
    char *again = seek_at(tm, TO);
    if (!first || !away || !again) {
        failures++;
    } else if (strcmp(first, again) != 0) {
        fprintf(stderr, "a re-seek to the same instant differed\n");
        failures++;
    }
    free(first);
    free(away);
    free(again);

    free(played);
    free(one_by_one);
    wickra_timemachine_free(tm);

    if (failures > 0) {
        fprintf(stderr, "%d check(s) failed\n", failures);
        return 1;
    }
    printf("play equals repeated seek\n");
    return 0;
}
