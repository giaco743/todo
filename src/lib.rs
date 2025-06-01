use std::fs::read_to_string;
use std::path::PathBuf;

use structre::structre;

#[structre(r".*?//[ \t]*TODO\((?P<ticket_id>[^)]+)\)(?::[ \t]*(?P<message>.*))?")]
#[derive(Debug, PartialEq, Eq)]
pub struct Todo {
    pub ticket_id: String,
    pub message: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TodoInCode {
    pub file: PathBuf,
    pub line: usize,
    pub todo: Todo,
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
                        .map(|todo| TodoInCode {
                            file: file.to_path_buf(),
                            line: n + 1,
                            todo: todo,
                        })
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
    #[test]
    fn test_parse_code_lines(#[case] line: &str, #[case] expected_todo: Todo) {
        assert_eq!(Todo::try_from(line).unwrap(), expected_todo);
    }
}
