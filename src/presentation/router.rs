use crate::presentation::handlers;
use crate::presentation::state::AppState;
use axum::Router;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/", axum::routing::get(handlers::index::index_handler))
        .route(
            "/article/{slug}",
            axum::routing::get(handlers::article::article_handler),
        )
        .route(
            "/thumbnails/thumbnail.webp",
            axum::routing::get_service(tower_http::services::ServeFile::new(
                "templates/thumbnails/thumbnail.webp",
            )),
        )
        .route(
            "/thumbnail",
            axum::routing::get(handlers::thumbnail::thumbnail_handler),
        )
        .nest_service("/static", tower_http::services::ServeDir::new("templates"))
        .with_state(state)
}
