use std::path::Path;

use git2::Repository;
use regex::Regex;

fn extract_issue_ids(commit_msg: &str) -> Vec<String> {
    let re = Regex::new(r"(?m)^Issue:\s*(.+)$").unwrap();

    if let Some(caps) = re.captures(commit_msg) {
        let issues = &caps[1];
        return issues
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
    }

    vec![]
}

fn new_commit_messages<P: AsRef<Path>>(path: P, base: &str) -> anyhow::Result<Vec<String>> {
    let repo = Repository::open(path)?;

    // Referenzen auf HEAD und origin/master
    let head = repo.revparse_single("HEAD")?.peel_to_commit()?;
    let base = repo.revparse_single(base)?.peel_to_commit()?;

    // Revwalk vorbereiten: alle Commits von HEAD bis origin/master (nur was HEAD voraus ist)
    let mut revwalk = repo.revwalk()?;
    revwalk.push(head.id())?;
    revwalk.hide(base.id())?;

    Ok(revwalk
        .into_iter()
        .filter_map(|commit_id| {
            let id = commit_id.ok()?;
            let commit = repo.find_commit(id).ok()?;
            let message = commit.message()?.to_string();
            Some(message)
        })
        .collect())
}

pub fn resolving_issues<P: AsRef<Path>>(path: P, base: &str) -> anyhow::Result<Vec<String>> {
    Ok(new_commit_messages(path, base)?
        .into_iter()
        .map(|msg| extract_issue_ids(&msg))
        .flatten()
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        r"todo: Title

Long descriptive text.

Issue: ICONSD-1234",
        Vec::from(["ICONSD-1234".to_string()])
    )]
    #[case(
        r"Issue: ICONSD-1234",
        Vec::from(["ICONSD-1234".to_string()])
    )]
    #[case(
        r"Issue: ICONSD-1234, ICONSD-4321",
        Vec::from(["ICONSD-1234".to_string(), "ICONSD-4321".to_string()])
    )]
    fn test_parse_id_from_comment(
        #[case] given_commit_msg: &str,
        #[case] expected_id: Vec<String>,
    ) {
        assert_eq!(extract_issue_ids(given_commit_msg), expected_id);
    }
}
