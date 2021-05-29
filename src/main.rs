mod github;

use anyhow::Result;
use clap::{ArgSettings, Clap};
fn main() -> Result<()> {
    let opts = Opts::parse();
    println!("options: {:#?}", opts);

    let github_client = github::Client::new(opts.username, opts.github_token);
    let results = github_client.get_pull_requests(opts.repo.as_str(), opts.base.as_str())?;
    println!("results: {:#?}", results);
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
