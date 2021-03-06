
enum Mode {
  On,
  Off,
  Toggle
}

fn part1 (input: String) -> String  {

  let mut houses = [ [false; 1000]; 1000];
  {
    let mut operation = |start:String, end:String, mode:Mode| {
      let start_parts: Vec<&str> = start.split(',').collect();
      let end_parts: Vec<&str> = end.split(',').collect();
      let x1 = start_parts[0].parse::<usize>().unwrap();
      let x2 = end_parts[0].parse::<usize>().unwrap();
      let y1 = start_parts[1].parse::<usize>().unwrap();
      let y2 = end_parts[1].parse::<usize>().unwrap();

      for x in x1..x2+1 {
        for y in y1..y2+1 {
          match mode {
            Mode::On => houses[x][y] = true,
            Mode::Off => houses[x][y] = false,
            Mode::Toggle => houses[x][y] = !houses[x][y],
          };
        }
      }
    };

    operation("0,0".to_string(), "999,999".to_string(), Mode::Off);

    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
      let parts: Vec<&str> = line.split(' ').collect();
      match parts[0] {
        "turn" => {
          match parts[1] {
            "on" => operation(parts[2].to_string(), parts[4].to_string(), Mode::On),
            "off" => operation(parts[2].to_string(), parts[4].to_string(), Mode::Off),
            _ => panic!("Failed to parse turn on/off field"),
          }
        },
        "toggle" => operation(parts[1].to_string(), parts[3].to_string(), Mode::Toggle),
        _ => panic!("Invalid operation!"),
      };
    }
  }

  let mut total = 0;
  {
    for x in 0..1000 {
      for y in 0..1000 {
        if houses[x][y] {
          total += 1;
        }
      }
    }
  }

  return total.to_string()
}

fn operation(houses:& mut [[i32; 1000]; 1000], start:String, end:String, mode:Mode) {
  let start_parts: Vec<&str> = start.split(',').collect();
  let end_parts: Vec<&str> = end.split(',').collect();
  let x1 = start_parts[0].parse::<usize>().unwrap();
  let x2 = end_parts[0].parse::<usize>().unwrap();
  let y1 = start_parts[1].parse::<usize>().unwrap();
  let y2 = end_parts[1].parse::<usize>().unwrap();

  for x in x1..x2+1 {
    for y in y1..y2+1 {
      match mode {
        Mode::On => houses[x][y] = houses[x][y] + 1,
        Mode::Off => {
          houses[x][y] = houses[x][y] - 1;
          if houses[x][y] < 0 {
            houses[x][y] = 0;
          }
        },
        Mode::Toggle => houses[x][y] = houses[x][y] + 2,
      };
    }
  }
}

fn part2 (input: String) -> String  {

  let mut houses:[[i32; 1000]; 1000] = [ [0; 1000]; 1000];
  {
    operation(&mut houses, "0,0".to_string(), "999,999".to_string(), Mode::Off);

    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
      let parts: Vec<&str> = line.split(' ').collect();
      match parts[0] {
        "turn" => {
          match parts[1] {
            "on" => operation(&mut houses, parts[2].to_string(), parts[4].to_string(), Mode::On),
            "off" => operation(&mut houses, parts[2].to_string(), parts[4].to_string(), Mode::Off),
            _ => panic!("Failed to parse turn on/off field"),
          }
        },
        "toggle" => operation(&mut houses, parts[1].to_string(), parts[3].to_string(), Mode::Toggle),
        _ => panic!("Invalid operation!"),
      };
    }
  }

  let mut total = 0;
  {
    for x in 0..1000 {
      for y in 0..1000 {
        total += houses[x][y];
      }
    }
  }

  return total.to_string()
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
