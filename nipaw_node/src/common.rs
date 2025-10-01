use std::sync::{LazyLock, Mutex};

pub(crate) static RT_RUNTIME: LazyLock<Mutex<tokio::runtime::Runtime>> = LazyLock::new(|| Mutex::new(tokio::runtime::Runtime::new().unwrap()));
