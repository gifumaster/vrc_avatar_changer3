# Avatar Changer

VRChat のアバター一覧を取得し、検索・タグ管理・OSC または API による切り替えを行う Windows デスクトップアプリです。

## ダウンロード

最新版は GitHub Releases から取得してください。

- Release ページ: [https://github.com/gifumaster/vrc_avatar_changer3/releases](https://github.com/gifumaster/vrc_avatar_changer3/releases)

配布物は通常次のどちらかです。

- `Avatar Changer_x.y.z_x64-setup.exe`
- `Avatar Changer_x.y.z_x64_en-US.msi`

通常は `setup.exe` を使えば問題ありません。

## できること

- VRChat ログイン
- 2FA 対応
- アバター一覧取得
- サムネイル付き表示
- 検索
- タグ追加とタグ絞り込み
- ランダムでアバター詳細を開く
- OSC または API によるアバター切り替え
- 切り替え方法の選択と OSC Host / Port 設定

## 使い方

1. アプリを起動します。
2. 必要なら右上のギアアイコンから設定パネルを開きます。
3. VRChat にログインします。
4. `Fetch Latest 20` または `Fetch All Avatars` でアバター一覧を取得します。
5. 検索欄やタグで絞り込みます。
6. カードをクリックして詳細を開き、`Switch` で設定した方式に応じて切り替えを実行します。

## 補足

- ID/PASSは保存しません。
- API認証情報は Windows Credential Manager に保存されるため漏洩することはありません。
- アバター一覧とサムネイルはローカルにキャッシュされます。
- アバター一覧の取得は UI から明示的に実行したときだけ行われます。
- サムネイル URL は期限切れになるため、取得済み画像をローカルに保存されます。サムネを更新した場合は再読み込みが必要です。

## アバター切り替え設定

VRChat 側で OSC を利用する場合、通常は次の既定値で動作します。

- Host: `127.0.0.1`
- Port: `9000`

OSC を使う場合は設定パネルで Host / Port を調整してください。
API を使う場合は、このアプリでログインして保存済みセッションがある状態で直接切り替えできます。

## 開発

### 技術スタック

- Tauri v2
- Rust
- Vue 3
- TypeScript

### ローカル開発

必要なもの:

- Node.js
- Rust
- Visual Studio Build Tools 2022

依存インストール:

```powershell
npm ci
```

開発起動:

```powershell
npm run tauri dev
```

本番ビルド:

```powershell
npm run tauri build
```

### ビルド成果物

Windows 向け成果物は次に生成されます。

- `src-tauri/target/release/bundle/nsis/`
- `src-tauri/target/release/bundle/msi/`

### Release Workflow

GitHub Actions で Windows ビルドと GitHub Release を作成します。

- Workflow: `.github/workflows/release.yml`
- Trigger: `main` への push または手動実行
