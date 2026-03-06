```text
# Avatar Changer

Windows desktop app for browsing and switching your VRChat avatars.

This project is a rebuilt version of `vrc_avatar_changer2`, with safer credential handling, local avatar caching, and OSC-based avatar switching.

## Current Features

- VRChat login
- 2FA support
- Session storage with Windows Credential Manager
- Avatar list fetch and local cache
- Thumbnail cache
- Search and tag filtering
- Random avatar detail open
- OSC avatar switching
- Configurable OSC host and port

## Tech Stack

- Tauri v2
- Rust
- Vue 3
- TypeScript

## Local Development

Requirements:

- Node.js
- Rust
- Visual Studio Build Tools 2022

Install dependencies:

```powershell
npm ci
```

Run in development mode:

```powershell
npm run tauri dev
```

Build production bundles:

```powershell
npm run tauri build
```

## Build Outputs

Windows bundles are generated under:

- `src-tauri/target/release/bundle/nsis/`
- `src-tauri/target/release/bundle/msi/`

## Release Workflow

This repository includes GitHub Actions workflow for Windows builds and GitHub Releases.

- Trigger: push to `main`
- Workflow file: `.github/workflows/release.yml`

## Notes

- Avatar data is cached locally.
- Thumbnail images are cached locally to avoid expired VRChat image URLs.
- Avatar list fetch is only performed when explicitly requested from the UI.
```