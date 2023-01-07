pub fn solution() {
    let input = std::fs::read_to_string("stocks/1.txt").expect("Incorrect path");
    let elves_inventory_list = input.split("\n\r")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|inventory| inventory.split("\r\n").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut max: u32 = 0;
    elves_inventory_list.iter().for_each(|inventory| {
        let mut sum: u32 = 0;
        inventory.iter().for_each(|item| sum += item
            .to_string().replace("\n", "")
            .replace("\r", "")
            .parse::<u32>().unwrap());

        if sum > max {max = sum;}
    });

    println!("Max calories is: {max}");
}