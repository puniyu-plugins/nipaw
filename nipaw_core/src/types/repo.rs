use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoInfo {
	pub id: String,
	pub owner: String,
	pub name: String,
	pub full_name: String,
	pub description: Option<String>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
	pub pushed_at: DateTime<Utc>,
}