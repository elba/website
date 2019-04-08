use std::path::Path;

use failure::Error;
use git2::{self, build::CheckoutBuilder, Cred, CredentialType, PushOptions, Repository};
use tempdir::TempDir;

use crate::CONFIG;

pub struct IndexRepo {
    repo: Repository,
    pub index_dir: TempDir,
}

impl IndexRepo {
    pub fn init() -> Result<Self, Error> {
        // git clone
        let index_dir = TempDir::new("elba_index")?;
        info!("cloning index repo to {:?}", index_dir.path());
        let repo = Repository::clone(&CONFIG.remote_index_url, index_dir.path())?;

        // git config
        let mut repo_cfg = repo.config().unwrap();
        repo_cfg.set_str("user.name", "elba-bot").unwrap();
        repo_cfg
            .set_str("user.email", "elba-bot@none.exist")
            .unwrap();

        let index_repo = IndexRepo { repo, index_dir };

        Ok(index_repo)
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

    pub fn commit_and_push(&self, msg: &str, file: &Path) -> Result<(), Error> {
        // git add
        let mut index = self.repo.index()?;
        index.add_path(&file.strip_prefix(self.index_dir.path())?)?;
        index.write()?;

        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;

        let head = self.repo.head()?;
        let parent = self.repo.find_commit(head.target().unwrap())?;
        let sig = self.repo.signature()?;

        // git commit -m
        self.repo
            .commit(Some("HEAD"), &sig, &sig, msg, &tree, &[&parent])?;

        // git push
        let mut remote = self.repo.find_remote("origin")?;

        let mut push_err_msg = None;
        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(credentials);
        callbacks.push_update_reference(|refname, status| {
            assert_eq!(refname, "refs/heads/master");
            push_err_msg = status.map(|s| s.to_string());
            Ok(())
        });

        remote.push(
            &["refs/heads/master"],
            Some(PushOptions::new().remote_callbacks(callbacks)),
        )?;

        if let Some(push_err_msg) = push_err_msg {
            return Err(format_err!("failed to push ref `{}`", &push_err_msg));
        }

        Ok(())
    }
}

fn credentials(
    _user: &str,
    _user_from_url: Option<&str>,
    _cred: CredentialType,
) -> Result<Cred, git2::Error> {
    Cred::userpass_plaintext(
        CONFIG
            .remote_index_user
            .as_ref()
            .map(|str| str.as_str())
            .unwrap_or(""),
        CONFIG
            .remote_index_pwd
            .as_ref()
            .map(|str| str.as_str())
            .unwrap_or(""),
    )
}
