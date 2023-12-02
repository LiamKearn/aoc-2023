use crate::{Game, Subset};
use nom::character::complete::line_ending;
use nom::combinator::{all_consuming, eof};
use nom::multi::fold_many1;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

fn parse_cube_color(i: &str) -> IResult<&str, &str> {
    alt((tag("red"), tag("green"), tag("blue")))(i)
}

fn parse_cube_count_segment(i: &str) -> IResult<&str, (usize, &str)> {
    separated_pair(
        map_res(digit1, str::parse::<usize>),
        tag(" "),
        parse_cube_color,
    )(i)
}

fn parse_handfull_subset(i: &str) -> IResult<&str, Subset> {
    let (tail, cube_counts) = separated_list1(tag(", "), parse_cube_count_segment)(i)?;
    let mut subset = Subset::default();
    for (count, color) in cube_counts {
        match color {
            "red" => subset.red += count,
            "green" => subset.green += count,
            "blue" => subset.blue += count,
            _ => unreachable!(),
        }
    }
    Ok((tail, subset))
}

fn parse_revealed_subsets(i: &str) -> IResult<&str, Vec<Subset>> {
    separated_list1(tag("; "), parse_handfull_subset)(i)
}

fn parse_game_meta(i: &str) -> IResult<&str, usize> {
    delimited(
        tag("Game "),
        map_res(digit1, str::parse::<usize>),
        tag(": "),
    )(i)
}

fn parse_game(i: &str) -> IResult<&str, Game> {
    map_res(
        tuple((parse_game_meta, parse_revealed_subsets)),
        // there is a spot of dirt below
        |(id, revealed_subsets)| -> Result<Game, ()> {
            Ok(Game {
                id,
                revealed_subsets,
            })
        },
    )(i)
}

pub fn parse_input(i: &str) -> IResult<&str, Vec<Game>> {
    all_consuming(fold_many1(
        terminated(parse_game, alt((line_ending, eof))),
        Vec::default,
        |mut session, game| {
            session.push(game);
            session
        },
    ))(i)
}
