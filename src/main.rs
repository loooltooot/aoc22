mod aoc22_1;
mod aoc22_2;
mod aoc22_3;
mod aoc22_4;
mod aoc22_5;
mod aoc22_6;
mod aoc22_7;

use text_io::read;

fn main() {
    print!("Choose day of Advent (1 | 2 | 3 | ... | 25): ");
    let day: u8 = read!();

    match day {
        1 => aoc22_1::solution(),
        2 => aoc22_2::solution(),
        3 => aoc22_3::solution(),
        4 => aoc22_4::solution(),
        5 => aoc22_5::solution(),
        6 => aoc22_6::solution(),
        7 => aoc22_7::solution(),
        8..=25 => println!("Not implemented yet"),
        _ => println!("Incorrect number...")
    }
}
