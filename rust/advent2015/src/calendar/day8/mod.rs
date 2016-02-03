use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn part1(input: String) -> String  {
  let mut f = File::open(Path::new(&input)).unwrap();
  let mut buffer = vec![0; 10];
  // Read the whole file to figure out how many raw bytes we have.
  let raw_len = f.read_to_end(&mut buffer).unwrap();

  let mut buffer = String::new();
  let mut f = File::open(Path::new(&input)).unwrap();
  let _ = f.read_to_string(&mut buffer);

  // Now count number of newlines
  let lines: Vec<&str> = buffer.lines().collect();
  let newline_count = lines.len() - 1;
  println!("There are {} newlines", newline_count);

  let mut total_str = 0;

  'outer: for line in lines {
    let mut chars = line.chars();
    chars.next(); // Skip the intro "

    let mut c = chars.next();
    while c != None {
      match c {
        Some('\\') => {
          total_str += 1;

          // Escape sequence.  Is it a single char or a hex escape?
          c = chars.next();
          match c {
            Some('x') => {
              // If this is a hex escape, skip 2 more chars
              chars.next();
              chars.next();
              c = chars.next();
            },
            _ => c = chars.next(),
          }
        },
        None => continue 'outer,
        _ => {
          c = chars.next();
          if c != None {
            total_str += 1;
          }
        },
      }
    }
  }

  let total_raw = raw_len - newline_count;
  return (total_raw - total_str).to_string();
}

fn part2 (input: String) -> String  {
  let mut f = File::open(Path::new(&input)).unwrap();
  let mut buffer = vec![0; 10];
  // Read the whole file to figure out how many raw bytes we have.
  let raw_len = f.read_to_end(&mut buffer).unwrap();

  let mut buffer = String::new();
  let mut f = File::open(Path::new(&input)).unwrap();
  let _ = f.read_to_string(&mut buffer);

  // Now count number of newlines
  let lines: Vec<&str> = buffer.lines().collect();
  let newline_count = lines.len() - 1;

  let mut total_str = 0;

  for line in lines {
    let escaped = line.escape_default();
    total_str += escaped.len() + 2;
  }

  let total_raw = raw_len - newline_count;
  return (total_str - total_raw).to_string();
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "./src/calendar/day8/input".to_string(),
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
