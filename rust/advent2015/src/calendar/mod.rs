
mod day1;

struct Puzzle {
  run: fn() -> String
}

impl Puzzle {
  pub fn new(run: fn () -> String) -> Puzzle
  {
    Puzzle{
      run: run
    }
  }
}

pub struct Day(Puzzle, Puzzle);

pub struct Calendar {
  pub days: Vec<Day>
}

impl Calendar {
  pub fn new() -> Calendar
  {
    Calendar {
      days: Vec::new()
    }
  }
}

