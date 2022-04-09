use actix_web::HttpResponse;

#[derive(Debug)]
pub struct TosDirectoryError {
    err: anyhow::Error,
}

impl std::fmt::Display for TosDirectoryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(formatter, "{}", self.err)
    }
}

impl actix_web::error::ResponseError for TosDirectoryError {}

impl From<anyhow::Error> for TosDirectoryError {
    fn from(err: anyhow::Error) -> TosDirectoryError {
        TosDirectoryError { err }
    }
}

pub type TosDirectoryResult = Result<HttpResponse, TosDirectoryError>;
