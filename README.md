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

### Rust project のまま実行する場合

```
GITHUB_TOKEN=${your_token} cargo run -- -u hayasshi -r hayasshi/fourkeys-collector -b main --from 202101 --to 202112
```

## Output

標準出力(Stdout)に CSV 形式で出力される。

### 変更のリードタイム

```csv
commit_author,commit_date,merged_at,duration_seconds_until_merged
hayasshi,2021-06-15 01:50:45,2021-06-15 02:01:02,617
hayasshi,2021-06-15 01:54:32,2021-06-15 02:01:02,390
hayasshi,2021-06-15 01:57:11,2021-06-15 02:01:02,231
hayasshi,2021-06-15 01:59:41,2021-06-15 02:01:02,81
hayasshi,2021-06-14 23:17:46,2021-06-15 01:43:48,8762
hayasshi,2021-06-14 23:30:37,2021-06-15 01:43:48,7991
hayasshi,2021-06-14 23:44:41,2021-06-15 01:43:48,7147
hayasshi,2021-06-14 23:54:26,2021-06-15 01:43:48,6562
hayasshi,2021-06-15 01:42:37,2021-06-15 01:43:48,71
hayasshi,2021-05-30 01:16:36,2021-06-12 17:51:34,1182898
hayasshi,2021-06-12 17:44:08,2021-06-12 17:51:34,446
hayasshi,2021-06-12 17:49:09,2021-06-12 17:51:34,145
hayasshi,2021-05-29 23:53:59,2021-05-29 23:57:05,186
hayasshi,2021-05-29 23:56:06,2021-05-29 23:57:05,59
hayasshi,2021-05-29 23:56:46,2021-05-29 23:57:05,19
hayasshi,2021-05-29 23:36:33,2021-05-29 23:47:02,629
hayasshi,2021-05-25 00:13:59,2021-05-25 21:35:43,76904
hayasshi,2021-05-24 22:31:11,2021-05-24 22:33:18,127
```
