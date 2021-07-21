# BTreeMap vs IndexMap vs HashMap (sort-on-insert) Benchmark

```bash
> RUSTFLAGS="-C link-args=-s -C target-cpu=native" cargo run --release

[INSERT-SORT-BENCH]
=> Elements: 16777216
=> Generate Randoms: 259.967474ms
=> BTreeMap: 16.036178465s
=> IndexMap: 7.736480535s
=> Sorting: 991.496953ms
=> HashMap: 3.915755535s
Index 0 -> 23367537843083360996853049615011
Index 1 -> 36942544125250485271791776020663
Index 2 -> 37410545273741381521326215882330
```
