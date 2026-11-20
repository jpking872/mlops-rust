pub fn get_lineup(team: String, starters: bool) -> Vec<String> {
    vec![
        "Aaron Rodgers".to_string(),
        "Jaylen Warren".to_string(),
        "DK Metcalf".to_string(),
    ] 
}

pub fn get_players(starters: bool) -> Vec<String> {
    if starters_only {
        vec![
            "Aaron Rodgers".to_string(),
            "Jaylen Warren".to_string(),
            "DK Metcalf".to_string(),
        ]
    } else {
        vec![
            "Aaron Rodgers".to_string(),
            "Mason Rudolph".to_string(),
            "Will Howard".to_string(),
            "Jaylen Warren".to_string(),
        ]
    }
}

fn main() {
    let players = get_players(true);

    for player in players {
        println!("{}", player);
    }
}