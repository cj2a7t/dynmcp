#[derive(Debug, Clone, Copy)]
pub enum DynMCPHttpStatus {
    Ok = 200,
    BadRequest = 400,
    Unauthorized = 401,
    NotFound = 404,
    InternalServerError = 500,
}

impl DynMCPHttpStatus {
    pub fn as_u16(self) -> u16 {
        self as u16
    }

    pub fn as_str(self) -> &'static str {
        match self {
            DynMCPHttpStatus::Ok => "200 OK",
            DynMCPHttpStatus::BadRequest => "400 Bad Request",
            DynMCPHttpStatus::Unauthorized => "401 Unauthorized",
            DynMCPHttpStatus::NotFound => "404 Not Found",
            DynMCPHttpStatus::InternalServerError => "500 Internal Server Error",
        }
    }
}
