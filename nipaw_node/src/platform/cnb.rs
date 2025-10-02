use crate::{
	common::RT_RUNTIME,
	option::{CommitListOptions, ReposListOptions},
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
	/// 设置访问令牌
	///
	/// ## 参数
	/// - `token` 访问令牌
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

	/// 设置代理
	///
	/// ## 参数
	/// - `proxy` 代理地址
	///
	/// 支持http,https,socks5协议
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

	/// 获取当前登录用户信息
	#[napi]
	pub async fn get_user_info(&self) -> napi::Result<UserInfo> {
		let client = create_client().await;
		let user_info = client
			.get_user_info()
			.await
			.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(user_info.into())
	}

	/// 获取指定用户信息
	///
	/// ## 参数
	/// - `user_name` 用户名称
	#[napi]
	pub async fn get_user_info_with_name(&self, user_name: String) -> napi::Result<UserInfo> {
		let client = create_client().await;
		let user_info = client
			.get_user_info_with_name(user_name.as_str())
			.await
			.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
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
	) -> napi::Result<ContributionResult> {
		let client = create_client().await;
		let contribution = client
			.get_user_contribution(user_name.as_str())
			.await
			.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(contribution.into())
	}

	/// 获取指定用户头像地址
	///
	/// ## 参数
	/// - `user_name` 用户名称
	#[napi]
	pub async fn get_user_avatar_url(&self, user_name: String) -> napi::Result<String> {
		let client = create_client().await;
		let avatar_url = client
			.get_user_avatar_url(user_name.as_str())
			.await
			.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(avatar_url)
	}

	/// 获取仓库信息
	///
	/// ## 参数
	/// - `owner` 仓库所有者
	/// - `repo` 仓库名称
	#[napi]
	pub async fn get_repo_info(&self, owner: String, repo: String) -> napi::Result<RepoInfo> {
		let client = create_client().await;
		let repo_info = client
			.get_repo_info((owner.as_str(), repo.as_str()))
			.await
			.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
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
	) -> napi::Result<String> {
		let client = create_client().await;
		let default_branch = client
			.get_repo_default_branch((owner.as_str(), repo.as_str()), use_token)
			.await
			.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(default_branch)
	}

	/// 获取当前用户仓库列表
	///
	/// ## 参数
	/// - `option` 仓库列表选项
	///
	/// 如果不设置令牌则会出错
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
	) -> napi::Result<Vec<RepoInfo>> {
		let client = create_client().await;
		let repo_infos = client
			.get_user_repos_with_name(user_name.as_str(), option.map(|o| o.into()))
			.await
			.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
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
		sha: Option<String>,
	) -> napi::Result<CommitInfo> {
		let client = create_client().await;
		let commit_info = client
			.get_commit_info((owner.as_str(), repo.as_str()), sha.as_deref())
			.await
			.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
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
	) -> napi::Result<Vec<CommitInfo>> {
		let client = create_client().await;
		let commit_infos = client
			.get_commit_infos((owner.as_str(), repo.as_str()), option.map(|o| o.into()))
			.await
			.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(commit_infos.into_iter().map(|v| v.into()).collect())
	}
}
