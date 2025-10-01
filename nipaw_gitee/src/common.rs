use chrono::{NaiveDate, Utc, Weekday};
use itertools::Itertools;
use nipaw_core::types::{
	repo::RepoInfo,
	user::{ContributionData, ContributionResult, UserInfo},
};
use scraper::Selector;
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
			owner: repo_info.get("owner").and_then(|v| v.get("login")).and_then(|v| v.as_str()).unwrap().to_string(),
			name: repo_info.get("name").and_then(|v| v.as_str()).unwrap().to_string(),
			full_name: repo_info.get("full_name").and_then(|v| v.as_str()).unwrap().to_string(),
			description: repo_info.get("description").and_then(|v| v.as_str()).map(|s| s.to_string()),
			created_at: repo_info.get("created_at").and_then(|v| v.as_str()).unwrap().to_string().parse().unwrap(),
			updated_at: repo_info.get("updated_at").and_then(|v| v.as_str()).unwrap().to_string().parse().unwrap(),
			pushed_at: repo_info.get("pushed_at").and_then(|v| v.as_str()).unwrap().to_string().parse().unwrap(),
		}
	}
}

pub(crate) struct ContributionHtml(pub(crate) String);

impl From<String> for ContributionHtml {
	fn from(value: String) -> Self {
		ContributionHtml(value)
	}
}

impl From<ContributionHtml> for ContributionResult {
	fn from(value: ContributionHtml) -> Self {
		let html = value.0;
		let document = scraper::Html::parse_document(&html);

		let selector = Selector::parse("div.right-side div.box").unwrap();

		let contributions: Vec<Vec<ContributionData>> = document
			.select(&selector)
			.filter_map(|element| {
				let data_content = element.value().attr("data-content")?;
				let date_str = element.value().attr("date")?;

				let count = data_content.split('个').next()?.rsplit(':').next()?.parse::<u32>().unwrap_or(0);
				let date = NaiveDate::parse_from_str(date_str, "%Y%m%d").ok()?.and_hms_opt(0, 0, 0)?.and_local_timezone(Utc).unwrap();

				Some(ContributionData { date, count })
			})
			.sorted_by_key(|c| c.date)
			.chunk_by(|c| {
				let naive_date = c.date.naive_utc().date();
				naive_date.week(Weekday::Mon)
			})
			.into_iter()
			.map(|(_, chunk)| chunk.collect::<Vec<_>>())
			.collect();

		let total = contributions.iter().flatten().map(|c| c.count).sum();

		ContributionResult { total, contributions }
	}
}
