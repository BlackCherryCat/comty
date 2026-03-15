// src/context.rs
use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Context {
    pub sender: Option<String>,
    pub receiver: Option<String>,
    pub timestamp: Option<String>,
    pub content: Option<String>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            sender: None,
            receiver: None,
            timestamp: None,
            content: None,
        }
    }

    pub fn parse(content: &str) -> Self {
        // Implement parsing logic here
        Context {
            sender: None,
            receiver: None,
            timestamp: None,
            content: Some(content.to_string()),
        }
    }
}
