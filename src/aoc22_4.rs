use std::ops::RangeInclusive;

pub fn solution() {
    let input = std::fs::read_to_string("stocks/4.txt")
        .expect("Incorrect path");

    let mut overlap_pairs: u32 = 0;
    input.lines().for_each(|line| {
        let ranges = {
            line.split(",")
                .map(|range| 
                {
                    let limits = range.split("-").collect::<Vec<&str>>();
                    limits[0].parse::<u32>().unwrap()..=limits[1].parse::<u32>().unwrap()
                })
                .collect::<Vec<RangeInclusive<u32>>>()
        };
        
        if ranges[0].clone().all(|num| ranges[1].contains(&num)) || ranges[1].clone().all(|num| ranges[0].contains(&num)) {
            overlap_pairs += 1;
        }
    });

    println!("Number of overlap pairs is {overlap_pairs}");
}