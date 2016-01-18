#![feature(const_fn, str_char, core)]
mod calendar;

fn main() {
    let calendar = calendar::Calendar::new();
    println!("There are {} days", calendar.days.len());
    println!("{}", (calendar.days[0].part2.run)());
}
