mod github;

use anyhow::Result;
use chrono::prelude::*;
use clap::{ArgSettings, Clap};

fn main() -> Result<()> {
    let opts = Opts::parse();
    println!("options: {:#?}", opts);

    let github_client = github::Client::new(opts.username, opts.github_token);

    let mut metrics = Vec::new();
    let tz_jst = FixedOffset::east(9 * 3600);

    let prs = github_client.get_merged_pull_requests(opts.repo.as_str(), opts.base.as_str())?;
    for pr in prs.iter().filter(|pr| pr.merged_at.is_some()) {
        let merged_at = DateTime::parse_from_rfc3339(pr.merged_at.as_ref().unwrap().as_str())?
            .with_timezone(&tz_jst);

        let commits = github_client.get_commits_by_url(pr.commits_url.as_str())?;
        for commit in commits {
            let commit_date = DateTime::parse_from_rfc3339(commit.commit.author.date.as_str())?
                .with_timezone(&tz_jst);
            let duration_seconds_until_merged = merged_at.timestamp() - commit_date.timestamp();

            metrics.push(GitHubMetrics {
                commit_author: commit.commit.author.name,
                commit_date: commit_date.format("%Y-%m-%d %H:%M:%S").to_string(),
                merged_at: merged_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                duration_seconds_until_merged,
            });
        }
    }

    let mut writer = csv::Writer::from_writer(std::io::stdout());
    for m in metrics {
        writer.serialize(m)?;
    }
    writer.flush()?;

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

#[derive(serde::Serialize, Debug)]
struct GitHubMetrics {
    pub commit_author: String,
    pub commit_date: String,
    pub merged_at: String,
    pub duration_seconds_until_merged: i64,
}
