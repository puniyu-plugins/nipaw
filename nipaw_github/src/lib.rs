mod client;
mod common;
mod middleware;

pub use nipaw_core::Client;

use crate::{
	client::{HTTP_CLIENT, PROXY_URL},
	common::{Html, JsonValue},
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

const API_URL: &str = "https://api.github.com";
const BASE_URL: &str = "https://github.com";

#[derive(Debug, Default)]
pub struct GitHubClient {
	pub token: Option<String>,
}

impl GitHubClient {
	pub fn new() -> Self {
		Self::default()
	}
}

#[async_trait]
impl Client for GitHubClient {
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
		let request = HTTP_CLIENT.get(url).bearer_auth(self.token.as_ref().unwrap());
		let resp = request.send().await?;
		let user_info: JsonValue = resp.json().await?;
		Ok(user_info.into())
	}

	async fn get_user_info_with_name(&self, user_name: &str) -> Result<UserInfo> {
		let url = format!("{}/users/{}", API_URL, user_name);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let user_info: JsonValue = resp.json().await?;
		Ok(user_info.into())
	}

	async fn get_user_avatar_url(&self, user_name: &str) -> Result<String> {
		let url = format!("{}/{}", BASE_URL, user_name);
		let request = HTTP_CLIENT.get(url).header("Accept", "image/*");
		let resp = request.send().await?;
		let html: Html = Html::from(resp.text().await?);
		let document = scraper::Html::parse_document(&html.0);

		let selector =
			scraper::Selector::parse("meta[name='octolytics-dimension-user_id']").unwrap();
		let user_id = document
			.select(&selector)
			.next()
			.and_then(|element| element.value().attr("content"))
			.map(|id| id.to_string())
			.unwrap();
		let avatar_url = format!("https://avatars.githubusercontent.com/u/{}?v=4", user_id);
		Ok(avatar_url)
	}

	async fn get_user_contribution(&self, user_name: &str) -> Result<ContributionResult> {
		let mut url = Url::parse(&format!("{}/{}", BASE_URL, user_name))?;
		url.query_pairs_mut()
			.append_pair("action", "show")
			.append_pair("controller", "profiles")
			.append_pair("tab", "contributions")
			.append_pair("user_id", user_name);

		let request = HTTP_CLIENT
			.get(url)
			.header("X-Requested-With", "XMLHttpRequest")
			.header("Accept", "text/html");
		let resp = request.send().await?;
		let html: Html = resp.text().await?.into();
		Ok(html.into())
	}

	async fn get_org_info(&self, org_name: &str) -> Result<OrgInfo> {
		let url = format!("{}/orgs/{}", API_URL, org_name);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			request = request.bearer_auth(token);
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
		let mut params = HashMap::new();
		if let Some(token) = &self.token {
			request = request.bearer_auth(token);
		}
		if let Some(option) = option {
			let per_page = option.per_page.unwrap_or_default().min(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or_default();
			params.insert("page", page.to_string());
		}
		let resp = request.send().await?;
		let repo_infos: Vec<JsonValue> = resp.json().await?;
		Ok(repo_infos.into_iter().map(|v| v.into()).collect())
	}

	async fn get_org_avatar_url(&self, org_name: &str) -> Result<String> {
		let url = format!("{}/orgs/{}", API_URL, org_name);
		let request = HTTP_CLIENT.get(url);
		let resp = request.send().await?;
		let org_html = resp.text().await?;

		let document = scraper::Html::parse_document(&org_html);
		let selector = scraper::Selector::parse("meta[name='hovercard-subject-tag']").unwrap();
		let element = document.select(&selector).next().unwrap();
		let org_id = element.value().attr("content").unwrap();
		let avatar_url = format!("https://avatars.githubusercontent.com/u/{}?v=4", org_id);
		Ok(avatar_url)
	}

	async fn get_repo_info(&self, repo_path: (&str, &str)) -> Result<RepoInfo> {
		let url = format!("{}/repos/{}/{}", API_URL, repo_path.0, repo_path.1);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			request = request.bearer_auth(token);
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
		match use_web_api {
			Some(true) => {
				let url = format!("{}/{}/{}/branches/all", BASE_URL, repo_path.0, repo_path.1);
				let request = HTTP_CLIENT
					.get(url)
					.header("X-Requested-With", "XMLHttpRequest")
					.header("Accept", "application/json");
				let resp = request.send().await?;
				let branches_info: JsonValue = resp.json().await?;
				let default_branch = branches_info
					.0
					.get("payload")
					.and_then(|payload| payload.get("branches"))
					.and_then(|branches| branches.as_array())
					.and_then(|branches| {
						branches.iter().find(|branch| {
							branch.get("isDefault").and_then(|v| v.as_bool()).unwrap_or(false)
						})
					})
					.and_then(|branch| branch.get("name").and_then(|v| v.as_str()))
					.map(|s| s.to_string())
					.unwrap();

				Ok(default_branch)
			}
			Some(false) | None => {
				let url = format!("{}/repos/{}/{}", API_URL, repo_path.0, repo_path.1);
				let mut request = HTTP_CLIENT.get(url);
				if let Some(token) = &self.token {
					request = request.bearer_auth(token);
				}
				let resp = request.send().await?;
				let repo_info: Value = resp.json().await?;
				let default_branch =
					repo_info.get("default_branch").and_then(|v| v.as_str()).unwrap().to_string();
				Ok(default_branch)
			}
		}
	}

	async fn get_user_repos(&self, option: Option<ReposListOptions>) -> Result<Vec<RepoInfo>> {
		let url = format!("{}/user/repos", API_URL);
		let mut request = HTTP_CLIENT.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = &self.token {
			request = request.bearer_auth(token);
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
		let mut request = HTTP_CLIENT.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = &self.token {
			request = request.bearer_auth(token);
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
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let mut commit_info: JsonValue = resp.json().await?;
		let author_avatar_url = commit_info
			.0
			.get("author")
			.and_then(|v| v.get("avatar_url"))
			.and_then(|v| v.as_str())
			.unwrap()
			.to_string();
		let committer_avatar_url = commit_info
			.0
			.get("committer")
			.and_then(|v| v.get("avatar_url"))
			.and_then(|v| v.as_str())
			.unwrap()
			.to_string();
		if let Some(author_obj) = commit_info
			.0
			.get_mut("commit")
			.and_then(|commit| commit.as_object_mut())
			.and_then(|commit_obj| commit_obj.get_mut("author"))
			.and_then(|author| author.as_object_mut())
		{
			author_obj.insert("avatar_url".to_string(), Value::String(author_avatar_url));
		}

		if let Some(committer_obj) = commit_info
			.0
			.get_mut("commit")
			.and_then(|commit| commit.as_object_mut())
			.and_then(|commit_obj| commit_obj.get_mut("committer"))
			.and_then(|committer| committer.as_object_mut())
		{
			committer_obj.insert("avatar_url".to_string(), Value::String(committer_avatar_url));
		}
		Ok(commit_info.into())
	}

	async fn get_commit_infos(
		&self,
		repo_path: (&str, &str),
		option: Option<CommitListOptions>,
	) -> Result<Vec<CommitInfo>> {
		let url = format!("{}/repos/{}/{}/commits", API_URL, repo_path.0, repo_path.1);
		let mut request = HTTP_CLIENT.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = &self.token {
			request = request.bearer_auth(token);
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
