use std::fs;

#[derive(Debug)]
pub enum Roll {
    Knockdown(i32),
    Strike,
    Spare,
}

type Game = Vec<Roll>;

// convert a string to a Roll if possible
fn str_to_roll(s: &str) -> Result<Roll, &str> {
    match s {
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
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

fn use_bonus_rolls(bonuses: (i32, i32), score: &i32) -> ((i32, i32), i32) {
    let mut points = *score;
    let mut new_bs = (bonuses.0, bonuses.1);
    if bonuses.0 != 0 {
        new_bs.0 -= 1;
        points += score;
    }

    if bonuses.1 != 0 {
        new_bs.1 -= 1;
        points += score;
    }

    (new_bs, points)
}

fn add_bonus_rolls(bonuses: (i32, i32), roll: &Roll) -> (i32, i32) {
    let ret_bonus = bonuses;
    match roll {
        Roll::Knockdown(_) => ret_bonus,
        Roll::Spare => {
            if bonuses.0 < bonuses.1 {
                (bonuses.0 + 1, bonuses.1)
            } else {
                (bonuses.0, bonuses.1 + 1)
            }
        }
        Roll::Strike => {
            if bonuses.0 < bonuses.1 {
                (bonuses.0 + 2, bonuses.1)
            } else {
                (bonuses.0, bonuses.1 + 2)
            }
        }
    }
}

fn calc_roll_points(bonuses: (i32, i32), roll: &Roll) -> ((i32, i32), i32) {
    match roll {
        Roll::Knockdown(score) => use_bonus_rolls(bonuses, score),
        Roll::Spare | Roll::Strike => {
            let (deducted_bonuses, points) = use_bonus_rolls(bonuses, &10);
            let added_bonuses = add_bonus_rolls(deducted_bonuses, roll);
            (added_bonuses, points)
        }
    }
}

// algo:
// can only have two bonuses in play at once (by third strike, first strike no longer
// has bonus rolls left
// * keep a map of two (or fewer) active bonuses
// * when a strike or spare happens, update it.
// * on any roll, take those bonus points and add to score
pub fn score_game(game: &Vec<Roll>) -> i32 {
    let mut bonus_rolls: (i32, i32) = (0, 0);
    game.iter().fold(0, |sum, roll| {
        let (updated_bonus, points) = calc_roll_points(bonus_rolls, roll);
        bonus_rolls = updated_bonus;

        sum + points
    })
}
