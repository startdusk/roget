pub mod algorithms;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Correctness {
    /// Green
    Correct,
    /// Yellow
    Misplaced,
    /// Gray
    Wrong,
}

impl Correctness {
    fn compute(answer: &str, guess: &str) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);
        let mut c = [Correctness::Wrong; 5];
        // Mark things green
        for (i, (a, g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                c[i] = Correctness::Correct;
            }
        }
        // Mark things yellow
        let mut used = [false; 5];
        for (i, &c) in c.iter().enumerate() {
            if c == Correctness::Correct {
                used[i] = true;
            }
        }
        for (i, g) in guess.chars().enumerate() {
            if c[i] == Correctness::Correct {
                // Already marked as green
                continue;
            }
            if answer.chars().enumerate().any(|(i, a)| {
                if a == g && !used[i] {
                    used[i] = true;
                    return true;
                }
                false
            }) {
                c[i] = Correctness::Misplaced;
            }
        }
        c
    }
}

pub struct Guess {
    word: String,
    mask: [Correctness; 5],
}

pub trait Guesser {
    // guesses 前面猜过单词的列表(猜错的)
    fn guess(&mut self, hostory: &[Guess]) -> String;
}

// 在 Rust 中, 字符串字面量的类型是 & 'static str,
// 因为它是被直接储存在编译后的二进制文件中的(直接把answers.txt的文本读到内存中)
pub fn play<G: Guesser>(answer: &'static str, mut guesser: G) -> Option<usize> {
    let mut history = Vec::new();
    // Wordle only allows six guesses.
    // We allow more to avoid chopping off the score distribution for stats puproses.
    for i in 1..=32
    /* for i := 1; i <= 32; i++ */
    {
        let guess = guesser.guess(&history);
        if guess == answer {
            return Some(i);
        }
        let correctness = Correctness::compute(answer, &guess);
        history.push(Guess {
            word: guess,
            mask: correctness,
        });
    }
    None
}

#[cfg(test)]
mod tests {
    mod compute {
        use crate::Correctness;

        macro_rules! mask {
            (C) => {Correctness::Correct};
            (M) => {Correctness::Misplaced};
            (W) => {Correctness::Wrong};
            ($($c:tt)+) => {[
                $(mask!($c)),+
            ]}
        }

        #[test]
        fn all_green() {
            assert_eq!(Correctness::compute("abcde", "abcde"), mask![C C C C C])
        }

        #[test]
        fn all_gray() {
            assert_eq!(Correctness::compute("abcde", "fghij"), mask![W W W W W])
        }
        #[test]
        fn all_yellow() {
            assert_eq!(Correctness::compute("abcde", "edbca"), mask![M M M M M])
        }
        #[test]
        fn repeat_green() {
            assert_eq!(Correctness::compute("aabbb", "aaccc"), mask![C C W W W])
        }
    }
}
