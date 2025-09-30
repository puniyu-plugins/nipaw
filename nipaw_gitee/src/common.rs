use nipaw_core::types::{repo::RepoInfo, user::UserInfo};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub(crate) struct JsonValue(Value);

impl From<JsonValue> for UserInfo {
	fn from(json_value: JsonValue) -> Self {
		let user_info = json_value.0;
		UserInfo {
			id: user_info.get("id").and_then(|v| v.as_u64()).unwrap().to_string(),
			login: user_info.get("login").and_then(|v| v.as_str()).unwrap().to_string(),
			name: user_info.get("name").and_then(|v| v.as_str()).unwrap().to_string(),
			avatar_url: user_info.get("avatar_url").and_then(|v| v.as_str()).unwrap().to_string(),
			email: user_info.get("email").and_then(|v| v.as_str()).map(|s| s.to_string()),
			followers: user_info.get("followers").and_then(|v| v.as_u64()).unwrap(),
			following: user_info.get("following").and_then(|v| v.as_u64()).unwrap(),
		}
	}
}

impl From<JsonValue> for RepoInfo {
	fn from(json_value: JsonValue) -> Self {
		let repo_info = json_value.0;
		RepoInfo {
			id: repo_info.get("id").and_then(|v| v.as_u64()).unwrap().to_string(),
			owner: repo_info
				.get("owner")
				.and_then(|v| v.get("login"))
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string(),
			name: repo_info.get("name").and_then(|v| v.as_str()).unwrap().to_string(),
			full_name: repo_info.get("full_name").and_then(|v| v.as_str()).unwrap().to_string(),
			description: repo_info
				.get("description")
				.and_then(|v| v.as_str())
				.map(|s| s.to_string()),
			created_at: repo_info
				.get("created_at")
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string()
				.parse()
				.unwrap(),
			updated_at: repo_info
				.get("updated_at")
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string()
				.parse()
				.unwrap(),
			pushed_at: repo_info
				.get("pushed_at")
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string()
				.parse()
				.unwrap(),
		}
	}
}
