use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
	#[error("Core error: {0}")]
	Core(#[from] nipaw_core::Error),
	#[error("Node error: {0}")]
	Node(#[from] napi::Error),
}

impl From<Error> for napi::Error {
	fn from(error: Error) -> Self {
		napi::Error::from_reason(error.to_string())
	}
}

impl From<Error> for napi::JsError {
	fn from(err: Error) -> Self {
		napi::Error::from_reason(err.to_string()).into()
	}
}
