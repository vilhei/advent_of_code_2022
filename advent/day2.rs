use crate::utils::{read_task_input_file, Task, TaskPartOne, TaskPartTwo};

pub struct Day2;

impl Task for Day2 {}

impl TaskPartOne for Day2 {
    fn task_part_one(&self, input_file: &str) -> Result<String, crate::utils::TaskError> {
        let file_contents = read_task_input_file(input_file)?;
        let separator = "\n";
        let splitted = file_contents.split(separator);

        let mut point_sum = 0;

        splitted.for_each(|line| {
            let mut itr = line.split_ascii_whitespace();
            let opponent_shape = itr.next().expect("Line did not have correct inputs");
            let my_shape = itr.next().expect("Line did not have correct inputs");

            let opponent_shape = match opponent_shape {
                "A" => Rps::Rock,
                "B" => Rps::Paper,
                "C" => Rps::Scissors,
                _ => panic!("Unexpected type {opponent_shape}"),
            };

            let my_shape = match my_shape {
                "X" => Rps::Rock,
                "Y" => Rps::Paper,
                "Z" => Rps::Scissors,
                _ => panic!("Unexpected type {my_shape}"),
            };

            point_sum += my_shape.outcome(&opponent_shape);
        });

        Ok(point_sum.to_string())
    }
}
impl TaskPartTwo for Day2 {
    fn task_part_two(&self, input_file: &str) -> Result<String, crate::utils::TaskError> {
        let file_contents = read_task_input_file(input_file)?;
        let separator = "\n";
        let splitted = file_contents.split(separator);

        let mut point_sum = 0;

        splitted.for_each(|line| {
            let mut itr = line.split_ascii_whitespace();
            let opponent_shape = itr.next().expect("Line did not have correct inputs");
            let my_shape = itr.next().expect("Line did not have correct inputs");

            let opponent_shape = match opponent_shape {
                "A" => Rps::Rock,
                "B" => Rps::Paper,
                "C" => Rps::Scissors,
                _ => panic!("Unexpected type {opponent_shape}"),
            };

            let my_shape = match my_shape {
                "X" => opponent_shape.shape_wins(),
                "Y" => opponent_shape,
                "Z" => opponent_shape.shape_loses_to(),
                _ => panic!("Unexpected type {my_shape}"),
            };

            point_sum += my_shape.outcome(&opponent_shape);
        });

        Ok(point_sum.to_string())
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for Rps {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Rps::Rock => match other {
                Rps::Rock => Some(std::cmp::Ordering::Equal),
                Rps::Paper => Some(std::cmp::Ordering::Less),
                Rps::Scissors => Some(std::cmp::Ordering::Greater),
            },
            Rps::Paper => match other {
                Rps::Rock => Some(std::cmp::Ordering::Greater),
                Rps::Paper => Some(std::cmp::Ordering::Equal),
                Rps::Scissors => Some(std::cmp::Ordering::Less),
            },
            Rps::Scissors => match other {
                Rps::Rock => Some(std::cmp::Ordering::Less),
                Rps::Paper => Some(std::cmp::Ordering::Greater),
                Rps::Scissors => Some(std::cmp::Ordering::Equal),
            },
        }
    }
}

impl Rps {
    fn shape_score(&self) -> usize {
        match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }

    fn shape_loses_to(&self) -> Self {
        match self {
            Rps::Rock => Rps::Paper,
            Rps::Paper => Rps::Scissors,
            Rps::Scissors => Rps::Rock,
        }
    }
    fn shape_wins(&self) -> Self {
        match self {
            Rps::Rock => Rps::Scissors,
            Rps::Paper => Rps::Rock,
            Rps::Scissors => Rps::Paper,
        }
    }

    fn outcome(&self, other: &Self) -> usize {
        let outcome_score = match self.partial_cmp(other).unwrap() {
            std::cmp::Ordering::Less => 0,
            std::cmp::Ordering::Equal => 3,
            std::cmp::Ordering::Greater => 6,
        };
        outcome_score + self.shape_score()
    }
}
