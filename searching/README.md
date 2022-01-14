# Language vs Algorithm Efficiency
- This is implementation like most developer do, not the most optimized like using unsafe code for rust and numpy for Python
- Complexity
  - Rust
    - Linear Search : Average Case : ```O(n)``` Worst case: ```O(n)```
    - Binary Search : Average Case : ```O(log_n)``` Worst case: ```O(log_n)```
    - Binary Search(Default) : Average Case : ```O(log_n)``` Worst case: ```O(log_n)```
  - Python
    - Linear Search : Average Case : ```O(n)``` Worst case: ```O(n)```
    - Binary Search : Average Case : ```O(log_n)``` Worst case: ```O(log_n)```
    - Binary Search(Recursive) : Average Case : ```O(log_n)``` Worst case: ```O(log_n)```

### Hardware/Software
- CPU : 2.3 GHz 8-Core Intel Core i9
- RAM : 16GB
- OS : macOS Monterey 12.1
- Rust : rustc 1.57.0
- Python : Python 3.9.8

### Rust Implementation(Run time)

| Size     | Linear Search | Binary Search | Binary Search(Default) |
|----------|---------------|---------------|------------------------|
| 10       | 54ns          | 48ns          | 48ns                   |
| 100      | 106ns         | 52ns          | 53ns                   |
| 1000     | 688ns         | 55ns          | 57ns                   |
| 10000    | 6.523µs       | 60ns          | 65ns                   |
| 100000   | 65.468µs      | 72ns          | 74ns                   |
| 1000000  | 516.199µs     | 61ns          | 62ns                   |
| 10000000 | 7.4088ms      | 112ns         | 93ns                   |

### Python Implementation(Run Time)

| Size     | Linear Search           | Binary Search           | Binary Search(Recursive) |
|----------|-------------------------|-------------------------|--------------------------|
| 10       | 0.0049000000004184585ms | 0.001100000000064938ms  | 0.006799999998463591ms   |
| 100      | 0.004899999998997373ms  | 0.0044999999985861905ms | 0.004300000001933313ms   |
| 1000     | 0.07269999999834909ms   | 0.08769999999884703ms   | 0.06350000000026057ms    |
| 10000    | 0.5787999999995463ms    | 0.5505000000013638ms    | 0.5918000000008306ms     |
| 100000   | 6.459200000000465ms     | 5.421299999997586ms     | 5.705200000001298ms      |
| 1000000  | 161.5052ms              | 160.71920000000006ms    | 160.48179999999996ms     |
| 10000000 | 1930.2393999999993ms    | 1928.661200000001ms     | 1933.7299999999984ms     |
