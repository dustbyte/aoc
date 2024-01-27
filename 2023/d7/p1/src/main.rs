use std::{cmp::Ordering, default, fs, iter::zip, process::exit};

fn read_file(path: String) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn get_file_name() -> String {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: {} input_file", &args[0]);
        exit(-1)
    }

    args[1].clone()
}

fn get_content() -> Vec<String> {
    read_file(get_file_name())
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Default, Debug)]
enum HandType {
    #[default]
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd)]
struct Hand {
    hand_type: HandType,
    cards: [u32; 5],
    bid: u32
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            type_ord if type_ord == Ordering::Equal => type_ord,
            _ => {
                for idx in 0..self.cards.len() {
                    let res = self.cards[idx].cmp(&other.cards[idx]);
                    if res != Ordering::Equal {
                        return res
                    }
                }
                Ordering::Equal
            }
        }
    }
}

fn parse_hand(line: &str) -> Hand {
    let mut hand = Hand{..Default::default()};
    let mut counts = [0u32; 13];
    let items = line.trim().split_once(" ").unwrap();

    hand.bid = items.1.parse().unwrap();

    items.0.chars().map(|c| match c {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => 0,
    }).enumerate().for_each(|(idx, card)| {
        hand.cards[idx] = card;
        counts[card as usize] += 1;
    });

    use HandType::*;

    counts.sort();
    counts.reverse();
    hand.hand_type = match &counts[0..4] {
        [5, 0, 0, 0] => FiveOfKind,
        [4, 1, 0, 0] => FourOfKind,
        [3, 2, 0, 0] => FullHouse,
        [3, 1, 1, 0] => ThreeOfKind,
        [2, 2, 1, 0] => TwoPair,
        [2, 1, 1, 1] => OnePair,
        _ => HighCard,
    };

    hand
}

fn main() {
    let content = get_content();
    let mut hands = content.iter().map(|line| parse_hand(&line)).collect::<Vec<Hand>>();
    hands.sort();
    let score = hands.iter().enumerate().map(|(idx, hand)| hand.bid * (idx as u32 + 1)).sum::<u32>();

    println!("{:#?}", score);
}
