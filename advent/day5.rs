use crate::utils::{read_task_input_file, Task, TaskError};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::{alpha1, newline};

use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use std::iter;

pub struct Day5;

impl Task for Day5 {
    fn task_part_one(&self, input_file: &str) -> Result<String, TaskError> {
        let file_contents = read_task_input_file(input_file)?;
        let (mut crates, moves) = parse_input(&file_contents);

        Self::complete_moves(&mut crates, moves, false);
        let answer = Self::parse_answer(&mut crates);

        Ok(answer)
    }

    fn task_part_two(&self, input_file: &str) -> Result<String, TaskError> {
        let file_contents = read_task_input_file(input_file)?;
        let (mut crates, moves) = parse_input(&file_contents);

        Self::complete_moves(&mut crates, moves, true);
        let answer = Self::parse_answer(&mut crates);

        Ok(answer)
    }
}

impl Day5 {
    fn complete_moves(crates: &mut [Vec<&str>], moves: Vec<Move>, multiple_in_move: bool) {
        for m in moves {
            let length = crates[m.from as usize].len();
            let to_move = crates[m.from as usize].drain((length - m.number as usize)..);
            let mut moved_crates;

            if multiple_in_move {
                moved_crates = to_move.collect::<Vec<&str>>();
            } else {
                moved_crates = to_move.rev().collect::<Vec<&str>>();
            }

            crates[m.to as usize].append(&mut moved_crates);
        }
    }

    fn parse_answer(crates: &mut [Vec<&str>]) -> String {
        let answer: String = crates
            .iter()
            .flat_map(|el| el.last())
            .fold("".to_string(), |acc, &curr| acc + curr);
        answer
    }
}

#[derive(Debug)]
struct Move {
    number: u32,
    from: u32,
    to: u32,
}

fn parse_input(input: &str) -> (Vec<Vec<&str>>, Vec<Move>) {
    let mut itr = input.split("\n\n");
    let crate_str = itr.next().expect("Input did not have crates");

    let mut crate_str = crate_str.split('\n').rev();
    let moves_str = itr.next().expect("Input did not have moves");

    let num_row = crate_str
        .next()
        .expect("Crates were empty")
        .replace(' ', "");
    let mut crates_horizontal: Vec<Vec<Option<&str>>> = Vec::with_capacity(num_row.len());

    for line in crate_str {
        let (_, result) =
            separated_list1(tag(" "), parse_crate)(line).expect("Failed to parse crates");
        crates_horizontal.push(result);
    }

    let mut crates_vertical: Vec<Vec<Option<&str>>> =
        iter::repeat(Vec::new()).take(num_row.len()).collect();

    for row in crates_horizontal {
        for (idx, el) in row.iter().enumerate() {
            crates_vertical[idx].push(*el);
        }
    }
    let crates_vertical: Vec<Vec<&str>> = crates_vertical
        .iter()
        .map(|vec| vec.iter().filter_map(|el| *el).collect())
        .collect();

    let (_, moves) =
        separated_list1(newline, parse_move)(moves_str).expect("Failed to parse moves");
    (crates_vertical, moves)
}

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = alt((
        tag("   "),
        delimited(complete::char('['), alpha1, complete::char(']')),
    ))(input)?;

    let result = match c {
        "   " => None,
        value => Some(value),
    };
    Ok((input, result))
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (input, number) = preceded(alt((tag("move "), tag(" move "))), complete::u32)(input)?;

    let (input, mut from) = preceded(tag(" from "), complete::u32)(input)?;

    let (input, mut to): (_, u32) = preceded(tag(" to "), complete::u32)(input)?;
    to -= 1;
    from -= 1;
    Ok((input, Move { number, from, to }))
}

#[cfg(test)]
mod tests {
    use crate::utils::Task;
    use crate::Day5;

    #[test]
    fn example_test() {
        let path = "./inputs/day5_test.txt";
        let advent = Day5;
        let result = advent.task_part_one(path).unwrap();
        let expected = "CMZ";
        assert_eq!(result, expected);
    }
}
