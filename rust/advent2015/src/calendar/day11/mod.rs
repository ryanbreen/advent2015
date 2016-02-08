
fn is_valid(input: &Vec<u8>) -> bool {

  let mut inc_count:u8 = 0;
  let mut inc_gate_met:bool = false;

  let mut previous_pair:u8 = 0;
  let mut pair_gate_met:bool = false;

  for i in 1..8 {
    match input[i] {
      105 | 108 | 111 => return false,
      _ => {
        if input[i] == (input[i-1]) + 1 // straight increment
          || (input[i-1] == 104 && input[i] == 106)
          || (input[i-1] == 107 && input[i] == 109)
          || (input[i-1] == 110 && input[i] == 112) {
          inc_count += 1;
          if inc_count == 2 {
            inc_gate_met = true;
          }
        } else {
          inc_count = 0;
        }

        if input[i] == (input[i-1]) {
          if previous_pair == input[i] {
            // noop
          } else if previous_pair == 0 {
            previous_pair = input[i];
          } else {
            pair_gate_met = true;
          }
        }
      }
    }
  }

  return inc_gate_met && pair_gate_met;
}

const MIN:u8 = 97;
const MAX:u8 = 122;
const MAXER:u8 = 123;

fn increment(bytes: &mut Vec<u8>) {
  for i in 0..8 {
    match bytes[7-i] {
      MAX | MAXER => bytes[7-i] = MIN,
      _ => {
        bytes[7-i] += 1;
        return;
      },
    }
  }
}

fn find_next_valid(input: String) -> String {
  let mut next = input.into_bytes();
  loop {
    increment(&mut next);
    if is_valid(&next) {
      return String::from_utf8(next).unwrap();
    }
  }
}

fn part1(input: String) -> String  {
  return find_next_valid(input).to_string();
}

fn part2 (input: String) -> String  {
  return find_next_valid(find_next_valid(input));
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "hepxcrrq".to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "hepxxyzz".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "heqaabcc".to_string());
}
