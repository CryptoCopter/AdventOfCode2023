// look at this abomination
// yes I know that using regex would be easier

use std::collections::HashMap;
use crate::day01::ParseReturn::{Continue, Fail, Success};

#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
enum Numbers {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

const ONE_CHARS: [char; 3] = ['o', 'n', 'e'];
const TWO_CHARS: [char; 3] = ['t', 'w', 'o'];
const THREE_CHARS: [char; 5] = ['t', 'h', 'r', 'e', 'e'];
const FOUR_CHARS: [char; 4] = ['f', 'o', 'u', 'r'];
const FIVE_CHARS: [char; 4] = ['f', 'i', 'v', 'e'];
const SIX_CHARS: [char; 3] = ['s', 'i', 'x'];
const SEVEN_CHARS: [char; 5] = ['s', 'e', 'v', 'e', 'n'];
const EIGHT_CHARS: [char; 5] = ['e', 'i', 'g', 'h', 't'];
const NINE_CHARS: [char; 4] = ['n', 'i', 'n', 'e'];

enum ParseReturn {
    Success(char),
    Fail,
    Continue,
}

#[derive(Debug)]
struct Parser<'a> {
    index: usize,
    possible: Vec<Numbers>,
    character_mapping: &'a HashMap<Numbers, Vec<char>>,
    character_lookup: &'a HashMap<Numbers, char>,
}

impl Parser<'_> {
    fn new<'a>(character_mapping: &'a HashMap<Numbers, Vec<char>>, character_lookup: &'a HashMap<Numbers, char>) -> Parser<'a> {
        let all = vec![Numbers::One, Numbers::Two, Numbers::Three, Numbers::Four, Numbers::Five, Numbers::Six, Numbers::Seven, Numbers::Eight, Numbers::Nine];
        Parser{index: 0, possible: all, character_mapping, character_lookup}
    }

    fn filter_possible(&self, c: &char) -> Vec<Numbers> {
        let mut filtered: Vec<Numbers> = Vec::new();

        for number in &self.possible {
            let word = self.character_mapping.get(number).unwrap();
            if word[self.index] == *c {
                filtered.push(number.clone());
            }
        }

        filtered
    }

    fn consume(&mut self, c: &char) -> ParseReturn {
        self.possible = self.filter_possible(c);
        self.index += 1;

        if self.possible.len() == 0 {
            return Fail
        }

        if self.possible.len() == 1 && self.character_mapping.get(&self.possible[0]).unwrap().len() == self.index {
            return Success(self.character_lookup.get(&self.possible[0]).unwrap().clone())
        }

        Continue
    }
}

pub fn solve(task_input: Vec<String>) -> u64 {
    let mut character_mapping = HashMap::new();
    character_mapping.insert(Numbers::One, ONE_CHARS.to_vec());
    character_mapping.insert(Numbers::Two, TWO_CHARS.to_vec());
    character_mapping.insert(Numbers::Three, THREE_CHARS.to_vec());
    character_mapping.insert(Numbers::Four, FOUR_CHARS.to_vec());
    character_mapping.insert(Numbers::Five, FIVE_CHARS.to_vec());
    character_mapping.insert(Numbers::Six, SIX_CHARS.to_vec());
    character_mapping.insert(Numbers::Seven, SEVEN_CHARS.to_vec());
    character_mapping.insert(Numbers::Eight, EIGHT_CHARS.to_vec());
    character_mapping.insert(Numbers::Nine, NINE_CHARS.to_vec());

    let mut character_lookup = HashMap::new();
    character_lookup.insert(Numbers::One, '1');
    character_lookup.insert(Numbers::Two, '2');
    character_lookup.insert(Numbers::Three, '3');
    character_lookup.insert(Numbers::Four, '4');
    character_lookup.insert(Numbers::Five, '5');
    character_lookup.insert(Numbers::Six, '6');
    character_lookup.insert(Numbers::Seven, '7');
    character_lookup.insert(Numbers::Eight, '8');
    character_lookup.insert(Numbers::Nine, '9');

    let mut total: u64 = 0;

    for line in task_input {
        let mut line_total: Vec<char> = Vec::new();
        let mut parsers: Vec<Parser> = Vec::new();
        for (_, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                line_total.push(c);
                parsers = Vec::new();
            } else {
                let mut active_parsers: Vec<Parser> = Vec::new();
                parsers.push(Parser::new(&character_mapping, &character_lookup));
                for mut parser in parsers {
                    match parser.consume(&c) {
                        Success(ch) => line_total.push(ch),
                        Continue => active_parsers.push(parser),
                        _ => ()
                    };
                }
                parsers = active_parsers;
            }
        }
        let combined: String = format!("{}{}", line_total.first().unwrap(), line_total.last().unwrap());
        total += combined.parse::<u64>().unwrap();
    }

    total
}