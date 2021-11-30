# reversi-rs by toduq

## Benchmark

|  No | empties | passed | result | answer | nodes | time |    NPS |
| --: | ------: | -----: | -----: | -----: | ----: | ---: | -----: |
|   1 |      14 |     ok | +18@62 | +18@62 |  0.6M | 0.1s | 0.1M/s |
|   2 |      14 |     ok | +10@24 | +10@24 |  0.4M | 0.1s | 0.1M/s |
|   3 |      14 |     ok |  +2@ 3 |  +2@ 3 |  2.2M | 0.4s | 0.4M/s |
|   4 |      14 |   fail |  +8@63 |  +0@63 |  0.5M | 0.1s | 0.1M/s |
|   5 |      14 |     ok | +32@62 | +32@62 |  0.1M | 0.0s | 0.0M/s |
|   6 |      14 |     ok | +14@ 0 | +14@ 0 |  0.1M | 0.0s | 0.0M/s |
|   7 |      14 |     ok |  +8@40 |  +8@40 |  1.7M | 0.3s | 0.3M/s |
|   8 |      15 |   fail |  +6@ 4 |  +8@ 4 |  1.1M | 0.2s | 0.2M/s |
|   9 |      15 |   fail |  -8@24 |  -8@54 |  1.1M | 0.2s | 0.2M/s |
|  10 |      15 |     ok | +10@ 9 | +10@ 9 |  1.4M | 0.3s | 0.3M/s |
|  11 |      15 |   fail | +26@10 | +30@17 |  0.5M | 0.1s | 0.1M/s |
|  12 |      15 |     ok |  -8@49 |  -8@49 |  3.4M | 0.6s | 0.6M/s |
|  13 |      16 |     ok | +14@49 | +14@49 | 10.2M | 1.8s | 1.8M/s |
|  14 |      16 |     ok | +18@16 | +18@16 |  9.4M | 1.7s | 1.7M/s |
|  15 |      16 |     ok |  +4@22 |  +4@22 | 35.5M | 6.5s | 6.5M/s |
|  16 |      16 |     ok | +24@61 | +24@61 |  6.4M | 1.2s | 1.2M/s |
|  17 |      16 |   fail | +10@61 |  +8@61 |  0.9M | 0.2s | 0.2M/s |
|  18 |      16 |     ok |  -2@14 |  -2@14 |  7.2M | 1.4s | 1.4M/s |
|  19 |      16 |     ok |  +8@41 |  +8@41 | 16.9M | 3.2s | 3.2M/s |
|  20 |       6 |     ok |  +6@39 |  +6@39 |  0.0M | 0.0s | 0.0M/s |
