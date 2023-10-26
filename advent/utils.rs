use std::fs;

pub enum TaskError {
    InvalidFilePath(String),
    NotImplemented(usize),
}

pub fn read_task_input_file(path: &str) -> Result<String, TaskError> {
    let Ok(file_contents) = fs::read_to_string(path) else {
        return Err(TaskError::InvalidFilePath(path.to_string()));
    };
    Ok(file_contents)
}

// pub trait Task: TaskPartOne + TaskPartTwo {}
pub trait Task {
    fn task_part_one(&self, _input_file: &str) -> Result<String, TaskError> {
        Err(TaskError::NotImplemented(1))
    }
    fn task_part_two(&self, _input_file: &str) -> Result<String, TaskError> {
        Err(TaskError::NotImplemented(2))
    }
}

// pub trait TaskPartOne {
//     fn task_part_one(&self, _input_file: &str) -> Result<String, TaskError> {
//         Err(TaskError::NotImplemented(1))
//     }
// }

// pub trait TaskPartTwo {
//     fn task_part_two(&self, _input_file: &str) -> Result<String, TaskError> {
//         Err(TaskError::NotImplemented(2))
//     }
// }
