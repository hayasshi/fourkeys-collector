static GITHUB_URL: &'static str = "https://api.github.com";

use anyhow::Result;
use serde::Deserialize;

pub struct Client {
    username: String,
    token: String,
}

impl Client {
    pub fn new(username: String, token: String) -> Self {
        Self { username, token }
    }

    pub fn get_pull_requests(&self, repo: &str, base: &str) -> Result<Vec<PullRequest>> {
        let url = format!(
            "{base_url}/repos/{orgrepo}/pulls",
            base_url = GITHUB_URL,
            orgrepo = repo
        );
        println!("{}", url);
        let client = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", format!("token {}", &self.token))
            .header("accept", "application/vnd.github.v3+json")
            .header("User-Agent", &self.username)
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
        let client = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", format!("token {}", &self.token))
            .header("accept", "application/vnd.github.v3+json")
            .header("User-Agent", &self.username);

        let result = client.send()?.json::<Vec<Release>>()?;
        Ok(result)
    }
}

#[derive(Deserialize, Debug)]
pub struct PullRequest {
    pub title: String,
    pub state: String,
    pub merged_at: Option<String>, // Some: merged, None: closed
    pub commits_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Release {
    pub name: String,
    pub tag_name: String,
    pub body: String,
    pub created_at: Option<String>,
    pub published_at: Option<String>,
}
