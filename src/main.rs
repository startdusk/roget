const GAMES: &str = include_str!("../answers.txt");

fn main() {
    for answer in GAMES.split_whitespace() {
        let guesser = roget::algorithms::Naive::new();
        roget::play(answer, guesser);
    }
}
