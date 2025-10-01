use crate::middleware::AuthMiddleware;
use reqwest::{
	Client,
	header::{HeaderMap, HeaderName, HeaderValue},
};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use std::sync::{LazyLock, OnceLock};

pub(crate) static HTTP_CLIENT: LazyLock<ClientWithMiddleware> = LazyLock::new(|| {
	let mut headers = HeaderMap::new();
	headers.insert(HeaderName::from_static("x-github-api-version"), HeaderValue::from_static("2022-11-28"));
	headers.insert(HeaderName::from_static("accept"), HeaderValue::from_static("application/vnd.github+json"));
	headers.insert(HeaderName::from_static("user-agent"), HeaderValue::from_static("nipaw"));

	let proxy_url = PROXY_URL.get();
	if let Some(proxy_url) = proxy_url {
		let proxy = reqwest::Proxy::all(proxy_url).unwrap();
		let client = Client::builder().default_headers(headers).proxy(proxy).build().unwrap();
		return ClientBuilder::new(client).with(AuthMiddleware).build();
	}

	let client = Client::builder().default_headers(headers).build().unwrap();
	ClientBuilder::new(client).with(AuthMiddleware).build()
});

pub(crate) static PROXY_URL: OnceLock<String> = OnceLock::new();
