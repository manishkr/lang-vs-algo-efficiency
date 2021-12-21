use std::fs;
use std::ops::Div;
use std::str::FromStr;
use std::time::{Duration, Instant};

use rand::Rng;

struct SearchData {
  size: usize,
  linear_search_time: Duration,
  binary_search_time: Duration,
}

fn linear_search(input: &Vec<i64>, element: i64) -> isize {
  let mut element_index = -1isize;
  for (index, value) in input.iter().enumerate() {
    if element == *value {
      element_index = index as isize;
      break;
    }
  }

  return element_index;
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
  element: i64,
  count: usize,
  searcher: &dyn Fn(&Vec<i64>, i64) -> isize,
) -> Duration {
  let mut sum = Duration::new(0, 0);
  for _ in 0..count {
    let input_clone = input.to_vec();
    let now = Instant::now();
    searcher(&input_clone, element);
    sum += now.elapsed();
  }
  return sum.div(count as u32);
}

fn main() {
  let files = fs::read_dir("../inputs").unwrap();
  let mut search_data_vec = Vec::new();
  for file in files {
    let file_name = file.unwrap().path().display().to_string();
    let input = read_all::<String>(&file_name[..]);
    let element = input.last().unwrap();
    let linear_search_duration = runtime_average(&input, *element, 10, &linear_search);
    let sort_data = SearchData {
      size: input.len(),
      linear_search_time: linear_search_duration,
      binary_search_time: linear_search_duration,
    };
    search_data_vec.push(sort_data);
  }
  search_data_vec.sort_by(|a, b| a.size.cmp(&b.size));

  for item in search_data_vec {
    println!(
      "|{:?} | {:?} | {:?} |",
      item.size, item.linear_search_time, item.binary_search_time
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
      let mut rng = rand::thread_rng();
      let element = rng.gen_range(0..input.len() as i64);
      let index = linear_search(&input, element);
      let default_index = match input.iter().position(|&x| x == element) {
        None => None,
        Some(i) => Some(i as isize)
      };
      assert_eq!(index, default_index.unwrap_or(-1));
    }
  }
}
