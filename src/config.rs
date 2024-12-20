use std::fs;
use std::path::PathBuf;
use std::process::Command;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use colored::*;
use std::env;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelConfig {
    pub api_key: String,
    pub api_url: String,
    pub model: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub models: HashMap<String, ModelConfig>,
    #[serde(default)]
    pub current_model: Option<String>,
    #[serde(default)]
    pub system_prompt: Option<String>,
    #[serde(default)]
    pub stream: bool,
}

impl Config {
    pub fn load() -> Result<Self> {
        if let Some(path) = Self::get_path() {
            if path.exists() {
                let content = fs::read_to_string(&path)?;
                Ok(toml::from_str(&content)?)
            } else {
                let config = Config::default();
                config.save()?;
                Ok(config)
            }
        } else {
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

    pub fn add_model(&mut self, name: String, api_key: String, api_url: String, model: String) -> Result<()> {
        let model_config = ModelConfig {
            api_key,
            api_url,
            model,
        };
        self.models.insert(name.clone(), model_config);
        // 如果是第一个模型，设置为当前模型
        if self.current_model.is_none() {
            self.current_model = Some(name.clone());
        }
        self.save()?;
        println!("model added: {}", name.green());
        Ok(())
    }

    pub fn remove_model(&mut self, name: &str) -> Result<()> {
        if self.models.remove(name).is_some() {
            // 如果删除的是当前模型，重置当前模型
            if self.current_model.as_deref() == Some(name) {
                self.current_model = self.models.keys().next().map(|k| k.to_string());
            }
            self.save()?;
            println!("model removed: {}", name.green());
            Ok(())
        } else {
            Err(anyhow::anyhow!("model not found: {}", name))
        }
    }

    pub fn set_current_model(&mut self, name: &str) -> Result<()> {
        if self.models.contains_key(name) {
            self.current_model = Some(name.to_string());
            self.save()?;
            println!("current model set to: {}", name.green());
            Ok(())
        } else {
            Err(anyhow::anyhow!("model not found: {}", name))
        }
    }

    pub fn list_models(&self) {
        if self.models.is_empty() {
            println!("no models configured yet");
            return;
        }

        println!("available models:");
        for (name, config) in &self.models {
            let current = if self.current_model.as_deref() == Some(name) {
                " (current)".yellow()
            } else {
                "".clear()
            };
            println!("- {}{}", name.green(), current);
            println!("  API URL: {}", config.api_url);
            println!("  Model: {}", config.model);
            println!("  API Key: {}", if config.api_key.is_empty() {
                "not set".red()
            } else {
                "set".green()
            });
        }
    }

    pub fn get_current_model(&self) -> Option<(&str, &ModelConfig)> {
        self.current_model.as_ref().and_then(|name| {
            self.models.get(name).map(|config| (name.as_str(), config))
        })
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
        println!("stream output {}", if enabled {
            "enabled".green()
        } else {
            "disabled".yellow()
        });
        Ok(())
    }
} 