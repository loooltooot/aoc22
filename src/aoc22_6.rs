pub fn solution() {
    let input = std::fs::read_to_string("stocks/6.txt")
        .expect("Incorrect path");
    
    let mut marker_characters = 0;
    let characters = input.chars().collect::<Vec<char>>();
    for i in 0..characters.len() {
        let mut pot_marker = characters[i..=i+3].iter().collect::<Vec<&char>>().clone();
        let mut is_marker = true;

        while let Some(c) = pot_marker.pop() {
            if pot_marker.contains(&c) {
                is_marker = false;
                break;
            }
        }

        if is_marker {
            marker_characters = i + 4; // +4 because we need to count characters and index is less by 1
            break;
        }
    }

    println!("Characters to determine marker - {marker_characters}");
}