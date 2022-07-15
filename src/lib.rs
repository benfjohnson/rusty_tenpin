use std::fs;

pub enum Roll {
    Knockdown(i32),
    Strike,
    Spare,
}

type Game = Vec<Roll>;

// convert a string to a Roll if possible
fn str_to_roll(s: &str) -> Result<Roll, &str> {
    match s {
        "0" | "1" | "2" | "3" | "5" | "6" | "7" | "8" | "9" => {
            Ok(Roll::Knockdown(str::parse::<i32>(s).unwrap()))
        }
        "X" => Ok(Roll::Strike),
        "/" => Ok(Roll::Spare),
        _ => Err("Not a roll!"),
    }
}

// filter out values in a game we don't care about
// ('-', game id)
fn str_to_game(game_str: &str) -> Game {
    game_str
        .split(",")
        .map(str::trim_end)
        .map(str_to_roll)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect()
}

pub fn file_to_games(file: &str) -> Vec<Vec<Roll>> {
    // read in game file, and filter out the last line which doesn't contain any content
    let file_str = fs::read_to_string(file).expect("Bad filename given");
    file_str
        .split("\n")
        .filter(|g| g.contains(","))
        .map(str_to_game)
        .collect()
}
