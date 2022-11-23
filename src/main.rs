use roget::Wordle;

const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let w = Wordle::new();
    for answer in GAMES.split_whitespace() {
        let guesser = roget::algorithms::Naive::new();
        if let Some(score) = w.play(answer, guesser) {
            println!("guessed '{}' score: {}", answer, score);
        } else {
            eprintln!("failed to guess!")
        }
    }
}
