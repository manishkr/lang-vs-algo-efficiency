# Languate vs Algorithm Efficiency
- This is implementation like most developer do not the most optimized like using unsafe code for rust and numpy for Python
- Complexity
  - Rust
    - Default Sort : https://doc.rust-lang.org/std/primitive.slice.html#method.sort
    - Quick Sort : Average Case : ```O(n * log_n)``` Worst case: ```O(n^2)```
    - Merge Sort: Average Case : ```Θ(n * log_n)``` Worst Case ```O(n * log_n)```
  - Python
    - Default Sort: https://bugs.python.org/file4451/timsort.txt
    - Quick Sort : Average Case : ```O(n * log_n)``` Worst case: ```O(n^2)```
    - Merge Sort: Average Case : ```Θ(n * log_n)``` Worst Case ```O(n * log_n)```
### Hardware/Software
- CPU : 2.3 GHz 8-Core Intel Core i9
- RAM : 16GB
- OS : macOS Monterey 12.0.1
- Rust : rustc 1.57.0
- Python : Python 3.9.8

### Rust Implementation(Run time)

| Size     | Default Sort | Quick Sort   | Merge Sort   |
|----------|--------------|--------------|--------------|
| 10       | 91ns         | 106ns        | 3.137µs      |
| 100      | 1.978µs      | 1.457µs      | 15.51µs      |
| 1000     | 31.712µs     | 40.676µs     | 213.48µs     |
| 10000    | 586.81µs     | 542.045µs    | 2.080679ms   |
| 100000   | 7.186128ms   | 6.323906ms   | 22.664707ms  |
| 1000000  | 68.630922ms  | 72.875506ms  | 288.602272ms |
| 10000000 | 735.242043ms | 781.060741ms | 2.561232739s |

### Python Implementation(Run Time)

| Size     | Default Sort            | Quick Sort             | Merge Sort             |
|----------|-------------------------|------------------------|------------------------|
| 10       | 0.0011000000313288183ms | 0.009400000044479384ms | 0.015200000029835792ms |
| 100      | 0.00380000001314329ms   | 0.09279999994760146ms  | 0.189599999953316ms    |
| 1000     | 0.10249999997995474ms   | 1.5739000000394299ms   | 2.6498999999603257ms   |
| 10000    | 1.1754000000337328ms    | 18.804799999998068ms   | 34.494900000004236ms   |
| 100000   | 16.15689999996448ms     | 269.07179999998334ms   | 429.6772000000374ms    |
| 1000000  | 267.7487ms              | 3783.9714ms            | 6054.124400000002ms    |
| 10000000 | 4287.181799999999ms     | 49963.87200000001ms    | 74605.24789999999ms    |
