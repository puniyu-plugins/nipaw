use crate::common::{make_repo_info, make_user_info};
use crate::types::{repo::RepoInfo, user::UserInfo};
use napi::tokio::sync::RwLock;
use napi_derive::napi;
use nipaw_core::Client;
use std::sync::LazyLock;

static GITCODE_CLIENT: LazyLock<RwLock<nipaw_gitcode::GitCodeClient>> =
	LazyLock::new(|| RwLock::new(nipaw_gitcode::GitCodeClient::default()));

#[derive(Debug, Default)]
#[napi]
pub struct GitCodeClient;

#[napi]
impl GitCodeClient {
	#[napi]
	pub fn set_token(&mut self, token: String) -> napi::Result<()> {
		let rt = tokio::runtime::Runtime::new()?;
		rt.block_on(async {
			let mut client = GITCODE_CLIENT.write().await;
			client
				.set_token(token.as_str())
				.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
			Ok(())
		})
	}

	#[napi]
	pub async fn get_user_info(&self) -> napi::Result<UserInfo> {
		let client = GITCODE_CLIENT.read().await;
		let user_info = client
			.get_user_info()
			.await
			.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(make_user_info(user_info))
	}

	#[napi]
	pub async fn get_user_info_with_name(&self, name: String) -> napi::Result<UserInfo> {
		let client = GITCODE_CLIENT.read().await;
		let user_info = client
			.get_user_info_with_name(name.as_str())
			.await
			.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(make_user_info(user_info))
	}

	#[napi]
	pub async fn get_repo_info(&self, owner: String, repo: String) -> napi::Result<RepoInfo> {
		let client = GITCODE_CLIENT.read().await;
		let repo_info = client
			.get_repo_info((owner.as_str(), repo.as_str()))
			.await
			.map_err(|e| napi::Error::from_reason(format!("{:?}", e)))?;
		Ok(make_repo_info(repo_info))
	}
}
