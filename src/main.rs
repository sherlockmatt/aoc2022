#[macro_use]
extern crate failure;
extern crate exitfailure;
// #[macro_use]
// extern crate lazy_static;

use exitfailure::ExitFailure;
use std::time::Instant;
use structopt::StructOpt;

mod puzzles;
mod utils;

#[derive(StructOpt)]
struct Cli {
    /// Path to the file containing your session cookie
    #[structopt(short, long, default_value = "session.txt")]
    session: std::path::PathBuf,
    /// Puzzle numbers to run, using range syntax e.g. 1-3,7,8-21,25
    #[structopt(short, long = "number", default_value = "1-25")]
    num: String,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();

    for i in utils::parse_range(args.num)? {
        println!("Starting puzzle number {}", i);

        let puzzle_input = utils::download(i, &args.session).unwrap_or_else(|e| {
            println!("ERROR: {}", e);
            return String::from("");
        });

        // Don't run the puzzle if there was no input
        if &puzzle_input != "" {
            // Start timing here because there's little point timing the download
            let start_time = Instant::now();
            let puzzle_answers = puzzles::run(i, &puzzle_input).unwrap_or_else(|e| {
                println!("ERROR: {}", e);
                return vec![];
            });

            for (j, ans) in puzzle_answers.iter().enumerate() {
                println!("Part {} answer: {}", j + 1, ans);
            }
            println!("Completed puzzle in {:.3?}", start_time.elapsed());
        }
    }

    Ok(())
}
