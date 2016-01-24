
fn part1 (input: String) -> String  {

  enum Mode {
    On,
    Off,
    Toggle
  }
  
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

fn part2 (input: String) -> String  {
  return input;
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