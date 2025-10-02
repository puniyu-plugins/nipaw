use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("set token is empty")]
	TokenEmpty,
	#[error("request error: {0}")]
	RequestError(#[from] reqwest::Error),
	#[error("middleware error: {0}")]
	MiddlewareError(#[from] reqwest_middleware::Error),
	#[error("url parse error: {0}")]
	URLParseError(#[from] url::ParseError),
	#[error("not found")]
	NotFound,
	#[error("unauthorized")]
	Unauthorized,
	#[error("rate limit")]
	RateLimit,
}
