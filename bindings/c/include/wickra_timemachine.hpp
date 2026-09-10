// Wickra Time Machine — C++ wrapper over the C ABI.
//
// Header-only, C++17, no dependency beyond the standard library and
// `wickra_timemachine.h` beside it. Link the same `wickra_timemachine` library the C
// binding does.
//
// What it adds over calling the C functions directly is the handling nobody
// wants to write twice: the handle is owned and freed, the two-call length
// protocol behind `wickra_timemachine_command` is done for you, and a failure comes
// back as an exception rather than a negative integer a caller can ignore.
//
//     #include <wickra_timemachine.hpp>
//
//     wickra::TimeMachine tm(R"({"book_depth":10,"tape_cap":64})");
//     std::string snapshot = tm.command(R"({"cmd":"seek","ts":20})");
//
// The timemachine is data-driven, so this wrapper deliberately stops at strings:
// the spec and the report are JSON, and which JSON library a caller uses is
// their choice, not this header's.

#ifndef WICKRA_TIMEMACHINE_HPP
#define WICKRA_TIMEMACHINE_HPP

#include <cstddef>
#include <cstdint>
#include <stdexcept>
#include <string>
#include <utility>

#include "wickra_timemachine.h"

namespace wickra {

/// Thrown when the library rejects a spec or a command.
class TimeMachineError : public std::runtime_error {
 public:
  explicit TimeMachineError(const std::string& what) : std::runtime_error(what) {}
};

/// An owning handle to a timemachine built from a spec.
///
/// Move-only, because the underlying handle is a unique resource: copying it
/// would free the same pointer twice.
class TimeMachine {
 public:
  /// Build a timemachine from a spec JSON string.
  ///
  /// Throws `TimeMachineError` if the spec is not valid JSON or not a valid spec.
  explicit TimeMachine(const std::string& spec_json)
      : handle_(wickra_timemachine_new(spec_json.c_str())) {
    if (handle_ == nullptr) {
      throw TimeMachineError("wickra_timemachine_new rejected the spec");
    }
  }

  ~TimeMachine() { wickra_timemachine_free(handle_); }

  TimeMachine(const TimeMachine&) = delete;
  TimeMachine& operator=(const TimeMachine&) = delete;

  TimeMachine(TimeMachine&& other) noexcept : handle_(other.handle_) {
    other.handle_ = nullptr;
  }

  TimeMachine& operator=(TimeMachine&& other) noexcept {
    if (this != &other) {
      wickra_timemachine_free(handle_);
      handle_ = other.handle_;
      other.handle_ = nullptr;
    }
    return *this;
  }

  /// Apply a command JSON and return the response JSON.
  ///
  /// The C entry point writes into a caller buffer and reports the length it
  /// needed, so this asks for the length first and then reads. A command the
  /// library understands but cannot carry out answers in band with
  /// `{"ok":false,"error":...}`; a negative return is a failure of the call
  /// itself and becomes an exception.
  std::string command(const std::string& cmd_json) {
    const std::int32_t needed =
        wickra_timemachine_command(handle_, cmd_json.c_str(), nullptr, 0);
    if (needed < 0) {
      throw TimeMachineError("wickra_timemachine_command failed with code " +
                          std::to_string(needed));
    }

    std::string out(static_cast<std::size_t>(needed), '\0');
    // The C side writes a trailing NUL, so the buffer has to hold one more byte
    // than the response itself.
    const std::int32_t written = wickra_timemachine_command(
        handle_, cmd_json.c_str(), out.data(),
        static_cast<std::uintptr_t>(out.size()) + 1);
    if (written < 0) {
      throw TimeMachineError("wickra_timemachine_command failed with code " +
                          std::to_string(written));
    }
    if (written != needed) {
      // The response changed length between the two calls, which cannot happen
      // for a handle only this thread is using. Saying so is better than
      // returning a string that is half of one answer and half of another.
      throw TimeMachineError("wickra_timemachine_command length changed between calls");
    }
    return out;
  }

  /// The library version.
  static std::string version() { return std::string(wickra_timemachine_version()); }

 private:
  WickraTimeMachine* handle_;
};

}  // namespace wickra

#endif  // WICKRA_TIMEMACHINE_HPP
