use crate::error::CoreError;
use crate::types::{repo::RepoInfo, user::UserInfo};
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
	fn set_token(&mut self, token: &str) -> Result<(), CoreError>;

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
	fn set_proxy(&mut self, proxy: &str) -> Result<(), CoreError>;

	/// 获取用户信息
	async fn get_user_info(&self) -> Result<UserInfo, CoreError>;
	/// 根据用户名获取用户信息
	///
	/// # 参数
	///
	/// * `user_name` - 用户名
	async fn get_user_info_with_name(&self, user_name: &str) -> Result<UserInfo, CoreError>;

	/// 获取仓库信息
	///
	/// # 参数
	///
	/// * `repo_path` - 仓库路径，格式为 `(owner, repo)`
	///
	async fn get_repo_info(&self, repo_path: (&str, &str)) -> Result<RepoInfo, CoreError>;

	/// 获取仓库默认分支
	///
	/// # 参数
	///
	/// * `repo_path` - 仓库路径，格式为 `(owner, repo)`
	///
	async fn get_default_branch(&self, repo_path: (&str, &str)) -> Result<String, CoreError>;
}
