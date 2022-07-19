mod lib;

use stats;
use std::cmp;
use std::sync::mpsc;
use std::thread;

// (DONE) iteration 1: read in a file, add up all the rolls naiively
// (DONE) iteration 2: mean/median/std deviation
// (DONE, with known error) iteration 3: get the proper score
// (DONE) iteration 4: let's get concurrent!

fn main() {
    let now = std::time::Instant::now();
    let games: Vec<Vec<lib::Roll>> = lib::file_to_games("./data.csv");

    // split all game line data into chunks (ideally 5) to concurrently score them:
    const NUM_THREADS: usize = 5;
    // at small input sizes this can eval to 0, so ensure at least one game per chunk
    let chunk_size = cmp::max(games.len() / NUM_THREADS, 1);
    let game_chunks = games.chunks(chunk_size).map(|c| c.to_vec());
    // should just be NUM_THREADS, but to be safe base this on game_chunks length (e.g. small input data size)
    let num_chunks = game_chunks.len();

    // binding to hold our eventual, concurrently-calculated chunks of game scores
    let mut scored_chunks: Vec<Vec<i32>> = Vec::new();

    // CONCURRENT CODE:
    // transmitter, receiver for multi-producer/single-consumer message passing
    let (tx, rx): (mpsc::Sender<Vec<i32>>, mpsc::Receiver<Vec<i32>>) = mpsc::channel();

    game_chunks.for_each(|game_chunk| {
        let tx = tx.clone();
        thread::spawn(move || {
            let chunk_scores = game_chunk.iter().map(lib::score_game).collect();
            tx.send(chunk_scores).unwrap();
        });
    });

    // this while loop will block main thread until each of our 5 parallel threads we spun up
    // finish scoring their games and tx-ing that data to our receiver
    while scored_chunks.len() < num_chunks {
        let received_scores = rx.recv().unwrap().clone();
        scored_chunks.push(received_scores);
    }

    // now we have data from all threads! flatten it.
    let scores: Vec<i32> = scored_chunks.iter().flatten().map(|s| *s).collect();

    println!("Hey! Some info for ya...");
    println!("Number of games parsed: {}", &scores.len());
    println!(
        "Mean, median, and standard deviation: {:?}, {:?}, and {:?} respectively",
        stats::mean(scores.iter().cloned()),
        stats::median(scores.iter().cloned()).unwrap(),
        stats::stddev(scores.iter().cloned())
    );
    println!("Elapsed time: {:.2?}", now.elapsed());
}
