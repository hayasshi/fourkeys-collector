static GITHUB_URL: &'static str = "https://api.github.com";

use serde::Deserialize;

pub struct Client {
    username: String,
    token: String,
}

impl Client {

    pub fn new(username: String, token: String) -> Self {
        Self {
            username,
            token
        }
    }

    pub fn get_pull_requests(&self, repo: &str, base: &str) -> Vec<PullRequest> {
        let url = format!("{base_url}/repos/{orgrepo}/pulls", base_url = GITHUB_URL, orgrepo = repo);
        println!("{}", url);
        let client = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", format!("token {}", &self.token))
            .header("accept", "application/vnd.github.v3+json")
            .header("User-Agent", &self.username)
            .query(&[("base", base), ("state", "closed")]);
        
        let result = client.send().expect("Failed send request.");
        result.json::<Vec<PullRequest>>().expect("Failed deserialize response.")
    }
}

#[derive(Deserialize, Debug)]
pub struct PullRequest {
    pub title: String,
    pub state: String,
    pub merged_at: Option<String>, // Some: merged, None: closed
    pub commits_url: String,
}
