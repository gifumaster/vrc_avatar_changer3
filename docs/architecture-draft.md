# 新アプリ設計たたき台

## 推奨スタック

- デスクトップ基盤: `Tauri v2`
- バックエンド: `Rust`
- フロントエンド: `Vue 3 + TypeScript`
- UI: 現時点では任意

Tauri を継続する理由:

- 既存の知識と資産を流用しやすい
- Windows 配布がしやすい
- 秘密情報管理や OSC を Rust 側へ寄せやすい

## 設計方針

- API 通信や秘密情報保存は Rust 側へ寄せる
- フロントは表示とユーザー操作に集中させる
- フロントから直接 VRChat API を叩かない
- キャッシュと秘密情報は別レイヤーで管理する

## モジュール案

### Frontend

- `pages` or `views`
- `components`
- `features/auth`
- `features/avatar-list`
- `features/search`
- `features/settings`
- `stores`
- `types`

### Rust

- `auth`
- `vrchat`
- `credential_store`
- `avatar_cache`
- `osc`
- `commands`
- `error`

## 画面案

### 1. ログイン画面

- ユーザー名
- パスワード
- 2FA 入力
- ログイン状態表示

### 2. メイン画面

- 検索入力
- タグショートカット
- 一覧更新ボタン
- キャッシュ更新日時
- サムネイルグリッド
- 詳細パネルまたはモーダル

### 3. 設定画面

- キャッシュ削除
- 保存済み認証情報の削除
- OSC 送信先設定

## データ保存方針

### 秘密情報

- OS の秘密情報ストアを使う
- 候補: Windows Credential Manager

### キャッシュ

- アバター一覧はアプリ管理領域へ保存
- JSON もしくは SQLite は今後選定
- 初期は JSON でもよいが、件数増や検索性を考えると SQLite も有力

### ユーザー設定

- タグ
- ウィンドウ設定
- 検索設定
- OSC 設定

## API 設計方針

- フロントからは Tauri command を呼ぶ
- command は UI 用 DTO を返す
- VRChat API の生レスポンスを直接 UI に渡さない

## 主要コマンド案

- `login`
- `verify_session`
- `submit_two_factor`
- `load_avatar_cache`
- `refresh_avatar_list`
- `get_avatar_detail`
- `switch_avatar_via_osc`
- `clear_credentials`
- `clear_cache`
- `load_settings`
- `save_settings`

## エラー方針

- 認証エラー
- 2FA 要求
- ネットワークエラー
- API レート制限
- キャッシュ破損
- OSC 送信失敗

これらを Rust 側でアプリ用エラー型へ正規化し、UI 側では表示専用に扱う。

## 実装順

1. プロジェクト初期化
2. Rust 側の認証・秘密情報保存
3. アバター一覧取得とキャッシュ
4. Vue 側の一覧 UI
5. 絞り込み UI
6. 設定画面
7. OSC 切替

## 未確定事項

- UI ライブラリを継続利用するか
- キャッシュ保存を JSON にするか SQLite にするか
- OSC 切替方式の具体仕様
