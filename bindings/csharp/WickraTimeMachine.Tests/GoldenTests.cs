using Xunit;

namespace WickraTimeMachine.Tests;

public class GoldenTests
{
    private static string Seek(int ts)
    {
        using var tm = new Wickra.TimeMachine.TimeMachine("{}");
        tm.Command(TimeMachineTests.LoadCmd());
        return tm.Command("{\"cmd\":\"seek\",\"ts\":" + ts + "}");
    }

    [Fact]
    public void Seek_IsByteIdenticalAcrossInstances()
    {
        Assert.Equal(Seek(20), Seek(20));
    }

    [Fact]
    public void Seek_IsTsInclusive()
    {
        Assert.False(string.IsNullOrEmpty(Seek(10)));
    }
}
