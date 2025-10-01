mod client;
mod common;
mod middleware;

use crate::common::ContributionHtml;
use crate::{
	client::{HTTP_CLIENT, PROXY_URL},
	common::JsonValue,
};
use async_trait::async_trait;
pub use nipaw_core::Client;
use nipaw_core::option::ReposListOptions;
use nipaw_core::{
	CoreError,
	types::{
		repo::RepoInfo,
		user::{ContributionResult, UserInfo},
	},
};
use serde_json::Value;
use std::collections::HashMap;

static API_URL: &str = "https://gitee.com/api/v5";
static BASE_URL: &str = "https://gitee.com";

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

	async fn get_user_info(&self) -> Result<UserInfo, CoreError> {
		if self.token.is_none() {
			return Err(CoreError::TokenEmpty);
		}
		let url = format!("{}/user", API_URL);
		let request = HTTP_CLIENT.get(url).query(&[("access_token", self.token.as_ref().unwrap().as_str())]);

		let resp = request.send().await?;
		let user_info: JsonValue = resp.json().await?;
		Ok(user_info.into())
	}

	async fn get_user_info_with_name(&self, user_name: &str) -> Result<UserInfo, CoreError> {
		let url = format!("{}/users/{}", API_URL, user_name);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			request = request.query(&[("access_token", token.as_str())]);
		}
		let resp = request.send().await?;
		let user_info: JsonValue = resp.json().await?;
		Ok(user_info.into())
	}

	async fn get_user_contribution(&self, user_name: &str) -> Result<ContributionResult, CoreError> {
		let url = format!("{}/{}", BASE_URL, user_name);
		let request = HTTP_CLIENT.get(url).header("X-Requested-With", "XMLHttpRequest").header("Accept", "application/javascript");
		let resp = request.send().await?;
		let html: ContributionHtml = resp.text().await?.into();
		Ok(html.into())
	}

	async fn get_repo_info(&self, repo_path: (&str, &str)) -> Result<RepoInfo, CoreError> {
		let url = format!("{}/repos/{}/{}", API_URL, repo_path.0, repo_path.1);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			request = request.query(&[("access_token", token.as_str())]);
		}
		let resp = request.send().await?;
		let repo_info: JsonValue = resp.json().await?;
		Ok(repo_info.into())
	}

	async fn get_default_branch(&self, repo_path: (&str, &str)) -> Result<String, CoreError> {
		let url = format!("{}/repos/{}/{}", API_URL, repo_path.0, repo_path.1);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			request = request.query(&[("access_token", token.as_str())]);
		}
		let resp = request.send().await?;
		let repo_info: Value = resp.json().await?;
		let default_branch = repo_info.get("default_branch").and_then(|v| v.as_str()).unwrap().to_string();
		Ok(default_branch)
	}

	async fn get_user_repos(&self, option: Option<ReposListOptions>) -> Result<Vec<RepoInfo>, CoreError> {
		let url = format!("{}/user/repos", API_URL);
		let request = HTTP_CLIENT.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = &self.token {
			params.insert("access_token", token.to_owned());
		}

		params.insert("type", "owner".to_string());
		params.insert("sort", "updated".to_string());

		if let Some(option) = option {
			let per_page = option.per_page.unwrap_or_default().min(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or_default();
			params.insert("page", page.to_string());
		}
		let resp = request.query(&params).send().await?;
		let repo_infos: Vec<JsonValue> = resp.json().await?;
		Ok(repo_infos.into_iter().map(|v| v.into()).collect())
	}

	async fn get_user_repos_with_name(&self, user_name: &str, option: Option<ReposListOptions>) -> Result<Vec<RepoInfo>, CoreError> {
		let url = format!("{}/users/{}/repos", API_URL, user_name);
		let request = HTTP_CLIENT.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = &self.token {
			params.insert("access_token", token.to_owned());
		}

		params.insert("type", "owner".to_string());
		params.insert("sort", "pushed".to_string());

		if let Some(option) = option {
			let per_page = option.per_page.unwrap_or_default().min(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or_default();
			params.insert("page", page.to_string());
		}
		let resp = request.query(&params).send().await?;
		let repo_infos: Vec<JsonValue> = resp.json().await?;
		Ok(repo_infos.into_iter().map(|v| v.into()).collect())
	}
}
