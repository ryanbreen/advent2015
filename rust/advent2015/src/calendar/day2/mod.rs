
fn calculate_wrapping_paper_needs (input: String) -> u32 {

  let parts = input.split("x").collect::<Vec<&str>>();

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
  for line in lines {
    total += calculate_wrapping_paper_needs(line.to_string());
  }

  return total.to_string()
}

fn calculate_ribbon_needs (input: String) -> u32 {

  let parts = input.split("x").collect::<Vec<&str>>();

  let mut sides = Vec::new();
  sides.push(parts[0].parse::<u32>().unwrap());
  sides.push(parts[1].parse::<u32>().unwrap());
  sides.push(parts[2].parse::<u32>().unwrap());

  sides.sort();

  let mut total = 0;
  total += sides[0]*2;
  total += sides[1]*2;
  total += sides[0]*sides[1]*sides[2];

  return total;
}

fn part2 (input: String) -> String  {
  let mut total = 0;

  let lines: Vec<&str> = input.lines().collect();
  for line in lines {
    total += calculate_ribbon_needs(line.to_string());
  }

  return total.to_string()
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
fn test() {
  let day = fill();
  assert_eq!((day.part1.run)(day.input.to_string()), "1598415".to_string());
  assert_eq!((day.part2.run)(day.input.to_string()), "3812909".to_string());
}
