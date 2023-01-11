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
        let (mut amount, from, to) = {
            let temp = line.split(" ").collect::<Vec<&str>>();

            (temp[1].parse::<usize>().unwrap(), 
            temp[3].parse::<usize>().unwrap() - 1, 
            temp[5].parse::<usize>().unwrap() - 1)
        };

        while amount > 0 {
            let pick = crates_stacks[from].pop().expect("Cannot pick crate");
            crates_stacks[to].push(pick);
            amount -= 1;
        }
    });

    crates_stacks.iter().for_each(|stack| {
        print!("{}", stack.last().unwrap());
    });
}