
fn part1(input: String) -> String  {
  return input;
}

fn part2 (input: String) -> String  {
  return input;
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
  assert_eq!((day.part1.run)(day.input.to_string()), "117".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "909".to_string());
}
