use std::str::CharRange;

fn calculate_wrapping_paper_needs (input: String) -> u32 {

  let mut parts = input.split("x").collect::<Vec<&str>>();

  let mut sides = Vec::new();
  sides.push(parts[0].parse::<u32>().unwrap());
  sides.push(parts[1].parse::<u32>().unwrap());
  sides.push(parts[2].parse::<u32>().unwrap());

  sides.sort();

  let mut total = 0;
  total += sides[0]*sides[1]*2;
  total += sides[1]*sides[2]*2;
  total += sides[0]*sides[2]*2;
  total += sides[0]*sides[1];

  return total;
}

fn part1 (input: String) -> String  {

  let mut total = 0;

  let lines: Vec<&str> = input.lines().collect();
  print!("{}\n", lines.len());
  for line in lines {
    print!("{}\n", line);
    total += calculate_wrapping_paper_needs(line.to_string());
  }

  return total.to_string()
}

fn part2 (input: String) -> String  {
  return "noop".to_string()
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