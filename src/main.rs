mod aoc22_1;
use text_io::read;

fn main() {
    print!("Choose day of Advent (1 | 2 | 3 | ... | 25): ");
    let day: u8 = read!();

    match day {
        1 => aoc22_1::solution(),
        _ => println!("Incorrect number...")
    }
}
