use crate::presentation::handlers;
use crate::presentation::state::AppState;
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
    middleware,
    response::Response,
};
use std::collections::HashSet;
use std::sync::Arc;

pub fn build_router(state: AppState) -> Router {
    let allowed_assets = state.allowed_assets.clone();

    Router::new()
        .route("/", axum::routing::get(handlers::index::index_handler))
        .route(
            "/article/{slug}",
            axum::routing::get(handlers::article::article_handler),
        )
        .route(
            "/sitemap.xml",
            axum::routing::get(handlers::sitemap::sitemap_handler),
        )
        .route(
            "/robots.txt",
            axum::routing::get(handlers::robots::robots_handler),
        )
        .route(
            "/feed.xml",
            axum::routing::get(handlers::feed::feed_handler),
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
        .nest(
            "/assets",
            Router::new()
                .fallback_service(tower_http::services::ServeDir::new("assets"))
                .layer(middleware::from_fn(move |req, next| {
                    filter_allowed_assets(req, next, allowed_assets.clone())
                })),
        )
        .fallback(handlers::not_found::not_found_handler)
        .with_state(state)
}

async fn filter_allowed_assets(
    request: Request<Body>,
    next: middleware::Next,
    allowed_assets: Arc<HashSet<String>>,
) -> Result<Response, StatusCode> {
    let path = request.uri().path();
    let relative_path = path.strip_prefix('/').unwrap_or(path);

    if allowed_assets.contains(relative_path) {
        return Ok(next.run(request).await);
    }

    Err(StatusCode::FORBIDDEN)
}
