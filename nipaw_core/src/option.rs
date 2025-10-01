use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ReposListOptions {
	/// 每页数量，默认 30，最大 100
	#[serde(default = "default_per_page")]
	pub per_page: Option<u32>,
	/// 页码，默认 1
	#[serde(default = "default_page")]
	pub page: Option<u32>,
}

fn default_per_page() -> Option<u32> {
	Some(30)
}

fn default_page() -> Option<u32> {
	Some(1)
}
