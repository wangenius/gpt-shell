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

pub struct OpenAIProvider {
    client: reqwest::Client,
    api_key: String,
    api_url: String,
    model: String,
}

impl OpenAIProvider {
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
}

#[async_trait::async_trait]
impl LLMProvider for OpenAIProvider {
    async fn chat(&self, messages: Vec<Message>, stream: bool, running: Arc<AtomicBool>) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>> {
        let request_body = serde_json::json!({
            "model": self.model,
            "messages": messages,
            "stream": stream
        });

        let response = self.client
            .post(&self.api_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let running = running.clone();
        let stream = response
            .bytes_stream()
            .take_while(move |_| {
                let continue_running = running.load(Ordering::SeqCst);
                async move { continue_running }
            })
            .map(|chunk| -> Result<String> {
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
                            if let Some(content) = response_json["choices"][0]["delta"]["content"].as_str() {
                                result.push_str(content);
                            }
                        }
                    }
                }
                Ok(result)
            });

        Ok(Box::pin(stream))
    }
} 