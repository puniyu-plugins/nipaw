use chrono::{NaiveDate, Utc, Weekday};
use itertools::Itertools;
use nipaw_core::types::user::ContributionData;
use nipaw_core::types::{
	repo::RepoInfo,
	user::{ContributionResult, UserInfo},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub(crate) struct JsonValue(pub(crate) Value);

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

impl From<JsonValue> for ContributionResult {
	fn from(value: JsonValue) -> Self {
		let contribution_result = value.0;

		let contributions: Vec<Vec<ContributionData>> = contribution_result
			.as_object()
			.unwrap()
			.iter()
			.map(|(date, count)| ContributionData {
				date: NaiveDate::parse_from_str(date, "%Y-%m-%d")
					.map(|nd| nd.and_hms_opt(0, 0, 0).unwrap().and_local_timezone(Utc).unwrap())
					.unwrap(),
				count: count.as_u64().unwrap() as u32,
			})
			.sorted_by_key(|c| c.date)
			.chunk_by(|c| {
				let naive_date = c.date.naive_utc().date();
				naive_date.week(Weekday::Mon)
			})
			.into_iter()
			.map(|(_, week_data)| week_data.collect::<Vec<_>>())
			.collect();

		let total = contributions.iter().flatten().map(|c| c.count).sum();

		ContributionResult { contributions, total }
	}
}
