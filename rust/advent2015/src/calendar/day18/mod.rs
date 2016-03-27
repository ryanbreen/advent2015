use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::fmt;

const GRID_SIZE:usize = 100;

struct Game {
  grid: [[bool;GRID_SIZE];GRID_SIZE]
}

impl Game {
  fn new() -> Self {
    Game {
      grid: [[false;GRID_SIZE];GRID_SIZE]
    }
  }

  fn step(&mut self) {
    let mut new_grid: [[bool;GRID_SIZE];GRID_SIZE] = [[false;GRID_SIZE];GRID_SIZE];

    // Walk the grid, collecting its state.
    for x in 0..GRID_SIZE {
      for y in 0..GRID_SIZE {

        let mut lit_neighbors = 0;
        if x > 0 {
          if y > 0 {
            if self.grid[x-1][y-1] {
              lit_neighbors += 1;
            }
          }

          if self.grid[x-1][y] {
            lit_neighbors += 1;
          }

          if y < GRID_SIZE - 1 {
            if self.grid[x-1][y+1] {
              lit_neighbors += 1;
            }
          }
        }

        if y > 0 {
          if self.grid[x][y-1] {
            lit_neighbors += 1;
          }
        }

        if y < GRID_SIZE - 1 {
          if self.grid[x][y+1] {
            lit_neighbors += 1;
          }
        }

        if x < GRID_SIZE - 1 {

          if y > 0 {
            if self.grid[x+1][y-1] {
              lit_neighbors += 1;
            }
          }

          if self.grid[x+1][y] {
            lit_neighbors += 1;
          }

          if y < GRID_SIZE - 1 {
            if self.grid[x+1][y+1] {
              lit_neighbors += 1;
            }
          }
        }

        if self.grid[x][y] {
          if lit_neighbors == 2 || lit_neighbors == 3 {
            new_grid[x][y] = true;
          } else {
            new_grid[x][y] = false;
          }
        } else {
          if lit_neighbors == 3 {
            new_grid[x][y] = true;
          }
        }

      }
    }

    self.grid = new_grid;
  }
}

#[allow(unused_must_use)]
impl fmt::Debug for Game {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for x in 0..GRID_SIZE {
      for y in 0..GRID_SIZE {
        let out = if self.grid[x][y] { '#' } else { '.' };
        write!(f, "{}", out);
      }
      write!(f, "\n");
    }
    Ok(())
  }
}

fn part1 (input: String) -> String {
  let mut buffer = String::new();
  let mut f = File::open(Path::new(&input)).unwrap();
  let _ = f.read_to_string(&mut buffer);

  let mut game = Game::new();

  let lines: Vec<&str> = buffer.lines().collect();
  let mut row = 0;
  for line in lines {
    let mut col = 0;
    for c in line.chars() {
      game.grid[row][col] = c == '#';
      col += 1;
    }
    row += 1;
  }

  for _ in 0..100 {
    game.step();
  }

  let mut count = 0;
  for x in 0..GRID_SIZE {
    for y in 0..GRID_SIZE {
      if game.grid[x][y] {
        count += 1;
      }
    }
  }

  return count.to_string();
}

fn part2 (input: String) -> String {
  let mut buffer = String::new();
  let mut f = File::open(Path::new(&input)).unwrap();
  let _ = f.read_to_string(&mut buffer);

  return "".to_string();
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "./src/calendar/day18/input".to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "814".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "17".to_string());
}
