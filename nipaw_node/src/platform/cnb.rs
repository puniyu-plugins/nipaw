use crate::{
	common::RT_RUNTIME,
	option::ReposListOptions,
	types::{
		commit::CommitInfo,
		repo::RepoInfo,
		user::{ContributionResult, UserInfo},
	},
};
use napi::tokio::sync::{RwLock, RwLockWriteGuard};
use napi_derive::napi;
use nipaw_cnb::CnbClient as NClient;
use nipaw_core::Client;
use std::sync::LazyLock;

static CNB_CLIENT: LazyLock<RwLock<NClient>> = LazyLock::new(|| RwLock::new(NClient::default()));

async fn create_client() -> RwLockWriteGuard<'static, NClient> {
	CNB_CLIENT.write().await
}
#[derive(Debug, Default)]
#[napi(constructor)]
pub struct CnbClient;

#[napi]
impl CnbClient {
	#[napi]
	pub fn set_token(&self, token: String) -> napi::Result<()> {
		let rt = RT_RUNTIME.lock().unwrap();
		rt.block_on(async {
			let mut client = CNB_CLIENT.write().await;
			client
				.set_token(token.as_str())
				.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
			Ok(())
		})
	}

	#[napi]
	pub fn set_proxy(&self, proxy: String) -> napi::Result<()> {
		let rt = RT_RUNTIME.lock().unwrap();
		rt.block_on(async {
			let mut client = CNB_CLIENT.write().await;
			client
				.set_proxy(proxy.as_str())
				.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
			Ok(())
		})
	}

	#[napi]
	pub async fn get_user_info(&self) -> napi::Result<UserInfo> {
		let client = create_client().await;
		let user_info = client
			.get_user_info()
			.await
			.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(user_info.into())
	}
	#[napi]
	pub async fn get_user_info_with_name(&self, user_name: String) -> napi::Result<UserInfo> {
		let client = create_client().await;
		let user_info = client
			.get_user_info_with_name(user_name.as_str())
			.await
			.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(user_info.into())
	}

	#[napi]
	pub async fn get_user_contribution(
		&self,
		user_name: String,
	) -> napi::Result<ContributionResult> {
		let client = create_client().await;
		let contribution = client
			.get_user_contribution(user_name.as_str())
			.await
			.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(contribution.into())
	}

	#[napi]
	pub async fn get_user_avatar_url(&self, user_name: String) -> napi::Result<String> {
		let client = create_client().await;
		let avatar_url = client
			.get_user_avatar_url(user_name.as_str())
			.await
			.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(avatar_url)
	}

	#[napi]
	pub async fn get_repo_info(&self, owner: String, repo: String) -> napi::Result<RepoInfo> {
		let client = create_client().await;
		let repo_info = client
			.get_repo_info((owner.as_str(), repo.as_str()))
			.await
			.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(repo_info.into())
	}

	#[napi]
	pub async fn get_default_branch(
		&self,
		owner: String,
		repo: String,
		use_token: Option<bool>,
	) -> napi::Result<String> {
		let client = create_client().await;
		let default_branch = client
			.get_default_branch((owner.as_str(), repo.as_str()), use_token)
			.await
			.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(default_branch)
	}

	#[napi]
	pub async fn get_user_repos(
		&self,
		option: Option<ReposListOptions>,
	) -> napi::Result<Vec<RepoInfo>> {
		let client = create_client().await;
		let repo_infos = client
			.get_user_repos(option.map(|o| o.into()))
			.await
			.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(repo_infos.into_iter().map(|v| v.into()).collect())
	}

	#[napi]
	pub async fn get_user_repos_with_name(
		&self,
		user_name: String,
		option: Option<ReposListOptions>,
	) -> napi::Result<Vec<RepoInfo>> {
		let client = create_client().await;
		let repo_infos = client
			.get_user_repos_with_name(user_name.as_str(), option.map(|o| o.into()))
			.await
			.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(repo_infos.into_iter().map(|v| v.into()).collect())
	}

	#[napi]
	pub async fn get_commit(
		&self,
		owner: String,
		repo: String,
		sha: String,
	) -> napi::Result<CommitInfo> {
		let client = create_client().await;
		let commit_info = client
			.get_commit_info((owner.as_str(), repo.as_str()), Some(sha.as_str()))
			.await
			.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(commit_info.into())
	}
}
