use crate::utils::{read_task_input_file, Task, TaskError};

pub struct Day6;

impl Task for Day6 {
    fn task_part_one(&self, input_file: &str) -> Result<String, TaskError> {
        let file_contents = read_task_input_file(input_file)?;
        find_n_distinct_characters(file_contents, 4)
    }

    fn task_part_two(&self, input_file: &str) -> Result<String, TaskError> {
        let file_contents = read_task_input_file(input_file)?;
        find_n_distinct_characters(file_contents, 14)
    }
}

fn find_n_distinct_characters(file_contents: String, n: usize) -> Result<String, TaskError> {
    let chars = file_contents.chars().collect::<Vec<char>>();
    assert!(n <= 14, "MAX N SIZE ALLOWED IS 14");
    let res = chars
        .windows(n)
        .enumerate()
        .find(|(_, window)| {
            let mut s2: [char; 14] = Default::default();

            for (idx, c) in (*window).iter().enumerate() {
                if s2.contains(c) {
                    return false;
                }
                s2[idx] = *c;
            }
            
            true
            // Using HashSet is a lot slower than the static alloy (~850 ms vs 0.15 ms on my machine).

            // let s: HashSet<&char> = HashSet::from_iter(*window);
            // s.len() >= n
        })
        .unwrap_or_else(|| panic!("Input did not have {n} consecutive unique characters"));

    let idx = res.0 + n;

    Ok(idx.to_string())
}

#[cfg(test)]
mod tests {
    use crate::utils::Task;
    use crate::Day6;
    use std::collections::BTreeMap;
    use std::fs;
    #[test]
    fn day6_task1_example_data() {
        let data: BTreeMap<usize, &str> = BTreeMap::from([
            (7, "mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            (5, "bvwbjplbgvbhsrlpgdmjqwftvncz"),
            (6, "nppdvjthqldpwncqszvftbrmjlhg"),
            (10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            (11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
        ]);

        let advent = Day6;
        for (expected, input) in data {
            let file_path = "./inputs/day6_test_file.txt";
            fs::write(file_path, input).expect("Failed to create temporary test file for day 6");
            let result = advent.task_part_one(file_path).expect("Task one failed");
            fs::remove_file(file_path).expect("Failed to remove the temporary test file");
            assert_eq!(expected.to_string(), result)
        }
    }

    #[test]
    fn day6_task2_example_data() {
        let data: BTreeMap<usize, &str> = BTreeMap::from([
            (19, "mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            (23, "bvwbjplbgvbhsrlpgdmjqwftvncz"),
            (23, "nppdvjthqldpwncqszvftbrmjlhg"),
            (29, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            (26, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
        ]);

        let advent = Day6;
        for (expected, input) in data {
            let file_path = "./inputs/day6_test_file.txt";
            fs::write(file_path, input).expect("Failed to create temporary test file for day 6");
            let result = advent.task_part_two(file_path).expect("Task one failed");
            fs::remove_file(file_path).expect("Failed to remove the temporary test file");
            assert_eq!(expected.to_string(), result)
        }
    }

    extern crate test;
    use test::Bencher;

    #[bench]
    fn day6_perf_task2(b: &mut Bencher) {
        let file_path = "./inputs/day6.txt";
        let file_contents = crate::utils::read_task_input_file(file_path).unwrap();

        b.iter(|| super::find_n_distinct_characters(file_contents.clone(), 14));
    }
}
