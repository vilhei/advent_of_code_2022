use std::collections::HashSet;

use crate::utils::{read_task_input_file, Task, TaskPartOne, TaskPartTwo};

pub struct Day3;

impl Task for Day3 {}

impl TaskPartOne for Day3 {
    fn task_part_one(&self, input_file: &str) -> Result<String, crate::utils::TaskError> {
        let file_contents = read_task_input_file(input_file)?;
        let splitted = file_contents.split('\n');

        let mut total_score = 0;

        splitted.for_each(|line| {
            let (left, right) = line.split_at(line.len() / 2);

            let right_set: HashSet<char> = HashSet::from_iter(right.chars());
            let left_set: HashSet<char> = HashSet::from_iter(left.chars());

            let common = left_set
                .intersection(&right_set)
                .next()
                .expect("Left and right side did not have common character");
            let score = match common {
                'a'..='z' => *common as u32 - 96,
                'A'..='Z' => *common as u32 - 38,

                _ => panic!("Not an english alphabet letter"),
            };
            total_score += score;
        });

        Ok(total_score.to_string())
    }
}
impl TaskPartTwo for Day3 {
    fn task_part_two(&self, input_file: &str) -> Result<String, crate::utils::TaskError> {
        let file_contents = read_task_input_file(input_file)?;
        let splitted = file_contents.split('\n');

        let mut total_score = 0;
        let split_vector = splitted.collect::<Vec<&str>>();
        let groups = split_vector.chunks(3);

        groups.for_each(|line| {
            // let (elf1, elf2, elf3) = (line[0], line[1], line[2]);
            let set1: HashSet<char> = HashSet::from_iter(line[0].chars());
            let set2: HashSet<char> = HashSet::from_iter(line[1].chars());
            let set3: HashSet<char> = HashSet::from_iter(line[2].chars());

            let sets = [&set1, &set2, &set3];
            let common = set1
                .iter()
                .find(|el| sets.iter().all(|s| s.contains(el)))
                .expect("3 lines did not contain same character");

            let score = match common {
                'a'..='z' => *common as u32 - 96,
                'A'..='Z' => *common as u32 - 38,
                _ => panic!("Not an english alphabet letter"),
            };
            total_score += score;
        });

        Ok(total_score.to_string())
    }
}
