use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use std::boxed::Box;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Replacement {
  from: String,
  to: String
}

struct CalibrationMachine {
  unique_molecules: Box<HashSet<String>>
}

impl CalibrationMachine {
  fn new() -> Self {
    CalibrationMachine {
      unique_molecules: Box::new(HashSet::new())
    }
  }

  fn permute(&mut self, target: &String, replacements: &Vec<Replacement>) -> usize {

    unsafe {
      for replacement in replacements {
        for i in 0 .. target.len() - (replacement.from.len() - 1) {
          if target.slice_unchecked(i, i+replacement.from.len()) == replacement.from {
            if i == 0 {
              (*self.unique_molecules).insert(format!("{}{}", replacement.to, target.slice_unchecked(replacement.from.len(), target.len())));
            } else if target.len() - replacement.from.len() == i {
              (*self.unique_molecules).insert(format!("{}{}", target.slice_unchecked(0, i), replacement.to));
            } else {
              (*self.unique_molecules).insert(format!("{}{}{}", target.slice_unchecked(0, i), replacement.to, target.slice_unchecked(i+replacement.from.len(), target.len())));
            }
          }
        }
      }
    }

    return (*self.unique_molecules).len();
  }
}

fn part1 (input: String) -> String {
  let mut buffer = String::new();
  let mut f = File::open(Path::new(&input)).unwrap();
  let _ = f.read_to_string(&mut buffer);

  let lines: Vec<&str> = buffer.lines().collect();

  let target:String = lines[lines.len()-1].to_string();
  let mut replacements:Vec<Replacement> = vec!();

  for i in 0..lines.len()-2 {
    let parts: Vec<&str> = lines[i].split(' ').collect();

    replacements.push(Replacement {
      from: parts[0].to_string(),
      to: parts[2].to_string()
    });
  }

  let mut calibration_machine = CalibrationMachine::new();
  let count = calibration_machine.permute(&target, &replacements);

  return count.to_string();
}

struct MedicineMachine {
}

impl MedicineMachine {
  fn new() -> Self {
    MedicineMachine {}
  }

  fn permute(&mut self, steps:usize, target: &String, replacements: &Vec<Replacement>) -> usize {

    if target == "e" {
      return steps;
    }

    unsafe {
      for replacement in replacements {

        // Skip a replacement that won't work for us
        if target.len() < replacement.to.len() {
          continue;
        }

        for i in 0 .. target.len() - (replacement.to.len() - 1) {
          if target.slice_unchecked(i, i+replacement.to.len()) == replacement.to {

            let candidate = if i == 0 {
              format!("{}{}", replacement.from, target.slice_unchecked(replacement.to.len(), target.len()))
            } else if target.len() - replacement.to.len() == i {
              format!("{}{}", target.slice_unchecked(0, i), replacement.from)
            } else {
              format!("{}{}{}", target.slice_unchecked(0, i), replacement.from, target.slice_unchecked(i+replacement.to.len(), target.len()))
            };

            let result = self.permute(steps + 1, &candidate, replacements);
            if result != 0 {
              return result;
            }
          }
        }
      }
    }

    return 0;
  }
}

fn part2 (input: String) -> String {
  let mut buffer = String::new();
  let mut f = File::open(Path::new(&input)).unwrap();
  let _ = f.read_to_string(&mut buffer);

  let lines: Vec<&str> = buffer.lines().collect();

  let target:String = lines[lines.len()-1].to_string();
  let mut replacements:Vec<Replacement> = vec!();

  for i in 0..lines.len()-2 {
    let parts: Vec<&str> = lines[i].split(' ').collect();

    replacements.push(Replacement {
      from: parts[0].to_string(),
      to: parts[2].to_string()
    });
  }

  // Sort replacements so largest "from" is first.
  replacements.sort_by(|a, b| a.from.len().cmp(&b.from.len()));

  let mut medicine_machine = MedicineMachine::new();
  let count = medicine_machine.permute(0, &target, &replacements);

  return count.to_string();
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "./src/calendar/day19/input".to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "518".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "200".to_string());
}
