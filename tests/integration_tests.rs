use assert_cmd::prelude::*;
use git2::{Commit, ObjectType, Repository};
use rmob::*;
use std::process::Command;
use tempfile::{tempdir, TempDir};

#[test]
fn it_works() -> BoxResult {
    let dir = tempdir()?;
    let repo = Repository::init(dir.path())?;

    init(&dir)?;

    // TODO: stub ~/.git-copirates
    sail(&dir)?;

    init_git_repo(&dir)?;

    assert_oneliner_commit_message_contains_coauthors(&dir, &repo)?;

    assert_multiline_commit_message_contains_coauthors(&dir, &repo)?;


    Ok(())
}

fn init(dir: &TempDir) -> BoxResult {
    let mut rmob = Command::cargo_bin("rmob")?;
    rmob.current_dir(dir.path()).arg("init").assert().success();
    let hook = dir.path().join(".git/hooks/").join(HOOK_NAME);
    assert!(hook.exists());

    Ok(())
}

fn sail(dir: &TempDir) -> BoxResult {
    let mut rmob = Command::cargo_bin("rmob")?;
    rmob.current_dir(dir.path()).arg("sail").arg("ek").assert().success();

    Ok(())
}

fn init_git_repo(dir: &TempDir) -> BoxResult {
    Command::new("git").current_dir(dir.path()).arg("init").assert().success();

    Ok(())
}

fn assert_oneliner_commit_message_contains_coauthors(dir: &TempDir, repo: &Repository) -> BoxResult {
    Command::new("git").current_dir(dir.path()).arg("commit").arg("-m").arg("Arrrrrr!").arg("--allow-empty").assert().success();

    let commit = find_last_commit(&repo)?;
    assert!(
        commit
            .message()
            .ok_or("no commit message")?
            .contains("Co-authored-by"),
        "Did not include the Co-Author for a commit message without comments (hashes)"
    );

    Ok(())
}

fn assert_multiline_commit_message_contains_coauthors(dir: &TempDir, repo: &Repository) -> BoxResult {
    const MULTILINE_MESSAGE: &str = r#"
# Please enter the commit message for your changes. Lines starting
# with '#' will be ignored, and an empty message aborts the commit.
#
# On branch integration-test
# Changes not staged for commit:
#	modified:   tests/integration_tests.rs
#
    "#;
    Command::new("git").current_dir(dir.path()).arg("commit").arg("-m").arg(MULTILINE_MESSAGE).arg("--allow-empty").assert().success();

    let commit = find_last_commit(&repo)?;
    assert!(
        commit
            .message()
            .ok_or("no commit message")?
            .contains("Co-authored-by"),
        "Did not include the Co-Author for a commit message without comments (hashes)"
    );

    Ok(())
}

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find commit"))
}
