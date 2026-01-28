// Backend Funtions
use rand::{rng, seq::SliceRandom};
use std::io::{self, Read, Write};

pub struct Card(pub i32, pub String);

pub fn zip<T: Copy, U: Copy>(a: &Vec<T>, b: &Vec<U>) -> Vec<(T, U)> {
    // Zip two vectors of different lengths into a single vector of tuples
    let mut result: Vec<(T, U)> = Vec::with_capacity(a.len() * b.len());
    let _ = a
        .iter()
        .map(|i| {
            let _ = b
                .iter()
                .map(|j| {
                    result.push((*i, *j));
                    ()
                })
                .collect::<Vec<()>>();
            ()
        })
        .collect::<Vec<()>>();

    return result;
}

pub fn terminal_command<R: Read>(reader: R) -> String {
    // Clear stream & await input
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::BufRead::read_line(&mut io::BufReader::new(reader), &mut input).expect("input error");

    // Process output
    let mut args = input.trim().split_whitespace();
    return args.next().unwrap_or("").trim().to_lowercase();
}

pub fn format_output(text: &str) -> String {
    return format!("\n----------------------\n{text}\n----------------------\n");
}

pub fn print_hand(user: &str, hand: &Vec<Card>, score: i32) {
    println!(
        "{} HAND : ( {:?} ) {:#?}",
        user.to_uppercase(),
        score,
        hand.iter()
            .map(|Card(n, t)| format!("{} of {}", n, t))
            .collect::<Vec<String>>(),
    );
}

pub fn score_hand(hand: &Vec<Card>) -> i32 {
    let values = hand.iter().map(|Card(n, _)| *n);
    let mut score = values.clone().sum();

    // Check Aces
    let ace_count = values
        .clone()
        .filter(|v| *v == 11)
        .collect::<Vec<i32>>()
        .len();

    if score > 21 && ace_count > 0 {
        // Score without aces
        let mut tmp_score: i32 = values.clone().filter(|v| *v != 11).sum();

        // Decide ace score
        for _ in 0..ace_count {
            if tmp_score + 11 <= 21 {
                tmp_score += 11;
            } else {
                tmp_score += 1;
            }
        }

        score = tmp_score;
    }

    return score;
}

pub fn deal_hand(mut deck: Vec<Card>) -> (Vec<Card>, Vec<Card>) {
    // Randomise deck & remove last elements
    let mut rng = rng();

    deck.shuffle(&mut rng);
    let card_one = deck.pop().expect("failed to get first card");
    let card_two = deck.pop().expect("failed to get second card");

    return (deck, vec![card_one, card_two]);
}

pub fn build_deck(decks: i32) -> Vec<Card> {
    let numbers = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 11];
    let suits = vec!["CLUBS", "DIAMONDS", "HEARTS", "SPADES"];
    // let suits = vec!["♣️", "♥️", "♦️", "♠️"];

    let mut pool = Vec::with_capacity(numbers.len() * suits.len() * decks as usize);

    for _ in 0..decks {
        pool.append(&mut zip(&numbers, &suits));
    }

    return pool
        .iter()
        .map(|(num, txt)| Card(*num, txt.to_string()))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_deck() {
        let deck = build_deck(1);
        assert_eq!(deck.len(), 52);
    }

    #[test]
    fn test_deal_hand() {
        // Setup deck and verify length
        let deck = build_deck(1);
        assert_eq!(deck.len(), 52);

        // Run test function
        let deal = deal_hand(deck);

        // Check cards have been taken from deck
        assert_eq!(deal.0.len(), 50);
        // Check cards taken from deck are in hand
        assert_eq!(deal.1.len(), 2);
    }

    #[test]
    fn test_score_hand() {
        let hand: Vec<Card> = vec![
            Card(11, String::from("CLUBS")),
            Card(10, String::from("DIAMONDS")),
            Card(9, String::from("HEARTS")),
        ];

        // Score function should reduce ace value to 1 & return total of 20
        let score = score_hand(&hand);
        assert_eq!(score, 20);
    }

    #[test]
    fn test_zip() {
        let test = zip(&vec!["a", "b"], &vec![1, 2, 3]);
        let expected = vec![("a", 1), ("a", 2), ("a", 3), ("b", 1), ("b", 2), ("b", 3)];
        assert_eq!(test, expected);
    }

    #[test]
    fn test_terminal_returns_first_word_lowercased() {
        let input = b"Hello WORLD extra\n";
        let result = terminal_command(&input[..]);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_terminal_trims_whitespace_and_handles_empty() {
        let input = b"   \n";
        let result = terminal_command(&input[..]);
        assert_eq!(result, "");
    }

    #[test]
    fn test_terminal_handles_single_word() {
        let input = b"Foobar\n";
        let result = terminal_command(&input[..]);
        assert_eq!(result, "foobar");
    }
}
