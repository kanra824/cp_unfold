# cp_unfold

競技プログラミング用のRustライブラリ展開ツールです。複数のファイルに分割されたライブラリコードを1つのファイルに展開します。

## 機能

- `use library::*` や `use crate::library::*` 形式のインポートを解決してコードを展開
- ネストされた `use` 文や `{a, b, c}` 形式の複数インポートに対応
- 重複インポートの削除
- `mod` および `pub mod` 宣言の自動削除
- inner attributes (`#![...]`) の保持

## インストール

```bash
cargo build --release
```

ビルドされたバイナリは `target/release/cp_unfold` に生成されます。

## 使い方

環境変数を設定してプログラムを実行します。

```bash
export CP_UNFOLD_FILE_DIR=/path/to/your/project/src
export CP_UNFOLD_LIBRARY_PATH=/path/to/your/project/src/library
export CP_UNFOLD_SRC=main.rs
export CP_UNFOLD_LIBRARY_NAME=library

cp_unfold > output.rs
```

### 環境変数

- `CP_UNFOLD_FILE_DIR` (必須): ソースファイルのディレクトリパス
- `CP_UNFOLD_LIBRARY_PATH` (オプション): ライブラリのディレクトリパス (デフォルト: `{CP_UNFOLD_FILE_DIR}/{CP_UNFOLD_LIBRARY_NAME}`)
- `CP_UNFOLD_SRC` (オプション): 展開するソースファイル名 (デフォルト: `main.rs`)
- `CP_UNFOLD_LIBRARY_NAME` (オプション): ライブラリのモジュール名 (デフォルト: `library`)

## 例

### ディレクトリ構造

```
src/
├── main.rs
└── library/
    ├── graph.rs
    └── union_find.rs
```

### main.rs

```rust
use library::graph::*;
use library::union_find::UnionFind;

fn main() {
    // your code
}
```

### 実行

```bash
export CP_UNFOLD_FILE_DIR=./src
./target/release/cp_unfold > submission.rs
```

展開されたコードが `submission.rs` に出力されます。

## 制限事項

- 相対インポート (`use super::*`) は非対応
- ライブラリコード内に循環依存がないことを前提としています
