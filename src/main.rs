use clap::{ArgEnum, Parser};
use roget::Guesser;

const GAMES: &str = include_str!("../answers.txt");

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, arg_enum)]
    implementation: Implementation,

    #[clap(short, long)]
    max: Option<usize>,
}

#[derive(ArgEnum, Clone, Debug)]
enum Implementation {
    Naive,
    Allocs,
}

fn main() {
    // cargo r --release -- --implementation naive --max 2
    let args = Args::parse();
    match args.implementation {
        Implementation::Naive => play(roget::algorithms::Naive::new, args.max),
        Implementation::Allocs => play(roget::algorithms::Allocs::new, args.max),
    }
}

fn play<G>(mut mk: impl FnMut() -> G, max: Option<usize>)
where
    G: Guesser,
{
    let w = roget::Wordle::new();
    for answer in GAMES.split_whitespace().take(max.unwrap_or(usize::MAX)) {
        let guesser = (mk)();
        if let Some(score) = w.play(answer, guesser) {
            println!("guessed `{}` in {}", answer, score);
        } else {
            eprintln!("failed to guess")
        }
    }
}
