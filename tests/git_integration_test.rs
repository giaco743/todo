use assert_unordered::assert_eq_unordered;
use todo::git;

#[test]
fn test_all_new_issues() {
    assert_eq_unordered!(
        git::resolving_issues("tests/test_git_repo".into()).unwrap(),
        vec![git::TicketId("TEST-1234".to_string())]
    );
}
