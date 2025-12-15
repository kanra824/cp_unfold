use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub file_dir: Option<String>,
    pub library_name: Option<String>,
    pub library_path: Option<String>,
}

pub struct RuntimeConfig {
    pub src: String,
    pub library_name: String,
    pub file_dir: PathBuf,
    pub library_path: Option<PathBuf>,
}

impl Config {
    fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|dir| dir.join("cp_unfold").join("config.toml"))
    }

    pub fn exists() -> bool {
        Self::config_path()
            .map(|path| path.exists())
            .unwrap_or(false)
    }

    pub fn load() -> Self {
        let config_path = match Self::config_path() {
            Some(path) => path,
            None => return Config::default(),
        };

        if !config_path.exists() {
            return Config::default();
        }

        let content = match fs::read_to_string(&config_path) {
            Ok(content) => content,
            Err(_) => return Config::default(),
        };

        toml::from_str(&content).unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path()
            .ok_or("Could not determine config directory")?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;

        Ok(())
    }

    /// 対話的に設定を初期化
    pub fn interactive_init(cli_library_name: Option<String>, cli_library_path: Option<PathBuf>) -> Self {
        eprint!("Enter file directory (source file location): ");
        io::stderr().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let file_dir_input = input.trim().to_string();

        if file_dir_input.is_empty() {
            eprintln!("Error: file_dir cannot be empty");
            std::process::exit(1);
        }

        Config {
            file_dir: Some(file_dir_input),
            library_name: cli_library_name,
            library_path: cli_library_path.as_ref().map(|p| p.display().to_string()),
        }
    }

    /// CLI引数から設定を作成
    pub fn from_cli_args(
        file_dir: Option<&PathBuf>,
        library_name: Option<String>,
        library_path: Option<&PathBuf>,
    ) -> Self {
        Config {
            file_dir: file_dir.map(|p| p.display().to_string()),
            library_name,
            library_path: library_path.map(|p| p.display().to_string()),
        }
    }

    /// CLI引数と設定ファイルをマージして実行時設定を作成
    pub fn merge_with_cli(
        &self,
        cli_src: Option<String>,
        cli_library_name: Option<String>,
        cli_file_dir: Option<PathBuf>,
        cli_library_path: Option<PathBuf>,
    ) -> Result<RuntimeConfig, String> {
        let src = cli_src
            .or_else(|| Some("main.rs".to_string()))
            .unwrap();

        let library_name = cli_library_name
            .or_else(|| self.library_name.clone())
            .unwrap_or_else(|| "library".to_string());

        let file_dir = cli_file_dir
            .or_else(|| self.file_dir.as_ref().map(PathBuf::from))
            .ok_or("Error: --file-dir is required (or set in config file at ~/.config/cp_unfold/config.toml)")?;

        let library_path = cli_library_path
            .or_else(|| self.library_path.as_ref().map(PathBuf::from));

        Ok(RuntimeConfig {
            src,
            library_name,
            file_dir,
            library_path,
        })
    }

    /// 設定を初期化してCLI引数とマージした実行時設定を返す
    pub fn initialize_and_merge(
        cli_src: Option<String>,
        cli_library_name: Option<String>,
        cli_file_dir: Option<PathBuf>,
        cli_library_path: Option<PathBuf>,
    ) -> Result<RuntimeConfig, String> {
        let mut config = Self::load();
        let config_exists = Self::exists();

        // 設定ファイルが存在せず、CLI引数もない場合は対話的に初期化
        if !config_exists && cli_file_dir.is_none() && config.file_dir.is_none() {
            config = Self::interactive_init(cli_library_name.clone(), cli_library_path.clone());
            
            if let Err(e) = config.save() {
                eprintln!("Warning: Could not save config file: {}", e);
            } else {
                eprintln!("Config saved to ~/.config/cp_unfold/config.toml");
            }
        }
        // CLI引数で設定が指定された場合も保存（初回のみ）
        else if !config_exists && (cli_file_dir.is_some() || cli_library_name.is_some() || cli_library_path.is_some()) {
            let new_config = Self::from_cli_args(
                cli_file_dir.as_ref(),
                cli_library_name.clone(),
                cli_library_path.as_ref(),
            );
            
            if let Err(e) = new_config.save() {
                eprintln!("Warning: Could not save config file: {}", e);
            } else {
                eprintln!("Config saved to ~/.config/cp_unfold/config.toml");
            }
            
            config = new_config;
        }

        // CLI引数と設定ファイルをマージ
        config.merge_with_cli(cli_src, cli_library_name, cli_file_dir, cli_library_path)
    }
}
