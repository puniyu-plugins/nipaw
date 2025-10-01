use napi::tokio::runtime::Runtime;
use std::sync::{LazyLock, Mutex};
pub(crate) static RT_RUNTIME: LazyLock<Mutex<Runtime>> =
	LazyLock::new(|| Mutex::new(Runtime::new().unwrap()));
