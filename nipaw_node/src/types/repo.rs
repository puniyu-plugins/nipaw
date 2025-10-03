use chrono::{DateTime, Utc};
use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[napi(object)]
pub struct RepoInfo {
	/// 仓库id
	pub id: String,
	/// 仓库所有者
	pub owner: String,
	/// 仓库名称
	pub name: String,
	/// 仓库全名
	pub full_name: String,
	/// 仓库描述
	pub description: Option<String>,
	/// 仓库可见性
	pub visibility: String,
	/// 是否是fork仓库
	pub fork: bool,
	/// 仓库fork数量
	pub fork_count: u32,
	/// 是否是公开仓库
	pub public: bool,
	/// 是否是私有仓库
	pub private: bool,
	/// 仓库语言
	pub language: Option<String>,
	/// 仓库星标数量
	pub star_count: u32,
	/// 仓库默认分支
	pub default_branch: String,
	/// 仓库创建时间
	pub created_at: DateTime<Utc>,
	/// 仓库更新时间
	pub updated_at: DateTime<Utc>,
	/// 仓库推送时间
	pub pushed_at: DateTime<Utc>,
}

impl From<nipaw_core::types::repo::RepoInfo> for RepoInfo {
	fn from(repo_info: nipaw_core::types::repo::RepoInfo) -> Self {
		RepoInfo {
			id: repo_info.id,
			owner: repo_info.owner,
			name: repo_info.name,
			full_name: repo_info.full_name,
			description: repo_info.description,
			visibility: repo_info.visibility,
			fork: repo_info.fork,
			fork_count: repo_info.fork_count as u32,
			public: repo_info.public,
			private: repo_info.private,
			language: repo_info.language,
			star_count: repo_info.star_count as u32,
			default_branch: repo_info.default_branch,
			created_at: repo_info.created_at,
			updated_at: repo_info.updated_at,
			pushed_at: repo_info.pushed_at,
		}
	}
}
