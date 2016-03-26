use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
struct Aunt {
  children: i8,
  cats: i8,
  samoyeds: i8,
  pomeranians: i8,
  akitas: i8,
  vizslas: i8,
  goldfish: i8,
  trees: i8,
  cars: i8,
  perfumes: i8,
}

impl Aunt {
  fn new() -> Self {
    Aunt {
      children: -1, cats: -1, samoyeds: -1, pomeranians: -1, akitas: -1,
      vizslas: -1, goldfish: -1, trees: -1, cars: -1, perfumes: -1
    }
  }
}

impl PartialEq for Aunt {
  fn eq(&self, other: &Self) -> bool {
    (other.children == -1 || self.children == other.children) &&
    (other.cats == -1 || self.cats == other.cats) &&
    (other.samoyeds == -1 || self.samoyeds == other.samoyeds) &&
    (other.pomeranians == -1 || self.pomeranians == other.pomeranians) &&
    (other.akitas == -1 || self.akitas == other.akitas) &&
    (other.vizslas == -1 || self.vizslas == other.vizslas) &&
    (other.goldfish == -1 || self.goldfish == other.goldfish) &&
    (other.trees == -1 || self.trees == other.trees) &&
    (other.cars == -1 || self.cars == other.cars) &&
    (other.perfumes == -1 || self.perfumes == other.perfumes)
  }
}

fn part1 (input: String) -> String {
  let mut buffer = String::new();
  let mut f = File::open(Path::new(&input)).unwrap();
  let _ = f.read_to_string(&mut buffer);

  let mut exemplar:Aunt = Aunt::new();
  exemplar.children = 3;
  exemplar.cats = 7;
  exemplar.samoyeds = 2;
  exemplar.pomeranians = 3;
  exemplar.akitas = 0;
  exemplar.vizslas = 0;
  exemplar.goldfish = 5;
  exemplar.trees = 3;
  exemplar.cars = 2;
  exemplar.perfumes = 1;

  let lines: Vec<&str> = buffer.lines().collect();
  for line in lines {
    let parts: Vec<&str> = line.split(' ').collect();
    let name = parts[1].split(':').next().unwrap().to_string();

    let mut aunt: Aunt = Aunt::new();

    let mut i = 2;
    while i < parts.len() {
      let type_name = parts[i].split(':').next().unwrap();
      let value = parts[i+1].split(',').next().unwrap().parse::<i8>().unwrap();
      match type_name {
        "children" => aunt.children = value,
        "cats" => aunt.cats = value,
        "samoyeds" => aunt.samoyeds = value,
        "pomeranians" => aunt.pomeranians = value,
        "akitas" => aunt.akitas = value,
        "vizslas" => aunt.vizslas = value,
        "goldfish" => aunt.goldfish = value,
        "trees" => aunt.trees = value,
        "cars" => aunt.cars = value,
        "perfumes" => aunt.perfumes = value,
        _ => return "OH NO!".to_string()
      }

      i += 2;
    }

    if exemplar == aunt {
      println!("{}", line);
      println!("{:?}\n{:?}", exemplar, aunt);
      return name;
    }
  }

  return "no match".to_string();
}

fn part2 (input: String) -> String {
  let mut buffer = String::new();
  let mut f = File::open(Path::new(&input)).unwrap();
  let _ = f.read_to_string(&mut buffer);

  return buffer;
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "./src/calendar/day16/input".to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "2696".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "15862900".to_string());
}
