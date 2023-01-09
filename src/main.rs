mod aoc22_1;
mod aoc22_2;
mod aoc22_3;
use text_io::read;

fn main() {
    print!("Choose day of Advent (1 | 2 | 3 | ... | 25): ");
    let day: u8 = read!();

    match day {
        1 => aoc22_1::solution(),
        2 => aoc22_2::solution(),
        3 => aoc22_3::solution(),
        4..=25 => println!("Not implemented yet"),
        _ => println!("Incorrect number...")
    }
}
