pub fn solution() {
    const ALL_TYPES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let input = std::fs::read_to_string("stocks/3.txt")
        .expect("Incorrect path");

    let mut errors_sum: usize = 0;
    input.lines().for_each(|line| {
        let (st_comp, nd_comp) = line.split_at(line.len() / 2);

        for c in st_comp.chars() {
            if nd_comp.contains(c) {
                errors_sum += ALL_TYPES.find(c).unwrap() + 1;
                break;
            };
        }
    });

    println!("Errors sum is: {errors_sum}");
}