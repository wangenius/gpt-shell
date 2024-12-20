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
        path.push("bots.toml");
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
    
    pub fn add_bot(&mut self, name: String, system_prompt: String) -> Result<()> {
        let bot = Bot {
            name: name.clone(),
            system_prompt,
        };
        self.bots.insert(name.clone(), bot);
        self.save()?;
        println!("已添加机器人: {}", name.green());
        Ok(())
    }
    
    pub fn remove_bot(&mut self, name: &str) -> Result<()> {
        if self.bots.remove(name).is_some() {
            self.save()?;
            println!("已删除机器人: {}", name.green());
            Ok(())
        } else {
            Err(anyhow::anyhow!("未找到机器人: {}", name))
        }
    }
    
    pub fn get_bot(&self, name: &str) -> Option<&Bot> {
        self.bots.get(name)
    }

    pub fn list_bots(&self) {
        if self.bots.is_empty() {
            println!("还没有添加任何机器人");
            return;
        }

        println!("可用的机器人：");
        for (name, bot) in &self.bots {
            println!("- {} (系统提示词: {})", name.green(), bot.system_prompt);
        }
    }
} 