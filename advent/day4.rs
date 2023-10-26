use crate::utils::{read_task_input_file, Task};

pub struct Day4;

impl Task for Day4 {
    fn task_part_one(&self, input_file: &str) -> Result<String, crate::utils::TaskError> {
        let file_contents = read_task_input_file(input_file)?;
        let lines = file_contents.split('\n');
        let lines = lines.map(|line| {
            let mut itr = line.split(',');
            let left = itr
                .next()
                .expect("Line did not contain 2 values separated by comma");
            let right = itr
                .next()
                .expect("Line did not contain 2 values separated by comma");

            itr = left.split('-');
            let left_s = itr
                .next()
                .expect("Left did not have 2 values separated by -");
            let left_e = itr
                .next()
                .expect("Left did not have 2 values separated by -");

            itr = right.split('-');
            let right_s = itr
                .next()
                .expect("Right did not have 2 values separated by -");
            let right_e = itr
                .next()
                .expect("Right did not have 2 values separated by -");

            [(left_s, left_e), (right_s, right_e)]
        });

        let mut fully_contained_total = 0;

        lines.for_each(|line| {
            let left_s = line[0].0.parse::<usize>().expect("Was not number");
            let right_s = line[1].0.parse::<usize>().expect("Was not number");

            let left_e = line[0].1.parse::<usize>().expect("Was not number");
            let right_e = line[1].1.parse::<usize>().expect("Was not number");

            if left_s >= right_s && left_e <= right_e {
                fully_contained_total += 1;
                return;
            }

            if right_s >= left_s && right_e <= left_e {
                fully_contained_total += 1;
            }
        });

        Ok(fully_contained_total.to_string())
    }

    fn task_part_two(&self, input_file: &str) -> Result<String, crate::utils::TaskError> {
        let file_contents = read_task_input_file(input_file)?;
        let lines = file_contents.split('\n');
        let lines = lines.map(|line| {
            let mut itr = line.split(',');
            let left = itr
                .next()
                .expect("Line did not contain 2 values separated by comma");
            let right = itr
                .next()
                .expect("Line did not contain 2 values separated by comma");

            itr = left.split('-');
            let left_s = itr
                .next()
                .expect("Left did not have 2 values separated by -");
            let left_e = itr
                .next()
                .expect("Left did not have 2 values separated by -");

            itr = right.split('-');
            let right_s = itr
                .next()
                .expect("Right did not have 2 values separated by -");
            let right_e = itr
                .next()
                .expect("Right did not have 2 values separated by -");

            [(left_s, left_e), (right_s, right_e)]
        });

        let mut fully_contained_total = 0;

        lines.for_each(|line| {
            let left_s = line[0].0.parse::<usize>().expect("Was not number");
            let right_s = line[1].0.parse::<usize>().expect("Was not number");

            let left_e = line[0].1.parse::<usize>().expect("Was not number");
            let right_e = line[1].1.parse::<usize>().expect("Was not number");

            if left_s >= right_s && left_s <= right_e {
                fully_contained_total += 1;
                return;
            }

            if left_e >= right_s && left_e <= right_e {
                fully_contained_total += 1;
                return;
            }

            if right_s >= left_s && right_s <= left_e {
                fully_contained_total += 1;
                return;
            }

            if right_e >= left_s && right_e <= left_e {
                fully_contained_total += 1;
            }
        });

        Ok(fully_contained_total.to_string())
    }
}
