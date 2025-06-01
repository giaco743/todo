use std::path::{Path, PathBuf};

use git2::Repository;
use structre::structre;

#[structre(r"(?m)^Issue:[ \t]*([A-Z]+-[0-9]+)[ \t]*$")]
#[derive(Debug, PartialEq, Eq)]
pub struct TicketId(pub String);

fn new_commit_messages(path: PathBuf) -> anyhow::Result<Vec<String>> {
    let repo = Repository::open(path)?;

    // Referenzen auf HEAD und origin/master
    let head = repo.revparse_single("HEAD")?.peel_to_commit()?;
    let base = repo.revparse_single("master")?.peel_to_commit()?;

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

pub fn resolving_issues(path: PathBuf) -> anyhow::Result<Vec<TicketId>> {
    new_commit_messages(path)?
        .into_iter()
        .map(|msg| TicketId::try_from(msg.as_str()).map_err(anyhow::Error::msg))
        .collect()
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
        TicketId("ICONSD-1234".to_string())
    )]
    #[case(
        r"Issue: ICONSD-1234",
        TicketId("ICONSD-1234".to_string())
    )]
    fn test_parse_id_from_comment(#[case] given_commit_msg: &str, #[case] expected_id: TicketId) {
        assert_eq!(TicketId::try_from(given_commit_msg).unwrap(), expected_id);
    }
}
