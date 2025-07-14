use axum::Router;

pub mod users;

pub fn init_router() -> Router {
    Router::new()
        .merge(users::users_router())
}