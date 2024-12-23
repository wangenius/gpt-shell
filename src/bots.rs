use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use std::process::Command;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use colored::*;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bot {
    pub name: String,
    pub system_prompt: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BotsConfig {
    pub bots: HashMap<String, Bot>,
    #[serde(default)]
    pub aliases: HashMap<String, String>,
    #[serde(default)]
    pub current: Option<String>,
}

impl BotsConfig {
    pub fn load() -> Result<Self> {
        if let Some(path) = Self::get_path() {
            if path.exists() {
                let content = fs::read_to_string(&path)?;
                Ok(toml::from_str(&content)?)
            } else {
                let config = BotsConfig::default();
                config.save()?;
                Ok(config)
            }
        } else {
            Ok(BotsConfig::default())
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
        path.push("bots.toml");
        Some(path)
    }

    pub fn open_config() -> Result<()> {
        if let Some(path) = Self::get_path() {
            if cfg!(windows) {
                // 首先尝试使用 VSCode
                if Command::new("code")
                    .arg(path.clone())
                    .spawn()
                    .is_err() {
                    // 如果 VSCode 不可用，尝试使用 Notepad++
                    if Command::new("notepad++")
                        .arg(path.clone())
                        .spawn()
                        .is_err() {
                        // 最后使用系统默认的记事本
                        Command::new("notepad")
                            .arg(path)
                            .spawn()?;
                    }
                }
            } else {
                // 在类Unix系统上，首先尝试使用环境变量中的默认编辑器
                if let Ok(editor) = env::var("EDITOR") {
                    Command::new(editor)
                        .arg(path.clone())
                        .spawn()?;
                } else {
                    // 如果没有设置 EDITOR，尝试常见的编辑器
                    let editors = ["code", "vim", "nano", "gedit"];
                    let mut opened = false;
                    for editor in editors {
                        if Command::new(editor)
                            .arg(path.clone())
                            .spawn()
                            .is_ok() {
                            opened = true;
                            break;
                        }
                    }
                    // 如果都失败了，使用 xdg-open
                    if !opened {
                        Command::new("xdg-open")
                            .arg(path)
                            .spawn()?;
                    }
                }
            }
        }
        Ok(())
    }
    
    pub fn add_bot(&mut self, name: String, system_prompt: String) -> Result<()> {
        let bot = Bot {
            name: name.clone(),
            system_prompt,
        };
        self.bots.insert(name.clone(), bot);
        self.save()?;
        println!("bot added: {}", name.green());
        Ok(())
    }
    
    pub fn remove_bot(&mut self, name: &str) -> Result<()> {
        if self.bots.remove(name).is_some() {
            self.save()?;
            println!("bot removed: {}", name.green());
            Ok(())
        } else {
            Err(anyhow::anyhow!("bot not found: {}", name))
        }
    }
    
    pub fn get_bot(&self, name: &str) -> Option<&Bot> {
        self.bots.get(name)
    }

    pub fn list_bots(&self) {
        if self.bots.is_empty() {
            println!("no bots added yet");
            return;
        }

        println!("available bots:");
        for (name, bot) in &self.bots {
            let current_marker = if Some(name) == self.current.as_ref() {
                "* ".bright_green()
            } else {
                "  ".into()
            };
            println!("{}{} (system prompt: {})", current_marker, name.green(), bot.system_prompt);
        }
    }

    pub fn set_alias(&mut self, bot: String, alias: String) -> Result<()> {
        if alias.len() != 1 {
            return Err(anyhow::anyhow!("alias must be a single character"));
        }
        // 检查机器人是否存在
        if !self.bots.contains_key(&bot) {
            return Err(anyhow::anyhow!("bot not found: {}", bot));
        }
        // 添加或更新别名
        self.aliases.insert(alias.clone(), bot.clone());
        self.save()?;
        println!("alias set: {} -> {}", alias.green(), bot.green());
        Ok(())
    }

    pub fn remove_alias(&mut self, alias: &str) -> Result<()> {
        if let Some(bot_name) = self.aliases.remove(alias) {
            self.save()?;
            println!("alias removed: {} -> {}", alias.green(), bot_name.green());
            Ok(())
        } else {
            Err(anyhow::anyhow!("alias not found: {}", alias))
        }
    }

    pub fn list_aliases(&self) {
        if self.aliases.is_empty() {
            println!("no aliases set yet");
            return;
        }

        println!("current aliases:");
        for (alias, bot_name) in &self.aliases {
            println!("- {} -> {}", alias.green(), bot_name.green());
        }
    }

    pub fn get_bot_by_alias(&self, alias: &str) -> Option<&String> {
        self.aliases.get(alias)
    }

    pub fn set_current(&mut self, name: &str) -> Result<()> {
        if !self.bots.contains_key(name) {
            return Err(anyhow::anyhow!("机器人不存在: {}", name));
        }
        self.current = Some(name.to_string());
        self.save()?;
        println!("当前机器人已设置为: {}", name.green());
        Ok(())
    }

    pub fn clear_current(&mut self) -> Result<()> {
        self.current = None;
        self.save()?;
        println!("已清除当前机器人设置");
        Ok(())
    }

    pub fn get_current(&self) -> Option<&Bot> {
        self.current.as_ref().and_then(|name| self.bots.get(name))
    }
} 