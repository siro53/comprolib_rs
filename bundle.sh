#!/bin/bash

# AtCoder提出用にコードをbundleするスクリプト
# 使い方: ./bundle.sh [オプション]
#
# オプション:
#   -b, --bin NAME        バイナリ名を指定 (デフォルト: カレントディレクトリ名)
#   -o, --output FILE     出力ファイル名 (デフォルト: submit.rs)
#   -c, --clipboard       クリップボードにコピー
#   -e, --exclude CRATE   除外するクレート (デフォルト: proconio)
#   --exclude-atcoder     AtCoder提供の全クレートを除外
#   --no-exclude          proconioを除外しない
#   -h, --help            ヘルプを表示

set -e

# デフォルト値
BIN_NAME=$(basename "$PWD")
OUTPUT_FILE="submit.rs"
CLIPBOARD=false
EXCLUDE_CRATES="proconio"
EXCLUDE_FLAG="--exclude proconio"

# AtCoderで提供されているクレート一覧
ATCODER_CRATES="ac-library-rs once_cell static_assertions varisat memoise argio bitvec counter hashbag pathfinding recur-fn indexing amplify amplify_derive amplify_num easy-ext multimap btreemultimap bstr az glidesort tap omniswap multiversion num num-bigint num-complex num-integer num-iter num-rational num-traits num-derive ndarray nalgebra alga libm rand getrandom rand_chacha rand_core rand_hc rand_pcg rand_distr petgraph indexmap regex lazy_static ordered-float ascii permutohedron superslice itertools itertools-num maplit either im-rc fixedbitset bitset-fixed proconio text_io rustc-hash smallvec"

# ヘルプ表示
show_help() {
    cat << EOF
AtCoder提出用コードbundleスクリプト

使い方: $0 [オプション]

オプション:
  -b, --bin NAME        バイナリ名を指定 (デフォルト: $BIN_NAME)
  -o, --output FILE     出力ファイル名 (デフォルト: submit.rs)
  -c, --clipboard       クリップボードにコピー
  -e, --exclude CRATE   除外するクレートを追加
  --exclude-atcoder     AtCoder提供の全クレートを除外
  --no-exclude          proconioを除外しない
  -h, --help            このヘルプを表示

例:
  $0                                    # submit.rsに出力 (proconioのみ除外)
  $0 -b abc123_d                        # バイナリ名を指定
  $0 -c                                 # クリップボードにコピー
  $0 -o bundled.rs                      # 出力ファイル名を指定
  $0 --no-exclude                       # proconioも展開
  $0 -e itertools -e superslice         # 複数のクレートを除外
  $0 --exclude-atcoder                  # AtCoder提供の全クレートを除外

EOF
}

# 引数解析
while [[ $# -gt 0 ]]; do
    case $1 in
        -b|--bin)
            BIN_NAME="$2"
            shift 2
            ;;
        -o|--output)
            OUTPUT_FILE="$2"
            shift 2
            ;;
        -c|--clipboard)
            CLIPBOARD=true
            shift
            ;;
        -e|--exclude)
            EXCLUDE_CRATES="$EXCLUDE_CRATES $2"
            EXCLUDE_FLAG="$EXCLUDE_FLAG --exclude $2"
            shift 2
            ;;
        --exclude-atcoder)
            EXCLUDE_CRATES="$ATCODER_CRATES"
            EXCLUDE_FLAG=""
            for crate in $ATCODER_CRATES; do
                EXCLUDE_FLAG="$EXCLUDE_FLAG --exclude $crate"
            done
            shift
            ;;
        --no-exclude)
            EXCLUDE_FLAG=""
            shift
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            echo "エラー: 不明なオプション: $1"
            echo "ヘルプを表示するには -h または --help を使用してください"
            exit 1
            ;;
    esac
done

# cargo equipコマンドを構築
# --mine オプションで自分のクレートを指定し、ライセンス表記の問題を回避
CARGO_EQUIP_CMD="cargo equip --bin $BIN_NAME $EXCLUDE_FLAG --remove docs --minify libs --no-check"

echo "=== AtCoder提出用コードbundle ==="
echo "バイナリ名: $BIN_NAME"
echo "出力ファイル: $OUTPUT_FILE"
if [ -n "$EXCLUDE_FLAG" ]; then
    echo "除外クレート: $EXCLUDE_CRATES"
else
    echo "除外クレート: なし (全て展開)"
fi
echo ""

# bundle実行
echo "実行中: $CARGO_EQUIP_CMD"
echo ""

if $CLIPBOARD; then
    # クリップボードにコピー
    if command -v clip.exe &> /dev/null; then
        # WSL (Windows)
        eval "$CARGO_EQUIP_CMD" | tee "$OUTPUT_FILE" | clip.exe
        echo ""
        echo "✓ $OUTPUT_FILE に保存しました"
        echo "✓ クリップボードにコピーしました (clip.exe)"
    elif command -v pbcopy &> /dev/null; then
        # macOS
        eval "$CARGO_EQUIP_CMD" | tee "$OUTPUT_FILE" | pbcopy
        echo ""
        echo "✓ $OUTPUT_FILE に保存しました"
        echo "✓ クリップボードにコピーしました (pbcopy)"
    elif command -v xclip &> /dev/null; then
        # Linux (xclip)
        eval "$CARGO_EQUIP_CMD" | tee "$OUTPUT_FILE" | xclip -selection clipboard
        echo ""
        echo "✓ $OUTPUT_FILE に保存しました"
        echo "✓ クリップボードにコピーしました (xclip)"
    elif command -v xsel &> /dev/null; then
        # Linux (xsel)
        eval "$CARGO_EQUIP_CMD" | tee "$OUTPUT_FILE" | xsel --clipboard --input
        echo ""
        echo "✓ $OUTPUT_FILE に保存しました"
        echo "✓ クリップボードにコピーしました (xsel)"
    else
        echo "警告: クリップボードツール (clip.exe, pbcopy, xclip, xsel) が見つかりません"
        echo "ファイルのみに保存します"
        eval "$CARGO_EQUIP_CMD" > "$OUTPUT_FILE"
        echo ""
        echo "✓ $OUTPUT_FILE に保存しました"
    fi
else
    # ファイルのみに保存
    eval "$CARGO_EQUIP_CMD" > "$OUTPUT_FILE"
    echo ""
    echo "✓ $OUTPUT_FILE に保存しました"
fi

# 行数とファイルサイズを表示
LINE_COUNT=$(wc -l < "$OUTPUT_FILE")
FILE_SIZE=$(wc -c < "$OUTPUT_FILE")
echo "行数: $LINE_COUNT"
echo "サイズ: $FILE_SIZE bytes"
echo ""
echo "AtCoderに提出する準備ができました！"
