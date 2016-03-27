use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::collections::HashSet;
use std::iter::FromIterator;

const TARGET:u8 = 150;

fn test_matches(matches: &mut HashSet<String>, current: String, total_match_count: &mut u32, sizes:&Vec<u8>, running_total:u8, index: usize) {

  let new_total = running_total + sizes[index];

  let new_string:String = if current.len() == 0 { index.to_string() } else { current.clone() + "-" + &index.to_string() };

  if new_total == TARGET {
    if !matches.contains(&new_string) {
      *total_match_count += 1;
      matches.insert(new_string.clone());
    }
  }

  if new_total < TARGET {
    // If we are still under target, recurse into each remaining item.
    for i in index + 1..sizes.len() {
      test_matches(matches, new_string.clone(), total_match_count, sizes, new_total, i);
    }
  }

  // This covers the case where we skip index.
  if index + 1 < sizes.len() {
    test_matches(matches, current, total_match_count, sizes, running_total, index + 1);
  }
}

fn part1 (input: String) -> String {
  let mut buffer = String::new();
  let mut f = File::open(Path::new(&input)).unwrap();
  let _ = f.read_to_string(&mut buffer);

  let mut sizes:Vec<u8> = vec!();
  let mut total_matches:u32 = 0;

  let mut matches:HashSet<String> = HashSet::new();

  let lines: Vec<&str> = buffer.lines().collect();
  for line in lines {
    sizes.push(line.parse::<u8>().unwrap());
  }

  sizes.sort();
  sizes.reverse();

  test_matches(&mut matches, "".to_string(), &mut total_matches, &sizes, 0, 0);

  return total_matches.to_string();
}

fn part2 (input: String) -> String {
  let mut buffer = String::new();
  let mut f = File::open(Path::new(&input)).unwrap();
  let _ = f.read_to_string(&mut buffer);

  let mut sizes:Vec<u8> = vec!();
  let mut total_matches:u32 = 0;

  let mut matches:HashSet<String> = HashSet::new();

  let lines: Vec<&str> = buffer.lines().collect();
  for line in lines {
    sizes.push(line.parse::<u8>().unwrap());
  }

  sizes.sort();
  sizes.reverse();

  test_matches(&mut matches, "".to_string(), &mut total_matches, &sizes, 0, 0);

  // Sort matches by count of -.
  let mut match_vec = Vec::from_iter(matches.iter());
  match_vec.sort_by(|a, b| a.split('-').count().cmp(&b.split('-').count()));

  let mut count = 0;
  let length = match_vec[0].split('-').count(); 
  for entry in match_vec {
    if length == entry.split('-').count() {
      count += 1;
    } else {
      break;
    }
  }

  return count.to_string();
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "./src/calendar/day17/input".to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "1638".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "17".to_string());
}
