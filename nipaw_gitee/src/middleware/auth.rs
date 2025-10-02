use async_trait::async_trait;
use http::Extensions;
use nipaw_core::CoreError;
use reqwest::{Request, Response, StatusCode};
use reqwest_middleware::{Error, Middleware, Next, Result};

pub struct AuthMiddleware;

#[async_trait]
impl Middleware for AuthMiddleware {
	async fn handle(
		&self,
		req: Request,
		extensions: &mut Extensions,
		next: Next<'_>,
	) -> Result<Response> {
		let res = next.run(req, extensions).await?;
		match res.status() {
			StatusCode::OK => Ok(res),
			StatusCode::UNAUTHORIZED => Err(Error::Middleware(CoreError::Unauthorized.into())),
			StatusCode::NOT_FOUND => Err(Error::Middleware(CoreError::NotFound.into())),
			StatusCode::FORBIDDEN | StatusCode::TOO_MANY_REQUESTS => {
				Err(Error::Middleware(CoreError::RateLimit.into()))
			}
			_ => Ok(res),
		}
	}
}
