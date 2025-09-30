use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[napi(object)]
pub struct UserInfo {
	/// 用户Id
	pub id: String,
	/// 登录用户名
	pub login: String,
	/// 用户昵称
	pub name: String,
	/// 用户邮箱
	pub email: Option<String>,
	/// 用户头像URL
	pub avatar_url: String,
	/// 关注者数量
	pub followers: u32,
	/// 关注的用户数量
	pub following: u32,
}

impl From<nipaw_core::types::user::UserInfo> for UserInfo {
	fn from(user_info: nipaw_core::types::user::UserInfo) -> Self {
		UserInfo {
			id: user_info.id,
			login: user_info.login,
			name: user_info.name,
			avatar_url: user_info.avatar_url,
			email: user_info.email,
			followers: user_info.followers as u32,
			following: user_info.following as u32,
		}
	}
}
