use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[derive(PartialEq, Eq)]
enum State {
  Travel,
  Rest
}

struct Reindeer {
  name: String,
  travel_rate: u8,
  travel_duration: u16,
  rest_duration: u16,
  state: State,
  countdown: u16,
  distance_traveled: u16
}

impl Reindeer {
  fn new(name: String, travel_rate: u8, travel_duration: u16, rest_duration: u16) -> Reindeer {
    Reindeer { name: name, travel_rate: travel_rate, travel_duration: travel_duration, rest_duration: rest_duration,
      state: State::Travel, countdown: travel_duration, distance_traveled: 0 }
  }

  fn tick(&mut self) {

    if self.state == State::Travel {
      self.distance_traveled += self.travel_rate as u16;
    }

    self.countdown-=1;

    if self.countdown == 0 {
      // State transition
      match self.state {
        State::Travel => {
          self.countdown = self.rest_duration;
          self.state = State::Rest;
        },
        State::Rest => {
          self.countdown = self.travel_duration;
          self.state = State::Travel;
        }
      }
    }
  }
}

fn part1 (input: String) -> String {
  let mut buffer = String::new();
  let mut f = File::open(Path::new(&input)).unwrap();
  let _ = f.read_to_string(&mut buffer);

  let mut reindeer: Vec<Reindeer> = Vec::new();

  let lines: Vec<&str> = buffer.lines().collect();
  for line in lines {
    let parts: Vec<&str> = line.split(' ').collect();
    reindeer.push(Reindeer::new(
      parts[0].to_string(),
      parts[3].parse::<u8>().unwrap(),
      parts[6].parse::<u16>().unwrap(),
      parts[13].parse::<u16>().unwrap())
    );
  }

  for _ in 0..2503 {
    for deer in &mut reindeer {
      deer.tick();
    }
  }

  let mut highest = 0;

  for deer in &reindeer {
    println!("{} went {}km", deer.name, deer.distance_traveled);
    if deer.distance_traveled > highest {
      highest = deer.distance_traveled;
    }
  }

  return highest.to_string();
}

fn part2 (input: String) -> String {
  let mut buffer = String::new();
  let mut f = File::open(Path::new(&input)).unwrap();
  let _ = f.read_to_string(&mut buffer);

  return buffer;
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "./src/calendar/day14/input".to_string(),
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
  assert_eq!((day.part2.run)(day.input.to_string()), "668".to_string());
}
