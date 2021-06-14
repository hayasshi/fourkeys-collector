# fourkeys-collector

[Four Keys プロジェクト](https://cloud.google.com/blog/ja/products/gcp/using-the-four-keys-to-measure-your-devops-performance) で定義される指標を収集します。

## features

- TODO: デプロイの頻度
- WIP: 変更のリードタイム
    - GitHub 上において、特定のブランチに向けられた Pull Requests のマージ時間と、それに含まれるコミットのコミット時間の差分を出力する
    - Output: csv file
    - Line format: `commit_author, commit_date, merged_at, duration_seconds_until_merged`
    - TODO: 期間を指定して出力できるようにする
