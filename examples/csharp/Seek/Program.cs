// A runnable C# example: load a small recorded feed and reconstruct the market
// snapshot at a past timestamp.
//
//   dotnet run --project examples/csharp/Seek
//
// Every language example loads the same feed and prints the same summary.
using System.Text.Json;
using Wickra.TimeMachine;

const string feed =
    """{"ts":10,"symbol":"SYM","feed":{"kind":"market","type":"trade","symbol":{"base":"AAA","quote":"USDT"},"price":"100","quantity":"1","aggressor":"Buy","timestamp":10}}""" + "\n" +
    """{"ts":20,"symbol":"SYM","feed":{"kind":"market","type":"trade","symbol":{"base":"AAA","quote":"USDT"},"price":"110","quantity":"2","aggressor":"Sell","timestamp":20}}""";

using var tm = new TimeMachine("{}");
tm.Command($$"""{"cmd":"load","data":{{JsonSerializer.Serialize(feed)}}}""");
JsonElement snapshot = JsonDocument.Parse(tm.Command("""{"cmd":"seek","ts":20}""")).RootElement;

Console.WriteLine($"wickra-timemachine {TimeMachine.Version()}");
Console.WriteLine($"snapshot ts: {snapshot.GetProperty("ts").GetInt64()}");
Console.WriteLine($"symbols: {snapshot.GetProperty("symbols").EnumerateObject().Count()}");
Console.WriteLine($"SYM last: {snapshot.GetProperty("symbols").GetProperty("SYM").GetProperty("last").GetDouble()}");
