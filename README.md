# fourkeys-collector

[Four Keys プロジェクト](https://cloud.google.com/blog/ja/products/gcp/using-the-four-keys-to-measure-your-devops-performance) で定義される指標を収集します。

## Features

- TODO: デプロイの頻度
- WIP: 変更のリードタイム
    - GitHub 上において、特定のブランチに向けられた Pull Requests のマージ時間と、それに含まれるコミットのコミット時間の差分を出力する
    - Output: csv file
    - Line format: `commit_author, commit_date, merged_at, duration_seconds_until_merged`
    - TODO: 期間を指定して出力できるようにする

特定のブランチに、PullRequest でマージされたら、リリースされたものとみなせるリリースフローを想定しています。

## Usage

```
USAGE:
    c4k [OPTIONS] --repo <repo> --username <username> --github-token <github-token>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --base <base>                    ベースブランチ [default: master]
        --from <from>                    対象期間 (JST) from `yyyyMM`
        --github-token <github-token>
            GitHub personal access token. scope=repo [env: GITHUB_TOKEN]

    -r, --repo <repo>                    対象リポジトリ名 `org/repo`
        --to <to>                        対象期間 (JST) to `yyyyMM`
    -u, --username <username>            GitHub アカウント名
```

e.g.

```
GITHUB_TOKEN=${your_token} c4k -u hayasshi -r hayasshi/fourkeys-collector -b main --from 202101 --to 202112
```
