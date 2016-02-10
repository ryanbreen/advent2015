use core::borrow::BorrowMut;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::collections::HashSet;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Connection {
  net_weight: i16,
  partner: String
}

struct Person {
  connections: Box<BTreeSet<Connection>>
}

fn test_connections(start: &String, people: &HashMap<String, Person>) -> i16 {

  let mut visited:HashSet<String> = HashSet::new();
  visited.insert((*start).to_string());

  let mut person = people.get(start).unwrap();

  let mut total:i16 = 0;

  'outer: while visited.len() < people.len() {
    let ref connections:BTreeSet<Connection> = *(person.connections);
    for connection in connections.iter().rev() {
      if !visited.contains(&connection.partner) {
        visited.insert(connection.partner.clone());
        total += connection.net_weight;
        person = people.get(&connection.partner).unwrap();
        continue 'outer;
      }
    }
  }

  // Finish it off by looping the final to the start.
  for connection in person.connections.iter() {
    if connection.partner == *start {
      total += connection.net_weight;
      break;
    }
  }

  return total;
}

fn part1(input: String) -> String  {

  let mut buffer = String::new();
  let mut f = File::open(Path::new(&input)).unwrap();
  let _ = f.read_to_string(&mut buffer);

  let mut people:HashMap<String, Person> = HashMap::new();

  let lines: Vec<&str> = buffer.lines().collect();
  for line in lines {
    let parts: Vec<&str> = line.split(' ').collect();

    let partner = parts[10].split('.').next().unwrap();

    let connection_parts = [ (parts[0], partner), (partner, parts[0]) ];
    for connection_part in &connection_parts {
      let principal = match people.entry(connection_part.0.to_string()) {
        Entry::Occupied(o) => o.into_mut(),
        Entry::Vacant(v) => v.insert(Person { connections: Box::new(BTreeSet::new()) })
      };

      let ref mut connections:BTreeSet<Connection> = *(principal.connections.borrow_mut());
      let mut connection_weight = 0;

      let stub_connection = Connection { partner: "fake".to_string(), net_weight: 0 };
      let mut matched_connection:Connection = stub_connection.clone();

      for connection in connections.iter().cloned() {
        if connection.partner == connection_part.1 {
          connection_weight = connection.net_weight;
          matched_connection = connection.clone();
        }
      }

      if matched_connection != stub_connection {
        connections.remove(&matched_connection);
      }

      if parts[2] == "gain" {
        connection_weight += parts[3].parse::<i16>().unwrap();
      } else {
        connection_weight -= parts[3].parse::<i16>().unwrap();
      }
      connections.insert(Connection { net_weight: connection_weight, partner: connection_part.1.to_string() });
    }
  }

  // At this point, we have all of our people and connections in a heap.  Let's iterate from person to person to
  // find the most utilitarian arrangement.
  let mut highest:i16 = 0;
  for (person, _) in &people {
    let current = test_connections(&person, &people);
    if current > highest {
      highest = current;
    }
  }

  return highest.to_string();
}

fn part2 (input: String) -> String {
  return input;
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "./src/calendar/day13/input".to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "709".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "909".to_string());
}
