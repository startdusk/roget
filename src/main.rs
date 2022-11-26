use clap::{command, Parser, ValueEnum};
use roget::{
    algorithms::{Allocs, Naive, OnceInit, Vecremain},
    Guesser, Wordle,
};

const GAMES: &str = include_str!("../answers.txt");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    implementation: Implementation,

    #[arg(short, long)]
    max: Option<usize>,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub(crate) enum Implementation {
    Naive,
    Allocs,
    Vecremain,
    OnceInit,
}

fn main() {
    let args = Args::parse();
    match args.implementation {
        Implementation::Naive => play(Naive::new, args.max),
        Implementation::Allocs => play(Allocs::new, args.max),
        Implementation::Vecremain => play(Vecremain::new, args.max),
        Implementation::OnceInit => play(OnceInit::new, args.max),
    }
}

fn play<G: Guesser>(mut mk: impl FnMut() -> G, max: Option<usize>) {
    let w = Wordle::new();
    for answer in GAMES.split_whitespace().take(max.unwrap_or(usize::MAX)) {
        let guesser = (mk)();
        if let Some(score) = w.play(answer, guesser) {
            println!("guessed '{}' score: {}", answer, score);
        } else {
            eprintln!("failed to guess!")
        }
    }
}
