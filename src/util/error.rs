use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ErrorKind {
    BasicError(String),
    ValidationError(String),
    ServerError(String),
    Hint(String),
    OtherError(String),
}

impl<'a> Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::BasicError(msg) => f.write_str(msg),
            ErrorKind::ValidationError(msg) => {
                f.write_fmt(format_args!("{}", msg))
            }
            ErrorKind::ServerError(msg) => f.write_str(msg),
            ErrorKind::Hint(msg) => f.write_str(msg),
            ErrorKind::OtherError(msg) => f.write_fmt(format_args!("other error: {}", msg)),
        }
    }
}

impl std::error::Error for ErrorKind {}

impl From<regex::Error> for ErrorKind {
    fn from(err: regex::Error) -> Self {
        ErrorKind::OtherError(err.to_string())
    }
}

impl From<serde_json::Error> for ErrorKind {
    fn from(err: serde_json::Error) -> Self {
        ErrorKind::OtherError(err.to_string())
    }
}

impl From<gloo_net::Error> for ErrorKind {
    fn from(err: gloo_net::Error) -> Self {
        ErrorKind::OtherError(err.to_string())
    }
}

pub trait ToError {
    fn to_basic_error(&self) -> ErrorKind;
    fn to_validation_error(&self) -> ErrorKind;
    fn to_server_error(&self) -> ErrorKind;
    fn to_hint(&self) -> ErrorKind;
}

impl<T> ToError for T
where
    T: AsRef<str>,
{
    fn to_basic_error(&self) -> ErrorKind {
        ErrorKind::BasicError(self.as_ref().to_string())
    }

    fn to_validation_error(&self) -> ErrorKind {
        ErrorKind::ValidationError(self.as_ref().to_string())
    }

    fn to_server_error(&self) -> ErrorKind {
        ErrorKind::ServerError(self.as_ref().to_string())
    }
    fn to_hint(&self) -> ErrorKind {
        ErrorKind::Hint(self.as_ref().to_string())
    }
}
