use std::str::CharRange;

fn to_base26(input: String) -> u64 {
  // Subtract every char down by 10.
  let mut i:usize = 0;
  let mut b26_str:String = String::new();
  while i < input.len() {
    let CharRange {ch, next} = input.char_range_at(i);
    if ch < 'k' {
      println!("{} {} {}", ch, (ch as u8), ((ch as u8) - 48) as char);
      b26_str.push(((ch as u8) - 48) as char);
    } else {
      println!("{} {} {}", ch, (ch as u8), ((ch as u8) - 10) as char);
      b26_str.push(((ch as u8) - 10) as char);
    }
    i = next;
  }

  println!("{}", b26_str);

  return u64::from_str_radix(&b26_str, 26).unwrap();
}

fn from_base26(input: u64) -> String {

  let mut b26_str:String = String::new();
  let mut val = input;

  println!("Converting {} to str", val);

  while val > 0 {
    let digit:u8 = (val % 26) as u8;
    val = val / 26;

    if digit < 10 {
      println!("{} {}", digit, ((digit + 48) as char));
      b26_str.push((digit + 48) as char);
    } else {
      println!("{} {}", digit, ((digit + 97) as char));
      b26_str.push((digit + 97) as char);
    }
  }

  return b26_str.to_string();
}

fn is_valid(input: String) -> bool {

  let mut i:usize = 1;
  let CharRange {ch, ..} = input.char_range_at(0);
  let mut last_char = ch;
  let mut last_char_count:usize = 1;
  let mut rvalue:String = String::new();

  while i < input.len() {
    let CharRange {ch, next} = input.char_range_at(i);
    if ch != last_char {
      // We need to add the count of the last char to the return string.
      rvalue.push_str(&last_char_count.to_string());
      rvalue.push(last_char);
      last_char_count = 1;
      last_char = ch;
    } else {
      last_char_count += 1;
    }
    i = next;
  }

  rvalue.push_str(&last_char_count.to_string());
  rvalue.push(last_char);

  return false;
}

fn increment(input: String) -> String {
  return input;
}

fn find_next_valid(input: String) -> String {
  let mut next = input;
  loop {
    next = increment(next);
    if is_valid(next.clone()) {
      return next;
    }
  }
}

fn part1(input: String) -> String  {
//  return find_next_valid(input);
  println!("Round-tripping {}", input);
  return from_base26(to_base26(input));
}

fn part2 (input: String) -> String  {
  return find_next_valid(input);
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
  assert_eq!((day.part1.run)(day.input.to_string()), "329356".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "4666278".to_string());
}
