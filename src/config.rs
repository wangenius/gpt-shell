use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::env;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use colored::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub api_key: String,
    #[serde(default = "default_api_url")]
    pub api_url: String,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default = "default_stream")]
    pub stream: bool,
    #[serde(default)]
    pub system_prompt: Option<String>,
}

fn default_api_url() -> String {
    "https://api.openai.com/v1/chat/completions".to_string()
}

fn default_model() -> String {
    "gpt-3.5-turbo".to_string()
}

fn default_stream() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            api_url: default_api_url(),
            model: default_model(),
            stream: default_stream(),
            system_prompt: None,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        if let Some(path) = Self::get_path() {
            if path.exists() {
                match fs::read_to_string(&path) {
                    Ok(content) => match toml::from_str(&content) {
                        Ok(config) => Ok(config),
                        Err(_) => {
                            println!("config file format error, using default config.");
                            println!("please use the following commands to set necessary config:");
                            println!("  gpt config key <your-api-key>  # set API key");
                            println!("  gpt config url <api-url>       # set API URL (optional)");
                            println!("  gpt config model <model-name>  # set default model (optional)");
                            Ok(Config::default())
                        }
                    },
                    Err(_) => {
                        println!("failed to read config file, using default config.");
                        Ok(Config::default())
                    }
                }
            } else {
                let config = Config::default();
                if let Err(e) = config.save() {
                    println!("warning: failed to save default config file: {}", e);
                }
                println!("created default config file, please set necessary config:");
                println!("  gpt config key <your-api-key>  # set API key");
                println!("  gpt config url <api-url>       # set API URL (optional)");
                println!("  gpt config model <model-name>  # set default model (optional)");
                Ok(config)
            }
        } else {
            println!("failed to determine config file location, using default config.");
            Ok(Config::default())
        }
    }
    
    pub fn save(&self) -> Result<()> {
        if let Some(path) = Self::get_path() {
            // ensure directory exists
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            let content = toml::to_string_pretty(self)?;
            fs::write(path, content)?;
        }
        Ok(())
    }
    
    pub fn get_path() -> Option<PathBuf> {
        let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).ok()?;
        let mut path = PathBuf::from(home);
        path.push(".gpt-shell");
        path.push("config.toml");
        Some(path)
    }

    pub fn open_config() -> Result<()> {
        if let Some(path) = Self::get_path() {
            if cfg!(windows) {
                Command::new("notepad")
                    .arg(path)
                    .spawn()?;
            } else {
                Command::new("xdg-open")
                    .arg(path)
                    .spawn()?;
            }
        }
        Ok(())
    }

    pub fn set_key(&mut self, key: String) -> Result<()> {
        self.api_key = key;
        self.save()?;
        println!("API key updated");
        Ok(())
    }

    pub fn set_url(&mut self, url: String) -> Result<()> {
        self.api_url = url;
        self.save()?;
        println!("API URL updated");
        Ok(())
    }

    pub fn set_model(&mut self, model: String) -> Result<()> {
        let model_str = model.clone();
        self.model = model;
        self.save()?;
        println!("default model updated to: {}", model_str.green());
        Ok(())
    }

    pub fn set_system_prompt(&mut self, prompt: Option<String>) -> Result<()> {
        self.system_prompt = prompt;
        self.save()?;
        println!("system prompt updated");
        Ok(())
    }

    pub fn set_stream(&mut self, enabled: bool) -> Result<()> {
        self.stream = enabled;
        self.save()?;
        println!("stream output {}", if enabled { "enabled" } else { "disabled" });
        Ok(())
    }
} 