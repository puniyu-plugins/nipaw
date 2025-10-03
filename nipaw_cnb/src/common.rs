use chrono::{NaiveDate, Utc, Weekday};
use itertools::Itertools;
use nipaw_core::types::{
	commit::{CommitData, CommitInfo, StatsInfo, UserInfo as CommitUserInfo},
	org::OrgInfo,
	repo::RepoInfo,
	user::{ContributionData, ContributionResult, UserInfo},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct JsonValue(pub(crate) Value);
impl From<JsonValue> for UserInfo {
	fn from(json_value: JsonValue) -> Self {
		let user_info = json_value.0;
		let login = user_info.get("username").and_then(|v| v.as_str()).unwrap();
		UserInfo {
			id: user_info.get("id").and_then(|v| v.as_str()).unwrap().to_string(),
			login: login.to_string(),
			name: user_info.get("nickname").and_then(|v| v.as_str()).map(|s| s.to_string()),
			avatar_url: user_info.get("avatar_url").and_then(|v| v.as_str()).unwrap().to_string(),
			email: user_info.get("email").and_then(|v| v.as_str()).map(|s| s.to_string()),
			followers: user_info.get("follower_count").and_then(|v| v.as_u64()).unwrap(),
			following: user_info.get("follow_count").and_then(|v| v.as_u64()).unwrap(),
			public_repo_count: user_info.get("repo_count").and_then(|v| v.as_u64()).unwrap_or(0),
		}
	}
}

impl From<JsonValue> for RepoInfo {
	fn from(json_value: JsonValue) -> Self {
		let repo_info = json_value.0;
		let is_public = repo_info
			.get("visibility_level")
			.and_then(|v| v.as_str())
			.map(|s| s.to_lowercase() == "public")
			.unwrap_or(false);
		RepoInfo {
			id: repo_info.get("id").and_then(|v| v.as_str()).unwrap().to_string(),
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
			visibility: repo_info
				.get("visibility_level")
				.and_then(|v| v.as_str())
				.map(|s| s.to_lowercase())
				.unwrap()
				.to_string(),
			fork: repo_info
				.get("forked_from_repo")
				.and_then(|v| v.get("path"))
				.and_then(|v| v.as_str())
				.is_some(),
			fork_count: repo_info.get("fork_count").and_then(|v| v.as_u64()).unwrap_or(0),
			public: is_public,
			private: !is_public,
			language: repo_info.get("language").and_then(|v| v.as_str()).map(|s| s.to_string()),
			star_count: repo_info.get("star_count").and_then(|v| v.as_u64()).unwrap_or(0),
			default_branch: repo_info
				.get("default_branch")
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string(),
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
				.get("updated_at")
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
			.map(|(date_str, data)| {
				let date = NaiveDate::parse_from_str(date_str, "%Y%m%d")
					.unwrap()
					.and_hms_opt(0, 0, 0)
					.unwrap()
					.and_local_timezone(Utc)
					.unwrap();
				let count = data.get("score").and_then(|v| v.as_u64()).unwrap_or(0) as u32;

				ContributionData { date, count }
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

		ContributionResult { total, contributions }
	}
}

impl From<JsonValue> for CommitInfo {
	fn from(value: JsonValue) -> Self {
		let commit_info = value.0;
		let commit_value = commit_info.get("commit").unwrap().clone();
		let stats_value = commit_info.get("stats").unwrap().clone();
		CommitInfo {
			sha: commit_info.get("sha").and_then(|v| v.as_str()).unwrap().to_string(),
			commit: JsonValue(commit_value).into(),
			stats: JsonValue(stats_value).into(),
		}
	}
}

impl From<JsonValue> for CommitData {
	fn from(value: JsonValue) -> Self {
		let commit_data = value.0;
		let author_value = commit_data.get("author").unwrap().clone();
		let committer_value = commit_data.get("committer").unwrap().clone();
		CommitData {
			author: JsonValue(author_value).into(),
			committer: JsonValue(committer_value).into(),
			message: commit_data.get("message").and_then(|v| v.as_str()).unwrap().to_string(),
		}
	}
}

impl From<JsonValue> for CommitUserInfo {
	fn from(value: JsonValue) -> Self {
		let user_info = value.0;
		CommitUserInfo {
			name: user_info.get("name").and_then(|v| v.as_str()).unwrap().to_string(),
			email: user_info.get("email").and_then(|v| v.as_str()).map(|s| s.to_string()),
			avatar_url: user_info.get("avatar_url").and_then(|v| v.as_str()).unwrap().to_string(),
			date: user_info
				.get("date")
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string()
				.parse()
				.unwrap(),
		}
	}
}

impl From<JsonValue> for StatsInfo {
	fn from(value: JsonValue) -> Self {
		let stats_info = value.0;
		StatsInfo {
			total: stats_info.get("total").and_then(|v| v.as_u64()).unwrap_or(0),
			additions: stats_info.get("additions").and_then(|v| v.as_u64()).unwrap_or(0),
			deletions: stats_info.get("deletions").and_then(|v| v.as_u64()).unwrap_or(0),
		}
	}
}

impl From<JsonValue> for OrgInfo {
	fn from(json_value: JsonValue) -> Self {
		let org_info = json_value.0;
		OrgInfo {
			id: org_info.get("id").and_then(|v| v.as_u64()).unwrap(),
			login: org_info.get("login").and_then(|v| v.as_str()).unwrap().to_string(),
			name: org_info.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()),
			email: org_info.get("email").and_then(|v| v.as_str()).map(|s| s.to_string()),
			avatar_url: org_info.get("avatar_url").and_then(|v| v.as_str()).unwrap().to_string(),
			description: org_info
				.get("description")
				.and_then(|v| v.as_str())
				.map(|s| s.to_string()),
			follow_count: org_info.get("followers").and_then(|v| v.as_u64()).unwrap_or(0),
		}
	}
}
