use std::str::CharRange;

fn part1 (input: String) -> String  {
  let mut i:usize = 0;
  let mut total:i32 = 0;
  while i < input.len() {
      let CharRange {ch, next} = input.char_range_at(i);
      if ch == '(' {
        total += 1;
      } else if ch == ')' {
        total -= 1;
      }
      i = next;
  }

  return total.to_string();
}

fn part2 (input: String) -> String  {
  let mut i:usize = 0;
  let mut total:i32 = 0;
  while i < input.len() {
      let CharRange {ch, next} = input.char_range_at(i);
      if ch == '(' {
        total += 1;
      } else if ch == ')' {
        total -= 1;
      }

      i = next;

      if total < 0 {
        return i.to_string();
      }
  }

  return i.to_string();
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

#[test]
fn test_part1() {
  let day = fill();
  assert_eq!((day.part1.run)(day.input.to_string()), "232".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "1783".to_string());
}
