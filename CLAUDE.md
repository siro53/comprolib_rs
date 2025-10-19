# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## このリポジトリについて

これは競技プログラミング用のRustライブラリコレクションです。ワークスペース構造を採用し、各アルゴリズムやデータ構造を独立したクレートとして管理しています。

## 主要なコマンド

### ビルドとテスト
```bash
# ワークスペース全体のビルド
cargo build --workspace

# 特定のクレートのビルド（例：Union-Find）
cargo build -p dsu

# 特定の検証コードの実行
cargo run -p yuki789_1

# 検証の実行（online-judge-verify-helperを使用）
oj-verify all

# 特定の問題の検証
oj-verify run verify/yukicoder/yuki789/yuki789_1/src/main.rs
```

### 開発用コマンド
```bash
# 新しいクレートの作成（例：新しいデータ構造）
cargo new crates/ds/new_structure

# ワークスペースのCargo.tomlにメンバー追加が必要
# members配列に "crates/ds/new_structure" を追加

# 依存関係の確認
cargo tree

# フォーマット
cargo fmt

# Clippy（静的解析）
cargo clippy --workspace
```

### AtCoder提出用のコードbundle

#### bundle.sh スクリプトを使用（推奨）
ルートディレクトリに`bundle.sh`スクリプトを提供しています。カレントディレクトリで実行するだけで簡単にbundleできます：

```bash
# カレントディレクトリでbundle（最もシンプル）
cd verify/library_checker/data_structure/unionfind
../../../../bundle.sh

# クリップボードにコピー（WSL/Linux/macOS対応）
# clip.exe, pbcopy, xclip, xselを自動検出
../../../../bundle.sh -c

# バイナリ名を明示的に指定
../../../../bundle.sh -b unionfind

# 出力ファイル名を指定
../../../../bundle.sh -o bundled.rs

# proconioも展開する場合（通常は不要）
../../../../bundle.sh --no-exclude

# 複数のクレートを除外
../../../../bundle.sh -e itertools -e superslice

# ヘルプを表示
../../../../bundle.sh -h
```

**スクリプトの特徴:**
- カレントディレクトリ名から自動的にバイナリ名を推測
- クリップボードツールを自動検出（`clip.exe`, `pbcopy`, `xclip`, `xsel`）
- デフォルトで`proconio`を除外（AtCoderでは標準提供のため）
- ファイルサイズと行数を表示
- `submit.rs`に出力（デフォルト）

#### cargo-equipを直接使用
より細かい制御が必要な場合は、`cargo-equip`を直接使用できます：

```bash
# 基本的な使い方
cd verify/yukicoder/yuki789/yuki789_1
cargo equip --bin yuki789_1 --exclude proconio --remove docs --minify libs

# 出力されたコードをクリップボードにコピー
cargo equip --bin yuki789_1 --exclude proconio --remove docs --minify libs | clip.exe      # WSL
cargo equip --bin yuki789_1 --exclude proconio --remove docs --minify libs | pbcopy        # macOS
cargo equip --bin yuki789_1 --exclude proconio --remove docs --minify libs | xclip -selection clipboard  # Linux

# ファイルに保存
cargo equip --bin yuki789_1 --exclude proconio --remove docs --minify libs > submit.rs
```

**cargo-equipオプションの説明:**
- `--exclude proconio`: AtCoderで標準提供されているクレートを除外
- `--remove docs`: ドキュメントコメントを削除してコードサイズを削減
- `--minify libs`: bundleされたライブラリ部分をminify（1行化）

**注意事項:**
- `proconio`のような外部クレート（crates.ioからのクレート）でproc-macroを使用しているものは展開が困難です
- 基本的には**自作のpath依存クレートのみを展開**し、**外部クレートは除外**するのが標準的な使い方です
- AtCoderでは`proconio`は標準提供されているため、除外することを推奨します

## コードベースの構造

### アーキテクチャの概要
- **ワークスペース設計**: 各アルゴリズム・データ構造が独立したクレート
- **検証駆動開発**: すべての実装はオンラインジャッジでの検証付き
- **トレイト中心設計**: 共通の数値トレイト（Zero, One, Bound等）とMonoidトレイトを基盤とする

