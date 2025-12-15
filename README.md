# cp_unfold

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

競技プログラミング用のRustコード展開ツール。複数ファイルに分割されたライブラリを1ファイルに統合し、提出用の単一ファイルを生成します。

A command-line tool for competitive programmers to flatten modular Rust projects into a single file for submission.

## ✨ 機能

- 🚀 **高速でシンプル**: 1コマンドでプロジェクトを展開
- 📦 **賢いインポート解決**: `use library::*`、`use super::*`、ネストされたインポートなど複雑なパターンに対応
- 🔄 **相対インポート対応**: `super::`による相対インポートを正しく解決
- 🎯 **重複排除**: 冗長なインポートを自動削除
- ⚙️ **永続的な設定**: プロジェクト設定を保存して繰り返し使用可能
- 💾 **対話的セットアップ**: 初回実行時に設定をガイド

## 📥 インストール

### ソースから

```bash
cargo install --path .
```

または [Releases](https://github.com/kanra824/cp_unfold/releases) からビルド済みバイナリをダウンロード。

## 🚀 クイックスタート

### 初回実行（対話的セットアップ）

```bash
cp_unfold
# Enter file directory (source file location): /home/user/project/src
# Config saved to ~/.config/cp_unfold/config.toml
```

### 2回目以降

```bash
# 保存された設定を使用
cp_unfold > submission.rs

# 特定のオプションを上書き
cp_unfold --src another.rs > output.rs
```

## 📖 使い方

### コマンドラインオプション

```bash
cp_unfold [OPTIONS]

Options:
  -f, --file-dir <FILE_DIR>          ソースファイルがあるディレクトリ
  -s, --src <SRC>                    展開するソースファイル名 [デフォルト: main.rs]
  -l, --library-name <LIBRARY_NAME>  ライブラリのインポート名 [デフォルト: library]
  -p, --library-path <LIBRARY_PATH>  ライブラリディレクトリのパス
  -h, --help                         ヘルプを表示
```

### 設定ファイル

設定は `~/.config/cp_unfold/config.toml` に保存されます:

```toml
file_dir = "/home/user/project/src"
library_name = "library"
library_path = "/home/user/project/src/library"
```

このファイルを直接編集するか、初回実行時にツールに作成させることができます。

## 📁 使用例

### プロジェクト構造

```
src/
├── main.rs
└── library/
    ├── graph.rs
    ├── union_find.rs
    └── math/
        └── modint.rs
```

### 入力: `main.rs`

```rust
use library::graph::*;
use library::union_find::UnionFind;
use library::math::modint::ModInt;

fn main() {
    let mut uf = UnionFind::new(100);
    let g = Graph::new(10);
    let m = ModInt::new(1000000007);
    // あなたの解答コード
}
```

### library/graph.rs

```rust
pub struct Graph {
    pub n: usize,
    pub edges: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Self { n, edges: vec![vec![]; n] }
    }
}
```

### 実行

```bash
cp_unfold > submission.rs
```

### 出力: 提出用の単一ファイル

すべてのインポートが解決され、ライブラリコードが1つのファイルに統合されます。

## 🎯 サポートされているインポートパターン

- ✅ `use library::module::*`
- ✅ `use crate::library::module::Type`
- ✅ `use library::{module1, module2}`
- ✅ `use super::sibling_module::*` (相対インポート)
- ✅ ネストされた中括弧: `use std::{io::{self, Read}, fs::File}`

## ⚙️ 仕組み

1. **解析**: メインソースファイルを読み込み、ライブラリインポートを識別
2. **解決**: 相対パス (`super::`) を含むすべてのインポートを再帰的に解決
3. **統合**: すべてのライブラリコードを1つの出力に結合
4. **重複排除**: 冗長なインポートと宣言を削除
5. **出力**: 単一のスタンドアロンファイルを生成

## 🛠️ 高度な使い方

### 複数プロジェクト

```bash
# プロジェクトA
cp_unfold --file-dir ~/projectA/src > solutionA.rs

# プロジェクトB
cp_unfold --file-dir ~/projectB/src > solutionB.rs
```

### カスタムライブラリ構造

```bash
cp_unfold --library-name mylib --library-path ./src/mylib
```

### クリップボードにパイプ (Linux)

```bash
cp_unfold | xclip -selection clipboard
```

### クリップボードにパイプ (macOS)

```bash
cp_unfold | pbcopy
```

## 🤝 コントリビューション

プルリクエストを歓迎します！

1. リポジトリをフォーク
2. フィーチャーブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add some amazing feature'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエストを作成

## 📝 ライセンス

このプロジェクトはMITライセンスの下で公開されています。詳細はLICENSEファイルを参照してください。

## 🙏 謝辞

きれいでモジュール化されたコードを維持しながら、単一ファイル提出要件を満たしたい競技プログラマーのために作られました。

## ⚠️ 制限事項

- ライブラリコード内に循環依存がないことを前提としています
- ライブラリインポートでの `use ... as` エイリアスには非対応（標準ライブラリのインポートでは使用可能）
- 相対インポート (`super::`) はファイルシステム構造に基づいて解決されます

---

**Happy Competitive Programming! 🚀**
