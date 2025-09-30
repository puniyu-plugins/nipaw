use crate::BASE_URL;
use nipaw_core::types::{repo::RepoInfo, user::UserInfo};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct JsonValue(Value);
impl From<JsonValue> for UserInfo {
	fn from(json_value: JsonValue) -> Self {
		let user_info = json_value.0;
		let login = user_info.get("nickname").and_then(|v| v.as_str()).unwrap();
		UserInfo {
			id: user_info.get("id").and_then(|v| v.as_str()).unwrap().to_string(),
			login: login.to_string(),
			name: user_info.get("nickname").and_then(|v| v.as_str()).unwrap().to_string(),
			avatar_url: format!("{}/users/{}/avatar/l", BASE_URL, login),
			email: user_info.get("email").and_then(|v| v.as_str()).map(|s| s.to_string()),
			followers: user_info.get("follower_count").and_then(|v| v.as_u64()).unwrap(),
			following: user_info.get("follow_count").and_then(|v| v.as_u64()).unwrap(),
		}
	}
}

impl From<JsonValue> for RepoInfo {
	fn from(json_value: JsonValue) -> Self {
		let repo_info = json_value.0;
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
}
