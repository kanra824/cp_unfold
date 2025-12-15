use std::fs::read_to_string;
use std::collections::BTreeSet;
use std::path::{PathBuf};

struct Unfold {
    // src_name: String, // e.g. "main.rs"
    library_import_name: String, // e.g. "library"
    file_path: PathBuf, // e.g. "/home/hoge/kyopro/src/main.rs"
    library_path: PathBuf, // e.g. "/home/hoge/kyopro/src/library"

    used_lib: BTreeSet<String>,
    used_lib_star: BTreeSet<String>,

    unfolded_path: BTreeSet<String>,
}

impl Unfold {
    fn new() -> Self {
        let src_name = std::env::var("CP_UNFOLD_SRC").unwrap_or("main.rs".to_string());

        let library_import_name_env = std::env::var("CP_UNFOLD_LIBRARY_NAME");
        let library_import_name = library_import_name_env.unwrap_or("library".to_string());

        let file_dir_str = std::env::var("CP_UNFOLD_FILE_DIR").expect("CP_UNFOLD_FILE_PATH environment variable not set");
        let file_dir = PathBuf::from(&file_dir_str);

        let file_path = file_dir.join(src_name);

        let library_path_env = std::env::var("CP_UNFOLD_LIBRARY_PATH").unwrap_or(file_dir.join(&library_import_name).to_str().unwrap().to_string());
        let library_path = PathBuf::from(&library_path_env);

        let used_lib = BTreeSet::new();
        let used_lib_star = BTreeSet::new();
        let unfolded_path = BTreeSet::new();

        Unfold {
            library_import_name,
            file_path,
            library_path,
            used_lib,
            used_lib_star,
            unfolded_path,
        }
    }

    fn unfold_curly_bracket_rec(now: &mut Vec<String>, idx: usize, import_path_v: &[String], res: &mut Vec<Vec<String>>) {
        if idx == import_path_v.len() {
            res.push(now.clone());
            return;
        }

        let str = import_path_v[idx].chars().collect::<Vec<_>>();

        if str[0] == '{' {
            // 終端のはずで、str を import_path_v の形に展開したうえで、now をそのまま渡して再帰を始めればいい
            let mut child_v: Vec<String> = vec![];
            let mut tmp_v = vec![];
            for i in 1..str.len()-1 {
                if str[i] == ',' {
                    child_v.push(tmp_v.iter().collect());
                    tmp_v = vec![];
                } else {
                    tmp_v.push(str[i]);
                }
            }
            child_v.push(tmp_v.iter().collect());
            for child in child_v {
                let import_path_v = Unfold::split_by_coloncolon(child);
                Unfold::unfold_curly_bracket_rec(now, 0, &import_path_v, res);
            }
        } else {
            now.push(import_path_v[idx].to_string());
            Unfold::unfold_curly_bracket_rec(now, idx + 1, import_path_v, res);
            now.pop();
        }
    }

    fn unfold_curly_bracket(import_path_v: &[String]) -> Vec<Vec<String>> {
        let mut now = vec![];
        let mut res = vec![];
        Unfold::unfold_curly_bracket_rec(&mut now, 0, import_path_v, &mut res);

        res
    }

    fn split_by_coloncolon(import_path: String) -> Vec<String> {
        let mut depth = 0; // depth == 0 のときだけ :: を split
        let mut prev = false;
        let mut import_path_v: Vec<String> = vec![];
        let mut tmp_v = vec![];
        let import_path_chars = import_path.chars().collect::<Vec<_>>();
        for (i, &c) in import_path_chars.iter().enumerate() {
            if c == '{' {
                depth += 1;
            } else if c == '}' {
                depth -= 1;
            }
            if c == ':' && import_path_chars[i+1] == ':' && depth == 0 {
                prev = true;
                import_path_v.push(tmp_v.iter().collect());
                tmp_v = vec![];
            } else if prev {
                prev = false;
            } else {
                tmp_v.push(c);
            }
        }
        import_path_v.push(tmp_v.iter().collect());
        import_path_v
    }

