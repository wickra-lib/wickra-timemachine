using System.Runtime.InteropServices;
using System.Text;

namespace Wickra.TimeMachine;

/// <summary>
/// A recorded-market time machine driven by JSON commands, built from a spec,
/// over the Wickra C ABI. Construct one from a spec JSON, drive it with command
/// JSON (<c>load</c>, <c>seek</c>, <c>state_at</c>, <c>play</c>, <c>version</c>)
/// and read back the response JSON — the same protocol as the CLI and every other binding.
/// </summary>
public sealed class TimeMachine : IDisposable
{
    private readonly TimeMachineHandle _handle;

    /// <summary>Build a time-machine handle from a spec JSON (<c>"{}"</c> uses the default spec).</summary>
    /// <exception cref="ArgumentException">The spec was not a valid spec.</exception>
    public TimeMachine(string specJson)
    {
        IntPtr ptr = Native.wickra_timemachine_new(Utf8(specJson));
        if (ptr == IntPtr.Zero)
        {
            throw new ArgumentException("wickra-timemachine: invalid spec", nameof(specJson));
        }
        _handle = new TimeMachineHandle(ptr);
    }

    /// <summary>Apply a command JSON and return the response JSON.</summary>
    /// <remarks>
    /// Uses the C ABI's length-out protocol: a first call learns the length, then
    /// the response is read into a caller-owned buffer. Domain errors (a bad
    /// command, an unknown command) come back in-band as <c>{"ok":false,...}</c>
    /// JSON, not as an exception.
    /// </remarks>
    /// <exception cref="InvalidOperationException">A required argument was unusable or a panic was caught.</exception>
    public string Command(string cmdJson)
    {
        ObjectDisposedException.ThrowIf(_handle.IsInvalid, this);

        byte[] cmd = Utf8(cmdJson);
        IntPtr h = _handle.DangerousGetHandle();
        int n = Native.wickra_timemachine_command(h, cmd, null, 0);
        if (n < 0)
        {
            throw new InvalidOperationException($"wickra-timemachine: command failed (code {n})");
        }
        var buf = new byte[n + 1];
        Native.wickra_timemachine_command(h, cmd, buf, (nuint)buf.Length);
        return Encoding.UTF8.GetString(buf, 0, n);
    }

    /// <summary>The library version.</summary>
    public static string Version() =>
        Marshal.PtrToStringUTF8(Native.wickra_timemachine_version()) ?? string.Empty;

    /// <summary>Free the native handle.</summary>
    public void Dispose() => _handle.Dispose();

    /// <summary>Encode a string as NUL-terminated UTF-8 for the C ABI.</summary>
    private static byte[] Utf8(string s)
    {
        int len = Encoding.UTF8.GetByteCount(s);
        var buf = new byte[len + 1];
        Encoding.UTF8.GetBytes(s, 0, s.Length, buf, 0);
        return buf;
    }
}

/// <summary>A safe handle owning a native search pointer.</summary>
internal sealed class TimeMachineHandle : SafeHandle
{
    public TimeMachineHandle(IntPtr handle)
        : base(IntPtr.Zero, ownsHandle: true) => SetHandle(handle);

    public override bool IsInvalid => handle == IntPtr.Zero;

    protected override bool ReleaseHandle()
    {
        Native.wickra_timemachine_free(handle);
        return true;
    }
}
