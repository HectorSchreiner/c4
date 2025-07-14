use ::axum::{http::StatusCode, response::IntoResponse};
use ::axum::{Router, routing::get, Json};
use ::serde::Serialize;

use crate::domains::users::{User, Username};


pub fn users_router() -> Router {
    Router::new()
        .route("/users", get(list_users))

    }

async fn hello() -> &'static str {
    "Hello, Users"
}

#[axum::debug_handler]
pub async fn list_users() -> Result<Json<Vec<User>>, ApiError> {
    let user = User::new(Username::new("Hector_HackermanXx".to_string()))
        .map_err(|e| ApiError::InternalServerError(format!("Failed to create user: {e}")))?;
    
    let users = vec![user];
    Ok(Json(users))
}


#[derive(Serialize)]
struct Message {
    message: String
}

#[derive(Serialize)]
pub enum ApiResponse {
    Ok,
    Created,
    JsonData(Vec<Message>),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Ok => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response(),
        }
    }
}

#[derive(Serialize)]
pub enum ApiError {
    BadRequest(String),
    Forbidden(String),
    Unauthorised(String),
    InternalServerError(String)
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::BadRequest(error) => (StatusCode::BAD_REQUEST, error).into_response(),
            Self::Forbidden(error) => (StatusCode::FORBIDDEN, error).into_response(),
            Self::Unauthorised(error) => (StatusCode::UNAUTHORIZED, error).into_response(),
            Self::InternalServerError(error) => (StatusCode::INTERNAL_SERVER_ERROR, error).into_response(),
        }
    }
}
