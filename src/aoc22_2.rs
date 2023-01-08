pub fn solution() {
    /* 
        A, X - for rock (1 pts)
        B, Y - for paper (2 pts)
        C, Z - for scissors (3 pts)

        Lose - 0 pts
        Draw - 3 pts
        Win - 6 pts

        Part two edits:
        X - lose
        Y - draw
        Z - win
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
                score += LOSE_PTS;
                match opp {
                    "A" => score += SCIS_PTS,
                    "B" => score += ROCK_PTS,
                    "C" => score += PAPE_PTS,
                    _ => panic!("This is not R&P&S...")
                }
            },
            (opp, "Y") => {
                score += DRAW_PTS;
                match opp {
                    "A" => score += ROCK_PTS,
                    "B" => score += PAPE_PTS,
                    "C" => score += SCIS_PTS,
                    _ => panic!("This is not R&P&S...")
                }
            },
            (opp, "Z") => {
                score += WIN_PTS;
                match opp {
                    "A" => score += PAPE_PTS,
                    "B" => score += SCIS_PTS,
                    "C" => score += ROCK_PTS,
                    _ => panic!("This is not R&P&S...")
                }
            },
            _ => panic!("This is not R&P&S...")
        }
    });

    println!("Your score is {score}");
}