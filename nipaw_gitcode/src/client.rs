use crate::middleware::AuthMiddleware;
use reqwest::{
	Client,
	header::{HeaderMap, HeaderName, HeaderValue},
};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use std::sync::{LazyLock, OnceLock};

pub(crate) static HTTP_CLIENT: LazyLock<ClientWithMiddleware> = LazyLock::new(|| {
	let mut headers = HeaderMap::new();
	headers.insert(HeaderName::from_static("accept"), HeaderValue::from_static("application/json"));
	headers.insert(HeaderName::from_static("user-agent"), HeaderValue::from_static("nipaw"));

	let client = Client::builder().default_headers(headers).build().unwrap();
	ClientBuilder::new(client).with(AuthMiddleware).build()
});

pub(crate) static PROXY_URL: OnceLock<String> = OnceLock::new();
