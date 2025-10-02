use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object)]
pub struct OrgInfo {
	/// 组织id
	pub id: u32,
	/// 登录名
	pub login: String,
	/// 组织名
	pub name: Option<String>,
	/// 组织邮箱
	pub email: Option<String>,
	/// 组织头像
	pub avatar_url: String,
	/// 组织描述
	pub description: Option<String>,
	/// 组织关注数
	pub follow_count: u32,
}

impl From<nipaw_core::types::org::OrgInfo> for OrgInfo {
	fn from(org_info: nipaw_core::types::org::OrgInfo) -> Self {
		Self {
			id: org_info.id as u32,
			login: org_info.login,
			name: org_info.name,
			email: org_info.email,
			avatar_url: org_info.avatar_url,
			description: org_info.description,
			follow_count: org_info.follow_count as u32,
		}
	}
}
