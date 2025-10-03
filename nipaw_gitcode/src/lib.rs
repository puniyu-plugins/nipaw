mod client;
mod common;
mod middleware;

pub use nipaw_core::Client;

use crate::{
	client::{HTTP_CLIENT, PROXY_URL},
	common::JsonValue,
};
use async_trait::async_trait;
use nipaw_core::option::OrgRepoListOptions;
use nipaw_core::types::org::OrgInfo;
use nipaw_core::{
	Result,
	error::Error,
	option::{CommitListOptions, ReposListOptions},
	types::{
		commit::CommitInfo,
		repo::RepoInfo,
		user::{ContributionResult, UserInfo},
	},
};
use reqwest::Url;
use serde_json::Value;
use std::collections::HashMap;

const API_URL: &str = "https://api.gitcode.com/api/v5";
const BASE_URL: &str = "https://gitcode.com";
const WEB_API_URL: &str = "https://web-api.gitcode.com";

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
	fn set_token(&mut self, token: &str) -> Result<()> {
		if token.is_empty() {
			return Err(Error::TokenEmpty);
		}
		self.token = Some(token.to_string());
		Ok(())
	}

	fn set_proxy(&mut self, proxy: &str) -> Result<()> {
		PROXY_URL.set(proxy.to_string()).unwrap();
		Ok(())
	}

	async fn get_user_info(&self) -> Result<UserInfo> {
		if self.token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/user", API_URL);
		let request =
			HTTP_CLIENT.get(url).query(&[("access_token", self.token.as_ref().unwrap().as_str())]);
		let resp = request.send().await?;
		let mut user_info: JsonValue = resp.json().await?;
		if let Some(user) = user_info.0.as_object_mut() {
			let user_name = user.get("username").and_then(|v| v.as_str()).unwrap();
			let repo_count = get_user_repo_count(user_name).await?;
			user.insert("repo_count".to_string(), Value::Number(repo_count.into()));
		}
		Ok(user_info.into())
	}

	async fn get_user_info_with_name(&self, user_name: &str) -> Result<UserInfo> {
		let url = format!("{}/users/{}", API_URL, user_name);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			request = request.query(&[("access_token", token.as_str())]);
		}
		let resp = request.send().await?;
		let mut user_info: JsonValue = resp.json().await?;
		if let Some(user) = user_info.0.as_object_mut() {
			let repo_count = get_user_repo_count(user_name).await?;
			user.insert("repo_count".to_string(), Value::Number(repo_count.into()));
		}
		Ok(user_info.into())
	}

	async fn get_user_avatar_url(&self, user_name: &str) -> Result<String> {
		let url = format!("{}/uc/api/v1/user/setting/profile?username={}", WEB_API_URL, user_name);
		let resp = HTTP_CLIENT.get(url).header("Referer", BASE_URL).send().await?;
		let user_info: Value = resp.json().await?;
		let avatar_url = user_info.get("avatar").and_then(|v| v.as_str()).unwrap().to_string();
		Ok(avatar_url)
	}

	async fn get_user_contribution(&self, user_name: &str) -> Result<ContributionResult> {
		let mut url =
			Url::parse(&format!("{}/uc/api/v1/events/{}/contributions", WEB_API_URL, user_name))?;
		url.query_pairs_mut().append_pair("username", user_name);
		let request = HTTP_CLIENT.get(url);
		let resp = request.header("Referer", BASE_URL).send().await?;
		let contribution_result: JsonValue = resp.json().await?;
		Ok(contribution_result.into())
	}

	async fn get_org_info(&self, org_name: &str) -> Result<OrgInfo> {
		let url = format!("{}/orgs/{}", WEB_API_URL, org_name);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			request = request.query(&[("access_token", token.as_str())]);
		}
		let resp = request.send().await?;
		let org_info: JsonValue = resp.json().await?;
		Ok(org_info.into())
	}

	async fn get_org_repos(
		&self,
		org_name: &str,
		option: Option<OrgRepoListOptions>,
	) -> Result<Vec<RepoInfo>> {
		let url = format!("{}/orgs/{}/repos", API_URL, org_name);
		let mut request = HTTP_CLIENT.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = &self.token {
			request = request.query(&[("access_token", token.as_str())]);
		}
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

	async fn get_org_avatar_url(&self, org_name: &str) -> Result<String> {
		let url = format!("{}/api/v2/groups/{}", WEB_API_URL, org_name);
		let resp = HTTP_CLIENT.get(url).header("Referer", BASE_URL).send().await?;
		let org_info: Value = resp.json().await?;
		let avatar_url = org_info.get("avatar").and_then(|v| v.as_str()).unwrap().to_string();
		Ok(avatar_url)
	}

	async fn get_repo_info(&self, repo_path: (&str, &str)) -> Result<RepoInfo> {
		let url = format!("{}/repos/{}/{}", API_URL, repo_path.0, repo_path.1);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			request = request.query(&[("access_token", token.as_str())]);
		}
		let resp = request.send().await?;
		let repo_info: JsonValue = resp.json().await?;
		Ok(repo_info.into())
	}

	async fn get_repo_default_branch(
		&self,
		repo_path: (&str, &str),
		use_web_api: Option<bool>,
	) -> Result<String> {
		let repo_info = match use_web_api {
			Some(true) => {
				let url =
					format!("{}/api/v2/projects/{}%2F{}", WEB_API_URL, repo_path.0, repo_path.1);
				let request = HTTP_CLIENT.get(url).header("Referer", BASE_URL);
				let resp = request.send().await?;
				let repo_info: JsonValue = resp.json().await?;
				repo_info.0
			}
			Some(false) | None => {
				let url = format!("{}/repos/{}/{}", API_URL, repo_path.0, repo_path.1);
				let mut request = HTTP_CLIENT.get(url);
				if let Some(token) = &self.token {
					request = request.query(&[("access_token", token.as_str())]);
				}
				let resp = request.send().await?;
				let repo_info: JsonValue = resp.json().await?;
				repo_info.0
			}
		};
		let default_branch =
			repo_info.get("default_branch").and_then(|v| v.as_str()).unwrap().to_string();
		Ok(default_branch)
	}

	async fn get_user_repos(&self, option: Option<ReposListOptions>) -> Result<Vec<RepoInfo>> {
		let url = format!("{}/user/repos", API_URL);
		let request = HTTP_CLIENT.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();

		if let Some(token) = &self.token {
			params.insert("access_token", token.to_owned());
		}

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

	async fn get_user_repos_with_name(
		&self,
		user_name: &str,
		option: Option<ReposListOptions>,
	) -> Result<Vec<RepoInfo>> {
		let url = format!("{}/users/{}/repos", API_URL, user_name);
		let request = HTTP_CLIENT.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();

		if let Some(token) = &self.token {
			params.insert("access_token", token.to_owned());
		}

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

	async fn get_commit_info(
		&self,
		repo_path: (&str, &str),
		sha: Option<&str>,
	) -> Result<CommitInfo> {
		let url = format!(
			"{}/repos/{}/{}/commits/{}",
			API_URL,
			repo_path.0,
			repo_path.1,
			sha.unwrap_or("HEAD")
		);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			request = request.query(&[("access_token", token.as_str())]);
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

	async fn get_commit_infos(
		&self,
		repo_path: (&str, &str),
		option: Option<CommitListOptions>,
	) -> Result<Vec<CommitInfo>> {
		let url = format!("{}/repos/{}/{}/commits", API_URL, repo_path.0, repo_path.1);
		let request = HTTP_CLIENT.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = &self.token {
			params.insert("access_token", token.to_owned());
		}

		if let Some(option) = option {
			let per_page = option.per_page.unwrap_or_default().min(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or_default();
			params.insert("page", page.to_string());
			if let Some(sha) = option.sha {
				params.insert("sha", sha.to_string());
			}
			if let Some(author) = option.author {
				params.insert("author", author.to_string());
			}
			if let Some(since) = option.since {
				params.insert("since", since.to_rfc3339());
			}
			if let Some(until) = option.until {
				params.insert("until", until.to_rfc3339());
			}
		}
		let resp = request.query(&params).send().await?;
		let commit_infos: Vec<JsonValue> = resp.json().await?;
		Ok(commit_infos.into_iter().map(|v| v.into()).collect())
	}
}

async fn get_user_repo_count(user_name: &str) -> Result<u64> {
	let mut url =
		Url::parse(format!("{}/api/v2/projects/profile/{}", WEB_API_URL, user_name).as_str())?;
	url.query_pairs_mut().append_pair("repo_query_type", "created");
	let request = HTTP_CLIENT.get(url).header("Referer", BASE_URL);
	let resp = request.send().await?;
	let repo_info: JsonValue = resp.json().await?;
	let repo_count = repo_info.0.get("total").and_then(|total| total.as_u64()).unwrap_or(0);
	Ok(repo_count)
}
