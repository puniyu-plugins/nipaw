use napi_derive::napi;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[napi(object)]
pub struct ReposListOptions {
	/// 每页数量，默认 30，最大 100
	pub per_page: Option<u32>,
	/// 页码，默认 1
	pub page: Option<u32>,
}

impl From<ReposListOptions> for nipaw_core::option::ReposListOptions {
	fn from(value: ReposListOptions) -> Self {
		nipaw_core::option::ReposListOptions { per_page: value.per_page, page: value.page }
	}
}
