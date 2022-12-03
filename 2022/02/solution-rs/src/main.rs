use std::cmp::{PartialEq, PartialOrd, Ordering};
use std::fs;

#[derive(Clone, Copy)]
enum Hands {
    Rock,
    Paper,
    Scissors,
}

impl Hands {
    fn losing(&self) -> Hands {
        match self {
            Hands::Rock => Hands::Scissors,
            Hands::Paper => Hands::Rock,
            Hands::Scissors => Hands::Paper,
        }
    }

    fn winning(&self) -> Hands {
        match self {
            Hands::Rock => Hands::Paper,
            Hands::Paper => Hands::Scissors,
            Hands::Scissors => Hands::Rock,
        }
    }

    fn value(&self) -> i32 {
        match self {
            Hands::Rock => 1,
            Hands::Paper => 2,
            Hands::Scissors => 3,
        }
    }
}

impl PartialEq for Hands {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Hands::Rock, Hands::Rock) => true,
            (Hands::Rock, Hands::Paper) => false,
            (Hands::Rock, Hands::Scissors) => false,
            (Hands::Paper, Hands::Rock) => false,
            (Hands::Paper, Hands::Paper) => true,
            (Hands::Paper, Hands::Scissors) => false,
            (Hands::Scissors, Hands::Rock) => false,
            (Hands::Scissors, Hands::Paper) => false,
            (Hands::Scissors, Hands::Scissors) => true,
        }
    }
}

impl PartialOrd for Hands {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Hands::Rock, Hands::Rock) => Some(Ordering::Equal),
            (Hands::Rock, Hands::Paper) => Some(Ordering::Less),
            (Hands::Rock, Hands::Scissors) => Some(Ordering::Greater),
            (Hands::Paper, Hands::Rock) => Some(Ordering::Greater),
            (Hands::Paper, Hands::Paper) => Some(Ordering::Equal),
            (Hands::Paper, Hands::Scissors) => Some(Ordering::Less),
            (Hands::Scissors, Hands::Rock) => Some(Ordering::Less),
            (Hands::Scissors, Hands::Paper) => Some(Ordering::Greater),
            (Hands::Scissors, Hands::Scissors) => Some(Ordering::Equal),
        }
    }
}

struct Hand {
    shape: Hands,
    value: i32,
}

impl Hand {
    fn new(letter: &str) -> Hand {
        let shape: Hands = match letter {
            "A" | "X" => Hands::Rock,
            "B" | "Y" => Hands::Paper,
            "C" | "Z" => Hands::Scissors,
            char => panic!("Invalid input: {}", char)
        };

        let value = shape.value();

        Hand {
            shape,
            value
        }
    }

    fn new_second(letter: &str, opponent: &Hand) -> Hand {
        let (shape, value) = match (letter, &opponent.shape) {
            ("X", shape) => (shape.losing(), shape.losing().value()),
            ("Y", shape) => (*shape, shape.value() + 3),
            ("Z", shape) => (shape.winning(), shape.winning().value() + 6),
            _ => panic!("Invalid input")
        };

        Hand {
            shape,
            value
        }
    }

    fn value(&self) -> i32 {
        self.value
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.shape == other.shape
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.shape.partial_cmp(&other.shape)
    }
}

fn main() {
    let points: i32 = fs::read_to_string("input")
                            .unwrap()
                            .split('\n')
                            .map(|hands| {
                                if !hands.is_empty() {
                                    let data = hands.split(' ').map(Hand::new).collect::<Vec<Hand>>();
                                    let you = &data[1];
                                    let opponent = &data[0];
                                    let mut points = you.value();
                                    if you > opponent {
                                        points += 6;
                                    } else if you == opponent {
                                        points += 3;
                                    } else {
                                        points += 0;
                                    }
                                    points
                                } else {
                                    0
                                }
                            }).sum();

    println!("One: {points}");


    let points2: i32 = fs::read_to_string("input")
                            .unwrap()
                            .split('\n')
                            .map(|hands| {
                                if !hands.is_empty() {
                                    let data = hands.split(' ').collect::<Vec<&str>>();
                                    let opponent = Hand::new(data[0]);
                                    let you = Hand::new_second(data[1], &opponent);
                                    you.value()
                                } else {
                                    0
                                }
                            }).sum();


    println!("Two: {points2}");
}