### ディレクトリ構成
```
crates/
├── ds/              # データ構造
│   ├── dsu/         # Union-Find（Disjoint Set Union）
│   ├── fenwick_tree/     # Fenwick Tree（Binary Indexed Tree）
│   └── segment_tree/     # セグメント木
│       ├── segment_tree/        # 通常のセグメント木
│       └── dynamic_segment_tree/ # 動的セグメント木
├── traits/          # 共通トレイト
│   ├── monoid/      # Monoidトレイト
│   └── numeric/     # 数値系トレイト（Zero, One, Bound, Infinity）
├── util/           # ユーティリティ
│   └── monoid_util/ # 具体的なMonoid実装（Add, Min, Max, Affine等）
├── modint/         # 剰余体
└── misc/           # その他のアルゴリズム
    ├── compress/    # 座標圧縮
    ├── cumulative_sum_2d/ # 2次元累積和
    └── rle/         # ランレングス圧縮

verify/             # 検証用コード
├── library_checker/ # Library Checkerでの検証
├── aoj/            # AOJ（Aizu Online Judge）での検証
└── yukicoder/      # yukicoderでの検証
```

### 重要な設計パターン

1. **Monoidパターン**: セグメント木等では`Monoid`トレイトを使用
2. **数値トレイト**: `Zero`, `One`, `Bound`, `Infinity`を活用した汎用的な数値処理
3. **検証ファーストアプローチ**: 実装→検証コード→オンラインジャッジでの確認

### クレート間の依存関係
- `traits/monoid` と `traits/numeric` が基盤トレイト
- `util/monoid_util` で具体的なMonoid実装を提供
- データ構造クレートは必要に応じて上記トレイトに依存
- 検証用クレートは対応する実装クレートに依存

## 新機能追加時の流れ

1. `crates/`下の適切なカテゴリに新しいクレートを作成
2. ワークスペースの`Cargo.toml`にメンバーとして追加
3. `verify/`下に対応する検証コードを作成（問題URLをコメントで記載）
4. 検証コードでは`// verification-helper: PROBLEM <URL>`でオンラインジャッジの問題を指定
5. GitHub Actionsで自動検証を実行

## AtCoder用コードの作成とbundle

### 問題を解くコードの作成
AtCoderの問題を解く際は、検証用クレートと同様の構造を使用します：

```bash
# 例：AtCoder ABC123のD問題
mkdir -p atcoder/abc123/d
cd atcoder/abc123/d
cargo init --name abc123_d

# Cargo.tomlで必要なクレートを依存に追加
# [dependencies]
# segment_tree = { path = "../../../crates/ds/segment_tree/segment_tree" }
# monoid = { path = "../../../crates/traits/monoid" }
# proconio = "0.5.0"
```

### bundleして提出
`bundle.sh`スクリプトを使用（推奨）：

```bash
cd atcoder/abc123/d
../../../bundle.sh -c
# クリップボードにコピーされるので、AtCoderに貼り付けて提出
```

または`cargo-equip`を直接使用：

```bash
cd atcoder/abc123/d
cargo equip --bin abc123_d --exclude proconio --remove docs --minify libs > submit.rs
# submit.rs の内容をAtCoderに提出
```

## 検証について

### 検証システムの仕組み
- `online-judge-verify-helper`を使用してオンラインジャッジでの自動検証を実装
- `oj-verify all`でオンラインジャッジでの検証を実行
- 各検証ファイルには`// verification-helper: PROBLEM <URL>`で問題URLを記載
- GitHub Actionsで自動的に検証が実行される
- 検証に必要な環境変数：`GITHUB_TOKEN`, `YUKICODER_TOKEN`, `GH_PAT`

### 検証用クレートの構造
検証用のクレートは実装クレートに依存し、問題を解くための実行ファイルとして作成される：

```toml
[dependencies]
segment_tree = { path = "../../../crates/ds/segment_tree/segment_tree" }
monoid_util = { path = "../../../crates/util/monoid_util" }
proconio = "0.4"
```

### 使用可能なオンラインジャッジ
- **Library Checker**: 基本的なデータ構造・アルゴリズムの検証
- **AOJ (Aizu Online Judge)**: より複雑な問題での検証
- **yukicoder**: 実際のコンテスト問題での検証