use nom::branch::alt;
use nom::bytes::complete::{is_a, is_not, take_till, take_until};
use nom::character::complete::{digit1, space0, space1};
use nom::character::is_newline;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use nom::{bytes::complete::tag, character::complete::newline, multi::separated_list1};

use crate::utils::{read_task_input_file, Task};

pub struct Day7;

#[derive(Debug)]
enum CLIAction {
    CD(String),
    LS,
    Item(usize),
}

impl Task for Day7 {
    fn task_part_one(&self, input_file: &str) -> Result<String, crate::utils::TaskError> {
        let file_contents = read_task_input_file(input_file)?;
        let commands = parse_commands(&file_contents);
        println!("{:?}", commands);
        Err(crate::utils::TaskError::NotImplemented(1))
    }

    fn task_part_two(&self, _input_file: &str) -> Result<String, crate::utils::TaskError> {
        Err(crate::utils::TaskError::NotImplemented(2))
    }
}

fn parse_commands(input: &str) -> Vec<CLIAction> {
    // let (_, res) = separated_list1(newline, parse_line)(input).expect("Parsing of lines failed");
    let mut res = Vec::new();
    for line in input.split('\n') {
        let (_, a) = parse_line(line).unwrap();
        res.push(a);
    }
    res
}

fn parse_line(input: &str) -> IResult<&str, CLIAction> {
    let cd_parser = separated_pair(preceded(tag("$ "), is_a("cd")), space1, is_not("\n"));
    let item_parser = separated_pair(digit1, space1, is_not("\n"));
    let ls_parser = separated_pair(preceded(tag("$ "), is_a("ls")), space0, is_a("\n")); // Just return newline to get tuple type for ls too

    let (input, res) = alt((cd_parser, ls_parser, item_parser))(input)?;

    let result: CLIAction = match res.0 {
        "cd" => CLIAction::CD(res.1.to_string()),
        "ls" => CLIAction::LS,
        _ => CLIAction::Item(
            res.0
                .parse::<usize>()
                .expect("Item did not have correct size"),
        ),
    };
    Ok((input, result))
}
