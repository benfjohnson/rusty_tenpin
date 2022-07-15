mod lib;

use stats;

// (DONE) iteration 1: read in a file, add up all the rolls naiively
// (DONE) iteration 2: mean/median/std deviation
// iteration 3: get the proper score
// iteration 4: let's get concurrent!

fn main() {
    let games = lib::file_to_games("./data.csv");

    // convert the rolls into a vector of (naiive) scores
    let scores: Vec<i32> = games.iter().map(lib::score_game).collect();

    println!("hey there!, some info for ya:");
    println!("number of games: {}", &scores.len());
    println!(
        "mean, median, and standard deviation: {:?}, {:?}, and {:?} respectively",
        stats::mean(scores.iter().cloned()),
        stats::median(scores.iter().cloned()).unwrap(),
        stats::stddev(scores.iter().cloned())
    );
    println!("SCORES: {:?}", scores);
}
