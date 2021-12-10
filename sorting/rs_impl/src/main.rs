use std::fs;
use std::ops::Div;
use std::str::FromStr;
use std::time::{Duration, Instant};

struct SortData {
    size: usize,
    default_sort_time: Duration,
    quick_sort_time: Duration,
    merge_sort_time: Duration,
}

fn partition(input: &mut Vec<i64>, start: isize, end: isize) -> isize {
    let pivot = input[end as usize];
    let mut i = start - 1;

    for j in start..end + 1 {
        if input[j as usize] <= pivot {
            i += 1;
            input.swap(i as usize, j as usize);
        }
    }

    return i;
}

fn do_quick_sort(input: &mut Vec<i64>, start: isize, end: isize) {
    if start >= end {
        return;
    }
    let pivot = partition(input, start, end);

    do_quick_sort(input, start, pivot - 1);
    do_quick_sort(input, pivot + 1, end);
}

fn quick_sort(input: &mut Vec<i64>) {
    let last_index = input.len() as isize - 1;
    do_quick_sort(input, 0, last_index);
}

fn merge(input: &mut Vec<i64>, left: &Vec<i64>, right: &Vec<i64>) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            input[k] = left[i];
            i += 1;
        } else {
            input[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    if i < left.len() {
        input[k..].copy_from_slice(&left[i..])
    }

    if j < right.len() {
        input[k..].copy_from_slice(&right[j..])
    }
}

fn merge_sort(input: &mut Vec<i64>) {
    if input.len() <= 1 {
        return;
    }

    let mid = input.len() / 2;
    let left = &input[0..mid];
    let right = &input[mid..];

    let mut left_vec = left.to_vec();
    let mut right_vec = right.to_vec();

    merge_sort(&mut left_vec);
    merge_sort(&mut right_vec);

    merge(input, &left_vec, &right_vec)
}

fn default_sort(input: &mut Vec<i64>) {
    input.sort()
}

fn read_all<T: FromStr>(file_name: &str) -> Vec<i64> {
    fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn runtime_average(
    input: &Vec<i64>,
    count: usize,
    sorter: &dyn Fn(&mut Vec<i64>) -> (),
) -> Duration {
    let mut sum = Duration::new(0, 0);
    for _ in 0..count {
        let mut input_clone = input.to_vec();
        let now = Instant::now();
        sorter(&mut input_clone);
        sum += now.elapsed();
    }
    return sum.div(count as u32);
}

fn main() {
    let files = fs::read_dir("../inputs").unwrap();
    let mut sort_data_vec = Vec::new();
    for file in files {
        let file_name = file.unwrap().path().display().to_string();
        let input = read_all::<String>(&file_name[..]);
        let default_sort_duration = runtime_average(&input, 10, &default_sort);
        let quick_sort_duration = runtime_average(&input, 10, &quick_sort);
        let merge_sort_duration = runtime_average(&input, 10, &merge_sort);
        let sort_data = SortData {
            size: input.len(),
            default_sort_time: default_sort_duration,
            quick_sort_time: quick_sort_duration,
            merge_sort_time: merge_sort_duration,
        };
        sort_data_vec.push(sort_data);
    }
    sort_data_vec.sort_by(|a, b| a.size.cmp(&b.size));

    for item in sort_data_vec {
        println!(
            "|{:?} | {:?} | {:?} | {:?} |",
            item.size, item.default_sort_time, item.quick_sort_time, item.merge_sort_time
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let files = fs::read_dir("../inputs").unwrap();
        for file in files {
            let file_name = file.unwrap().path().display().to_string();
            let input = read_all::<String>(&file_name[..]);
            let mut input_clone_1 = input.to_vec();
            let mut input_clone_2 = input.to_vec();
            quick_sort(&mut input_clone_1);
            input_clone_2.sort();
            assert_eq!(input_clone_1, input_clone_2)
        }
    }
    #[test]
    fn test_merge_sort() {
        let files = fs::read_dir("../inputs").unwrap();
        for file in files {
            let file_name = file.unwrap().path().display().to_string();
            let input = read_all::<String>(&file_name[..]);
            let mut input_clone_1 = input.to_vec();
            let mut input_clone_2 = input.to_vec();
            merge_sort(&mut input_clone_1);
            input_clone_2.sort();
            assert_eq!(input_clone_1, input_clone_2)
        }
    }
}
