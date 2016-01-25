use crypto::digest::Digest;
use crypto::md5::Md5;

fn part1 (input: String) -> String  {
  let mut md5summer = Md5::new();
  let mut inc = 0;
  let mut out;

  loop {
    md5summer.input_str(&format!("{}{}", input, inc));

    out = md5summer.result_str();
    if out.starts_with("00000") {
      break;
    }

    md5summer.reset();
    inc+=1;
  }

  return inc.to_string();
}

fn part2 (input: String) -> String  {
  let mut md5summer = Md5::new();
  let mut inc = 0;
  let mut out;

  loop {
    md5summer.input_str(&format!("{}{}", input, inc));

    out = md5summer.result_str();
    if out.starts_with("000000") {
      break;
    }

    md5summer.reset();
    inc+=1;
  }

  return inc.to_string();
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "bgvyzdsv".to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "254575".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "1038736".to_string());
}
