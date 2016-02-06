use std::str::CharRange;

fn permute(input: String) -> String {

  let mut i:usize = 1;
  let CharRange {ch, ..} = input.char_range_at(0);
  let mut last_char = ch;
  let mut last_char_count:usize = 1;
  let mut rvalue:String = String::new();

  while i < input.len() {
    let CharRange {ch, next} = input.char_range_at(i);
    if ch != last_char {
      // We need to add the count of the last char to the return string.
      rvalue.push_str(&last_char_count.to_string());
      rvalue.push(last_char);
      last_char_count = 1;
      last_char = ch;
    } else {
      last_char_count += 1;
    }
    i = next;
  }

  rvalue.push_str(&last_char_count.to_string());
  rvalue.push(last_char);

  return rvalue;
}

fn part1(input: String) -> String  {
  let mut permuted = input;
  for _ in 0..40 {
    permuted = permute(permuted);
  }

  return permuted.len().to_string();
}

fn part2 (input: String) -> String  {
  let mut permuted = input;
  for _ in 0..50 {
    permuted = permute(permuted);
  }

  return permuted.len().to_string();
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "3113322113".to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "329356".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "4666278".to_string());
}
