mod client;
mod common;
mod middleware;

use crate::client::{HTTP_CLIENT, PROXY_URL};
use crate::common::make_user_info;
use async_trait::async_trait;
pub use nipaw_core::Client;
use nipaw_core::CoreError;
use nipaw_core::types::user::UserInfo;
use serde_json::Value;

static BASE_URL: &str = "https://cnb.cool";
static API_URL: &str = "https://api.cnb.cool";

#[derive(Debug, Default)]
pub struct CnbClient {
	pub token: Option<String>,
}

impl CnbClient {
	pub fn new() -> Self {
		Self::default()
	}
}

#[async_trait]
impl Client for CnbClient {
	fn set_token(&mut self, token: String) -> Result<(), CoreError> {
		if token.is_empty() {
			return Err(CoreError::TokenEmpty);
		}
		self.token = Some(token);
		Ok(())
	}

	fn set_proxy(&mut self, proxy: String) -> Result<(), CoreError> {
		PROXY_URL.set(proxy).unwrap();
		Ok(())
	}

	#[inline]
	async fn get_user_info(&self) -> Result<UserInfo, CoreError> {
		if self.token.is_none() {
			return Err(CoreError::TokenEmpty);
		}
		let url = format!("{}/user", API_URL);
		let request = HTTP_CLIENT
			.get(url)
			.header("Authorization", format!("Bearer {}", self.token.as_ref().unwrap()));
		let resp = request.send().await?;
		let user_info: Value = resp.json().await?;
		Ok(make_user_info(user_info))
	}

	#[inline]
	async fn get_user_info_with_name(&self, user_name: String) -> Result<UserInfo, CoreError> {
		let url = format!("{}/users/{}", API_URL, user_name);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			request = request.header("Authorization", format!("Bearer {}", token));
		}
		let resp = request.send().await?;
		let user_info: Value = resp.json().await?;
		Ok(make_user_info(user_info))
	}
}
