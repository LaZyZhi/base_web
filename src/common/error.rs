use salvo::http::{ParseError, StatusCode, StatusError};
use salvo::oapi::{self, EndpointOutRegister, ToSchema};
use salvo::prelude::*;
use salvo::Writer;
use thiserror::Error;

use crate::common::api_response::ApiResponse;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("public: `{0}`")]
    Public(String),
    #[error("internal: `{0}`")]
    Internal(String),
    #[error("salvo internal error: `{0}`")]
    Salvo(#[from] ::salvo::Error),
    #[error("http status error: `{0}`")]
    HttpStatus(#[from] StatusError),
    #[error("http parse error:`{0}`")]
    HttpParse(#[from] ParseError),
    #[error("anyhow error:`{0}`")]
    Anyhow(#[from] anyhow::Error),
    #[error("seaorm db error:`{0}`")]
    Seaorm(#[from] sea_orm::DbErr),
    #[error("validation error:`{0}`")]
    Validation(#[from] validator::ValidationErrors),
}

impl AppError {
    pub fn public<S: Into<String>>(msg: S) -> Self {
        Self::Public(msg.into())
    }

    pub fn internal<S: Into<String>>(msg: S) -> Self {
        Self::Internal(msg.into())
    }
}

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let (status_code, error_msg) = match &self {
            Self::HttpStatus(e) => (e.code, e.brief.clone()),
            Self::Public(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            Self::Internal(msg) => {
                tracing::error!(msg = msg, "internal error");
                (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
            }
            Self::Validation(_) => (StatusCode::BAD_REQUEST, "Validation error".to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error".to_string()),
        };
        
        res.status_code(status_code);
        
        // Create the error response in the required format
        let error_response: ApiResponse<String> = ApiResponse {
            code: status_code.as_u16(),
            msg: "fail".to_string(),
            data: error_msg,
        };
        
        res.render(Json(error_response));
    }
}

impl EndpointOutRegister for AppError {
    fn register(components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        // Register the ApiResponse schema for error responses
        let schema = ApiResponse::<String>::to_schema(components);
        let error_response = oapi::Response::new("Error response")
            .add_content("application/json", schema);
            
        operation.responses.insert(
            StatusCode::INTERNAL_SERVER_ERROR.as_str(),
            error_response.clone(),
        );
        operation.responses.insert(
            StatusCode::NOT_FOUND.as_str(),
            error_response.clone(),
        );
        operation.responses.insert(
            StatusCode::BAD_REQUEST.as_str(),
            error_response.clone(),
        );
        operation.responses.insert(
            StatusCode::UNAUTHORIZED.as_str(),
            error_response,
        );
    }
}