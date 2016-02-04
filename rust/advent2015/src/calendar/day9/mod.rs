use core::borrow::BorrowMut;
use std::collections::BTreeMap;
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
  routes: Box<BTreeMap<Route, u16>>
}

fn test_route(start: &String, cities: &HashMap<String, City>) -> u16 {

  let mut visited:HashSet<String> = HashSet::new();
  visited.insert((*start).to_string());

  let mut city = cities.get(start).unwrap();

  let mut total:u16 = 0;

  'outer: while visited.len() < cities.len() {
    let ref routes:BTreeMap<Route, u16> = *(city.routes);
    for (route, _) in routes.iter() {
      if !visited.contains(&route.destination) {
        visited.insert(route.destination.clone());
        total += route.distance;
        city = cities.get(&route.destination).unwrap();
        continue 'outer;
      }
    }
  }

  return total;
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
        Entry::Vacant(v) => v.insert(City { routes: Box::new(BTreeMap::new()) })
      };

      let ref mut routes:BTreeMap<Route, u16> = *(city.routes.borrow_mut());
      routes.insert(Route { distance: parts[4].parse::<u16>().unwrap(), destination: route_legs.1.to_string() }, 1);
    }
  }

  // At this point, we have all of our cities and routes in a heap.  Let's iterate from city to city to
  // find shortest path.
  let mut lowest:u16 = 32767;
  for (city_name, _) in &cities {
    let current = test_route(&city_name, &cities);
    if current < lowest {
      lowest = current;
    }
  }

  return lowest.to_string();
}

fn test_route_reverse(start: &String, cities: &HashMap<String, City>) -> u16 {

  let mut visited:HashSet<String> = HashSet::new();
  visited.insert((*start).to_string());

  let mut city = cities.get(start).unwrap();

  let mut total:u16 = 0;

  'outer: while visited.len() < cities.len() {
    let ref routes:BTreeMap<Route, u16> = *(city.routes);
    for (route, _) in routes.iter().rev() {
      if !visited.contains(&route.destination) {
        visited.insert(route.destination.clone());
        total += route.distance;
        city = cities.get(&route.destination).unwrap();
        continue 'outer;
      }
    }
  }

  return total;
}

fn part2 (input: String) -> String  {

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
        Entry::Vacant(v) => v.insert(City { routes: Box::new(BTreeMap::new()) })
      };

      let ref mut routes:BTreeMap<Route, u16> = *(city.routes.borrow_mut());
      routes.insert(Route { distance: parts[4].parse::<u16>().unwrap(), destination: route_legs.1.to_string() }, 1);
    }
  }

  // At this point, we have all of our cities and routes in a heap.  Let's iterate from city to city to
  // find shortest path.
  let mut highest:u16 = 0;
  for (city_name, _) in &cities {
    let current = test_route_reverse(&city_name, &cities);
    if current > highest {
      highest = current;
    }
  }

  return highest.to_string();
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
  assert_eq!((day.part1.run)(day.input.to_string()), "117".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "909".to_string());
}
