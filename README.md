# cp_unfold

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A command-line tool for competitive programmers to flatten modular Rust projects into a single file for submission.

競技プログラミング用のRustコード展開ツール。複数ファイルに分割されたライブラリを1ファイルに統合します。

## ✨ Features

- 🚀 **Fast and Simple**: One-command solution to unfold your Rust projects
- 📦 **Smart Import Resolution**: Handles complex import patterns including `use library::*`, `use super::*`, and nested imports
- 🔄 **Relative Import Support**: Resolves `super::` imports correctly
- 🎯 **Duplicate Elimination**: Automatically removes redundant imports
- ⚙️ **Persistent Configuration**: Save your project settings for repeated use
- 💾 **Interactive Setup**: First-run wizard guides you through configuration

## 📥 Installation

### From source

```bash
cargo install --path .
```

Or download pre-built binaries from [Releases](https://github.com/kanra824/cp_unfold/releases).

## 🚀 Quick Start

### First Run (Interactive Setup)

```bash
cp_unfold
# Enter file directory (source file location): /home/user/project/src
# Config saved to ~/.config/cp_unfold/config.toml
```

### Subsequent Runs

```bash
# Use saved configuration
cp_unfold > submission.rs

# Override specific options
cp_unfold --src another.rs > output.rs
```

## 📖 Usage

### Command-line Options

```bash
cp_unfold [OPTIONS]

Options:
  -f, --file-dir <FILE_DIR>          Directory containing the source file
  -s, --src <SRC>                    Source file name to unfold [default: main.rs]
  -l, --library-name <LIBRARY_NAME>  Library import name [default: library]
  -p, --library-path <LIBRARY_PATH>  Path to the library directory
  -h, --help                         Print help
```

### Configuration File

Settings are stored at `~/.config/cp_unfold/config.toml`:

```toml
file_dir = "/home/user/project/src"
library_name = "library"
library_path = "/home/user/project/src/library"
```

Edit this file directly or let the tool create it on first run.

## 📁 Example

### Project Structure

```
src/
├── main.rs
└── library/
    ├── graph.rs
    ├── union_find.rs
    └── math/
        └── modint.rs
```

### Input: `main.rs`

```rust
use library::graph::*;
use library::union_find::UnionFind;
use library::math::modint::ModInt;

fn main() {
    let mut uf = UnionFind::new(100);
    let g = Graph::new(10);
    let m = ModInt::new(1000000007);
    // your solution code here
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

### Run

```bash
cp_unfold > submission.rs
```

### Output: Single file ready for submission

All imports are resolved and library code is inlined into one file.

## 🎯 Supported Import Patterns

- ✅ `use library::module::*`
- ✅ `use crate::library::module::Type`
- ✅ `use library::{module1, module2}`
- ✅ `use super::sibling_module::*` (relative imports)
- ✅ Nested braces: `use std::{io::{self, Read}, fs::File}`

## ⚙️ How It Works

1. **Parse**: Reads your main source file and identifies library imports
2. **Resolve**: Recursively resolves all imports including relative paths (`super::`)
3. **Merge**: Combines all library code into a single output
4. **Deduplicate**: Removes redundant imports and declarations
5. **Output**: Generates a single standalone file

## 🛠️ Advanced Usage

### Multiple Projects

```bash
# Project A
cp_unfold --file-dir ~/projectA/src > solutionA.rs

# Project B
cp_unfold --file-dir ~/projectB/src > solutionB.rs
```

### Custom Library Structure

```bash
cp_unfold --library-name mylib --library-path ./src/mylib
```

### Pipe to Clipboard (Linux)

```bash
cp_unfold | xclip -selection clipboard
```

### Pipe to Clipboard (macOS)

```bash
cp_unfold | pbcopy
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

Built for competitive programmers who want to maintain clean, modular code while meeting single-file submission requirements.

## ⚠️ Limitations

- Assumes no circular dependencies in library code
- Does not support `use ... as` aliasing in library imports (only in standard library imports)
- Relative imports (`super::`) are resolved based on file system structure

---

**Happy Competitive Programming! 🚀**
