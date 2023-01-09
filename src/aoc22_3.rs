pub fn solution() {
    const ALL_TYPES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let input = std::fs::read_to_string("stocks/3.txt")
        .expect("Incorrect path");

    let mut badges_sum: usize = 0;
    let mut group_rucksacks: Vec<&str> = Vec::with_capacity(3);
    input.lines().zip((1..=3).into_iter().cycle()).for_each(|(line, counter)| {
        match counter {
            1 | 2 => group_rucksacks.push(line),
            3 => {
                let (st_elf, nd_elf, rd_elf) = (
                    group_rucksacks.pop().unwrap(),
                    group_rucksacks.pop().unwrap(),
                    line
                );
        
                for c in st_elf.chars() {
                    if nd_elf.contains(c) && rd_elf.contains(c) {
                        badges_sum += ALL_TYPES.find(c).unwrap() + 1;
                        break;
                    };
                }
            },
            _ => panic!("Error in iter")
        }
    });

    println!("Badges sum is: {badges_sum}");
}