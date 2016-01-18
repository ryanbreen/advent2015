#![feature(const_fn, str_char)]
mod calendar;

fn main() {
    let calendar = calendar::Calendar::new();
    println!("{}", (calendar.days[0].part2.run)(calendar.days[0].input));
}
