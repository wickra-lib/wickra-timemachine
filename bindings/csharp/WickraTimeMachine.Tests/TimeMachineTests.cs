using System.Text.Json;
using Wickra.TimeMachine;
using Xunit;

namespace WickraTimeMachine.Tests;

public class TimeMachineTests
{
    // A deterministic two-record feed on SYM: a rising then falling trade.
    internal const string Feed =
        "{\"ts\":10,\"symbol\":\"SYM\",\"feed\":{\"kind\":\"market\",\"type\":\"trade\"," +
        "\"symbol\":{\"base\":\"AAA\",\"quote\":\"USDT\"},\"price\":\"100\",\"quantity\":\"1\"," +
        "\"aggressor\":\"Buy\",\"timestamp\":10}}\n" +
        "{\"ts\":20,\"symbol\":\"SYM\",\"feed\":{\"kind\":\"market\",\"type\":\"trade\"," +
        "\"symbol\":{\"base\":\"AAA\",\"quote\":\"USDT\"},\"price\":\"105\",\"quantity\":\"1\"," +
        "\"aggressor\":\"Sell\",\"timestamp\":20}}";

    internal static string LoadCmd() =>
        "{\"cmd\":\"load\",\"data\":" + JsonSerializer.Serialize(Feed) + "}";

    internal static TimeMachine Loaded()
    {
        var tm = new TimeMachine("{}");
        tm.Command(LoadCmd());
        return tm;
    }

    [Fact]
    public void Version_IsNonEmpty()
    {
        Assert.False(string.IsNullOrEmpty(TimeMachine.Version()));
    }

    [Fact]
    public void Seek_ReconstructsSnapshot()
    {
        using var tm = Loaded();
        JsonElement snap = JsonDocument.Parse(tm.Command("{\"cmd\":\"seek\",\"ts\":20}")).RootElement;

        Assert.Equal(20, snap.GetProperty("ts").GetInt64());
        double last = snap.GetProperty("symbols").GetProperty("SYM").GetProperty("last").GetDouble();
        Assert.True(Math.Abs(last - 105.0) < 1e-9);
    }

    [Fact]
    public void InvalidSpec_Throws()
    {
        Assert.Throws<ArgumentException>(() => new TimeMachine("{ not valid json"));
    }
}
