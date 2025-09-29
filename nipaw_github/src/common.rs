use nipaw_core::types::{repo::RepoInfo, user::UserInfo};
use serde_json::Value;

pub fn make_user_info(user_info: Value) -> UserInfo {
	UserInfo {
		id: user_info["id"].as_u64().unwrap().to_string(),
		login: user_info["login"].as_str().unwrap().to_string(),
		name: user_info["name"].as_str().unwrap().to_string(),
		avatar_url: user_info["avatar_url"].as_str().unwrap().to_string(),
		email: user_info["email"].as_str().map(|s| s.to_string()),
		followers: user_info["followers"].as_u64().unwrap(),
		following: user_info["following"].as_u64().unwrap(),
	}
}

pub fn make_repo_info(repo_info: Value) -> RepoInfo {
	RepoInfo {
		id: repo_info["id"].as_u64().unwrap().to_string(),
		owner: repo_info["owner"]["login"].as_str().unwrap().to_string(),
		name: repo_info["name"].as_str().unwrap().to_string(),
		full_name: repo_info["full_name"].as_str().unwrap().to_string(),
		description: repo_info["description"].as_str().map(|s| s.to_string()),
		created_at: repo_info["created_at"].as_str().unwrap().to_string().parse().unwrap(),
		updated_at: repo_info["updated_at"].as_str().unwrap().to_string().parse().unwrap(),
		pushed_at: repo_info["pushed_at"].as_str().unwrap().to_string().parse().unwrap(),
	}
}
