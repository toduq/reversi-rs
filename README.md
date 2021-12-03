# reversi-rs by toduq

A Reversi program which is under development.

Heavily inspired by the following pages.

- https://github.com/primenumber/issen-rs
- https://speakerdeck.com/primenumber/solveothello?slide=62

## Benchmark

FFO end-game solver benchmark.

- MacBook Pro (15-inch, 2018)
- Single Thread

```
$ cargo run --release -- --solve-ffo --ffo-start 1 --ffo-end 37
```

|  No | empties | passed | result | answer | nodes |  time |    NPS |
| --: | ------: | -----: | -----: | -----: | ----: | ----: | -----: |
|   1 |      14 |     ok | +18@62 | +18@62 |  0.0M |  0.0s | 3.4M/s |
|   2 |      14 |     ok | +10@24 | +10@24 |  0.0M |  0.0s | 2.7M/s |
|   3 |      14 |     ok |  +2@ 3 |  +2@ 3 |  0.1M |  0.0s | 2.9M/s |
|   4 |      14 |     ok |  +0@63 |  +0@63 |  0.1M |  0.0s | 2.9M/s |
|   5 |      14 |     ok | +32@62 | +32@62 |  0.0M |  0.0s | 2.3M/s |
|   6 |      14 |     ok | +14@ 0 | +14@ 0 |  0.0M |  0.0s | 2.8M/s |
|   7 |      14 |     ok |  +8@40 |  +8@40 |  0.0M |  0.0s | 3.1M/s |
|   8 |      15 |     ok |  +8@ 4 |  +8@ 4 |  0.2M |  0.1s | 2.8M/s |
|   9 |      15 |     ok |  -8@24 |  -8@54 |  0.0M |  0.0s | 2.6M/s |
|  10 |      15 |     ok | +10@ 9 | +10@ 9 |  0.1M |  0.0s | 2.7M/s |
|  11 |      15 |     ok | +30@17 | +30@17 |  0.1M |  0.0s | 2.5M/s |
|  12 |      15 |     ok |  -8@49 |  -8@49 |  0.2M |  0.1s | 2.9M/s |
|  13 |      16 |     ok | +14@49 | +14@49 |  0.1M |  0.0s | 2.9M/s |
|  14 |      16 |     ok | +18@16 | +18@16 |  0.2M |  0.1s | 3.1M/s |
|  15 |      16 |     ok |  +4@22 |  +4@22 |  0.1M |  0.0s | 3.3M/s |
|  16 |      16 |     ok | +24@61 | +24@61 |  0.3M |  0.1s | 3.7M/s |
|  17 |      16 |     ok |  +8@61 |  +8@61 |  0.0M |  0.0s | 3.0M/s |
|  18 |      16 |     ok |  -2@14 |  -2@14 |  0.2M |  0.1s | 3.3M/s |
|  19 |      16 |     ok |  +8@41 |  +8@41 |  0.3M |  0.1s | 3.4M/s |
|  20 |       6 |     ok |  +6@39 |  +6@39 |  0.0M |  0.0s | 2.1M/s |
|  21 |      15 |     ok |  +0@38 |  +0@38 |  0.2M |  0.0s | 3.8M/s |
|  22 |      17 |     ok |  +2@62 |  +2@62 |  1.3M |  0.4s | 3.3M/s |
|  23 |      18 |     ok |  +4@ 8 |  +4@ 8 |  1.1M |  0.3s | 3.3M/s |
|  24 |      19 |     ok |  +0@18 |  +0@18 |  4.4M |  1.3s | 3.4M/s |
|  25 |      19 |     ok |  +0@ 6 |  +0@ 6 |  4.4M |  1.3s | 3.3M/s |
|  26 |      20 |     ok |  +0@59 |  +0@59 | 25.9M |  8.3s | 3.1M/s |
|  27 |      20 |     ok |  -2@49 |  -2@49 |  6.5M |  2.0s | 3.3M/s |
|  28 |      20 |     ok |  +0@ 9 |  +0@ 5 | 18.3M |  5.8s | 3.2M/s |
|  29 |      20 |     ok | +10@14 | +10@14 |  3.6M |  1.1s | 3.2M/s |
|  30 |      20 |     ok |  +0@22 |  +0@22 | 23.0M |  7.1s | 3.2M/s |
|  31 |      20 |     ok |  -2@46 |  -2@46 |  7.7M |  2.7s | 2.9M/s |
|  32 |      20 |     ok |  -4@22 |  -4@22 | 25.3M |  8.4s | 3.0M/s |
|  33 |      20 |     ok |  -8@52 |  -8@52 | 21.8M |  6.9s | 3.2M/s |
|  34 |      20 |     ok |  -2@10 |  -2@10 | 39.6M | 11.3s | 3.5M/s |
|  35 |      21 |     ok |  +0@50 |  +0@50 | 23.6M |  6.9s | 3.4M/s |
|  36 |      21 |     ok |  +0@49 |  +0@49 | 72.3M | 21.4s | 3.4M/s |
|  37 |      22 |     ok | -20@14 | -20@14 | 70.1M | 23.1s | 3.0M/s |
