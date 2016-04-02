use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

struct Computer {
  a: u64,
  b: u64,
  pc: usize
}

impl Computer {

  fn hlf(&mut self, r:&str) {
    if r == "a" {
      self.a = self.a / 2;
    } else {
      self.b = self.b / 2;
    }

    self.pc += 1;
  }

  fn tpl(&mut self, r:&str) {
    if r == "a" {
      self.a = self.a * 3;
    } else {
      self.b = self.b * 3;
    }

    self.pc += 1;
  }

  fn inc(&mut self, r:&str) {
    if r == "a" {
      self.a = self.a + 1;
    } else {
      self.b = self.b + 1;
    }

    self.pc += 1;
  }

  fn jmp(&mut self, offset:&str) {
    let distance:usize = offset[1..offset.len()].parse::<usize>().unwrap();
    if &offset[0..1] == "+" {
      self.pc += distance;
    } else {
      self.pc -= distance;
    }
  }

  fn jie(&mut self, r:&str, offset:&str) {
    let jmp:bool;;

    if r == "a," {
      jmp = self.a % 2 == 0;
    } else {
      jmp = self.b % 2 == 0;
    }

    if jmp {
      let distance:usize = offset[1..offset.len()].parse::<usize>().unwrap();
      if &offset[0..1] == "+" {
        self.pc += distance;
      } else {
        self.pc -= distance;
      }
    } else {
      self.pc += 1;
    }
  }

  fn jio(&mut self, r:&str, offset:&str) {
    let jmp:bool;

    if r == "a," {
      jmp = self.a == 1;
    } else {
      jmp = self.b == 1;
    }

    if jmp {
      let distance:usize = offset[1..offset.len()].parse::<usize>().unwrap();
      if &offset[0..1] == "+" {
        self.pc += distance;
      } else {
        self.pc -= distance;
      }
    } else {
      self.pc += 1;
    }
  }

  fn run(&mut self, tape: &Vec<&str>) {
    loop {
      if self.pc >= tape.len() {
        return;
      }

      let instruction = tape[self.pc];

      let instruction_parts:Vec<&str> = instruction.split_whitespace().collect();
      match instruction_parts[0] {
        "hlf" => self.hlf(instruction_parts[1]),
        "tpl" => self.tpl(instruction_parts[1]),
        "inc" => self.inc(instruction_parts[1]),
        "jmp" => self.jmp(instruction_parts[1]),
        "jie" => self.jie(instruction_parts[1], instruction_parts[2]),
        "jio" => self.jio(instruction_parts[1], instruction_parts[2]),
        _ => println!("Invalid instruction {}", instruction_parts[0]),//panic!("invalid instruction") },
      }
    }
  }
}

fn part1 (input: String) -> String {
  let mut buffer = String::new();
  let mut f = File::open(Path::new(&input)).unwrap();
  let _ = f.read_to_string(&mut buffer);

  let lines: Vec<&str> = buffer.lines().collect();
  let mut c:Computer = Computer {
    a: 0,
    b: 0,
    pc: 0,
  };
  c.run(&lines);

  return c.b.to_string();
}

fn part2 (input: String) -> String {
  let mut buffer = String::new();
  let mut f = File::open(Path::new(&input)).unwrap();
  let _ = f.read_to_string(&mut buffer);

  let lines: Vec<&str> = buffer.lines().collect();
  let mut c:Computer = Computer {
    a: 1,
    b: 0,
    pc: 0,
  };
  c.run(&lines);

  return c.b.to_string();
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "./src/calendar/day23/input".to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "255".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "334".to_string());
}
