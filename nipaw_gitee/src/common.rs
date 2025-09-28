use nipaw_core::types::user::UserInfo;
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
