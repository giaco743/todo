use std::fs::read_to_string;
use std::path::PathBuf;

use structre::structre;

#[structre(r".*?//[ \t]*TODO\((?P<ticket_id>[^)]+)\)(?::[ \t]*(?P<message>.*))?")]
#[derive(Debug, PartialEq, Eq)]
struct Todo {
    pub ticket_id: String,
    pub message: Option<String>,
}

impl Todo {
    fn with_location(self, file: PathBuf, line: usize) -> TodoInCode {
        let Todo { ticket_id, message } = self;
        TodoInCode {
            file,
            line,
            ticket_id,
            message,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TodoInCode {
    pub file: PathBuf,
    pub line: usize,
    pub ticket_id: String,
    pub message: Option<String>,
}

pub fn todos(files: Vec<PathBuf>) -> anyhow::Result<Vec<TodoInCode>> {
    let mut todos: Vec<TodoInCode> = Vec::new();
    for file in files {
        todos.append(
            &mut read_to_string(&file)?
                .lines()
                .enumerate()
                .filter_map(|(n, line)| {
                    Todo::try_from(line)
                        .map(|todo| todo.with_location(file.to_path_buf(), n + 1))
                        .ok()
                })
                .collect(),
        );
    }
    Ok(todos)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::standalone_todo_with_message("// TODO(ID-1234): Do this soon", Todo{ticket_id: "ID-1234".to_string(), message: Some("Do this soon".to_string())})]
    #[case::standalone_todo_empty_message("// TODO(ID-1234):", Todo{ticket_id: "ID-1234".to_string(), message: Some("".to_string())})]
    #[case::standalone_todo_empty_message("// TODO(ID-1234)", Todo{ticket_id: "ID-1234".to_string(), message: None})]
    #[case::standalone_todo_with_no_whitespace_message("// TODO(ID-1234):Do this soon", Todo{ticket_id: "ID-1234".to_string(), message: Some("Do this soon".to_string())})]
    #[case::inline_todo_with_message("let x = 5; // TODO(ID-1234): Do this soon", Todo{ticket_id: "ID-1234".to_string(), message: Some("Do this soon".to_string())})]
    #[case::no_whitespace_inline_todo_with_message("let x = 5;// TODO(ID-1234): Do this soon", Todo{ticket_id: "ID-1234".to_string(), message: Some("Do this soon".to_string())})]
    fn test_parse_code_lines(#[case] line: &str, #[case] expected_todo: Todo) {
        assert_eq!(Todo::try_from(line).unwrap(), expected_todo);
    }
}
