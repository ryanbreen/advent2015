use std::collections::HashMap;

trait Gate {
  fn calculate(&self, &HashMap<String, Box<Gate>>, cached_values: &mut HashMap<String, u16>) -> u16;
}

// Convenience method for turning a string that may be a label for a gate or a
// scalar into a scalar.
fn resolve_value(value: String, gates: &HashMap<String, Box<Gate>>, cached_values: &mut HashMap<String, u16>) -> u16 {
  match value.parse::<u16>() {
    Ok(scalar) => return scalar,
    Err(_) => {
      println!("Looking for label {}", value);
      return gates[&value].calculate(gates, cached_values);
    },
  }
}

// A gate that takes a value or wire as its input
struct PassthroughGate {
  label: String,
  input: String,
  not: bool
}

impl PassthroughGate {
  fn new(label: String, input: String, not: bool) -> PassthroughGate {
    PassthroughGate { label: label, input: input, not: not }
  }
}

impl Gate for PassthroughGate {
  fn calculate(&self, gates: &HashMap<String, Box<Gate>>, cached_values: &mut HashMap<String, u16>) -> u16 {

    if cached_values.contains_key(&self.label) {
      return cached_values[&self.label];
    }

    let mut value:u16 = resolve_value(self.input.clone(), gates, cached_values);
    if self.not {
      println!("Original: {:0>8b}", value);
      value = !value;
      println!("Notted: {:0>8b}", value);
    }

    println!("{} == {}", self.input, value);

    cached_values.insert(self.label.clone(), value);

    return value;
  }
}

// A gate that takes two values and applies an operator
struct LogicGate {
  operator: String,
  label: String,
  input_a: String,
  input_b: String
}

impl LogicGate {
  fn new(label: String, operator: String, input_a: String, input_b: String) -> LogicGate {
    LogicGate { operator: operator, label: label, input_a: input_a, input_b: input_b }
  }
}

impl Gate for LogicGate {
  fn calculate(&self, gates: &HashMap<String, Box<Gate>>, cached_values: &mut HashMap<String, u16>) -> u16 {
    if cached_values.contains_key(&self.label) {
      return cached_values[&self.label];
    }

    println!("Attempting to resolve left leg {}", self.input_a);
    let value_a = resolve_value(self.input_a.clone(), gates, cached_values);
    println!("Attempting to resolve right leg {}", self.input_b);
    let value_b = resolve_value(self.input_b.clone(), gates, cached_values);

    let val:u16;
    match self.operator.as_ref() {
      "OR" => val = value_a | value_b,
      "AND" => val = value_a & value_b,
      "LSHIFT" => val = value_a << value_b,
      "RSHIFT" => val = value_a >> value_b,
      _ => panic!("Unknown operator {}", self.operator),
    }

    println!("{} {} {} == {}", value_a, self.operator, value_b, val);

    cached_values.insert(self.label.clone(), val);
    return val;
  }
}

fn part1(input: String) -> String  {

  let mut gates:HashMap<String, Box<Gate>> = HashMap::new();
  let lines: Vec<&str> = input.lines().collect();
  for line in lines {
    let parts: Vec<&str> = line.split(' ').collect();
    match parts.len() {
      5 => {
        println!("Adding gate that ends at {}: {} {} {}", parts[4], parts[1], parts[0], parts[2]);
        gates.insert(parts[4].to_string(), Box::new(LogicGate::new(parts[4].to_string(),parts[1].to_string(), parts[0].to_string(), parts[2].to_string())));
      },
      4 => {
        println!("Adding gate that ends at {}: NOT {}", parts[3], parts[1]);
        gates.insert(parts[3].to_string(), Box::new(PassthroughGate::new(parts[3].to_string(),parts[1].to_string(), true)));
      },
      3 => {
        println!("Adding gate that ends at {}: {}", parts[2], parts[0]);
        gates.insert(parts[2].to_string(), Box::new(PassthroughGate::new(parts[2].to_string(),parts[0].to_string(), false)));
      },
      _ => panic!("Invalid operation!"),
    };
  }

  let mut cached_values:HashMap<String, u16> = HashMap::new();
  return gates["a"].calculate(&gates, &mut cached_values).to_string();
}

fn part2 (input: String) -> String  {
  let mut gates:HashMap<String, Box<Gate>> = HashMap::new();
  let lines: Vec<&str> = input.lines().collect();
  for line in lines {
    let parts: Vec<&str> = line.split(' ').collect();
    match parts.len() {
      5 => {
        println!("Adding gate that ends at {}: {} {} {}", parts[4], parts[1], parts[0], parts[2]);
        gates.insert(parts[4].to_string(), Box::new(LogicGate::new(parts[4].to_string(),parts[1].to_string(), parts[0].to_string(), parts[2].to_string())));
      },
      4 => {
        println!("Adding gate that ends at {}: NOT {}", parts[3], parts[1]);
        gates.insert(parts[3].to_string(), Box::new(PassthroughGate::new(parts[3].to_string(),parts[1].to_string(), true)));
      },
      3 => {
        println!("Adding gate that ends at {}: {}", parts[2], parts[0]);
        gates.insert(parts[2].to_string(), Box::new(PassthroughGate::new(parts[2].to_string(),parts[0].to_string(), false)));
      },
      _ => panic!("Invalid operation!"),
    };
  }

  let mut cached_values:HashMap<String, u16> = HashMap::new();
  cached_values.insert("b".to_string(), 46065);
  return gates["a"].calculate(&gates, &mut cached_values).to_string();
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
  assert_eq!((day.part1.run)(day.input.to_string()), "46065".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "14134".to_string());
}
