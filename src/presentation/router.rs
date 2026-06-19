use crate::domain::model::error::AppError;
use crate::presentation::handlers;
use crate::presentation::state::AppState;
use axum::{
    Router,
    body::Body,
    http::{HeaderValue, Method, Request, StatusCode, header},
    middleware,
    response::Response,
};
use std::collections::HashSet;
use std::sync::Arc;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::set_header::SetResponseHeaderLayer;

pub fn build_router(state: AppState) -> Result<Router, AppError> {
    let allowed_assets = state.allowed_assets.clone();

    let cors_origins: Vec<HeaderValue> = state
        .settings
        .cors_origins()
        .iter()
        .map(|o| {
            o.parse::<HeaderValue>()
                .map_err(|e| AppError::InvalidConfig(format!("Invalid CORS origin '{}': {}", o, e)))
        })
        .collect::<Result<Vec<HeaderValue>, AppError>>()?;

    let cors_layer = CorsLayer::new()
        .allow_origin(AllowOrigin::list(cors_origins))
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE]);

    let mut img_domains = state.settings.csp_origins();
    if state.settings.csp_allowed_origins.is_none() {
        img_domains.extend(state.allowed_external_origins.iter().cloned());
    }
    let img_domains_str = img_domains.join(" ");

    let csp_domains = state.settings.csp_origins().join(" ");
    let csp_value = format!(
        "default-src 'self'; style-src 'self' {}; font-src 'self' {}; script-src 'self' {}; img-src 'self' data: {};",
        csp_domains, csp_domains, csp_domains, img_domains_str
    );
    let csp_header_value = HeaderValue::from_str(&csp_value)
        .map_err(|e| AppError::InvalidConfig(format!("Invalid CSP header configuration: {}", e)))?;

    let router = Router::new()
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
        .layer(cors_layer)
        .layer(SetResponseHeaderLayer::overriding(
            header::CONTENT_SECURITY_POLICY,
            csp_header_value,
        ))
        .with_state(state);

    Ok(router)
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
