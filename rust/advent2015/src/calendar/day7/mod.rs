use std::collections::HashMap;

trait Gate {
  fn calculate(&self, &HashMap<&'static str, &Gate>) -> u16;
}

// A gate that takes a value or wire as its input
#[derive(Hash, Eq, PartialEq, Debug)]
struct PassthroughGate {
  label: &'static str,
  input: &'static str
}

impl PassthroughGate {
  fn new(label: &'static str, input: &'static str) -> PassthroughGate {
    PassthroughGate { label: label, input: input }
  }
}

impl Gate for PassthroughGate {
  fn calculate(&self, gates: &HashMap<&'static str, &Gate>) -> u16 {
    match self.input.parse::<u16>() {
      Ok(value) => return value,
      Err(e) => {
        println!("Looking for {}", self.input);
        return gates[self.input].calculate(gates);
      },
    }
  }
}


fn part1 (input: String) -> String  {

  println!("Allocating values");

  let a = PassthroughGate::new("a", "8");
  let b = PassthroughGate::new("b", "a");

  println!("Creating hash");

  let mut gates:HashMap<&'static str, &Gate> = HashMap::new();
  println!("Created hash");

  gates.insert("a", &a);
  gates.insert("b", &b);

  println!("{}", b.calculate(&gates));

  return "ok".to_string()
}

fn part2 (input: String) -> String  {
  return input.to_string()
}

pub fn fill() -> super::Day {
  return super::Day {
    input: include_str!("input").to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "400410".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "15343601".to_string());
}
