use roget::Wordle;

const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let wordle = Wordle::new();
    for answer in GAMES.split_whitespace() {
        let guesser = roget::algorithms::Naive::new();
        wordle.play(answer, guesser);
    }
}
