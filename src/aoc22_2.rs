pub fn solution() {
    /* 
        A, X - for rock (1 pts)
        B, Y - for paper (2 pts)
        C, Z - for scissors (3 pts)

        Lose - 0 pts
        Draw - 3 pts
        Win - 6 pts
    */
    const ROCK_PTS: u32 = 1;
    const PAPE_PTS: u32 = 2;
    const SCIS_PTS: u32 = 3;
    const LOSE_PTS: u32 = 0;
    const DRAW_PTS: u32 = 3;
    const WIN_PTS: u32 = 6;

    let input = std::fs::read_to_string("stocks/2.txt")
        .expect("Incorrect path");
    
    let mut score: u32 = 0;
    input.lines().for_each(|line| {
        let turn: (&str, &str) = {
            let temp = line.split(" ").collect::<Vec<&str>>();
            (temp[0], temp[1])
        };

        match turn {
            (opp, "X") => {
                score += ROCK_PTS;
                match opp {
                    "A" => score += DRAW_PTS,
                    "B" => score += LOSE_PTS,
                    "C" => score += WIN_PTS,
                    _ => panic!("This is not R&P&S...")
                }
            },
            (opp, "Y") => {
                score += PAPE_PTS;
                match opp {
                    "A" => score += WIN_PTS,
                    "B" => score += DRAW_PTS,
                    "C" => score += LOSE_PTS,
                    _ => panic!("This is not R&P&S...")
                }
            },
            (opp, "Z") => {
                score += SCIS_PTS;
                match opp {
                    "A" => score += LOSE_PTS,
                    "B" => score += WIN_PTS,
                    "C" => score += DRAW_PTS,
                    _ => panic!("This is not R&P&S...")
                }
            },
            _ => panic!("This is not R&P&S...")
        }
    });

    println!("Your score is {score}");
}