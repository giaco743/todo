use assert_unordered::assert_eq_unordered;
use todo::git;

#[test]
fn test_all_new_issues() {
    assert_eq_unordered!(
        git::resolving_issues("tests/test_git_repo", "master").unwrap(),
        vec![
            "TEST-1234".to_string(),
            "TEST-2222".to_string(),
            "TEST-3333".to_string(),
            "TEST-4444".to_string()
        ]
    );
}
