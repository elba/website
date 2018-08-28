use std::fs;
use std::path::Path;

use failure::Error;
use git2::{self, build::CheckoutBuilder, Cred, CredentialType, PushOptions, Repository};

use crate::CONFIG;

pub struct IndexRepo {
    repo: Repository,
}

impl IndexRepo {
    pub fn init() -> Result<Self, Error> {
        // git clone
        let repo = match Repository::open(&CONFIG.index_path) {
            Ok(repo) => repo,
            Err(_) => {
                fs::remove_dir_all(&CONFIG.index_path)?;
                Repository::clone(&CONFIG.remote_index_url, &CONFIG.index_path)?
            }
        };

        // git config
        let mut repo_cfg = repo.config().unwrap();
        repo_cfg.set_str("user.name", "elba-bot").unwrap();
        repo_cfg
            .set_str("user.email", "elba-bot@none.exist")
            .unwrap();

        let index_repo = IndexRepo { repo };

        index_repo.fetch_and_reset()?;

        Ok(index_repo)
    }

    pub fn commit_and_push(&self, msg: &str, file: &Path) -> Result<(), Error> {
        // git add
        let mut index = self.repo.index()?;
        index.add_path(&file.strip_prefix(&CONFIG.index_path)?)?;
        index.write()?;

        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;

        let head = self.repo.head()?;
        let parent = self.repo.find_commit(head.target().unwrap())?;
        let sig = self.repo.signature()?;

        // git commit -m
        self.repo
            .commit(Some("HEAD"), &sig, &sig, msg, &tree, &[&parent])?;

        let mut remote = self.repo.find_remote("origin")?;

        let mut push_err_msg = None;
        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(credentials);
        callbacks.push_update_reference(|refname, status| {
            assert_eq!(refname, "refs/heads/master");
            push_err_msg = status.map(|s| s.to_string());
            Ok(())
        });

        // git push
        remote.push(
            &["refs/heads/master"],
            Some(PushOptions::new().remote_callbacks(callbacks)),
        )?;

        if let Some(push_err_msg) = push_err_msg {
            return Err(format_err!("Failed to push ref `{}`.", &push_err_msg));
        }

        self.fetch_and_reset()?;

        Ok(())
    }

    pub fn fetch_and_reset(&self) -> Result<(), Error> {
        // git pull origin
        let mut remote = self.repo.find_remote("origin")?;
        remote.fetch(&["refs/heads/master:refs/heads/master"], None, None)?;

        // git checkout HEAD -f
        self.repo.set_head("refs/heads/master")?;
        self.repo
            .checkout_head(Some(CheckoutBuilder::new().force()))?;

        Ok(())
    }
}

fn credentials(
    _user: &str,
    _user_from_url: Option<&str>,
    _cred: CredentialType,
) -> Result<Cred, git2::Error> {
    Cred::userpass_plaintext(&CONFIG.remote_index_user, &CONFIG.remote_index_pwd)
}
