#![feature(const_fn)]
mod calendar;

fn main() {
    let calendar = calendar::Calendar::new();
    //println!("{}", calendar.days[0][0]());
    println!("There are {} days", calendar.days.len());
}