    fn unfold_rec(&mut self, file_path: &PathBuf) -> Result<(String, String), std::io::Error> {
        let content = read_to_string(file_path.to_str().unwrap())?;
        let mut res = String::new();
        let mut res_inner_directive = String::new();
        for line in content.lines() {
            let str_v: Vec<&str> = line.split_whitespace().collect();

            // 空行
            if str_v.len() == 0 {
                res += "\n";
                continue;
            }

            // mod, pub mod
            // CP_UNFOLD_LIBRARY_PATH にライブラリのディレクトリは指定されているものとする
            if str_v[0] == "mod" || (str_v.len() >= 2 && str_v[0] == "pub" && str_v[1] == "mod") {
                continue;
            } 

            // #!
            // inner_directive
            let tmp_v = str_v[0].chars().collect::<Vec<_>>();
            if tmp_v.len() >= 2 && tmp_v[0] == '#' && tmp_v[1] == '!' {
                res_inner_directive += line;
                res_inner_directive += "\n";
                continue;
            }


            // 相対読み込み (use super::graph::* など) は使わないものとする
            // (実際どう読むんだ？まあサイクルがないことは保証されるはずだからグラフ生やせばどうとでもなる気がする)
            // 二文字目で全部読み込まれるものとする (そうじゃないケースあるかな)
            let ofs = if str_v[0] == "use" {
                0
            } else if str_v.len() >= 2 && str_v[0] == "pub" && str_v[1] == "use" {
                1
            } else {
                res += line;
                res += "\n";
                continue;
            };

            // 後ろ全部繋げる
            let mut import_path =
                str_v.
                    iter().
                    enumerate().
                    filter(|(idx, _)| *idx > ofs).
                    fold(String::new(), |str, (_, val)| str + *val);
            import_path.pop(); // セミコロンを取る
            let import_path_v = Unfold::split_by_coloncolon(import_path);

            if import_path_v[0] != self.library_import_name && import_path_v[0] != "crate" {
                // {} を展開して、self.used_lib に放り込む
                // * の対応が大変！
                // 後でチェック
                let res_import_v = Unfold::unfold_curly_bracket(&import_path_v);
                for res_import in res_import_v {

                    let import_path = res_import.join("::");

                    if res_import.last().unwrap() == "*" {
                        self.used_lib_star.insert(import_path);
                    } else {
                        self.used_lib.insert(import_path);
                    }
                }
            } else {
                // self.used_lib に含まれていたらスルー
                // library::hoge::fuga::* か crate::library::hoge::fuga::* で {library_path}/hoge/fuga.rs の中身を import しているとみなす
                // super は無視

                // library::より下から見る
                // {library_path}/hoge/fuga.rs をトップレベルとして指定して、unfold する (used_lib は共通)
                let ofs = if import_path_v[0] == "crate" {
                    2
                } else {
                    1
                };

                let mut path = self.library_path.clone();
                for i in ofs..import_path_v.len()-1 {
                    let join_str = if i == import_path_v.len() - 2 {
                        &(import_path_v[i].clone() + ".rs")
                    } else {
                        &import_path_v[i]
                    };
                    path = path.join(join_str);
                }

                if self.unfolded_path.contains(path.to_str().unwrap()) {
                    continue;
                }
                self.unfolded_path.insert(path.to_str().unwrap().to_string());

                let (child_res, _) = self.unfold_rec(&path)?;
                res += &child_res;
                res += "\n";
            }
        }
        Ok((res, res_inner_directive))
    }

    fn unfold_use(&mut self) -> Result<String, std::io::Error> {
        let mut res_use = String::new();
        // used_lib の中身が used_lib_star とマッチしないかどうかチェック
        for import_path in &self.used_lib_star {
            res_use += "use ";
            res_use += import_path;
            res_use += ";\n";
        }
        for import_path in &self.used_lib {
            let mut current_path = String::new();

            let mut contained = false;
            for (i, part) in import_path.split("::").enumerate() {
                if i != 0 {
                    current_path += "::";
                }
                current_path += part;

                if self.used_lib_star.contains(&(current_path.clone() + "::*")) {
                    contained = true;
                    break;
                }
            }
            if contained {
                continue;
            }
            res_use += "use ";
            res_use += &current_path;
            res_use += ";\n";
        }


        Ok(res_use)
    }

    fn unfold(&mut self, file_path: &PathBuf) -> Result<String, std::io::Error> {
        let (res, res_inner_directive) = self.unfold_rec(file_path)?;
        let res_use = self.unfold_use()?;
        Ok(res_inner_directive + &res_use + &res)
    }
}


fn main() {
    let mut unfold = Unfold::new();
    let file_path = unfold.file_path.clone();
    let res = unfold.unfold(&file_path);

    match res {
        Ok(val) => {
            println!("{}", val);
        },
        Err(val) => {
            println!("{:?}", val);
        }
    }

}
