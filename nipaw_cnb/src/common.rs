use crate::BASE_URL;
use nipaw_core::types::user::UserInfo;
use serde_json::Value;

pub fn make_user_info(user_info: Value) -> UserInfo {
	let login = user_info["nickname"].as_str().unwrap();
	UserInfo {
		id: user_info["id"].as_u64().unwrap().to_string(),
		login: login.to_string(),
		name: user_info["nickname"].as_str().unwrap().to_string(),
		avatar_url: format!("{}/users/{}/avatar/l", BASE_URL, login),
		email: user_info["email"].as_str().map(|s| s.to_string()),
		followers: user_info["follower_count"].as_u64().unwrap(),
		following: user_info["follow_count"].as_u64().unwrap(),
	}
}
