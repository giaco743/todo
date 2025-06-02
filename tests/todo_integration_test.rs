use assert_unordered::assert_eq_unordered;
use todo::todos::{todos, TodoInCode};

#[test]
fn all_files() {
    let expected = vec![
        TodoInCode {
            file: "tests/data/a.cds".into(),
            line: 6,
            ticket_id: "issue1234".to_string(),
            message: Some("Do this and that".to_string()),
        },
        TodoInCode {
            file: "tests/data/b.txt".into(),
            line: 3,
            ticket_id: "issue1234".to_string(),
            message: None,
        },
        TodoInCode {
            file: "tests/data/b.txt".into(),
            line: 5,
            ticket_id: "issue4321".to_string(),
            message: Some("Do this do that".to_string()),
        },
        TodoInCode {
            file: "tests/data/rs_file.rs".into(),
            line: 1,
            ticket_id: "id1234".to_string(),
            message: Some("Do something".to_string()),
        },
    ];
    let given_paths = vec![
        "tests/data/a.cds".into(),
        "tests/data/b.txt".into(),
        "tests/data/rs_file.rs".into(),
    ];
    assert_eq_unordered!(expected, todos(given_paths).unwrap());
}
