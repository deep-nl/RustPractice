#[derive(thiserror::Error,Debug)]
// 常规的错误处理机制
// https://juejin.cn/post/7097023377864392718#heading-9
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),
    #[error(transparent)]
    IO(#[from] std::io::Error)
}

pub type Result<T> = core::result::Result<T,Error>;