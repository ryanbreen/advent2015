#![feature(const_fn)]
mod calendar;

fn main() {
    let calendar = calendar::Calendar::new();
    println!("There are {} days", calendar.days.len());
    println!("{}", (calendar.days[0].part1.run)());
}
