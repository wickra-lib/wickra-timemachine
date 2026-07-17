// A runnable C++ example: load a small recorded feed and reconstruct the market
// snapshot at a past timestamp through the wickra-timemachine C ABI.
#include <cstdint>
#include <iostream>
#include <string>
#include <vector>

#include "wickra_timemachine.h"

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
            data += "\n";
        } else {
            data += c;
        }
    }
    return "{\"cmd\":\"load\",\"data\":\"" + data + "\"}";
}

std::string run(WickraTimeMachine *tm, const std::string &cmd) {
    int len = wickra_timemachine_command(tm, cmd.c_str(), nullptr, 0);
    if (len < 0) {
        return {};
    }
    std::vector<char> buf(static_cast<size_t>(len) + 1);
    wickra_timemachine_command(tm, cmd.c_str(), buf.data(), buf.size());
    return std::string(buf.data(), static_cast<size_t>(len));
}

}  // namespace

int main() {
    WickraTimeMachine *tm = wickra_timemachine_new("{}");
    if (!tm) {
        std::cerr << "failed to build time machine\n";
        return 1;
    }
    run(tm, loadCommand());
    std::string snapshot = run(tm, R"({"cmd":"seek","ts":20})");

    std::cout << "wickra-timemachine " << wickra_timemachine_version() << "\n";
    std::cout << "snapshot bytes: " << snapshot.size() << "\n";

    wickra_timemachine_free(tm);
    return 0;
}
