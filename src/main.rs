mod github;

use anyhow::Result;
use clap::{ArgSettings, Clap};
fn main() -> Result<()> {
    let opts = Opts::parse();
    println!("options: {:#?}", opts);

    let github_client = github::Client::new(opts.username, opts.github_token);

    let mut metrics = Vec::new();
    let prs = github_client.get_merged_pull_requests(opts.repo.as_str(), opts.base.as_str())?;
    for pr in prs.iter().filter(|pr| pr.merged_at.is_some()) {
        let commits = github_client.get_commits_by_url(pr.commits_url.as_str())?;
        for commit in commits {
            metrics.push(GitHubMetrics {
                pr_merged_at: pr.merged_at.as_ref().unwrap().to_string(),
                commit_author: commit.commit.author.name,
                commit_date: commit.commit.author.date,
            });
        }
    }
    for m in metrics {
        println!("{:?}", m);
    }

    Ok(())
}

#[derive(Clap, Debug)]
#[clap(name = "fourkeys-collector", version)]
struct Opts {
    /// 対象リポジトリ名 `org/repo`
    repo: String,

    /// GitHub アカウント名
    username: String,

    /// GitHub personal access token. scope=repo
    #[clap(long, env, setting = ArgSettings::HideEnvValues)]
    github_token: String,

    /// ベースブランチ
    #[clap(short, long, default_value = "master")]
    base: String,

    /// 対象期間 from (JST) `yyyyMM`
    #[clap(long)]
    from: Option<String>,

    /// 対象期間 to (JST) `yyyyMM`
    #[clap(long)]
    to: Option<String>,
}

#[derive(Debug)]
struct GitHubMetrics {
    pub pr_merged_at: String,
    pub commit_author: String,
    pub commit_date: String,
}
