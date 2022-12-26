use anyhow::Ok;
use reqwest::{Method, header::HeaderMap};
use url::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffConfig {
    #[serde(flatten)]
    pub profiles: HashMap<String, DiffProfile>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiffProfile {
    pub req1: RequestProfile,
    pub req2: RequestProfile,
    pub res: ResponseProfile,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestProfile {
    #[serde(with = "http_serde::method", default)]
    pub method: Method,
    pub url: Url,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub params: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "HeaderMap::is_empty", with = "http_serde::header_map", default)]
    pub headers: HeaderMap,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseProfile {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_headers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_body: Vec<String>,
}

impl DiffConfig {
    pub async fn load_yaml(path: &str) -> Result<Self>
    {
        let content = fs::read_to_string(path).await?;
        Self::from_yaml(&content)
    }

    pub fn from_yaml(content: &str) -> anyhow::Result<Self>
    {
        Ok(serde_yaml::from_str(content)?)
    }

    pub fn get_profile(&self, name: &str) -> Option<&DiffProfile>
    {
        self.profiles.get(name)
    }
}