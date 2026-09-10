// A runnable C++ example: load a small recorded feed and reconstruct the market
// snapshot at a past timestamp.
//
// This goes through `wickra_timemachine.hpp`, the C++ hull shipped beside the C
// header, because that hull is what a C++ caller is meant to use: it owns and
// frees the handle, runs the two-call length protocol behind
// `wickra_timemachine_command` for you, and turns a refusal into an exception
// rather than a negative integer that is easy to ignore. Calling the C
// functions directly from C++ works too -- `seek.c` shows that -- but then the
// hull would be shipped without anything building it.
#include <iostream>
#include <string>

#include "wickra_timemachine.hpp"

namespace {

const std::string kFeed =
    R"({"ts":10,"symbol":"SYM","feed":{"kind":"market","type":"trade","symbol":{"base":"AAA","quote":"USDT"},"price":"100","quantity":"1","aggressor":"Buy","timestamp":10}})"
    "\n"
    R"({"ts":20,"symbol":"SYM","feed":{"kind":"market","type":"trade","symbol":{"base":"AAA","quote":"USDT"},"price":"110","quantity":"2","aggressor":"Sell","timestamp":20}})";

std::string loadCommand() {
    std::string data;
    for (char c : kFeed) {
        if (c == '"') {
            data += "\\\"";
        } else if (c == '\n') {
            // Two characters, a backslash and an `n`. A literal newline inside a
            // JSON string is a control character, which the parser refuses.
            data += "\\n";
        } else {
            data += c;
        }
    }
    return "{\"cmd\":\"load\",\"data\":\"" + data + "\"}";
}

}  // namespace

int main() {
    try {
        wickra::TimeMachine tm("{}");
        tm.command(loadCommand());
        const std::string snapshot = tm.command(R"({"cmd":"seek","ts":20})");

        std::cout << "wickra-timemachine " << wickra::TimeMachine::version() << "\n";
        std::cout << "snapshot bytes: " << snapshot.size() << "\n";

        // Without this the example prints a byte count either way, so a load the
        // core refused reads as a short snapshot rather than as a failure --
        // which is exactly how the broken escape above survived in a passing
        // test. 110 is SYM's last trade price at ts 20, the same value the Java
        // example checks.
        if (snapshot.find("\"last\":110") == std::string::npos) {
            std::cerr << "the seek did not reconstruct SYM: " << snapshot << "\n";
            return 1;
        }
    } catch (const wickra::TimeMachineError &err) {
        // Every failure arrives here: a spec the core rejects, a command it does
        // not know, a response that changed length between the two ABI calls.
        std::cerr << err.what() << "\n";
        return 1;
    }
    return 0;
}
