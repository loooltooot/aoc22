pub fn solution() {
    let input = std::fs::read_to_string("stocks/5.txt")
        .expect("Incorrect path");
    
    let mut crates_stacks: Vec<Vec<char>> = vec![
        vec!['H', 'C', 'R'],
        vec!['B', 'J', 'H', 'L', 'S', 'F'],
        vec!['R', 'M', 'D', 'H', 'J', 'T', 'Q'],
        vec!['S', 'G', 'R', 'H', 'Z', 'B', 'J'],
        vec!['R', 'P', 'F', 'Z', 'T', 'D', 'C', 'B'],
        vec!['T', 'H', 'C', 'G'],
        vec!['S', 'N', 'V', 'Z', 'B', 'P', 'W', 'L'],
        vec!['R', 'J', 'Q', 'G', 'C'],
        vec!['L', 'D', 'T', 'R', 'H', 'P', 'F', 'S']
    ];

    input.lines().for_each(|line| {
        let (amount, mut from, mut to, from_index, to_index) = {
            let temp = line.split(" ").collect::<Vec<&str>>();
            let from_index = temp[3].parse::<usize>().unwrap() - 1;
            let to_index = temp[5].parse::<usize>().unwrap() - 1;

            (temp[1].parse::<usize>().unwrap(), 
            crates_stacks[from_index].clone(), 
            crates_stacks[to_index].clone(),
            from_index, to_index)
        };

        let stack_len = from.len();
        from[(stack_len - amount)..]
            .iter()
            .for_each(|item| to.push(*item));

        from.truncate(stack_len - amount);

        crates_stacks[from_index] = from;
        crates_stacks[to_index] = to;
    });

    crates_stacks.iter().for_each(|stack| {
        print!("{}", stack.last().unwrap());
    });
}