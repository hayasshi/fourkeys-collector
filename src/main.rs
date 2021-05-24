use clap::{ArgSettings, Clap};

fn main() {
    let opts = Opts::parse();
    println!("{:#?}", opts);
}

#[derive(Clap, Debug)]
#[clap(name = "fourkeys-collector", version)]
struct Opts {
    /// 対象リポジトリ名 `org/repo`
    repo: String,

    /// GitHub personal access token. scope=repo:status
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
