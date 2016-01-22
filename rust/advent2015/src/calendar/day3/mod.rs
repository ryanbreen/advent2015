use std::collections::HashSet;
use std::str::CharRange;

fn part1 (input: String) -> String  {
  let mut x = 0;
  let mut y = 0;

  let mut i:usize = 0;

  let mut visited_houses = HashSet::new();
  visited_houses.insert(format!("{},{}", x, y));

  while i < input.len() {
    let CharRange {ch, next} = input.char_range_at(i);
    if ch == '^' {
      y += 1;
    } else if ch == 'v' {
      y -= 1;
    } else if ch == '>' {
      x += 1;
    } else if ch == '<' {
      x -= 1;
    }
    i = next;

    visited_houses.insert(format!("{},{}", x, y));
  }

  return visited_houses.len().to_string();
}

fn part2 (input: String) -> String  {
  return "noop".to_string();
}

pub fn fill() -> super::Day {
  return super::Day {
    input: include_str!("input").to_string(),
    part1: super::Puzzle {
      run: part1,
    },
    part2: super::Puzzle {
      run: part2,
    }
  };
}