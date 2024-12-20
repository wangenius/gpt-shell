use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use futures::{Stream, StreamExt};
use std::pin::Pin;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[async_trait::async_trait]
pub trait LLMProvider: Send + Sync {
    async fn chat(&self, messages: Vec<Message>, stream: bool, running: Arc<AtomicBool>) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>>;
}

pub struct Provider {
    client: reqwest::Client,
    api_key: String,
    api_url: String,
    model: String,
}

impl Provider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
            api_url: "https://api.openai.com/v1/chat/completions".to_string(),
            model: "gpt-3.5-turbo".to_string(),
        }
    }

    pub fn with_url(mut self, url: String) -> Self {
        self.api_url = url;
        self
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    fn is_qwen(&self) -> bool {
        self.api_url.contains("dashscope.aliyuncs.com")
    }

    fn is_deepseek(&self) -> bool {
        self.api_url.contains("api.deepseek.com")
    }
}

#[async_trait::async_trait]
impl LLMProvider for Provider {
    async fn chat(&self, messages: Vec<Message>, stream: bool, running: Arc<AtomicBool>) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>> {
        let request_body = if self.is_qwen() {
            serde_json::json!({
                "model": self.model,
                "input": {
                    "messages": messages
                },
                "parameters": {
                    "stream": stream,
                    "incremental_output": true
                }
            })
        } else {
            serde_json::json!({
                "model": self.model,
                "messages": messages,
                "stream": stream
            })
        };

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        
        if self.is_qwen() {
            headers.insert("Authorization", format!("Bearer {}", self.api_key).parse().unwrap());
            headers.insert("X-DashScope-SSE", "enable".parse().unwrap());
        } else {
            headers.insert("Authorization", format!("Bearer {}", self.api_key).parse().unwrap());
        }

        let api_url = if self.is_qwen() {
            format!("{}/chat/completions", self.api_url.trim_end_matches('/').trim_end_matches("/v1"))
        } else {
            self.api_url.clone()
        };

        let response = self.client
            .post(&api_url)
            .headers(headers)
            .json(&request_body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!(
                "API request failed with status {}: {}",
                status,
                error_text
            ));
        }

        let running = running.clone();
        let is_qwen = self.is_qwen();
        let is_deepseek = self.is_deepseek();
        
        let stream = response
            .bytes_stream()
            .take_while(move |_| {
                let continue_running = running.load(Ordering::SeqCst);
                async move { continue_running }
            })
            .map(move |chunk| -> Result<String> {
                let chunk = chunk.map_err(anyhow::Error::from)?;
                let text = String::from_utf8_lossy(&chunk);
                
                let mut result = String::new();
                for line in text.lines() {
                    if line.starts_with("data: ") {
                        let json_str = &line[6..];
                        if json_str.trim() == "[DONE]" {
                            continue;
                        }
                        if let Ok(response_json) = serde_json::from_str::<serde_json::Value>(json_str) {
                            if let Some(error_msg) = response_json["error_msg"].as_str() {
                                return Err(anyhow::anyhow!("API Error: {}", error_msg));
                            }
                            
                            if is_qwen {
                                if let Some(content) = response_json["output"]["choices"][0]["message"]["content"].as_str() {
                                    result.push_str(content);
                                } else if let Some(content) = response_json["output"]["text"].as_str() {
                                    result.push_str(content);
                                }
                            } else if is_deepseek {
                                if let Some(content) = response_json["choices"][0]["delta"]["content"].as_str() {
                                    result.push_str(content);
                                } else if let Some(content) = response_json["choices"][0]["message"]["content"].as_str() {
                                    result.push_str(content);
                                }
                            } else {
                                if let Some(content) = response_json["choices"][0]["delta"]["content"].as_str() {
                                    result.push_str(content);
                                }
                            }
                        }
                    }
                }
                Ok(result)
            });

        Ok(Box::pin(stream))
    }
}