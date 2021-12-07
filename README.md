# Languate vs Algorithm Efficiency
- This is implementation like most developer do not the most optimized like using unsafe code for rust and numpy for Python
- Complexity
  - Rust
    - Default Sort : https://doc.rust-lang.org/std/primitive.slice.html#method.sort
    - Quick Sort : Average Case : ```O(n * log_n)``` Worst case: ```O(n^2)```
  - Python
    - Default Sort: https://bugs.python.org/file4451/timsort.txt
    - Quick Sort : Average Case : ```O(n * log_n)``` Worst case: ```O(n^2)```
### Hardware/Software
- CPU : 2.3 GHz 8-Core Intel Core i9
- RAM : 16GB
- OS : macOS Monterey 12.0.1
- Rust : rustc 1.57.0
- Python : Python 3.9.8

### Rust Implementation(Run time)

| Size     | Default Sort | Quick Sort  |
|----------|--------------|-------------|
| 10       | 54ns         | 81ns        |
| 100      | 1.607µs      | 1.15µs      |
| 1000     | 25.546µs     | 34.326µs    |
| 10000    | 389.979µs    | 489.772µs   |
| 100000   | 5.223253ms   | 6.191972ms  |
| 1000000  | 61.696151ms  | 79.374239ms |
| 10000000 | 734.937791ms | 867.62281ms |

### Python Implementation(Run Time)

| Size     | Default Sort            | Quick Sort             |
|----------|-------------------------|------------------------|
| 10       | 0.0013999999964653398ms | 0.009600000009868381ms |
| 100      | 0.004200000000764703ms  | 0.10260000000243963ms  |
| 1000     | 0.0883000000044376ms    | 1.5360000000100626ms   |
| 10000    | 1.150600000016766ms     | 19.615099999975882ms   |
| 100000   | 14.703800000006595ms    | 266.45679999999174ms   |
| 1000000  | 287.5334ms              | 3786.2493999999997ms   |
| 10000000 | 4955.289899999998ms     | 52876.6736ms           |
