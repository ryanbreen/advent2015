#![feature(const_fn, str_char)]
mod calendar;

extern crate getopts;
use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
  let brief = format!("Usage: {} [options]", program);
  print!("{}", opts.usage(&brief));
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.reqopt("d", "", "The day to test (0-24).", "DAY");
  opts.reqopt("p", "", "The puzzle to test (0-1).", "PUZZLE");
  opts.optopt("i", "", "Alternate input text.", "INPUT");
  opts.optflag("h", "help", "print this help menu");
  let matches = match opts.parse(&args[1..]) {
      Ok(m) => { m }
      Err(f) => { panic!(f.to_string()) }
  };
  if matches.opt_present("h") {
      print_usage(&program, opts);
      return;
  }
  let day = matches.opt_str("d").expect("Missing value").parse::<usize>().unwrap();
  let first_puzzle:bool = matches.opt_str("p").expect("Missing value") == "0";

  let calendar = calendar::Calendar::new();
  if first_puzzle {
    println!("{}", (calendar.days[0].part1.run)(calendar.days[day].input));
  } else {
    println!("{}", (calendar.days[0].part2.run)(calendar.days[day].input));
  }
}
