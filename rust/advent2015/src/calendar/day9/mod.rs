use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::collections::HashSet;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Route {
  distance: u16,
  destination: String
}

struct City {
  name: String,
  routes: Box<BinaryHeap<Route>>
}

fn part1(input: String) -> String  {

  let mut buffer = String::new();
  let mut f = File::open(Path::new(&input)).unwrap();
  let _ = f.read_to_string(&mut buffer);

  let mut cities:HashMap<String, City> = HashMap::new();

  let lines: Vec<&str> = buffer.lines().collect();
  for line in lines {
    let parts: Vec<&str> = line.split(' ').collect();

    let route_parts = [ (parts[0], parts[2]), (parts[2], parts[0]) ];
    for route_legs in &route_parts {
      let city = match cities.entry(route_legs.0.to_string()) {
        Entry::Occupied(o) => o.into_mut(),
        Entry::Vacant(v) => v.insert(City { name: route_legs.0.to_string(), routes: Box::new(BinaryHeap::new()) })
      };

      let my_route = Route { distance: parts[4].parse::<u16>().unwrap(), destination: route_legs.1.to_string() };
      println!("{} {}", my_route.distance, my_route.destination);
    }

  }

  return input.to_string();
}

fn part2 (input: String) -> String  {
  return input.to_string();
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "./src/calendar/day9/input".to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "1342".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "2074".to_string());
}
