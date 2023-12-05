use std::num::ParseIntError;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

const REDS_MAX: u32 = 12;
const GREENS_MAX: u32 = 13;
const BLUES_MAX: u32 = 14;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub struct Game {
    id: u32,
    sub_games: Vec<SubGame>,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct SubGame {
    reds: u32,
    greens: u32,
    blues: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn count_from_string(input: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(input, 10)
}

impl Game {
    pub fn parse(game: &str) -> IResult<&str, Self> {
        let id_and_game_data_pair =
            separated_pair(Self::parse_game_id, char(':'), Self::parse_subgames);
        map(id_and_game_data_pair, |(id, sub_games)| {
            return Game { id, sub_games };
        })(game)
    }

    fn parse_game_id(input: &str) -> IResult<&str, u32> {
        let (remainder, (_, id)) =
            separated_pair(tag("Game"), char(' '), map_res(digit1, count_from_string))(input)?;
        Ok((remainder, id))
    }

    fn parse_subgames(input: &str) -> IResult<&str, Vec<SubGame>> {
        let input = input.trim();
        separated_list1(tag("; "), SubGame::parse)(input)
    }

    pub fn possible(&self) -> bool {
        self.sub_games.iter().all(SubGame::possible)
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl SubGame {
    fn parse(subgame: &str) -> IResult<&str, Self> {
        let subgame = subgame.trim();
        let subgame_parser = separated_list1(tag(","), Color::parse);
        let mut subgame_parser = map(subgame_parser, |colors| {
            let mut blues = 0;
            let mut greens = 0;
            let mut reds = 0;
            for color in colors.iter() {
                match color {
                    Color::Red(count) => reds += count,
                    Color::Green(count) => greens += count,
                    Color::Blue(count) => blues += count,
                }
            }
            SubGame {
                reds,
                blues,
                greens,
            }
        });

        subgame_parser(subgame)
    }

    fn possible(&self) -> bool {
        self.reds <= REDS_MAX && self.greens <= GREENS_MAX && self.blues <= BLUES_MAX
    }
}

impl Color {
    fn parse(color_and_count: &str) -> IResult<&str, Self> {
        let color_and_count = color_and_count.trim();
        let color_count_parser =
            separated_pair(map_res(digit1, count_from_string), char(' '), alpha1);
        let mut color_parser = map(color_count_parser, |(count, color)| {
            if color == "green" {
                Self::Green(count)
            } else if color == "red" {
                Self::Red(count)
            } else if color == "blue" {
                Self::Blue(count)
            } else {
                panic!("Cannot parse color: {color}");
            }
        });

        color_parser(color_and_count)
    }
}
