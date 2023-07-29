use std::fmt;

pub type ClientResult<T> = Result<T, ClientError>;

#[derive(Debug)]
pub enum ClientError {
    ErrorStatus(String),
    TitleNotFound,
    ThesesNotFound,
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ErrorStatus(status) => format!("Статус '{status}'"),
                Self::TitleNotFound => "Заголовок не найден".into(),
                Self::ThesesNotFound => "Пересказ не найден".into(),
            }
        )
    }
}
