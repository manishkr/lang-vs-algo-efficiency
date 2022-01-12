use std::fs;
use std::ops::Div;
use std::str::FromStr;
use std::time::{Duration, Instant};

struct SearchData {
  size: usize,
  linear_search_time: Duration,
  binary_search_time: Duration,
  default_search_duration: Duration,
}

fn linear_search(input: &Vec<i64>, element: i64) -> Result<usize, usize> {
  for (index, value) in input.iter().enumerate() {
    if element == *value {
      return Ok(index);
    }
  }

  return Err(0);
}

fn binary_search(input: &Vec<i64>, element: i64) -> Result<usize, usize> {
  let mut start = 0;
  let mut end = input.len();
  while start < end {
    let mid = start + (end - start) / 2;
    if let Some(current) = input.get(mid) {
      if *current > element {
        end = mid;
      } else if *current < element {
        start = mid + 1;
      } else {
        return Ok(mid);
      }
    }
  }

  return Err(start);
}

fn default_search(input: &Vec<i64>, element: i64) -> Result<usize, usize> {
  return input.binary_search(&element);
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
  searcher: &dyn Fn(&Vec<i64>, i64) -> Result<usize, usize>,
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
    let mut input = read_all::<String>(&file_name[..]);
    input.sort();
    let element = input.last().unwrap();
    let linear_search_duration = runtime_average(&input, *element, 10, &linear_search);
    let binary_search_duration = runtime_average(&input, *element, 10, &binary_search);
    let default_search_duration = runtime_average(&input, *element, 10, &default_search);
    let sort_data = SearchData {
      size: input.len(),
      linear_search_time: linear_search_duration,
      binary_search_time: binary_search_duration,
      default_search_duration: default_search_duration,
    };
    search_data_vec.push(sort_data);
  }
  search_data_vec.sort_by(|a, b| a.size.cmp(&b.size));

  for item in search_data_vec {
    println!(
      "|{:?} | {:?} | {:?} | {:?} |",
      item.size,
      item.linear_search_time,
      item.binary_search_time,
      item.default_search_duration
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rand::Rng;

  #[test]
  fn test_linear_search() {
    let files = fs::read_dir("../inputs").unwrap();
    for file in files {
      let file_name = file.unwrap().path().display().to_string();
      let input = read_all::<String>(&file_name[..]);
      let mut rng = rand::thread_rng();
      let element = rng.gen_range(0..input.len() as i64);
      let index = linear_search(&input, element);
      let default_index = match input.iter().position(|&x| x == element) {
        Some(i) => Ok(i),
        None => Err(0usize),
      };
      assert_eq!(index, default_index);
    }
  }

  #[test]
  fn test_binary_search() {
    let files = fs::read_dir("../inputs").unwrap();
    for file in files {
      let file_name = file.unwrap().path().display().to_string();
      let mut input = read_all::<String>(&file_name[..]);
      input.sort();
      input.dedup();
      let mut rng = rand::thread_rng();
      let element = rng.gen_range(0..input.len() as i64);
      let index = binary_search(&input, element);
      let default_index = input.binary_search(&element);
      assert_eq!(index, default_index);
    }
  }
}
