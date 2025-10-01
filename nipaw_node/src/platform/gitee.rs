use crate::{
	common::RT_RUNTIME,
	option::ReposListOptions,
	types::{
		repo::RepoInfo,
		user::{ContributionResult, UserInfo},
	},
};
use napi::tokio::sync::RwLock;
use napi_derive::napi;
use nipaw_core::Client;
use std::sync::LazyLock;

static GITEE_CLIENT: LazyLock<RwLock<nipaw_gitee::GiteeClient>> = LazyLock::new(|| RwLock::new(nipaw_gitee::GiteeClient::default()));

#[derive(Debug, Default)]
#[napi]
pub struct GiteeClient;

#[napi]
impl GiteeClient {
	#[napi]
	pub fn set_token(&self, token: String) -> napi::Result<()> {
		let rt = RT_RUNTIME.lock().unwrap();
		rt.block_on(async {
			let mut client = GITEE_CLIENT.write().await;
			client.set_token(token.as_str()).map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
			Ok(())
		})
	}

	#[napi]
	pub fn set_proxy(&self, proxy: String) -> napi::Result<()> {
		let rt = RT_RUNTIME.lock().unwrap();
		rt.block_on(async {
			let mut client = GITEE_CLIENT.write().await;
			client.set_proxy(proxy.as_str()).map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
			Ok(())
		})
	}
	#[napi]
	pub async fn get_user_info(&self) -> napi::Result<UserInfo> {
		let client = GITEE_CLIENT.read().await;
		let user_info = client.get_user_info().await.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(user_info.into())
	}

	#[napi]
	pub async fn get_user_info_with_name(&self, name: String) -> napi::Result<UserInfo> {
		let client = GITEE_CLIENT.read().await;
		let user_info = client.get_user_info_with_name(name.as_str()).await.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(user_info.into())
	}

	#[napi]
	pub async fn get_user_contribution(&self, user_name: String) -> napi::Result<ContributionResult> {
		let client = GITEE_CLIENT.read().await;
		let contribution = client.get_user_contribution(user_name.as_str()).await.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(contribution.into())
	}
	#[napi]
	pub async fn get_repo_info(&self, owner: String, repo: String) -> napi::Result<RepoInfo> {
		let client = GITEE_CLIENT.read().await;
		let repo_info = client.get_repo_info((owner.as_str(), repo.as_str())).await.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(repo_info.into())
	}

	#[napi]
	pub async fn get_default_branch(&self, owner: String, repo: String) -> napi::Result<String> {
		let client = GITEE_CLIENT.read().await;
		let default_branch =
			client.get_default_branch((owner.as_str(), repo.as_str())).await.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(default_branch)
	}

	#[napi]
	pub async fn get_user_repos(&self, options: Option<ReposListOptions>) -> napi::Result<Vec<RepoInfo>> {
		let client = GITEE_CLIENT.read().await;
		let repos_list = client.get_user_repos(options.map(|o| o.into())).await.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(repos_list.into_iter().map(|repo| repo.into()).collect())
	}

	#[napi]
	pub async fn get_user_repos_with_name(&self, user_name: String, options: Option<ReposListOptions>) -> napi::Result<Vec<RepoInfo>> {
		let client = GITEE_CLIENT.read().await;
		let repos_list = client
			.get_user_repos_with_name(user_name.as_str(), options.map(|o| o.into()))
			.await
			.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(repos_list.into_iter().map(|repo| repo.into()).collect())
	}
}
