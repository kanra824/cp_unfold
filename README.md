# cp_unfold
競技プログラミング用のRustコード展開ツール。複数ファイルに分割されたライブラリを1ファイルに統合し、提出用の単一ファイルを生成します。

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

## サポートしていない構文
- ライブラリインポートでの `use ... as` エイリアスには非対応
- 複数行にわたるケース
```
use std::{
    io::{self, Read},
    fs::File
};
```
