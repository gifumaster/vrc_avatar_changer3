# Repository Instructions

- ファイルは明示的に UTF-8 で開くこと。
- `main` にプッシュして更新するときは、必ずバージョンを上げること。別途指示がない限り、一番下のバージョンを 1 つ上げること。
- バージョン更新時は `package.json`、`src-tauri/Cargo.toml`、`src-tauri/tauri.conf.json` を揃えること。
- 変更前とコミット前に `git status --short` を確認すること。
- ユーザーが明示しない限り、自分が触っていない未コミット変更はコミットに含めないこと。
- 実装後は最低でも `npm run build` と `cargo check` で確認すること。
- リリース時のコミットメッセージは `Release x.y.z` 形式を使うこと。
- Tauri のローカルファイル表示に関わる変更では `assetProtocol` と `scope` の設定を確認すること。
