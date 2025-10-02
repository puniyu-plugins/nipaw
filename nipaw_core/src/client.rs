use crate::{
	error::Error,
	option::{CommitListOptions, ReposListOptions},
	types::{
		commit::CommitInfo,
		repo::RepoInfo,
		user::{ContributionResult, UserInfo},
	},
};
use async_trait::async_trait;

#[async_trait]
pub trait Client: Send + Sync {
	/// 设置token
	///
	/// # 参数
	///
	/// * `token` - token
	///
	/// # 示例
	///
	/// ```ignore
	/// client.set_token("token").unwrap();
	/// ```
	fn set_token(&mut self, token: &str) -> Result<(), Error>;

	/// 设置代理
	///
	/// # 参数
	///
	/// * `proxy` - 代理字符串
	///
	/// # 示例
	///
	/// ```ignore
	/// client.set_proxy("http://127.0.0.1:7890").unwrap();
	/// ```
	fn set_proxy(&mut self, proxy: &str) -> Result<(), Error>;

	/// 获取当前授权用户信息
	async fn get_user_info(&self) -> Result<UserInfo, Error>;

	/// 根据用户名获取用户信息
	///
	/// # 参数
	///
	/// * `user_name` - 用户名
	async fn get_user_info_with_name(&self, user_name: &str) -> Result<UserInfo, Error>;

	/// 获取用户头像URL
	///
	/// # 参数
	///
	/// * `user_name` - 用户名
	async fn get_user_avatar_url(&self, user_name: &str) -> Result<String, Error>;

	/// 获取指定用户贡献数据
	async fn get_user_contribution(&self, user_name: &str) -> Result<ContributionResult, Error>;

	/// 获取仓库信息
	///
	/// # 参数
	///
	/// * `repo_path` - 仓库路径，格式为 `(owner, repo)`
	///
	async fn get_repo_info(&self, repo_path: (&str, &str)) -> Result<RepoInfo, Error>;

	/// 获取仓库默认分支
	///
	/// # 参数
	///
	/// * `repo_path` - 仓库路径，格式为 `(owner, repo)`
	/// * `use_token` - 是否使用token获取仓库默认分支, 默认为 `false`
	///
	/// ## 说明
	/// * 当 `use_token` 为 `true` 时, 会走OPENAPI获取仓库默认分支, 否则走WEBAPI获取仓库默认分支
	///
	async fn get_repo_default_branch(
		&self,
		repo_path: (&str, &str),
		use_token: Option<bool>,
	) -> Result<String, Error>;

	/// 获取用户仓库信息列表
	///
	/// # 参数
	///
	/// * `option` - 获取仓库列表选项, 详见 [ReposListOptions]
	///
	async fn get_user_repos(
		&self,
		option: Option<ReposListOptions>,
	) -> Result<Vec<RepoInfo>, Error>;

	/// 根据用户名获取用户仓库信息列表
	///
	/// # 参数
	///
	/// * `user_name` - 用户名
	/// * `option` - 获取仓库列表选项, 详见 [ReposListOptions]
	async fn get_user_repos_with_name(
		&self,
		user_name: &str,
		option: Option<ReposListOptions>,
	) -> Result<Vec<RepoInfo>, Error>;

	/// 获取仓库提交信息
	///
	/// # 参数
	///
	/// * `repo_path` - 仓库路径，格式为 `(owner, repo)`
	/// * `sha` - 提交ID, 默认为最新提交
	///
	async fn get_commit_info(
		&self,
		repo_path: (&str, &str),
		sha: Option<&str>,
	) -> Result<CommitInfo, Error>;

	/// 获取仓库所有提交信息
	///
	/// # 参数
	/// * `repo_path` - 仓库路径，格式为 `(owner, repo)`
	/// * `option` - 获取提交列表选项, 详见 [CommitListOptions]
	async fn get_commit_infos(
		&self,
		repo_path: (&str, &str),
		option: Option<CommitListOptions>,
	) -> Result<Vec<CommitInfo>, Error>;
}
