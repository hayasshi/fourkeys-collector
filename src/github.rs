static GITHUB_URL: &str = "https://api.github.com";

use anyhow::Result;
use reqwest::blocking::RequestBuilder;
use serde::Deserialize;

pub struct Client {
    username: String,
    token: String,
}

impl Client {
    pub fn new(username: String, token: String) -> Self {
        Self { username, token }
    }

    fn with_headers(&self, builder: RequestBuilder) -> RequestBuilder {
        builder
            .header("Authorization", format!("token {}", &self.token))
            .header("accept", "application/vnd.github.v3+json")
            .header("User-Agent", &self.username)
    }

    pub fn get_merged_pull_requests(&self, repo: &str, base: &str) -> Result<Vec<PullRequest>> {
        let url = format!(
            "{base_url}/repos/{orgrepo}/pulls",
            base_url = GITHUB_URL,
            orgrepo = repo
        );
        println!("{}", url);
        let builder = reqwest::blocking::Client::new().get(url);
        let client = self
            .with_headers(builder)
            .query(&[("base", base), ("state", "closed")]);

        let result = client.send()?.json::<Vec<PullRequest>>()?;
        Ok(result)
    }

    pub fn get_releases(&self, repo: &str) -> Result<Vec<Release>> {
        let url = format!(
            "{base_url}/repos/{orgrepo}/releases",
            base_url = GITHUB_URL,
            orgrepo = repo
        );
        println!("{}", url);
        let builder = reqwest::blocking::Client::new().get(url);
        let client = self.with_headers(builder);

        let result = client.send()?.json::<Vec<Release>>()?;
        Ok(result)
    }

    pub fn get_commits_by_url(&self, url: &str) -> Result<Vec<Commit>> {
        println!("{}", url);
        let client = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", format!("token {}", &self.token))
            .header("accept", "application/vnd.github.v3+json")
            .header("User-Agent", &self.username);

        let result = client.send()?.json::<Vec<Commit>>()?;
        Ok(result)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct PullRequest {
    pub title: String,
    pub state: String,
    pub merged_at: Option<String>, // Some: merged, None: closed
    pub commits_url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Release {
    pub name: String,
    pub tag_name: String,
    pub body: String,
    pub created_at: Option<String>,
    pub published_at: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Commit {
    pub url: String,
    pub commit: CommitContent,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CommitContent {
    pub url: String,
    pub message: String,
    pub author: Committer,    // コードを書いた人
    pub committer: Committer, // コミットした人（rebase 等でコードを書いた人と分離することがある）
}

#[derive(Deserialize, Debug, Clone)]
pub struct Committer {
    pub name: String,
    pub email: String,
    pub date: String,
}
