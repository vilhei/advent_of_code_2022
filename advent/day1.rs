use std::{env, fs};

use crate::utils::{read_task_input_file, Task, TaskError, TaskPartOne, TaskPartTwo};

pub struct Day1;

impl Task for Day1 {}

impl TaskPartOne for Day1 {
    fn task_part_one(&self, input_file: &str) -> Result<String, TaskError> {
        let file_contents = read_task_input_file(input_file)?;
        let separator = "\r\n\r\n";
        let splitted = file_contents.split(separator);

        let mut most_calories = 0;

        splitted.for_each(|group| {
            let calories = group
                .split("\r\n")
                .map(|s| s.parse::<isize>().unwrap())
                .sum::<isize>();
            if calories > most_calories {
                most_calories = calories;
            }
        });

        Ok(most_calories.to_string())
    }
}

impl TaskPartTwo for Day1 {
    fn task_part_two(&self, input_file: &str) -> Result<String, TaskError> {
        let file_contents = read_task_input_file(input_file)?;
        let separator = "\r\n\r\n";
        let splitted = file_contents.split(separator);

        let mut calories: Vec<isize> = splitted
            .map(|group| {
                group
                    .split("\r\n")
                    .map(|s| s.parse::<isize>().unwrap())
                    .sum::<isize>()
            })
            .collect();

        calories.sort();

        Ok(calories[calories.len() - 3..calories.len()]
            .iter()
            .sum::<isize>()
            .to_string())
    }
}
