use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Route {
  distance: u16,
  destination: String
}

struct City {
  name: String,
  routes: Box<BinaryHeap<Route>>
}

fn part1(input: String) -> String  {
  let mut cities:HashMap<String, City> = HashMap::new();

  let lines: Vec<&str> = input.lines().collect();
  for line in lines {
    let parts: Vec<&str> = line.split(' ').collect();

  }

  return input.to_string();
}

fn part2 (input: String) -> String  {
  return input.to_string();
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "./src/calendar/day/input".to_string(),
    part1: super::Puzzle {
      run: part1,
    },
    part2: super::Puzzle {
      run: part2,
    }
  };
}

#[test]
fn test_part1() {
  let day = fill();
  assert_eq!((day.part1.run)(day.input.to_string()), "1342".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "2074".to_string());
}
