mod client;
mod common;
mod middleware;

use crate::{
	client::{HTTP_CLIENT, PROXY_URL},
	common::JsonValue,
};
use async_trait::async_trait;
use chrono::{Datelike, Local};
pub use nipaw_core::Client;
use nipaw_core::types::commit::CommitInfo;
use nipaw_core::types::user::ContributionResult;
use nipaw_core::{
	CoreError,
	option::ReposListOptions,
	types::{repo::RepoInfo, user::UserInfo},
};
use reqwest::Url;
use serde_json::Value;
use std::collections::HashMap;

static BASE_URL: &str = "https://cnb.cool";
static API_URL: &str = "https://api.cnb.cool";

#[derive(Debug, Default)]
pub struct CnbClient {
	pub token: Option<String>,
}

impl CnbClient {
	pub fn new() -> Self {
		Self::default()
	}
}

#[async_trait]
impl Client for CnbClient {
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
		let request = HTTP_CLIENT
			.get(url)
			.header("Authorization", format!("Bearer {}", self.token.as_ref().unwrap()));
		let resp = request.send().await?;
		let mut user_info: JsonValue = resp.json().await?;

		if let Some(username) = user_info.0.get("username").and_then(|v| v.as_str()) {
			let avatar_url = self.get_user_avatar_url(username).await?;
			user_info
				.0
				.as_object_mut()
				.unwrap()
				.insert("avatar_url".to_string(), Value::String(avatar_url));
		}
		Ok(user_info.into())
	}

	async fn get_user_info_with_name(&self, user_name: &str) -> Result<UserInfo, CoreError> {
		let url = format!("{}/users/{}", API_URL, user_name);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			request = request.header("Authorization", format!("Bearer {}", token));
		}
		let resp = request.send().await?;
		let mut user_info: JsonValue = resp.json().await?;

		if let Some(username) = user_info.0.get("username").and_then(|v| v.as_str()) {
			let avatar_url = self.get_user_avatar_url(username).await?;
			user_info
				.0
				.as_object_mut()
				.unwrap()
				.insert("avatar_url".to_string(), Value::String(avatar_url));
		}
		Ok(user_info.into())
	}

	async fn get_user_avatar_url(&self, user_name: &str) -> Result<String, CoreError> {
		let url = format!("{}/users/{}/avatar/l", BASE_URL, user_name);
		let resp = HTTP_CLIENT.get(url).send().await?;
		let avatar_url = resp.url().to_string();
		Ok(avatar_url)
	}

	async fn get_user_contribution(
		&self,
		user_name: &str,
	) -> Result<ContributionResult, CoreError> {
		let mut url = Url::parse(&format!("{}/users/{}/calendar", BASE_URL, user_name))?;
		let year = Local::now().year();
		url.query_pairs_mut().append_pair("year", &year.to_string());
		let resp =
			HTTP_CLIENT.get(url).header("Accept", " application/vnd.cnb.web+json").send().await?;
		let contribution_result: JsonValue = resp.json().await?;
		Ok(contribution_result.into())
	}

	async fn get_repo_info(&self, repo_path: (&str, &str)) -> Result<RepoInfo, CoreError> {
		let url = format!("{}/repos/{}/{}", API_URL, repo_path.0, repo_path.1);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let repo_info: JsonValue = resp.json().await?;
		Ok(repo_info.into())
	}

	async fn get_default_branch(
		&self,
		repo_path: (&str, &str),
		use_token: Option<bool>,
	) -> Result<String, CoreError> {
		match use_token {
			Some(true) => {
				if self.token.is_none() {
					return Err(CoreError::TokenEmpty);
				}
				let url = format!("{}/repos/{}/{}/-/git/head", API_URL, repo_path.0, repo_path.1);
				let mut request = HTTP_CLIENT.get(url);
				if let Some(token) = &self.token {
					request = request.bearer_auth(token);
				}
				let resp = request.send().await?;
				let repo_info: JsonValue = resp.json().await?;
				let default_branch =
					repo_info.0.get("name").and_then(|v| v.as_str()).unwrap().to_string();
				Ok(default_branch)
			}
			Some(false) | None => {
				let url = format!(
					"{}/repos/{}/{}/-/git/overview-branches?limit=5",
					BASE_URL, repo_path.0, repo_path.1
				);
				let request = HTTP_CLIENT.get(url).header("Accept", "application/vnd.cnb.web+json");
				let resp = request.send().await?;
				let repo_info: JsonValue = resp.json().await?;
				let default_branch = repo_info
					.0
					.get("default_branch")
					.and_then(|v| v.get("name"))
					.and_then(|v| v.as_str())
					.map(|s| s.trim_start_matches("refs/heads/"))
					.unwrap()
					.to_string();
				Ok(default_branch)
			}
		}
	}

	async fn get_user_repos(
		&self,
		option: Option<ReposListOptions>,
	) -> Result<Vec<RepoInfo>, CoreError> {
		let url = format!("{}/user/repos", API_URL);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			request = request.bearer_auth(token);
		}
		let mut params: HashMap<&str, String> = HashMap::new();
		params.insert("type", "owner".to_owned());
		params.insert("sort", "pushed".to_owned());
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

	async fn get_user_repos_with_name(
		&self,
		user_name: &str,
		option: Option<ReposListOptions>,
	) -> Result<Vec<RepoInfo>, CoreError> {
		let url = format!("{}/users/{}/repos", API_URL, user_name);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			request = request.bearer_auth(token);
		}
		let mut params: HashMap<&str, String> = HashMap::new();
		params.insert("role", "owner".to_owned());
		params.insert("order_by", "last_updated_at".to_owned());

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

	async fn get_commit_info(
		&self,
		repo_path: (&str, &str),
		sha: Option<&str>,
	) -> Result<CommitInfo, CoreError> {
		let url = format!(
			"{}/{}/{}/-/git/commits/{}",
			API_URL,
			repo_path.0,
			repo_path.1,
			sha.unwrap_or("HEAD")
		);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let mut commit_info: JsonValue = resp.json().await?;
		let author_name = commit_info
			.0
			.get("commit")
			.and_then(|commit| commit.as_object())
			.and_then(|commit_obj| commit_obj.get("author"))
			.and_then(|author| author.as_object())
			.and_then(|author_obj| author_obj.get("name"))
			.and_then(|name| name.as_str())
			.unwrap()
			.to_string();

		let committer_name = commit_info
			.0
			.get("commit")
			.and_then(|commit| commit.as_object())
			.and_then(|commit_obj| commit_obj.get("committer"))
			.and_then(|committer| committer.as_object())
			.and_then(|committer_obj| committer_obj.get("name"))
			.and_then(|name| name.as_str())
			.unwrap()
			.to_string();

		if let Some(author) = commit_info
			.0
			.get_mut("commit")
			.and_then(|commit| commit.as_object_mut())
			.and_then(|commit_obj| commit_obj.get_mut("author"))
			.and_then(|author| author.as_object_mut())
		{
			let avatar_url = self.get_user_avatar_url(author_name.as_str()).await?;
			author.insert("avatar_url".to_string(), Value::String(avatar_url));
		}
		if let Some(committer) = commit_info
			.0
			.get_mut("commit")
			.and_then(|commit| commit.as_object_mut())
			.and_then(|commit_obj| commit_obj.get_mut("committer"))
			.and_then(|committer| committer.as_object_mut())
		{
			let avatar_url = self.get_user_avatar_url(committer_name.as_str()).await?;
			committer.insert("avatar_url".to_string(), Value::String(avatar_url));
		}
		Ok(commit_info.into())
	}
}
