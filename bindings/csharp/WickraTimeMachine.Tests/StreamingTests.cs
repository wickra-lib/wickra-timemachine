using System.Globalization;
using System.Text;
using Wickra.TimeMachine;
using Xunit;

namespace WickraTimeMachine.Tests;

/// <summary>
/// <c>play</c> streams frames; <c>seek</c> re-folds each instant independently.
///
/// <c>wickra-timemachine-core</c> proves the two agree in Rust, but that says nothing
/// about the boundary this binding crosses: a binding that dropped a frame from
/// the played array, or mis-serialised one, would hand back a sequence that
/// looks plausible and is not the engine's.
///
/// That equality is the whole claim of the engine — a seek is a deterministic
/// re-fold, not an interpolation — so a binding that breaks it breaks the
/// product, not a detail.
/// </summary>
public class StreamingTests
{
    private const long From = 10L;
    private const long To = 30L;
    private const long Step = 10L;

    private static string Seek(TimeMachine tm, long ts) =>
        tm.Command(string.Format(
            CultureInfo.InvariantCulture, "{{\"cmd\":\"seek\",\"ts\":{0}}}", ts));

    [Fact]
    public void PlayEqualsRepeatedSeek()
    {
        using var tm = TimeMachineTests.Loaded();

        string played = tm.Command(string.Format(
            CultureInfo.InvariantCulture,
            "{{\"cmd\":\"play\",\"from\":{0},\"to\":{1},\"step\":{2}}}", From, To, Step));

        var oneByOne = new StringBuilder("[");
        for (long ts = From; ts <= To; ts += Step)
        {
            if (ts > From)
            {
                oneByOne.Append(',');
            }

            oneByOne.Append(Seek(tm, ts));
        }

        Assert.Equal(oneByOne.Append(']').ToString(), played);
    }

    [Fact]
    public void AReSeekToTheSameInstantIsByteIdentical()
    {
        using var tm = TimeMachineTests.Loaded();

        string first = Seek(tm, To);
        // Seek somewhere else and back: a re-fold must not depend on where it
        // came from, which is what makes scrubbing a timeline meaningful.
        Seek(tm, From);
        Assert.Equal(first, Seek(tm, To));
    }
}
