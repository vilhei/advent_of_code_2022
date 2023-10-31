use std::collections::HashMap;
use std::collections::VecDeque;

use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::character::complete::space1;
use nom::IResult;

use nom::sequence::preceded;
use nom::sequence::separated_pair;

use crate::utils::{read_task_input_file, Task};

pub struct Day7;

struct Directory {
    parent_dir: Option<String>,
    items: Vec<ItemType>,
    name: String,
    size: usize,
}
impl Directory {
    fn new() -> Self {
        Directory {
            parent_dir: Default::default(),
            items: Default::default(),
            name: Default::default(),
            size: 0,
        }
    }
    fn add_item(&mut self, item: ItemType) {
        self.items.push(item);
        match item {
            ItemType::File(_) => todo!(),
            ItemType::Dir(_) => todo!(),
        }
        self.update_size()
    }

    fn update_size(&mut self, val: usize) {
        self.size += val;
    }

    // fn size(&self, dirs: &HashMap<String, Directory>) -> usize {
    //     let mut total = 0;

    //     for item in &self.items {
    //         match item {
    //             ItemType::File(s) => total += s,
    //             ItemType::Dir(d) => total += dirs.get(d).expect("Dir not found").size(dirs),
    //         }
    //         if total > 100001 {
    //             break;
    //         }
    //     }
    //     total
    // }
}

impl From<String> for Directory {
    fn from(name: String) -> Self {
        let mut d = Directory::new();
        d.name = name.clone();
        d
    }
}

impl Default for Directory {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
enum ItemType {
    File(usize),
    Dir(String),
}

#[derive(Debug)]
enum CLIAction {
    CD(String),
    LS,
    Item(ItemType),
}

impl Task for Day7 {
    fn task_part_one(&self, input_file: &str) -> Result<String, crate::utils::TaskError> {
        let file_contents = read_task_input_file(input_file)?;
        let mut commands = parse_commands(&file_contents);
        // println!("{:?}", commands);
        // dbg!(&commands[0..=10]);
        let mut directories: HashMap<String, Directory> = HashMap::new();

        // Assume first command is cd to root
        let first_command = commands.pop_front().expect("No commands were found");
        let root;
        let mut curr_dir;
        match first_command {
            CLIAction::CD(name) => {
                directories.insert(name.clone(), Directory::from(name.clone()));
                root = name;
                curr_dir = root;
            }
            CLIAction::LS => panic!("First command was not cd to the root folder"),
            CLIAction::Item(_) => panic!("First command was not cd to the root folder"),
        };

        for command in commands {
            match command {
                CLIAction::CD(dir) => curr_dir = handle_cd(dir,curr_dir, &mut directories),
                CLIAction::LS => {}
                CLIAction::Item(i) => {
                    directories
                        .get_mut(&curr_dir)
                        .expect("Did not find directory with given name")
                        .items
                        .push(i);
                }
                // CLIAction::Item(ItemType::Dir(_)) => todo!(),
                // CLIAction::Item(ItemType::File(_)) => todo!(),
            }
        }

        let sum_of_dirs_under_100k = directories
            .iter()
            .filter(|(_, d)| d.size(&directories) <= 100000)
            .fold(0, |acc, (_, val)| acc + val.size(&directories));
        Ok(sum_of_dirs_under_100k.to_string())
        // Err(crate::utils::TaskError::NotImplemented(1))
    }

    fn task_part_two(&self, _input_file: &str) -> Result<String, crate::utils::TaskError> {
        Err(crate::utils::TaskError::NotImplemented(2))
    }
}

fn handle_cd(dir_name: String, curr_dir: String, dirs: &mut HashMap<String, Directory>) -> String {
    if dirs.contains_key(&dir_name) {
        return dir_name;
    }

    if dir_name == ".." {
        if curr_dir == "/" {
            return curr_dir;
        }
        return dirs
            .get(&curr_dir)
            .unwrap()
            .parent_dir
            .as_ref()
            .expect("Did not have parent")
            .to_string();
    }

    let mut new_dir = Directory::from(dir_name.clone());
    new_dir.parent_dir = Some(curr_dir);
    dirs.insert(dir_name.clone(), new_dir);
    dir_name
}

fn parse_commands(input: &str) -> VecDeque<CLIAction> {
    // let (_, res) = separated_list1(newline, parse_line)(input).expect("Parsing of lines failed");
    let mut res = VecDeque::new();
    for line in input.split('\n') {
        let (_, a) = parse_line(line).unwrap();
        res.push_back(a);
    }
    res
}

// fn get_cd_parse(input: &str) -> IResult<&str, &str> {
//     tag("$ cd ")(input)
// }
fn parse_line(input: &str) -> IResult<&str, CLIAction> {
    // let get_cd = tag::<&str, &str, nom::error::Error<&str>>("$ cd ");
    let get_cd = tag::<&str, &str, nom::error::Error<&str>>("$ cd ");
    let mut cd_parser = preceded(get_cd, is_not("\n"));

    let mut item_parser = separated_pair(
        alphanumeric1::<&str, nom::error::Error<&str>>,
        space1,
        is_not("\n"),
    );

    if input.contains("cd ") {
        let (input, res) = cd_parser(input).expect("CD line did not contain target");
        return Ok((input, CLIAction::CD(res.to_string())));
    }

    if input.contains(" ls") {
        return Ok((input, CLIAction::LS));
    }

    let (input, res) = item_parser(input).expect("Item was not in form of 'size name'");
    let ls = match res.0 {
        "dir" => CLIAction::Item(ItemType::Dir(res.1.to_string())),
        _ => CLIAction::Item(ItemType::File(
            res.0.parse().expect("File line did not contain size"),
        )),
    };
    Ok((input, ls))

    // let (input, res) = alt((cd_parser, ls_parser, item_parser))(input)?;

    // let result: CLIAction = match res.0 {
    //     "cd" => CLIAction::CD(res.1.to_string()),
    //     "ls" => CLIAction::LS,
    //     _ => CLIAction::Item(
    //         res.0
    //             .parse::<usize>()
    //             .expect("Item did not have correct size"),
    //     ),
    // };
    // Ok((input, result))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{utils::Task, Day7};

    #[test]
    fn day7_test_data() {
        let advent = Day7;
        let file_path = "./inputs/day7_test.txt";
        let result = advent.task_part_one(file_path).unwrap();
        let expected = "95437";
        assert_eq!(expected, result);
    }
}
