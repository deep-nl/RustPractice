#[derive(thiserror::Error,Debug)]
// 常规的错误处理机制
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),
    #[error(transparent)]
    IO(#[from] std::io::Error)
}

pub type Result<T> = core::result::Result<T,Error>;