# yew-tower-defense-game

ウトガルド城風・**立体古城防衛**タワーディフェンス（夜間・城壁よじ登り・城内室内戦の二重防衛）。

技術スタックの方針:

- **Yew + Yewdux** … 司令部 UI と UI 状態（sim 状態は載せない）
- **Rust / Wasm** … シミュレーション・AI・ルール
- **Three.js (JS)** … 描画（薄い Bridge で同期）
- **Trunk** … 開発・ビルド

Cursor の rules / skills / agents の正本は [`my-cursor-settings`](https://github.com/maixhashi/my-cursor-settings)（`common` / `personal-pc` / `stacks/rust` / `overlays/yew-tower-defense-game`）。本リポの `.cursor/` は `cursor-settings apply` の配布先で、Git 管理外。

ゲームプログラミングパターンは段階導入し、各 PR の **Design Patterns** 節で使ったものだけ明示する。

## セットアップ（推奨: Docker）

ホストに Rust / Trunk を入れずに開発できます。

```bash
docker compose up --build
```

`http://127.0.0.1:8080` で仮 UI（「古城防衛戦」）が表示されます。

ワンショットのビルド確認:

```bash
docker compose run --rm web trunk build
```

## セットアップ（ローカル toolchain）

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve
```

## Cursor 設定の適用

```bash
export PATH="/path/to/my-cursor-settings/bin:$PATH"
cursor-settings apply /path/to/yew-tower-defense-game \
  --layers common,personal-pc --stacks rust
```

## E2E スクリーンショット（Playwright）

アプリを先に起動してから実行します（`webServer` で Trunk を二重起動しません）。

```bash
docker compose up --build
# 別ターミナル
npm ci
npm run e2e:install
npm run e2e:screenshot
```

成果物は `e2e/artifacts/`（gitignore）。CI では artifact としてアップロードされます。
