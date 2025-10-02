use crate::{
	common::RT_RUNTIME,
	error::Error,
	option::{CommitListOptions, ReposListOptions},
	types::{
		commit::CommitInfo,
		repo::RepoInfo,
		user::{ContributionResult, UserInfo},
	},
};
use napi::tokio::sync::{RwLock, RwLockWriteGuard};
use napi_derive::napi;
use nipaw_core::Client;
use nipaw_gitcode::GitCodeClient as NClient;
use std::sync::LazyLock;

static GITCODE_CLIENT: LazyLock<RwLock<NClient>> =
	LazyLock::new(|| RwLock::new(NClient::default()));

async fn create_client() -> RwLockWriteGuard<'static, NClient> {
	GITCODE_CLIENT.write().await
}

#[derive(Debug, Default)]
#[napi(constructor)]
pub struct GitCodeClient;

#[napi]
impl GitCodeClient {
	/// 设置访问令牌
	///
	/// ## 参数
	/// - `token` 访问令牌
	#[napi]
	pub fn set_token(&self, token: String) -> Result<(), Error> {
		let rt = RT_RUNTIME.lock().unwrap();
		rt.block_on(async {
			let mut client = GITCODE_CLIENT.write().await;
			client.set_token(token.as_str())?;
			Ok(())
		})
	}

	/// 设置代理
	///
	/// ## 参数
	/// - `proxy` 代理地址
	///
	/// 支持http,https,socks5协议
	#[napi]
	pub fn set_proxy(&self, proxy: String) -> Result<(), Error> {
		let rt = RT_RUNTIME.lock().unwrap();
		rt.block_on(async {
			let mut client = GITCODE_CLIENT.write().await;
			client.set_proxy(proxy.as_str())?;
			Ok(())
		})
	}

	/// 获取当前登录用户信息
	#[napi]
	pub async fn get_user_info(&self) -> Result<UserInfo, Error> {
		let client = create_client().await;
		let user_info = client.get_user_info().await?;
		Ok(user_info.into())
	}

	/// 获取指定用户信息
	///
	/// ## 参数
	/// - `user_name` 用户名称
	#[napi]
	pub async fn get_user_info_with_name(&self, name: String) -> Result<UserInfo, Error> {
		let client = create_client().await;
		let user_info = client.get_user_info_with_name(name.as_str()).await?;
		Ok(user_info.into())
	}

	/// 获取指定用户贡献信息
	///
	/// ## 参数
	/// - `user_name` 用户名称
	#[napi]
	pub async fn get_user_contribution(
		&self,
		user_name: String,
	) -> Result<ContributionResult, Error> {
		let client = create_client().await;
		let contribution = client.get_user_contribution(user_name.as_str()).await?;
		Ok(contribution.into())
	}

	/// 获取指定用户头像地址
	///
	/// ## 参数
	/// - `user_name` 用户名称
	#[napi]
	pub async fn get_user_avatar_url(&self, user_name: String) -> Result<String, Error> {
		let client = create_client().await;
		let avatar_url = client.get_user_avatar_url(user_name.as_str()).await?;
		Ok(avatar_url)
	}

	/// 获取仓库信息
	///
	/// ## 参数
	/// - `owner` 仓库所有者
	/// - `repo` 仓库名称
	#[napi]
	pub async fn get_repo_info(&self, owner: String, repo: String) -> Result<RepoInfo, Error> {
		let client = create_client().await;
		let repo_info = client.get_repo_info((owner.as_str(), repo.as_str())).await?;
		Ok(repo_info.into())
	}

	/// 获取仓库默认分支
	///
	/// ## 参数
	/// - `owner` 仓库所有者
	/// - `repo` 仓库名称
	/// - `use_token` 是否使用令牌, 默认为 `false`
	///
	/// 当设置为 `true` 时, 会使用OPENAPI获取, 否则使用WEB API获取
	#[napi]
	pub async fn get_repo_default_branch(
		&self,
		owner: String,
		repo: String,
		use_token: Option<bool>,
	) -> Result<String, Error> {
		let client = create_client().await;
		let default_branch =
			client.get_repo_default_branch((owner.as_str(), repo.as_str()), use_token).await?;
		Ok(default_branch)
	}

	/// 获取指定用户仓库列表
	///
	/// ## 参数
	/// - `user_name` 用户名称
	/// - `option` 仓库列表选项
	///
	#[napi]
	pub async fn get_user_repos(
		&self,
		option: Option<ReposListOptions>,
	) -> Result<Vec<RepoInfo>, Error> {
		let client = create_client().await;
		let repo_infos = client.get_user_repos(option.map(|o| o.into())).await?;
		Ok(repo_infos.into_iter().map(|v| v.into()).collect())
	}

	/// 获取指定用户仓库列表
	///
	/// ## 参数
	/// - `user_name` 用户名称
	/// - `option` 仓库列表选项
	///
	#[napi]
	pub async fn get_user_repos_with_name(
		&self,
		user_name: String,
		option: Option<ReposListOptions>,
	) -> Result<Vec<RepoInfo>, Error> {
		let client = create_client().await;
		let repo_infos =
			client.get_user_repos_with_name(user_name.as_str(), option.map(|o| o.into())).await?;
		Ok(repo_infos.into_iter().map(|v| v.into()).collect())
	}

	/// 获取仓库提交信息
	///
	/// ## 参数
	/// - `owner` 仓库所有者
	/// - `repo` 仓库名称
	/// - `sha` 提交SHA, 如果不设置则会获取默认分支的最新提交
	#[napi]
	pub async fn get_commit_info(
		&self,
		owner: String,
		repo: String,
		sha: String,
	) -> Result<CommitInfo, Error> {
		let client = create_client().await;
		let commit_info =
			client.get_commit_info((owner.as_str(), repo.as_str()), Some(sha.as_str())).await?;
		Ok(commit_info.into())
	}

	/// 获取仓库提交列表
	///
	/// ## 参数
	/// - `owner` 仓库所有者
	/// - `repo` 仓库名称
	/// - `option` 提交列表选项
	#[napi]
	pub async fn get_commit_infos(
		&self,
		owner: String,
		repo: String,
		option: Option<CommitListOptions>,
	) -> Result<Vec<CommitInfo>, Error> {
		let client = create_client().await;
		let commit_infos = client
			.get_commit_infos((owner.as_str(), repo.as_str()), option.map(|o| o.into()))
			.await?;
		Ok(commit_infos.into_iter().map(|v| v.into()).collect())
	}
}
