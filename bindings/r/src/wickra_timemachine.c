/* R .Call glue for the wickra-timemachine C ABI hub. */
#include <R.h>
#include <Rinternals.h>
#include <R_ext/Rdynload.h>
#include <stddef.h>
#include "wickra_timemachine.h"

/* --- handle lifetime ----------------------------------------------------- */

static void wktimemachine_finalize(SEXP ext) {
    WickraTimeMachine *h = (WickraTimeMachine *)R_ExternalPtrAddr(ext);
    if (h) {
        wickra_timemachine_free(h);
    }
    R_ClearExternalPtr(ext);
}

static WickraTimeMachine *handle_of(SEXP ext) {
    WickraTimeMachine *h = (WickraTimeMachine *)R_ExternalPtrAddr(ext);
    if (!h) {
        Rf_error("wickra-timemachine: handle is closed");
    }
    return h;
}

/* --- exported .Call entries ---------------------------------------------- */

SEXP wktimemachine_version(void) {
    return Rf_mkString(wickra_timemachine_version());
}

SEXP wktimemachine_new(SEXP spec_json) {
    const char *spec = CHAR(STRING_ELT(spec_json, 0));
    WickraTimeMachine *h = wickra_timemachine_new(spec);
    if (!h) {
        Rf_error("wickra-timemachine: invalid spec");
    }
    SEXP ext = PROTECT(R_MakeExternalPtr(h, R_NilValue, R_NilValue));
    R_RegisterCFinalizerEx(ext, wktimemachine_finalize, TRUE);
    UNPROTECT(1);
    return ext;
}

SEXP wktimemachine_command(SEXP ext, SEXP cmd_json) {
    WickraTimeMachine *h = handle_of(ext);
    const char *cmd = CHAR(STRING_ELT(cmd_json, 0));

    /* Length-out protocol: learn the length, then read into a caller buffer.
       Domain errors come back in-band as {"ok":false,...} JSON, not a negative
       code; only unusable arguments / a caught panic return < 0. */
    int len = wickra_timemachine_command(h, cmd, NULL, 0);
    if (len < 0) {
        Rf_error("wickra-timemachine: command failed (code %d)", len);
    }
    char *buf = (char *)R_alloc((size_t)len + 1, 1);
    wickra_timemachine_command(h, cmd, buf, (size_t)len + 1);
    return Rf_mkString(buf);
}

/* --- registration -------------------------------------------------------- */

static const R_CallMethodDef CallEntries[] = {
    {"wktimemachine_version", (DL_FUNC)&wktimemachine_version, 0},
    {"wktimemachine_new", (DL_FUNC)&wktimemachine_new, 1},
    {"wktimemachine_command", (DL_FUNC)&wktimemachine_command, 2},
    {NULL, NULL, 0}};

void R_init_wickratimemachine(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);
}
