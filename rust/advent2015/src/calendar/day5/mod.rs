use std::str::CharRange;

fn is_nice(name: String) -> bool {
  let mut i:usize = 0;
  let mut last_ch:char = '_';
  let mut vowels:usize = 0;
  let mut has_double:bool = false;
  while i < name.len() {
      let CharRange {ch, next} = name.char_range_at(i);
      if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
        vowels += 1;
      }

      if last_ch == ch {
        has_double = true;
      }

      if last_ch == 'a' && ch == 'b' {
        return false;
      }

      if last_ch == 'c' && ch == 'd' {
        return false;
      }

      if last_ch == 'p' && ch == 'q' {
        return false;
      }

      if last_ch == 'x' && ch == 'y' {
        return false;
      }

      last_ch = ch;
      i = next;
  }

  return has_double && vowels >= 3;
}

fn part1 (input: String) -> String  {

  let mut total = 0;

  let lines: Vec<&str> = input.lines().collect();
  for line in lines {
    if is_nice(line.to_string()) {
      total += 1;
    }
  }

  return total.to_string()
}

fn part2 (input: String) -> String  {
  return input;
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