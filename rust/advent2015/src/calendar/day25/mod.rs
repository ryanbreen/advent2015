
struct Matrix {
  row:u64,
  col:u64,
  val: u64,
  max_row: u64
}

impl Matrix {
  fn step(&mut self) {
    if self.row == 1 {
      self.row = self.max_row + 1;
      self.max_row += 1;
      self.col = 1;
    } else {
      self.col += 1;
      self.row -= 1;
    }
    self.val = (self.val * 252533) % 33554393;
  }
}

fn part1 (_: String) -> String {
  let mut matrix:Matrix = Matrix { row: 1, col: 1, val: 20151125, max_row: 1 };

  loop {
    matrix.step();

    if matrix.row == 2978 && matrix.col == 3083 {
      return matrix.val.to_string();
    }
  }

}

fn part2 (_: String) -> String {
  return 0.to_string();
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "barth".to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "2650453".to_string());
}
