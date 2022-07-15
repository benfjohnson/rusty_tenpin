mod lib;

use lib::Roll;

// iteration 1: read in a file, add up all the rolls naiively
// iteration 2: mean/median/std deviation
// iteration 3: get the proper score
// iteration 4: let's get concurrent!

fn main() {
    let games = lib::file_to_games("./data.csv");

    // convert the rolls into a vector of (naiive) scores
    let naiive_scores: Vec<i32> = games
        .iter()
        .map(|rolls| {
            rolls.iter().fold(0, |sum, roll| {
                let score = match roll {
                    Roll::Knockdown(digit) => *digit,
                    _ => 10,
                };

                sum + score
            })
        })
        .collect();

    println!("hey there! {:?}, {}", &naiive_scores, &naiive_scores.len());
}
