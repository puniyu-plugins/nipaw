use crate::BASE_URL;
use nipaw_core::types::user::UserInfo;
use serde_json::Value;
use nipaw_core::types::repo::RepoInfo;

pub fn make_user_info(user_info: Value) -> UserInfo {
	let login = user_info["nickname"].as_str().unwrap();
	UserInfo {
		id: user_info["id"].as_str().unwrap().to_string(),
		login: login.to_string(),
		name: user_info["nickname"].as_str().unwrap().to_string(),
		avatar_url: format!("{}/users/{}/avatar/l", BASE_URL, login),
		email: user_info["email"].as_str().map(|s| s.to_string()),
		followers: user_info["follower_count"].as_u64().unwrap(),
		following: user_info["follow_count"].as_u64().unwrap(),
	}
}
pub fn make_repo_info(repo_info: Value) -> RepoInfo {
	RepoInfo {
		id: repo_info["id"].as_str().unwrap().to_string(),
		owner: repo_info["owner"]["login"].as_str().unwrap().to_string(),
		name: repo_info["name"].as_str().unwrap().to_string(),
		full_name: repo_info["full_name"].as_str().unwrap().to_string(),
		description: repo_info["description"].as_str().map(|s| s.to_string()),
		created_at: repo_info["created_at"].as_str().unwrap().to_string().parse().unwrap(),
		updated_at: repo_info["updated_at"].as_str().unwrap().to_string().parse().unwrap(),
		pushed_at: repo_info["updated_at"].as_str().unwrap().to_string().parse().unwrap(),
	}
}
