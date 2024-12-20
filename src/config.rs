use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::env;
use std::collections::HashMap;
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
    #[serde(default)]
    pub aliases: HashMap<String, String>,
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
            aliases: HashMap::new(),
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
                            println!("配置文件格式有误，将使用默认配置。");
                            println!("请使用以下命令设置必要的配置：");
                            println!("  gpt config key <your-api-key>  # 设置API密钥");
                            println!("  gpt config url <api-url>       # 设置API URL（可选）");
                            println!("  gpt config model <model-name>  # 设置默认模型（可选）");
                            Ok(Config::default())
                        }
                    },
                    Err(_) => {
                        println!("无法读取配置文件，将使用默认配置。");
                        Ok(Config::default())
                    }
                }
            } else {
                let config = Config::default();
                if let Err(e) = config.save() {
                    println!("警告：无法保存默认配置文件：{}", e);
                }
                println!("已创建默认配置文件，请设置必要的配置：");
                println!("  gpt config key <your-api-key>  # 设置API密钥");
                println!("  gpt config url <api-url>       # 设置API URL（可选）");
                println!("  gpt config model <model-name>  # 设置默认模型（可选）");
                Ok(config)
            }
        } else {
            println!("无法确定配置文件位置，将使用默认配置。");
            Ok(Config::default())
        }
    }
    
    pub fn save(&self) -> Result<()> {
        if let Some(path) = Self::get_path() {
            // 确保目录存在
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
        println!("API密钥已更新");
        Ok(())
    }

    pub fn set_url(&mut self, url: String) -> Result<()> {
        self.api_url = url;
        self.save()?;
        println!("API URL已更新");
        Ok(())
    }

    pub fn set_model(&mut self, model: String) -> Result<()> {
        let model_str = model.clone();
        self.model = model;
        self.save()?;
        println!("默认模型已更新为: {}", model_str.green());
        Ok(())
    }

    pub fn set_system_prompt(&mut self, prompt: Option<String>) -> Result<()> {
        self.system_prompt = prompt;
        self.save()?;
        println!("系统提示词已更新");
        Ok(())
    }

    pub fn set_stream(&mut self, enabled: bool) -> Result<()> {
        self.stream = enabled;
        self.save()?;
        println!("流式输出已{}启用", if enabled { "" } else { "禁" });
        Ok(())
    }

    pub fn set_alias(&mut self, bot_name: String, alias: String) -> Result<()> {
        if alias.len() != 1 {
            return Err(anyhow::anyhow!("别名必须是单个字符"));
        }
        let alias_str = alias.clone();
        self.aliases.insert(alias, bot_name.clone());
        self.save()?;
        println!("已设置别名: {} -> {}", alias_str.green(), bot_name.green());
        Ok(())
    }

    pub fn remove_alias(&mut self, alias: &str) -> Result<()> {
        if let Some(bot_name) = self.aliases.remove(alias) {
            self.save()?;
            println!("已删除别名: {} -> {}", alias.green(), bot_name.green());
            Ok(())
        } else {
            Err(anyhow::anyhow!("未找到别名: {}", alias))
        }
    }

    pub fn list_aliases(&self) {
        if self.aliases.is_empty() {
            println!("还没有设置任何别名");
            return;
        }

        println!("当前别名：");
        for (alias, bot_name) in &self.aliases {
            println!("- {} -> {}", alias.green(), bot_name.green());
        }
    }

    pub fn get_bot_by_alias(&self, alias: &str) -> Option<&String> {
        self.aliases.get(alias)
    }
} 