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