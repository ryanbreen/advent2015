
mod day1;

pub struct Puzzle {
  pub run: fn() -> &'static str
}

impl Puzzle {
  pub fn new(run: fn () -> &'static str) -> Puzzle
  {
    Puzzle{
      run: run
    }
  }
}

pub struct Day {
  pub part1: Puzzle,
  pub part2: Puzzle
}

pub struct Calendar {
  pub days: Vec<Day>
}

impl Calendar {
  pub fn new() -> Calendar
  {
    let mut days = Vec::new();
    days.push(day1::day1);

    Calendar {
      days: days
    }
  }
}

