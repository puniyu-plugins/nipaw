mod client;
mod common;
mod middleware;

use crate::{
	client::{HTTP_CLIENT, PROXY_URL},
	common::JsonValue,
};
use async_trait::async_trait;
pub use nipaw_core::Client;
use nipaw_core::{
	CoreError,
	types::{repo::RepoInfo, user::UserInfo},
};
use serde_json::Value;
use std::collections::HashMap;

static API_URL: &str = "https://api.gitcode.com/api/v5";
static BASE_URL: &str = "https://gitcode.com";
static WEB_API_URL: &str = "https://web-api.gitcode.com";

#[derive(Debug, Default)]
pub struct GitCodeClient {
	pub token: Option<String>,
}

impl GitCodeClient {
	pub fn new() -> Self {
		Self::default()
	}
}

#[async_trait]
impl Client for GitCodeClient {
	fn set_token(&mut self, token: &str) -> Result<(), CoreError> {
		if token.is_empty() {
			return Err(CoreError::TokenEmpty);
		}
		self.token = Some(token.to_string());
		Ok(())
	}

	fn set_proxy(&mut self, proxy: &str) -> Result<(), CoreError> {
		PROXY_URL.set(proxy.to_string()).unwrap();
		Ok(())
	}

	async fn get_user_info(&self) -> Result<UserInfo, CoreError> {
		if self.token.is_none() {
			return Err(CoreError::TokenEmpty);
		}
		let url = format!("{}/user", API_URL);
		let request =
			HTTP_CLIENT.get(url).query(&[("access_token", self.token.as_ref().unwrap().as_str())]);
		let resp = request.send().await?;
		let user_info: JsonValue = resp.json().await?;
		Ok(user_info.into())
	}

	async fn get_user_info_with_name(&self, user_name: &str) -> Result<UserInfo, CoreError> {
		let url = format!("{}/users/{}", API_URL, user_name);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			let mut params = HashMap::new();
			params.insert("access_token", token.as_str());
			request = request.query(&params);
		}
		let resp = request.send().await?;
		let user_info: JsonValue = resp.json().await?;
		Ok(user_info.into())
	}

	async fn get_repo_info(&self, repo_path: (&str, &str)) -> Result<RepoInfo, CoreError> {
		let url = format!("{}/repos/{}/{}", API_URL, repo_path.0, repo_path.1);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			let mut params = HashMap::new();
			params.insert("access_token", token.as_str());
			request = request.query(&params);
		}
		let resp = request.send().await?;
		let repo_info: JsonValue = resp.json().await?;
		Ok(repo_info.into())
	}

	async fn get_default_branch(&self, repo_path: (&str, &str)) -> Result<String, CoreError> {
		let url = format!("{}/repos/{}/{}", API_URL, repo_path.0, repo_path.1);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			let mut params = HashMap::new();
			params.insert("access_token", token.as_str());
			request = request.query(&params);
		}
		let resp = request.send().await?;
		let repo_info: Value = resp.json().await?;
		let default_branch = repo_info["default_branch"].as_str().unwrap().to_string();
		Ok(default_branch)
	}
}

async fn get_user_avatar_url(user_name: &str) -> Result<String, CoreError> {
	let url = format!("{}/uc/api/v1/user/setting/profile?username={}", WEB_API_URL, user_name);
	let resp = HTTP_CLIENT.get(url).header("Referer", BASE_URL).send().await?;
	let user_info: Value = resp.json().await?;
	let avatar_url = user_info.get("avatar").and_then(|v| v.as_str()).unwrap().to_string();
	Ok(avatar_url)
}
