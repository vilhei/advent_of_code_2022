mod main_utils;

use std::env;

use advents::{utils::Task, *};
use main_utils::handle_answer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Give a day to run as argument");
    }

    let day: u64 = args[1].parse().expect("Give a number");
    let file_path = format!("./inputs/day{day}.txt");
    let advent: Box<dyn Task> = match day {
        1 => Box::new(Day1),
        2 => Box::new(Day2),
        3 => Box::new(Day3),
        4 => Box::new(Day4),
        5 => Box::new(Day5),
        _ => panic!("Not acceptable day argument"),
    };

    let part_one_result = advent.task_part_one(&file_path);
    handle_answer(&part_one_result, day);
    let part_two_result = advent.task_part_two(&file_path);
    handle_answer(&part_two_result, day);
}
