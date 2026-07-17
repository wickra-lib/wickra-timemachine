// A runnable Go example: load a small recorded feed and reconstruct the market
// snapshot at a past timestamp.
//
//	go run examples/go/seek.go
//
// Every language example loads the same feed and prints the same summary.
package main

import (
	"encoding/json"
	"fmt"

	wickra "github.com/wickra-lib/wickra-timemachine/bindings/go"
)

const feed = `{"ts":10,"symbol":"SYM","feed":{"kind":"market","type":"trade","symbol":{"base":"AAA","quote":"USDT"},"price":"100","quantity":"1","aggressor":"Buy","timestamp":10}}` + "\n" +
	`{"ts":20,"symbol":"SYM","feed":{"kind":"market","type":"trade","symbol":{"base":"AAA","quote":"USDT"},"price":"110","quantity":"2","aggressor":"Sell","timestamp":20}}`

func main() {
	tm, err := wickra.New("{}")
	if err != nil {
		panic(err)
	}
	defer tm.Close()

	data, _ := json.Marshal(feed)
	if _, err := tm.Command(`{"cmd":"load","data":` + string(data) + `}`); err != nil {
		panic(err)
	}
	raw, err := tm.Command(`{"cmd":"seek","ts":20}`)
	if err != nil {
		panic(err)
	}
	var snapshot struct {
		Ts      int64 `json:"ts"`
		Symbols map[string]struct {
			Last float64 `json:"last"`
		} `json:"symbols"`
	}
	if err := json.Unmarshal([]byte(raw), &snapshot); err != nil {
		panic(err)
	}
	fmt.Printf("wickra-timemachine %s\n", wickra.Version())
	fmt.Printf("snapshot ts: %d\n", snapshot.Ts)
	fmt.Printf("symbols: %d\n", len(snapshot.Symbols))
	fmt.Printf("SYM last: %g\n", snapshot.Symbols["SYM"].Last)
}
