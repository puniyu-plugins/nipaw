use crate::types::repo::RepoInfo;
use crate::types::user::UserInfo;
use std::sync::{LazyLock, Mutex};

pub fn make_user_info(user_info: nipaw_core::types::user::UserInfo) -> UserInfo {
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

pub fn make_repo_info(repo_info: nipaw_core::types::repo::RepoInfo) -> RepoInfo {
	RepoInfo {
		id: repo_info.id,
		owner: repo_info.owner,
		name: repo_info.name,
		full_name: repo_info.full_name,
		description: repo_info.description,
		created_at: repo_info.created_at,
		updated_at: repo_info.updated_at,
		pushed_at: repo_info.pushed_at,
	}
}

pub(crate) static RT_RUNTIME: LazyLock<Mutex<tokio::runtime::Runtime>> =
	LazyLock::new(|| Mutex::new(tokio::runtime::Runtime::new().unwrap()));
