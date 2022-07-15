use std::fs;

// iteration 1: read in a file, add up all the rolls naiively
// iteration 2: mean/median/std deviation
// iteration 3: get the proper score
// iteration 4: let's get concurrent!

fn main() -> Result<(), std::io::Error>{
    // read in game file, and filter out the last line which doesn't contain any content
    let file_str = fs::read_to_string("./data.csv")?;
    let games = file_str.split("\n").filter(|g| g.contains(","));


    // convert the rolls into a vector of (naiive) scores
    let naiive_scores : Vec<i32> = games.map(|game| {
        let rolls = game.split(",")
            .map(|g| g.trim_end()) // trim whitespace and newline char at end
            .filter(|r| r.len() == 1); 

        return rolls.fold(0, |sum, roll| {
            let score = match roll {
                "X" => 10,
                "/" => 10,
                "-" => 0,
                digit => str::parse::<i32>(digit).expect("given unexpected file input")
            };

            sum + score
        });
    }).collect();

    println!("hey there! {:?}, {}", &naiive_scores, &naiive_scores.len());

    Ok(())
}
