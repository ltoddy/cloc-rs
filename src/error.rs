use std::fmt;
use std::io::Error;

#[derive(Debug)]
pub enum ClocError {
    // 未识别的文件, 可以识别的文件记录在`src/config.rs`的Config结构体中
    Unrecognized,
    // 非文本文件, 例如二进制文件不做统计
    NonTextFile,
    // Io异常
    Io(std::io::Error),
    // 运行时的命令行参数的输入了无效参数
    InvalidCommandArgs,
}

impl std::error::Error for ClocError {}

impl fmt::Display for ClocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO
        match self {
            ClocError::Unrecognized => write!(f, "Unrecognized"),
            ClocError::NonTextFile => write!(f, "NonTextFile"),
            ClocError::Io(_) => write!(f, "Io"),
            InvalidCommandArgs => write!(f, "Invalid command args"),
        }
    }
}

impl From<std::io::Error> for ClocError {
    fn from(e: Error) -> Self {
        ClocError::Io(e)
    }
}
