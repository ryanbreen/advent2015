use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn test(numbers: &Vec<u8>, running_tally: usize, running_qe: u64, count: usize, target: usize,
  i: usize, lim: usize, lowest_qe: &mut u64) {

  if count > lim {
    return;
  }

  if i >= numbers.len() {
    return;
  }

  // Recurse skipping i
  test(numbers, running_tally, running_qe, count, target, i+1, lim, lowest_qe);

  let my_tally = running_tally + numbers[i] as usize;
  let my_count = count + 1;

  if my_tally > target {
    return;
  }

  let my_qe = running_qe * numbers[i] as u64;

  if my_tally == target && my_qe < *lowest_qe {
    *lowest_qe = my_qe;
  } else {
    // Recurse into all tails
    for j in i+1..numbers.len() {
      //println!("Recursing from {} to a tail beginning at {}", i, j);
      test(numbers, my_tally, my_qe, my_count, target, j, lim, lowest_qe);
    }
  }
}

fn part1 (input: String) -> String {
  let mut buffer = String::new();
  let mut f = File::open(Path::new(&input)).unwrap();
  let _ = f.read_to_string(&mut buffer);

  let lines: Vec<&str> = buffer.lines().collect();
  let mut numbers: Vec<u8> = vec!();

  let mut total:usize = 0;
  for line in lines {
    let num = line.parse::<u8>().unwrap();
    numbers.push(num);
    total += num as usize;
  }

  let partition:usize = total / 3;

  numbers.sort();
  //numbers.reverse();
  println!("{:?}", numbers);

  let mut lowest_qe:u64 = 0xFFFFFFFFFFFFFFFF;

  for i in 1..10 {
    test(&numbers, 0, 1, 0, partition, 0, i, &mut lowest_qe);
    if lowest_qe != 0xFFFFFFFFFFFFFFFF {
      break;
    } else {
      println!("No match at size {}", i);
    }
  }

  return lowest_qe.to_string();
}

fn part2 (input: String) -> String {
  let mut buffer = String::new();
  let mut f = File::open(Path::new(&input)).unwrap();
  let _ = f.read_to_string(&mut buffer);

  let lines: Vec<&str> = buffer.lines().collect();
  let mut numbers: Vec<u8> = vec!();

  let mut total:usize = 0;
  for line in lines {
    let num = line.parse::<u8>().unwrap();
    numbers.push(num);
    total += num as usize;
  }

  let partition:usize = total / 4;

  numbers.sort();
  //numbers.reverse();
  println!("{:?}", numbers);

  let mut lowest_qe:u64 = 0xFFFFFFFFFFFFFFFF;

  for i in 1..10 {
    test(&numbers, 0, 1, 0, partition, 0, i, &mut lowest_qe);
    if lowest_qe != 0xFFFFFFFFFFFFFFFF {
      break;
    } else {
      println!("No match at size {}", i);
    }
  }

  return lowest_qe.to_string();
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "./src/calendar/day24/input".to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "11846773891".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "80393059".to_string());
}
