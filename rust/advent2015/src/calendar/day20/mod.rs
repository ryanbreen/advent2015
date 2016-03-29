
fn count_for_house(num: u32) -> u32 {
  let limit = ((num as f64).sqrt() as u32) + 1;

  let mut counter:u32 = 0;
  for i in 1..limit {
    if num % i == 0 {
      // Add a visit for the number and its complement (unless it's a square);
      counter += 10*i;
      if num / i != i {
        counter += 10*(num / i);
      }
    }
  }

  return counter;
}

fn adjusted_count_for_house(num: u32) -> u32 {
  let limit = ((num as f64).sqrt() as u32) + 1;

  let mut counter:u32 = 0;
  for i in 1..limit {
    if num % i == 0 {
      // Add a visit for the number and its complement (unless it's a square);
      if num <= 50 * i {
        counter += 11*i;
      }

      if num / i != i && num <= 50 * (num / i) {
        counter += 11 * (num / i);
      }
    }
  }

  return counter;
}

#[allow(dead_code)]
fn sanity_check() {
  for i in 1..10 {
    println!("{} {}", i, count_for_house(i));
  }
}

fn part1 (input: String) -> String {
  //sanity_check();
  let target = input.parse::<u32>().unwrap();

  let mut i = 1;
  loop {
    let count = count_for_house(i);

    if count >= target {
      return i.to_string();
    }

    i += 1;
  }
}

fn part2 (input: String) -> String {
  let target = input.parse::<u32>().unwrap();

  let mut i = 1;
  loop {
    let count = count_for_house(i);

    if count >= target {
      let count = adjusted_count_for_house(i);
      if count >= target {
        return i.to_string();
      }
    }

    i += 1;
  }
}

pub fn fill() -> super::Day {
  return super::Day {
    input: "36000000".to_string(),
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
  assert_eq!((day.part1.run)(day.input.to_string()), "831600".to_string());
}

#[test]
fn test_part2() {
  let day = fill();
  assert_eq!((day.part2.run)(day.input.to_string()), "884520".to_string());
}
