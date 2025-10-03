use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
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
	pub fork_count: u64,
	/// 是否是公开仓库
	pub public: bool,
	/// 是否是私有仓库
	pub private: bool,
	/// 仓库语言
	pub language: Option<String>,
	/// 仓库星标数量
	pub star_count: u64,
	/// 仓库默认分支
	pub default_branch: String,
	/// 仓库创建时间
	pub created_at: DateTime<Utc>,
	/// 仓库更新时间
	pub updated_at: DateTime<Utc>,
	/// 仓库推送时间
	pub pushed_at: DateTime<Utc>,
}
