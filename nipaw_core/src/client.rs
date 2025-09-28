use crate::error::CoreError;
use crate::types::user::UserInfo;
use async_trait::async_trait;

#[async_trait]
pub trait Client: Send + Sync {
	fn set_token(&mut self, token: String) -> Result<(), CoreError>;

	fn set_proxy(&mut self, proxy: String) -> Result<(), CoreError>;

	async fn get_user_info(&self) -> Result<UserInfo, CoreError>;
	async fn get_user_info_with_name(&self, user_name: String) -> Result<UserInfo, CoreError>;
}
