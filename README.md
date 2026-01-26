# cp_unfold
競技プログラミング用のRustコード展開ツールです。
複数ファイルに分割されたライブラリを1ファイルに統合し、提出用の単一ファイルを生成します。

## インストール

```bash
git clone {このリポジトリ}
cd cp_unfold
cargo install --path .
```

## 使い方

### 初回実行

```bash
cp_unfold
# Enter file directory (source file location): /path/to/your/project/src
# Config saved to ~/.config/cp_unfold/config.toml
```

初回実行時に対話的にプロジェクトディレクトリを設定します。設定は `~/.config/cp_unfold/config.toml` に保存されます。

### 2回目以降

```bash
cp_unfold > submission.rs
```

保存された設定を使って展開します。

### オプション

```bash
cp_unfold [OPTIONS]

Options:
  -f, --file-dir <FILE_DIR>          ソースファイルがあるディレクトリ
  -s, --src <SRC>                    展開するソースファイル名 [default: main.rs]
  -l, --library-name <LIBRARY_NAME>  ライブラリのインポート名 [default: library]
  -p, --library-path <LIBRARY_PATH>  ライブラリディレクトリのパス
  -h, --help                         ヘルプを表示
```

## 例

### プロジェクト構造

```
src/
├── main.rs
└── library/
    ├── graph.rs
    └── union_find.rs
```

### main.rs

```rust
mod library;
use library::graph::*;
use library::union_find::UnionFind;

fn main() {
    let g = Graph::new(10);
    let mut uf = UnionFind::new(100);
    // あなたの解答コード
}
```

### 実行

```bash
cp_unfold > submission.rs
```

`submission.rs` にすべてのライブラリコードが統合された提出用ファイルが生成されます。

## 使える構文

- `use library::module::*`
- `use crate::library::module::Type`
- `use library::{module1, module2}`
- `use super::sibling_module::*` (相対インポート)
- `use std::{io::{self, Read}, fs::File}` (ネストされた中括弧)
- 複数行にわたる use 文
```rust
use std::{
    io::{self, Read},
    fs::File
};
```

## サポートしていない構文
- ライブラリインポートでの `use ... as` エイリアスには非対応

## プロジェクト構造

このプロジェクトはライブラリとバイナリのハイブリッドクレートです。

```
src/
├── lib.rs      # ライブラリのルート（公開API）
├── main.rs     # バイナリのエントリーポイント
├── config.rs   # 設定管理
└── unfold.rs   # コア展開ロジック

tests/
├── unit_test.rs         # ユニットテスト（8個）
└── integration_test.rs  # 統合テスト（6個）
```

ライブラリとしても使用可能：
```rust
use cp_unfold::{Unfold, Config};

let mut unfold = Unfold::from_args(
    "main.rs".to_string(),
    "library".to_string(),
    src_dir,
    None,
);
let result = unfold.unfold()?;
```

## テスト

### すべてのテストを実行

```bash
cargo test
```

### ユニットテストのみ実行

```bash
cargo test --lib
```

### 統合テストのみ実行

```bash
cargo test --test integration_test
```

### テストカバレッジ

- **ユニットテスト（8個）**: `split_by_coloncolon` と `unfold_curly_bracket` の各種パターン
  - 単純な `::` 分割
  - 中括弧を含む分割
  - ネストした中括弧の展開
  - 複雑な use 文の展開

- **統合テスト（6個）**: 実際のファイルを使った end-to-end テスト
  - 単純なライブラリインポート
  - 複数行 use 文
  - ネストした中括弧
  - ワイルドカードインポート (`*`)
  - 相対インポート (`super::`)
  - ライブラリAPIの直接使用
