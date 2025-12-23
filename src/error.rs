use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("获取字体存储锁失败")]
    LockFailed,
    #[error("无效的字体数据")]
    InvalidFontData,
    #[error("无法获取字体族名")]
    FamilyNameNotFound,
}
