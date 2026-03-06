# 旧アプリ調査メモ

対象リポジトリ: `vrc_avatar_changer2-main`

## 概要

- Windows 向けの個人利用 VRChat アバター管理ツール
- 技術スタックは `Vue 3 + Vuetify + Tauri 2 RC + Rust`
- 実処理の大半はフロントエンドにあり、Rust 側は Tauri プラグイン初期化のみ

## 現在の主要機能

- VRChat API を使ったログイン
- 2FA 判定とコード送信
- 自分のアバター一覧取得
- サムネイル付き一覧表示
- キーワード絞り込み
- ローカルタグ定義とタグ絞り込み
- VRChat API を使ったアバター変更
- 単体アバター情報の更新

## 実装の中心

- API 通信: `src/composable/useVRChatApi.js`
- 画面と状態管理: `src/components/AvatarList.vue`
- タグ設定: `src/components/TagEditor.vue`
- タグ入力 UI: `src/components/TagInput.vue`
- Tauri 初期化: `src-tauri/src/lib.rs`

## 問題点

### 1. 秘密情報の保存が unsafe

- `authCookie` が `localStorage` に平文保存されている
- 取得済みのアバター一覧も `localStorage` 保存
- タグ設定も `localStorage` 保存

影響:

- ローカル端末上での秘密情報保護が弱い
- WebView 上のストレージ依存が強く、将来の移行が面倒
- 認証とキャッシュの責務が UI に混在している

### 2. アバター変更が VRChat API 直叩き

- `/avatars/{id}/select` を直接叩いている
- 新版の要件である OSC ベース切替に一致しない

影響:

- 新版では切替方式を差し替える必要がある
- API 取得系と切替系を別モジュールに分ける必要がある

### 3. UI コンポーネントに責務が集中

- `AvatarList.vue` が以下をまとめて持っている
- 認証状態
- 初期化
- トークン検証
- 一覧取得
- 一覧キャッシュ
- フィルタリング
- ランダム選択
- 設定ダイアログ状態

影響:

- 保守しにくい
- テストしにくい
- 状態遷移の不具合を埋め込みやすい

### 4. 型安全性が弱い

- UI は TypeScript 構成だが API 層が `useVRChatApi.js`
- レスポンス型や DTO の明示がない

影響:

- リファクタ時に壊しやすい
- 画面側が API 仕様へ直接依存する

### 5. 文字化けした日本語が混在

- README と Vue ファイルに文字化けが見える
- エンコーディング混在の可能性が高い

影響:

- 表示品質が悪い
- 維持管理コストが上がる

## 旧アプリの処理フロー

1. 起動時に `localStorage.authCookie` を読む
2. トークンがあれば `/auth/user` で有効性を確認
3. `localStorage.avatarList` があれば表示
4. 必要に応じて `/avatars` を 100 件ずつ最大 20 回取得
5. 一覧をローカル保存
6. アバター変更は `/avatars/{id}/select` へ PUT

## 再利用できる点

- 一覧取得機能の要件
- 絞り込み UX の方向性
- タグショートカットの考え方
- サムネイル中心の UI という方針

## 捨てるべき点

- トークンの `localStorage` 保存
- UI と API の密結合
- アバター変更 API 直叩き
- 単一コンポーネント集中型の状態管理

## 新版で分離すべきモジュール

- `auth`: ログイン、2FA、トークン検証、秘密情報保存
- `vrchat-api`: アバター一覧取得、詳細取得
- `avatar-cache`: キャッシュ保存と失効制御
- `osc`: アバター切替送信
- `ui`: 画面表示とユーザー操作

