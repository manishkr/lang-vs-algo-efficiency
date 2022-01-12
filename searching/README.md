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
| 10       | 38ns          | 32ns          | 36ns                   |
| 100      | 108ns         | 37ns          | 38ns                   |
| 1000     | 729ns         | 44ns          | 50ns                   |
| 10000    | 7.009µs       | 63ns          | 71ns                   |
| 100000   | 70.522µs      | 143ns         | 133ns                  |
| 1000000  | 528.566µs     | 98ns          | 78ns                   |
| 10000000 | 9.301706ms    | 469ns         | 468ns                  |

### Python Implementation(Run Time)

| Size     | Linear Search           | Binary Search           | Binary Search(Recursive) |
|----------|-------------------------|-------------------------|--------------------------|
| 10       | 0.0012000000012335477ms | 0.0011999999983913767ms | 0.001300000000981072ms   |
| 100      | 0.004500000001428361ms  | 0.004399999998838666ms  | 0.0038999999986799594ms  |
| 1000     | 0.04319999999893298ms   | 0.04320000000177515ms   | 0.04310000000060654ms    |
| 10000    | 0.47560000000004266ms   | 0.47459999999972524ms   | 0.4807999999997037ms     |
| 100000   | 6.288799999998673ms     | 5.176499999997475ms     | 5.385399999997276ms      |
| 1000000  | 159.7464999999999ms     | 175.0743ms              | 159.74090000000007ms     |
| 10000000 | 1922.6735999999996ms    | 1898.8289999999993ms    | 1906.3981999999983ms     |
