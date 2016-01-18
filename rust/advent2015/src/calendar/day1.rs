use std::str::CharRange;

fn part1 () -> &'static str  {
  let input = "(())";
  let mut i = 0;
  while i < input.len() {
      let CharRange {ch, next} = input.char_range_at(i);
      println!("{}: {}", i, ch);
      i = next;
  }

  return stringify!(i)
}

pub const day1 : super::Day = super::Day {
  part1: super::Puzzle {
    run: part1,
  },
  part2: super::Puzzle {
    run: part1,
  }
};