mod client;
mod common;
mod middleware;

use crate::client::{HTTP_CLIENT, PROXY_URL};
use crate::common::{make_repo_info, make_user_info};
use async_trait::async_trait;
pub use nipaw_core::Client;
use nipaw_core::CoreError;
use nipaw_core::types::repo::RepoInfo;
use nipaw_core::types::user::UserInfo;
use serde_json::Value;
use std::collections::HashMap;

static API_URL: &str = "https://gitee.com/api/v5";
#[derive(Debug, Default)]
pub struct GiteeClient {
	pub token: Option<String>,
}

impl GiteeClient {
	pub fn new() -> Self {
		Self::default()
	}
}

#[async_trait]
impl Client for GiteeClient {
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

	#[inline]
	async fn get_user_info(&self) -> Result<UserInfo, CoreError> {
		if self.token.is_none() {
			return Err(CoreError::TokenEmpty);
		}
		let url = format!("{}/user", API_URL);
		let request =
			HTTP_CLIENT.get(url).query(&[("access_token", self.token.as_ref().unwrap().as_str())]);

		let resp = request.send().await?;
		let user_info: Value = resp.json().await?;
		Ok(make_user_info(user_info))
	}

	#[inline]
	async fn get_user_info_with_name(&self, user_name: &str) -> Result<UserInfo, CoreError> {
		let url = format!("{}/users/{}", API_URL, user_name);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			let mut params = HashMap::new();
			params.insert("access_token", token.as_str());
			request = request.query(&params);
		}
		let resp = request.send().await?;
		let user_info: Value = resp.json().await?;
		Ok(make_user_info(user_info))
	}

	#[inline]
	async fn get_repo_info(&self, repo_path: (&str, &str)) -> Result<RepoInfo, CoreError> {
		let url = format!("{}/repos/{}/{}", API_URL, repo_path.0, repo_path.1);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			let mut params = HashMap::new();
			params.insert("access_token", token.as_str());
			request = request.query(&params);
		}
		let resp = request.send().await?;
		let repo_info: Value = resp.json().await?;
		Ok(make_repo_info(repo_info))
	}
}
